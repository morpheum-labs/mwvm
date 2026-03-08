//! `LocalMemory` — Persistent agent memory with brute-force vector search.
//!
//! Thread-safe concurrent key-value store with cosine-similarity vector search.
//! Uses `DashMap` for lock-free KV access and a simple but correct brute-force
//! ANN implementation suitable for local development workloads.
//!
//! Implements [`morpheum_primitives::vm::MemoryBackend`] — the shared contract
//! between MWVM (off-chain) and Mormcore (on-chain) persistent memory backends.

use std::sync::{
    atomic::{AtomicU64, Ordering},
    Arc, RwLock,
};

use async_trait::async_trait;
use dashmap::DashMap;
use tracing::debug;

use morpheum_primitives::agent::MemoryEntry;
use morpheum_primitives::errors::PrimitivesError;
use morpheum_primitives::vm::MemoryBackend;

/// Errors specific to the memory subsystem.
#[derive(Debug, thiserror::Error)]
pub enum MemoryError {
    /// Key exceeds the 2 KiB maximum size.
    #[error("key too large (max 2 KiB)")]
    KeyTooLarge,

    /// The supplied embedding dimension does not match the index.
    #[error("vector dimension mismatch: expected {expected}, got {actual}")]
    DimensionMismatch {
        /// Dimension the index was created with.
        expected: usize,
        /// Dimension of the vector that was supplied.
        actual: usize,
    },
}

/// A stored vector with its metadata.
#[derive(Clone, Debug)]
struct StoredVector {
    id: u64,
    embedding: Vec<f32>,
    /// Blake3 hash of the embedding bytes (computed once at insert time).
    content_hash: [u8; 32],
}

/// Search result returned by [`LocalMemory::search`].
#[derive(Clone, Debug)]
pub struct SearchResult {
    /// Stable monotonic identifier for this entry.
    pub id: u64,
    /// Cosine similarity score (higher is better, max 1.0).
    pub score: f32,
}

/// Thread-safe persistent memory with KV storage and vector search.
#[derive(Clone)]
pub struct LocalMemory {
    kv: Arc<DashMap<[u8; 32], Vec<u8>>>,
    vectors: Arc<RwLock<Vec<StoredVector>>>,
    next_id: Arc<AtomicU64>,
    vector_dim: usize,
}

impl LocalMemory {
    /// Create a new repository with a default embedding dimension of 1536.
    #[must_use]
    pub fn new() -> Self {
        Self::with_dimension(1536)
    }

    /// Create with a custom vector dimension.
    #[must_use]
    pub fn with_dimension(vector_dim: usize) -> Self {
        Self {
            kv: Arc::new(DashMap::new()),
            vectors: Arc::new(RwLock::new(Vec::new())),
            next_id: Arc::new(AtomicU64::new(1)),
            vector_dim,
        }
    }

    /// Deterministic key hash using blake3.
    fn hash_key(key: &[u8]) -> [u8; 32] {
        blake3::hash(key).into()
    }

    // ── KV Operations ──────────────────────────────────────────────────

    /// Load a value by key. Returns `None` if absent.
    ///
    /// # Errors
    ///
    /// Returns [`MemoryError::KeyTooLarge`] if the key exceeds 2 KiB.
    pub fn load(&self, key: &[u8]) -> Result<Option<Vec<u8>>, MemoryError> {
        if key.len() > 2048 {
            return Err(MemoryError::KeyTooLarge);
        }
        let hash = Self::hash_key(key);
        Ok(self.kv.get(&hash).map(|v| v.value().clone()))
    }

    /// Store (upsert) a value under `key`.
    ///
    /// # Errors
    ///
    /// Returns [`MemoryError::KeyTooLarge`] if the key exceeds 2 KiB.
    pub fn store(&self, key: &[u8], value: Vec<u8>) -> Result<(), MemoryError> {
        if key.len() > 2048 {
            return Err(MemoryError::KeyTooLarge);
        }
        let hash = Self::hash_key(key);
        let len = value.len();
        self.kv.insert(hash, value);
        debug!(key_hash = %hex::encode(&hash[..8]), size = len, "stored blob");
        Ok(())
    }

    // ── Vector Operations ──────────────────────────────────────────────

    /// Insert an embedding vector. Returns the assigned stable ID.
    ///
    /// # Errors
    ///
    /// Returns [`MemoryError::DimensionMismatch`] if the vector's length differs
    /// from the index dimension.
    ///
    /// # Panics
    ///
    /// Panics if the internal vector lock is poisoned.
    pub fn insert_vector(&self, embedding: Vec<f32>) -> Result<u64, MemoryError> {
        if embedding.len() != self.vector_dim {
            return Err(MemoryError::DimensionMismatch {
                expected: self.vector_dim,
                actual: embedding.len(),
            });
        }
        let id = self.next_id.fetch_add(1, Ordering::Relaxed);
        let content_hash: [u8; 32] =
            blake3::hash(bytemuck::cast_slice(&embedding)).into();
        let entry = StoredVector {
            id,
            embedding,
            content_hash,
        };
        self.vectors
            .write()
            .expect("vector lock poisoned")
            .push(entry);
        Ok(id)
    }

    /// Brute-force cosine similarity search. Returns up to `k` results sorted
    /// by descending similarity score.
    ///
    /// # Panics
    ///
    /// Panics if the internal vector lock is poisoned.
    #[must_use]
    pub fn search(&self, query: &[f32], k: usize) -> Vec<SearchResult> {
        self.search_scored(query, k)
            .into_iter()
            .map(|(score, id, _)| SearchResult { id, score })
            .collect()
    }

