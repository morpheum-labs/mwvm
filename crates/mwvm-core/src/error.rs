//! Unified error type for the MWVM core engine.

use morpheum_primitives::errors::PrimitivesError;
use thiserror::Error;

/// Unified error type for MWVM Core.
#[derive(Error, Debug)]
#[non_exhaustive]
pub enum MwvmError {
    /// Domain / primitives layer validation failure.
    #[error("primitives validation failed: {0}")]
    Primitives(#[from] PrimitivesError),

    /// WASM engine or linker error (wraps wasmtime's `anyhow::Error`).
    #[error("wasm engine error: {0:#}")]
    Wasm(anyhow::Error),

    /// Host function registration failed.
    #[error("host function '{name}' registration failed: {source:#}")]
    HostRegistration {
        /// Name of the host function that failed to register.
        name: &'static str,
        /// Underlying error from wasmtime.
        source: anyhow::Error,
    },

    /// Inference host call failed.
    #[error("inference error: {0:#}")]
    Inference(anyhow::Error),

    /// zkML verification failed.
    #[error("zkML verification failed: {0:#}")]
    ZkmlVerification(anyhow::Error),

    /// TEE attestation verification failed.
    #[error("TEE verification failed: {0:#}")]
    TeeVerification(anyhow::Error),

    /// Vector search failed.
    #[error("vector search failed: {0:#}")]
    VectorSearch(anyhow::Error),

    /// Store-context operation failed.
    #[error("store_context failed: {0:#}")]
    StoreContext(anyhow::Error),

    /// Memory subsystem error.
    #[error("memory error: {0}")]
    Memory(#[from] crate::memory::MemoryError),

    /// Continuous batching error.
    #[error("batching error: {0:#}")]
    Batching(anyhow::Error),

    /// Simulation mode not supported in the current build.
    #[error("simulation mode '{mode}' is not supported in this build")]
    SimulationModeNotSupported {
        /// The unsupported mode name.
        mode: String,
    },

    /// Fork simulation failed.
    #[error("fork simulation failed: {0:#}")]
    ForkSimulation(anyhow::Error),

    /// Invalid engine configuration.
    #[error("invalid engine configuration: {0}")]
    Config(String),

    /// Generic internal error.
    #[error("internal error: {0:#}")]
    Internal(anyhow::Error),
}

impl MwvmError {
    /// Wrap any `anyhow::Error` as a WASM engine error.
    pub fn wasm(err: impl Into<anyhow::Error>) -> Self {
        Self::Wasm(err.into())
    }
}

impl From<anyhow::Error> for MwvmError {
    fn from(e: anyhow::Error) -> Self {
        Self::Internal(e)
    }
}

impl From<std::io::Error> for MwvmError {
    fn from(e: std::io::Error) -> Self {
        Self::Internal(e.into())
    }
}
