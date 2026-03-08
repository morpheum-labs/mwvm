//! DID Resolver — W3C `did:morpheum:` document resolution.
//!
//! **Endpoints**:
//! - `GET  /did/{agent_id}` — Returns a DID Document
//! - `POST /did/verify`     — Stub for Verifiable Presentation verification

use std::sync::Arc;

use axum::{
    extract::{Path, State},
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use tracing::{debug, info};

use crate::gateway::AppState;

// ── DID types (self-contained, no external didkit dependency) ────────────────

/// Minimal DID Document.
#[derive(Debug, Serialize)]
pub struct DidDocument {
    /// The DID subject (e.g. `did:morpheum:abc123`).
    pub id: String,
    /// Verification methods (public keys).
    pub verification_method: Vec<VerificationMethod>,
    /// Service endpoints (MCP, A2A, x402).
    pub service: Vec<Service>,
}

/// A verification method entry.
#[derive(Debug, Serialize)]
pub struct VerificationMethod {
    /// Fragment ID.
    pub id: String,
    /// Controller DID.
    pub controller: String,
    /// Key type (e.g. `Ed25519VerificationKey2020`).
    #[serde(rename = "type")]
    pub type_: String,
}

/// A DID service endpoint.
#[derive(Debug, Serialize)]
pub struct Service {
    /// Fragment ID.
    pub id: String,
    /// Service type.
    #[serde(rename = "type")]
    pub type_: String,
    /// Endpoint URL.
    #[serde(rename = "serviceEndpoint")]
    pub service_endpoint: String,
}

/// VP verification request (placeholder).
#[derive(Debug, Deserialize)]
pub struct VerifyRequest {
    /// Arbitrary VP JSON.
    pub presentation: serde_json::Value,
}

/// VP verification response.
#[derive(Debug, Serialize)]
pub struct VerifyResponse {
    /// Whether the VP is considered valid.
    pub verified: bool,
}

// ── Router ───────────────────────────────────────────────────────────────────

/// Mount DID routes.
pub fn routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/did/{agent_id}", get(handle_resolve))
        .route("/did/verify", post(handle_verify))
        .with_state(state)
}

// ── Handlers ─────────────────────────────────────────────────────────────────

async fn handle_resolve(
    State(_state): State<Arc<AppState>>,
    Path(agent_id): Path<String>,
) -> Json<DidDocument> {
    info!(agent_id = %agent_id, "DID Document requested");

    Json(DidDocument {
        id: format!("did:morpheum:{agent_id}"),
        verification_method: vec![VerificationMethod {
            id: format!("did:morpheum:{agent_id}#key-1"),
            controller: format!("did:morpheum:{agent_id}"),
            type_: "Ed25519VerificationKey2020".into(),
        }],
        service: vec![
            Service {
                id: format!("did:morpheum:{agent_id}#mcp"),
                type_: "MCP".into(),
                service_endpoint: "/mcp".into(),
            },
            Service {
                id: format!("did:morpheum:{agent_id}#a2a"),
                type_: "A2A".into(),
                service_endpoint: "/a2a".into(),
            },
        ],
    })
}

async fn handle_verify(
    State(_state): State<Arc<AppState>>,
    Json(_req): Json<VerifyRequest>,
) -> Json<VerifyResponse> {
    debug!("VP verification requested (stub)");
    Json(VerifyResponse { verified: false })
}
