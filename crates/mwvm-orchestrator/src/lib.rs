//! # MWVM Orchestrator
//!
//! **Production-grade multi-agent swarm runtime** for MWVM.
//!
//! Provides the high-level [`Swarm`] type for spawning, coordinating, and
//! messaging thousands of agents with shared memory and deterministic
//! simulation support.

#![forbid(unsafe_code)]
#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    missing_docs,
    rustdoc::missing_crate_level_docs,
    future_incompatible,
    rust_2018_idioms
)]
#![cfg_attr(docsrs, feature(doc_auto_cfg, doc_cfg))]

pub mod message_bus;
pub mod swarm;

// ── Re-exports from mwvm-sdk (DRY) ──────────────────────────────────────────

pub use mwvm_sdk::{EngineBuilder, MwvmEngine, SdkConfig, SimulationMode};

// Public high-level types
pub use message_bus::MessageBus;
pub use swarm::{Swarm, SwarmBuilder};

/// Convenient public Result type for orchestrator users.
pub type Result<T> = std::result::Result<T, OrchestratorError>;

// =============================================================================
// Orchestrator-specific Error
// =============================================================================

/// All errors surfaced by the MWVM Orchestrator.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum OrchestratorError {
    /// Propagated SDK error.
    #[error("sdk error: {0}")]
    Sdk(#[from] mwvm_sdk::SdkError),

    /// Internal orchestration failure.
    #[error("orchestration error: {0:#}")]
    Internal(#[from] anyhow::Error),

    /// Configuration error.
    #[error("configuration error: {0}")]
    Config(String),
}

// =============================================================================
// Prelude
// =============================================================================

/// Convenient prelude — `use mwvm_orchestrator::prelude::*;`
pub mod prelude {
    pub use super::{
        EngineBuilder, MessageBus, MwvmEngine, OrchestratorError, Result, SdkConfig,
        SimulationMode, Swarm, SwarmBuilder,
    };

    pub use mwvm_sdk::prelude::*;
    pub use tracing::{debug, error, info, warn};
}