    /// Returns the configured embedding dimension.
    #[must_use]
    pub const fn vector_dim(&self) -> usize {
        self.vector_dim
    }

    /// Core scored search — returns `(score, id, content_hash)` tuples.
    ///
    /// Shared by the inherent [`search`](Self::search) and the
    /// [`MemoryBackend::search`] implementation.
    ///
    /// # Panics
    ///
    /// Panics if the internal vector lock is poisoned.
    fn search_scored(&self, query: &[f32], k: usize) -> Vec<(f32, u64, [u8; 32])> {
        if query.len() != self.vector_dim || k == 0 {
            return Vec::new();
        }

        let query_norm = dot(query, query).sqrt();
        if query_norm == 0.0 {
            return Vec::new();
        }

        let vectors = self.vectors.read().expect("vector lock poisoned");
        let mut scored: Vec<(f32, u64, [u8; 32])> = vectors
            .iter()
            .map(|v| {
                let v_norm = dot(&v.embedding, &v.embedding).sqrt();
                let similarity = if v_norm == 0.0 {
                    0.0
                } else {
                    dot(query, &v.embedding) / (query_norm * v_norm)
                };
                (similarity, v.id, v.content_hash)
            })
            .collect();
        drop(vectors);

        scored.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));
        scored.truncate(k);
        scored
    }
}

impl Default for LocalMemory {
    fn default() -> Self {
        Self::new()
    }
}

// ─── MemoryBackend trait implementation (DRY contract with Mormcore) ────────

#[async_trait]
impl MemoryBackend for LocalMemory {
    async fn load(&self, key: &[u8]) -> morpheum_primitives::errors::Result<Option<Vec<u8>>> {
        self.load(key)
            .map_err(|e| PrimitivesError::InvalidMemoryEntry(e.to_string()))
    }

    async fn store(&self, key: &[u8], value: Vec<u8>) -> morpheum_primitives::errors::Result<()> {
        self.store(key, value)
            .map_err(|e| PrimitivesError::InvalidMemoryEntry(e.to_string()))
    }

    fn search(&self, query: &[f32], k: u32) -> Vec<(f32, MemoryEntry)> {
        self.search_scored(query, k as usize)
            .into_iter()
            .map(|(score, _id, content_hash)| {
                let entry = MemoryEntry {
                    content_hash,
                    agent_hash: [0; 32],
                    timestamp: 0,
                    memory_type: 0,
                    _pad: [0; 7],
                };
                (score, entry)
            })
            .collect()
    }
}

/// Fast dot product.
fn dot(a: &[f32], b: &[f32]) -> f32 {
    a.iter().zip(b.iter()).map(|(x, y)| x * y).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kv_roundtrip() {
        let mem = LocalMemory::new();
        mem.store(b"hello", b"world".to_vec()).unwrap();
        let val = mem.load(b"hello").unwrap();
        assert_eq!(val, Some(b"world".to_vec()));
    }

    #[test]
    fn kv_absent_key() {
        let mem = LocalMemory::new();
        assert_eq!(mem.load(b"nope").unwrap(), None);
    }

    #[test]
    fn vector_search_basic() {
        let mem = LocalMemory::with_dimension(3);
        mem.insert_vector(vec![1.0, 0.0, 0.0]).unwrap();
        mem.insert_vector(vec![0.0, 1.0, 0.0]).unwrap();
        mem.insert_vector(vec![0.9, 0.1, 0.0]).unwrap();

        let results = mem.search(&[1.0, 0.0, 0.0], 2);
        assert_eq!(results.len(), 2);
        // First result should be the exact match
        assert!((results[0].score - 1.0).abs() < 1e-5);
    }

    #[test]
    fn vector_dim_mismatch() {
        let mem = LocalMemory::with_dimension(3);
        assert!(mem.insert_vector(vec![1.0, 2.0]).is_err());
    }

    // ── MemoryBackend trait tests ──────────────────────────────────────

    #[tokio::test]
    async fn memory_backend_kv_roundtrip() {
        let mem = LocalMemory::new();
        MemoryBackend::store(&mem, b"key1", b"value1".to_vec())
            .await
            .unwrap();
        let val = MemoryBackend::load(&mem, b"key1").await.unwrap();
        assert_eq!(val, Some(b"value1".to_vec()));
    }

    #[tokio::test]
    async fn memory_backend_kv_absent() {
        let mem = LocalMemory::new();
        let val = MemoryBackend::load(&mem, b"missing").await.unwrap();
        assert_eq!(val, None);
    }

    #[test]
    fn memory_backend_search_returns_memory_entry() {
        let mem = LocalMemory::with_dimension(3);
        mem.insert_vector(vec![1.0, 0.0, 0.0]).unwrap();
        mem.insert_vector(vec![0.9, 0.1, 0.0]).unwrap();

        let results = MemoryBackend::search(&mem, &[1.0, 0.0, 0.0], 2);
        assert_eq!(results.len(), 2);
        // Score should be close to 1.0 for the exact match
        assert!((results[0].0 - 1.0).abs() < 1e-5);
        // content_hash should be non-zero (computed from embedding bytes)
        assert!(results[0].1.content_hash.iter().any(|&b| b != 0));
    }

    #[test]
    fn local_memory_is_send_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<LocalMemory>();
    }
}
