//! Simulation layer — Fork, Replay, and Offline execution modes.
//!
//! Provides deterministic fork / restore, replay tracing, and pure-offline
//! execution for developer workflows.

use std::sync::{
    atomic::{AtomicU64, Ordering},
    Arc,
};

use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;
use tracing::debug;

use morpheum_primitives::vm::types::InferenceRequest;

use crate::engine::MwvmEngine;
use crate::memory::LocalMemory;
use crate::{MwvmError, Result};

/// Strategy enum for simulation mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum SimulationMode {
    /// Pure local execution — no external calls.
    Offline,
    /// Fork the current state for isolated what-if scenarios.
    Fork,
    /// Record inputs/outputs for deterministic replay.
    Replay,
}

/// A recorded inference step for deterministic replay.
#[derive(Debug, Clone, Serialize, Deserialize)]
struct ReplayEntry {
    request: InferenceRequest,
    response: Vec<u8>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
struct ReplayTrace {
    entries: Vec<ReplayEntry>,
}

/// Simulation context wrapping a [`MwvmEngine`].
#[derive(Clone)]
pub struct Simulator {
    engine: MwvmEngine,
    mode: SimulationMode,
    forks: Arc<DashMap<u64, LocalMemory>>,
    replay_trace: Arc<Mutex<ReplayTrace>>,
    next_fork_id: Arc<AtomicU64>,
}

impl Simulator {
    /// Create a new simulator wrapping an existing engine.
    #[must_use]
    pub fn new(engine: MwvmEngine, mode: SimulationMode) -> Self {
        Self {
            engine,
            mode,
            forks: Arc::new(DashMap::new()),
            replay_trace: Arc::new(Mutex::new(ReplayTrace::default())),
            next_fork_id: Arc::new(AtomicU64::new(1)),
        }
    }

    /// The underlying engine.
    #[must_use]
    pub const fn engine(&self) -> &MwvmEngine {
        &self.engine
    }

    /// Current simulation mode.
    #[must_use]
    pub const fn mode(&self) -> SimulationMode {
        self.mode
    }

    // ── Fork ────────────────────────────────────────────────────────────

    /// Snapshot the current memory state. Returns a fork ID.
    ///
    /// # Errors
    ///
    /// Returns [`MwvmError::SimulationModeNotSupported`] when the simulator
    /// is not in `Fork` mode.
    pub fn fork(&self) -> Result<u64> {
        if self.mode != SimulationMode::Fork {
            return Err(MwvmError::SimulationModeNotSupported {
                mode: "Fork".into(),
            });
        }
        let snapshot = self.engine.memory().clone();
        let id = self.next_fork_id.fetch_add(1, Ordering::Relaxed);
        self.forks.insert(id, snapshot);
        debug!(fork_id = id, "fork created");
        Ok(id)
    }

    /// Check whether a fork exists.
    #[must_use]
    pub fn fork_exists(&self, fork_id: u64) -> bool {
        self.forks.contains_key(&fork_id)
    }

    // ── Replay ──────────────────────────────────────────────────────────

    /// Clear and start a new replay recording.
    ///
    /// # Errors
    ///
    /// Returns [`MwvmError::SimulationModeNotSupported`] when the simulator
    /// is not in `Replay` mode.
    pub async fn start_recording(&self) -> Result<()> {
        if self.mode != SimulationMode::Replay {
            return Err(MwvmError::SimulationModeNotSupported {
                mode: "Replay".into(),
            });
        }
        self.replay_trace.lock().await.entries.clear();
        debug!("replay recording started");
        Ok(())
    }

    /// Record an inference step (called internally by the infer host).
    ///
    /// Not yet wired from the host dispatch layer — will be connected once
    /// the full replay pipeline is integrated.
    #[allow(dead_code)]
    pub(crate) async fn record_inference(&self, req: InferenceRequest, response: Vec<u8>) {
        if self.mode == SimulationMode::Replay {
            self.replay_trace
                .lock()
                .await
                .entries
                .push(ReplayEntry {
                    request: req,
                    response,
                });
        }
    }

    /// Replay the recorded trace deterministically.
    ///
    /// # Errors
    ///
    /// Returns [`MwvmError::SimulationModeNotSupported`] when the simulator
    /// is not in `Replay` mode.
    pub async fn replay(&self) -> Result<Vec<Vec<u8>>> {
        if self.mode != SimulationMode::Replay {
            return Err(MwvmError::SimulationModeNotSupported {
                mode: "Replay".into(),
            });
        }
        let results: Vec<Vec<u8>> = self
            .replay_trace
            .lock()
            .await
            .entries
            .iter()
            .map(|e| e.response.clone())
            .collect();
        debug!(steps = results.len(), "replayed trace");
        Ok(results)
    }

    // ── Offline ─────────────────────────────────────────────────────────

    /// Returns `true` when the simulator is in pure offline mode.
    #[must_use]
    pub fn is_offline(&self) -> bool {
        self.mode == SimulationMode::Offline
    }
}
