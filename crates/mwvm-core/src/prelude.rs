//! Convenient prelude for `mwvm-core` users.
//!
//! Bring everything you need into scope with:
//! ```rust
//! use mwvm_core::prelude::*;
//! ```

// Domain types (shared with Mormcore — DRY)
pub use morpheum_primitives::vm::types::{InferenceRequest, TeeAttestation, ZkmlProof};

// Shared VM traits (host contracts — DRY with Mormcore)
pub use morpheum_primitives::vm::traits::{MemoryBackend, VmHost};

// Canonical host function names (DRY with Mormcore)
pub use morpheum_primitives::vm::opcodes::{
    ALL_HOST_FUNCTIONS, HOST_ACTOR_SEND, HOST_INFER, HOST_NAMESPACE, HOST_STORE_CONTEXT,
    HOST_TEE_VERIFY, HOST_VECTOR_SEARCH, HOST_ZKML_VERIFY,
};

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
