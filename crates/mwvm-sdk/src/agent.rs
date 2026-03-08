//! High-level Agent API — the primary interface for MWVM SDK users.
//!
//! ```rust,no_run
//! use mwvm_sdk::prelude::*;
//!
//! # fn example() -> mwvm_sdk::Result<()> {
//! let agent = Agent::builder()
//!     .wasm_bytes(b"\x00asm\x01\x00\x00\x00")
//!     .build()?;
//! # Ok(())
//! # }
//! ```

use tracing::info;

use crate::{Result, SdkConfig, SdkError};
use mwvm_core::{AgentRuntime, EngineBuilder, MwvmEngine};

/// A running WASM agent with ergonomic helper methods.
///
/// Owns the underlying [`AgentRuntime`] and requires `&mut self` for calls
/// (WASM stores are inherently single-threaded).
pub struct Agent {
    runtime: AgentRuntime,
    engine: MwvmEngine,
}

impl Agent {
    /// Start building a new agent with the fluent builder.
    pub fn builder() -> AgentBuilder {
        AgentBuilder::new()
    }

    /// Direct access to the underlying core runtime (for reading state).
    #[must_use]
    pub const fn runtime(&self) -> &AgentRuntime {
        &self.runtime
    }

    /// Mutable access to the underlying core runtime.
    ///
    /// Use this for calling WASM exports:
    /// ```rust,no_run
    /// # use mwvm_sdk::prelude::*;
    /// # fn example(agent: &mut Agent) -> mwvm_sdk::Result<()> {
    /// agent.runtime_mut().call::<(), ()>("_start", ())?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn runtime_mut(&mut self) -> &mut AgentRuntime {
        &mut self.runtime
    }

    /// The engine backing this agent.
    #[must_use]
    pub const fn engine(&self) -> &MwvmEngine {
        &self.engine
    }
}

// =============================================================================
// Builder (fluent, type-safe construction)
// =============================================================================

/// Fluent builder for [`Agent`].
#[must_use]
#[derive(Default)]
pub struct AgentBuilder {
    wasm_bytes: Option<Vec<u8>>,
    config: Option<SdkConfig>,
}

impl AgentBuilder {
    /// Create a new builder with defaults.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the compiled WASM bytes for this agent.
    pub fn wasm_bytes(mut self, bytes: impl Into<Vec<u8>>) -> Self {
        self.wasm_bytes = Some(bytes.into());
        self
    }

    /// Attach custom SDK configuration.
    pub fn config(mut self, config: SdkConfig) -> Self {
        self.config = Some(config);
        self
    }

    /// Build the agent (synchronous — engine creation is CPU-bound).
    ///
    /// # Errors
    ///
    /// Returns [`SdkError::Config`] if `wasm_bytes` was not provided, or
    /// [`SdkError::Core`] if engine creation or WASM instantiation fails.
    pub fn build(self) -> Result<Agent> {
        let wasm = self
            .wasm_bytes
            .ok_or_else(|| SdkError::Config("wasm_bytes is required".into()))?;

        let cfg = self.config.unwrap_or_default();

        let mut builder = EngineBuilder::new();
        if cfg.model_serving {
            builder = builder.with_model_serving();
        }
        if cfg.tee_simulation {
            builder = builder.with_tee_simulation();
        }
        builder = builder.with_max_instances(cfg.max_concurrent_instances);

        let engine = builder.build().map_err(SdkError::Core)?;
        let runtime = engine
            .create_agent_runtime(&wasm)
            .map_err(SdkError::Core)?;

        info!("Agent built successfully");
        Ok(Agent { runtime, engine })
    }
}
