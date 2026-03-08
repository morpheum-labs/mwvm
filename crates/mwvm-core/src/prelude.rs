//! Convenient prelude for `mwvm-core` users.
//!
//! Bring everything you need into scope with:
//! ```rust
//! use mwvm_core::prelude::*;
//! ```

// Domain types (shared with Mormcore — DRY)
pub use morpheum_primitives::vm::types::{InferenceRequest, TeeAttestation, ZkmlProof};

// Core public API
pub use crate::{EngineBuilder, LocalMemory, MwvmEngine, MwvmError, Result};

// Memory & batching (core concerns)
pub use crate::batcher::ContinuousBatcher;

// Simulation & debugging (off-chain only)
pub use crate::simulation::{SimulationMode, Simulator};

// Re-export common traits for host extension
pub use morpheum_primitives::traits::Validatable;

// Common utilities
pub use tracing::{debug, error, info, warn};
