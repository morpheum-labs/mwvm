//! SDK-level runtime wrapper.
//!
//! [`SdkRuntime`] owns an [`MwvmEngine`] and provides high-level helpers for
//! creating agents and attaching simulation contexts.

use tracing::debug;

use crate::config::SdkConfig;
use crate::{Result, SdkError};
use mwvm_core::{AgentRuntime, EngineBuilder, MwvmEngine, SimulationMode, Simulator};

/// High-level runtime facade managing engine lifecycle.
pub struct SdkRuntime {
    engine: MwvmEngine,
    simulator: Option<Simulator>,
}

impl SdkRuntime {
    /// Create a runtime from an [`SdkConfig`].
    ///
    /// # Errors
    ///
    /// Returns [`SdkError::Core`] if the underlying engine cannot be created.
    pub fn from_config(config: &SdkConfig) -> Result<Self> {
        let mut builder = EngineBuilder::new();

        if config.model_serving {
            builder = builder.with_model_serving();
        }
        if config.tee_simulation {
            builder = builder.with_tee_simulation();
        }
        builder = builder.with_max_instances(config.max_concurrent_instances);

        let engine = builder.build().map_err(SdkError::Core)?;
        debug!("SdkRuntime created from config");

        Ok(Self {
            engine,
            simulator: None,
        })
    }

    /// Create a runtime with default settings.
    ///
    /// # Errors
    ///
    /// Returns [`SdkError::Core`] if the underlying engine cannot be created.
    pub fn new() -> Result<Self> {
        Self::from_config(&SdkConfig::default())
    }

    /// Attach a simulation context (fork / replay / offline).
    #[must_use]
    pub fn with_simulation(mut self, mode: SimulationMode) -> Self {
        self.simulator = Some(Simulator::new(self.engine.clone(), mode));
        self
    }

    /// Instantiate a WASM agent from compiled bytes.
    ///
    /// # Errors
    ///
    /// Returns [`SdkError::Core`] if WASM compilation or instantiation fails.
    pub fn create_agent(&self, wasm_bytes: &[u8]) -> Result<AgentRuntime> {
        self.engine
            .create_agent_runtime(wasm_bytes)
            .map_err(SdkError::Core)
    }

    /// The underlying core engine.
    #[must_use]
    pub const fn engine(&self) -> &MwvmEngine {
        &self.engine
    }

    /// The attached simulator (if any).
    #[must_use]
    pub const fn simulator(&self) -> Option<&Simulator> {
        self.simulator.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn runtime_from_default() {
        let rt = SdkRuntime::new();
        assert!(rt.is_ok());
    }

    #[tokio::test]
    async fn runtime_from_config() {
        let cfg = SdkConfig::new().model_serving(false);
        let rt = SdkRuntime::from_config(&cfg);
        assert!(rt.is_ok());
    }
}
