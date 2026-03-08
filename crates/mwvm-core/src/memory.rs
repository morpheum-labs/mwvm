//! `LocalMemory` — Persistent agent memory with brute-force vector search.
//!
//! Thread-safe concurrent key-value store with cosine-similarity vector search.
//! Uses `DashMap` for lock-free KV access and a simple but correct brute-force
//! ANN implementation suitable for local development workloads.

use std::sync::{
    atomic::{AtomicU64, Ordering},
    Arc, RwLock,
};

use dashmap::DashMap;
use tracing::debug;

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
        let entry = StoredVector { id, embedding };
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
        if query.len() != self.vector_dim || k == 0 {
            return Vec::new();
        }

        let query_norm = dot(query, query).sqrt();
        if query_norm == 0.0 {
            return Vec::new();
        }

        let vectors = self.vectors.read().expect("vector lock poisoned");
        let mut scored: Vec<SearchResult> = vectors
            .iter()
            .map(|v| {
                let v_norm = dot(&v.embedding, &v.embedding).sqrt();
                let similarity = if v_norm == 0.0 {
                    0.0
                } else {
                    dot(query, &v.embedding) / (query_norm * v_norm)
                };
                SearchResult {
                    id: v.id,
                    score: similarity,
                }
            })
            .collect();
        drop(vectors);

        scored.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
        scored.truncate(k);
        scored
    }

    /// Returns the configured embedding dimension.
    #[must_use]
    pub const fn vector_dim(&self) -> usize {
        self.vector_dim
    }
}

impl Default for LocalMemory {
    fn default() -> Self {
        Self::new()
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
}
