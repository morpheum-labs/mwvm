//! # MWVM SDK
//!
//! **High-level, ergonomic Rust SDK** for running Morpheum AI agents locally or
//! connected to the L1.
//!
//! This is the crate developers import: `mwvm_sdk::Agent`.
//! It provides a clean, thin facade over `mwvm-core` while adding convenient
//! configuration, error handling, and a strong prelude.
//!
//! **Design** (SOLID + Rust best practices):
//! - Thin facade pattern (no logic duplication)
//! - Strong prelude for ergonomic imports
//! - Re-exports from core (DRY guarantee)
//! - SDK-specific error type for user-friendly messages
//! - Feature-aware and fully documented

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

pub mod agent;
pub mod config;
pub mod runtime;

// ─────────────────────────────────────────────────────────────────────────────
// Re-exports from mwvm-core (DRY — single source of truth)
// ─────────────────────────────────────────────────────────────────────────────

// Allow downstream crates to reach into core if needed.
pub use mwvm_core;
pub use mwvm_core::{EngineBuilder, LocalMemory, MwvmEngine, SimulationMode, Simulator};

// Public high-level API types
pub use agent::{Agent, AgentBuilder};
pub use config::SdkConfig;
pub use runtime::SdkRuntime;

/// Convenient public Result type (SDK users see clean errors).
pub type Result<T> = std::result::Result<T, SdkError>;

// =============================================================================
// SDK-specific Error (user-friendly wrapper)
// =============================================================================

/// All errors surfaced by the MWVM SDK.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum SdkError {
    /// Errors coming from the core engine.
    #[error("core engine error: {0}")]
    Core(#[from] mwvm_core::MwvmError),

    /// Configuration / builder errors.
    #[error("configuration error: {0}")]
    Config(String),

    /// Agent execution / runtime errors.
    #[error("agent execution error: {0:#}")]
    Execution(#[from] anyhow::Error),
}

// =============================================================================
// Public Prelude (recommended for users)
// =============================================================================

/// Convenient prelude — bring everything you need into scope with:
///
/// ```rust
/// use mwvm_sdk::prelude::*;
/// ```
pub mod prelude {
    pub use super::{
        Agent, AgentBuilder, EngineBuilder, LocalMemory, MwvmEngine, Result, SdkConfig, SdkError,
        SdkRuntime, SimulationMode, Simulator,
    };

    // Re-export core primitives and common utilities
    pub use morpheum_primitives::vm::types::{InferenceRequest, TeeAttestation, ZkmlProof};
    pub use tracing::{debug, error, info, warn};
}
