//! Unified Gateway facade — mounts all protocol routers under a single Axum app.

use std::net::SocketAddr;
use std::sync::Arc;

use axum::Router;
use tokio::net::TcpListener;
use tracing::info;

use mwvm_sdk::MwvmEngine;

use crate::{GatewayError, Result};

// =============================================================================
// Configuration
// =============================================================================

/// Runtime configuration for the gateway.
///
/// Four boolean flags control which protocol endpoints are mounted.
/// This is the simplest correct representation for "enabled / disabled"
/// feature toggles and is idiomatic for config structs.
#[derive(Debug, Clone)]
#[allow(clippy::struct_excessive_bools)]
pub struct GatewayConfig {
    /// Bind address (default: `0.0.0.0:8080`).
    pub bind: SocketAddr,
    /// Enable MCP endpoints.
    pub enable_mcp: bool,
    /// Enable A2A endpoints.
    pub enable_a2a: bool,
    /// Enable DID resolver.
    pub enable_did: bool,
    /// Enable x402 handler.
    pub enable_x402: bool,
}

impl Default for GatewayConfig {
    fn default() -> Self {
        Self {
            bind: ([0, 0, 0, 0], 8080).into(),
            enable_mcp: true,
            enable_a2a: true,
            enable_did: true,
            enable_x402: true,
        }
    }
}

// =============================================================================
// Shared state injected into all route handlers
// =============================================================================

/// Shared gateway state accessible from every handler.
#[derive(Clone)]
pub struct AppState {
    /// The core engine backing every agent operation.
    pub engine: MwvmEngine,
}

// =============================================================================
// Builder
// =============================================================================

/// Fluent builder for [`Gateway`].
#[must_use]
#[derive(Default)]
pub struct GatewayBuilder {
    config: GatewayConfig,
    engine: Option<MwvmEngine>,
}

impl GatewayBuilder {
    /// Create a new builder with defaults.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the bind address (accepts anything that can be parsed as `SocketAddr`).
    pub const fn bind(mut self, addr: SocketAddr) -> Self {
        self.config.bind = addr;
        self
    }

    /// Attach the [`MwvmEngine`] that powers all agent operations.
    pub fn engine(mut self, engine: MwvmEngine) -> Self {
        self.engine = Some(engine);
        self
    }

    /// Toggle MCP.
    pub const fn enable_mcp(mut self, v: bool) -> Self {
        self.config.enable_mcp = v;
        self
    }

    /// Toggle A2A.
    pub const fn enable_a2a(mut self, v: bool) -> Self {
        self.config.enable_a2a = v;
        self
    }

    /// Toggle DID.
    pub const fn enable_did(mut self, v: bool) -> Self {
        self.config.enable_did = v;
        self
    }

    /// Toggle x402.
    pub const fn enable_x402(mut self, v: bool) -> Self {
        self.config.enable_x402 = v;
        self
    }

    /// Build the gateway. All protocol routers are mounted here.
    ///
    /// # Errors
    ///
    /// Returns [`GatewayError::Config`] if the engine was not provided.
    pub fn build(self) -> Result<Gateway> {
        let engine = self
            .engine
            .ok_or_else(|| GatewayError::Config("engine is required".into()))?;

        let state = Arc::new(AppState { engine });

        let mut router = Router::new();

        if self.config.enable_mcp {
            router = router.merge(crate::mcp_server::routes(state.clone()));
        }
        if self.config.enable_a2a {
            router = router.merge(crate::a2a_server::routes(state.clone()));
        }
        if self.config.enable_did {
            router = router.merge(crate::did_resolver::routes(state.clone()));
        }
        if self.config.enable_x402 {
            router = router.merge(crate::x402_handler::routes(state));
        }

        Ok(Gateway {
            router,
            config: self.config,
        })
    }
}

// =============================================================================
// Gateway
// =============================================================================

/// The unified HTTP gateway server.
pub struct Gateway {
    router: Router,
    config: GatewayConfig,
}

impl Gateway {
    /// Start building a new gateway.
    pub fn builder() -> GatewayBuilder {
        GatewayBuilder::new()
    }

    /// Serve forever (graceful shutdown via tokio signal).
    ///
    /// # Errors
    ///
    /// Returns [`GatewayError::Config`] if the bind address is unavailable,
    /// or [`GatewayError::Protocol`] if the server encounters an I/O error.
    pub async fn serve(self) -> Result<()> {
        let listener = TcpListener::bind(self.config.bind)
            .await
            .map_err(|e| {
                GatewayError::Config(format!("failed to bind {}: {e}", self.config.bind))
            })?;

        info!(
            bind = %self.config.bind,
            mcp = self.config.enable_mcp,
            a2a = self.config.enable_a2a,
            did = self.config.enable_did,
            x402 = self.config.enable_x402,
            "MWVM Gateway listening"
        );

        axum::serve(listener, self.router)
            .await
            .map_err(|e| GatewayError::Protocol(e.into()))?;

        Ok(())
    }

    /// The inner `Router` (useful for testing).
    pub fn into_router(self) -> Router {
        self.router
    }
}
