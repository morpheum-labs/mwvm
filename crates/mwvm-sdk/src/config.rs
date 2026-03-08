//! MWVM SDK Configuration.
//!
//! Layered, production-grade configuration with sensible defaults.
//!
//! **Usage**:
//! ```rust
//! use mwvm_sdk::SdkConfig;
//! let config = SdkConfig::new().model_serving(false).max_instances(256);
//! ```

use serde::{Deserialize, Serialize};

use mwvm_core::EngineConfig;

/// Main configuration for the MWVM SDK.
///
/// All fields have production-ready defaults.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct SdkConfig {
    /// Enable local model serving + continuous batching (default: true).
    pub model_serving: bool,

    /// Enable TEE/zkML simulation (off-chain only).
    pub tee_simulation: bool,

    /// Maximum concurrent WASM instances (memory safety).
    pub max_concurrent_instances: usize,

    /// Default `max_tokens` value for `agent.infer()` calls.
    pub default_max_tokens: u32,

    /// Log level for tracing (e.g. "info", "debug", "trace").
    pub log_level: String,
}

impl Default for SdkConfig {
    fn default() -> Self {
        Self {
            model_serving: true,
            tee_simulation: false,
            max_concurrent_instances: 1024,
            default_max_tokens: 2048,
            log_level: "info".to_owned(),
        }
    }
}

impl SdkConfig {
    /// Create a new config with production defaults.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Convert to the core engine configuration (DRY guarantee).
    #[must_use]
    pub fn into_engine_config(self) -> EngineConfig {
        EngineConfig {
            model_serving: self.model_serving,
            tee_simulation: self.tee_simulation,
            max_concurrent_instances: self.max_concurrent_instances,
        }
    }

    // ── Builder-style convenience methods ────────────────────────────────

    /// Toggle local model serving.
    #[must_use]
    pub const fn model_serving(mut self, enabled: bool) -> Self {
        self.model_serving = enabled;
        self
    }

    /// Toggle TEE/zkML simulation.
    #[must_use]
    pub const fn tee_simulation(mut self, enabled: bool) -> Self {
        self.tee_simulation = enabled;
        self
    }

    /// Set maximum concurrent WASM instances.
    #[must_use]
    pub const fn max_instances(mut self, n: usize) -> Self {
        self.max_concurrent_instances = n;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_config() {
        let config = SdkConfig::default();
        assert!(config.model_serving);
        assert_eq!(config.max_concurrent_instances, 1024);
        assert_eq!(config.default_max_tokens, 2048);
        assert_eq!(config.log_level, "info");
    }

    #[test]
    fn builder_methods() {
        let config = SdkConfig::new().model_serving(false).max_instances(256);
        assert!(!config.model_serving);
        assert_eq!(config.max_concurrent_instances, 256);
    }

    #[test]
    fn config_is_send_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<SdkConfig>();
    }
}
