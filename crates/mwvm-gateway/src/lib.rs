//! # MWVM Gateway
//!
//! **Production-grade HTTP/JSON-RPC gateways** for MCP, A2A, DID/VC, and x402.
//!
//! Exposes canonical endpoints (`/mcp`, `/a2a`, `/did/{agent-id}`, `/x402/pay`)
//! and routes every request to the underlying `MwvmEngine` with full observability.
//!
//! Protocol types are defined locally — no external crate dependencies for
//! MCP/A2A/DID/x402 since stable crates do not yet exist.

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

pub mod a2a_server;
pub mod did_resolver;
pub mod gateway;
pub mod mcp_server;
pub mod x402_handler;

// ── Re-exports ───────────────────────────────────────────────────────────────

pub use gateway::{Gateway, GatewayBuilder, GatewayConfig};

/// Convenient public Result type.
pub type Result<T> = std::result::Result<T, GatewayError>;

// =============================================================================
// Gateway-specific Error
// =============================================================================

/// All errors surfaced by the MWVM Gateway.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum GatewayError {
    /// Propagated SDK error.
    #[error("sdk error: {0}")]
    Sdk(#[from] mwvm_sdk::SdkError),

    /// HTTP / protocol error.
    #[error("protocol error: {0:#}")]
    Protocol(#[from] anyhow::Error),

    /// Configuration error.
    #[error("configuration error: {0}")]
    Config(String),
}

impl axum::response::IntoResponse for GatewayError {
    fn into_response(self) -> axum::response::Response {
        let status = match &self {
            Self::Config(_) => axum::http::StatusCode::BAD_REQUEST,
            _ => axum::http::StatusCode::INTERNAL_SERVER_ERROR,
        };
        let body = serde_json::json!({ "error": self.to_string() });
        (status, axum::Json(body)).into_response()
    }
}

// =============================================================================
// Prelude
// =============================================================================

/// Convenient prelude — `use mwvm_gateway::prelude::*;`
pub mod prelude {
    pub use super::{Gateway, GatewayBuilder, GatewayConfig, GatewayError, Result};
    pub use mwvm_sdk::prelude::*;
    pub use tracing::{debug, error, info, warn};
}
