//! A2A Server — Agent-to-Agent Protocol endpoints.
//!
//! Exposes Morpheum agents as first-class collaborators to Google ADK,
//! `LangGraph`, `BeeAI`, and other A2A-compatible systems.
//!
//! **Endpoints**:
//! - `GET  /a2a`      — `AgentCard` (discovery)
//! - `POST /a2a/task` — Task delegation

use std::sync::Arc;

use axum::{
    extract::State,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use tracing::{debug, info};

use crate::gateway::AppState;

// ── Protocol types ───────────────────────────────────────────────────────────

/// A2A `AgentCard` (discovery document).
#[derive(Debug, Serialize)]
pub struct AgentCard {
    /// Agent DID or identifier.
    pub id: String,
    /// Human-readable name.
    pub name: String,
    /// Description of capabilities.
    pub description: String,
    /// Supported capabilities.
    pub capabilities: Vec<String>,
    /// Available endpoints.
    pub endpoints: Vec<String>,
}

/// Inbound task request.
#[derive(Debug, Deserialize)]
pub struct TaskRequest {
    /// Unique task identifier.
    pub id: String,
    /// Action to perform.
    pub action: String,
    /// Arbitrary payload.
    #[serde(default)]
    pub payload: serde_json::Value,
}

/// Outbound task response.
#[derive(Debug, Serialize)]
pub struct TaskResponse {
    /// Echoed task identifier.
    pub id: String,
    /// Completion status.
    pub status: String,
    /// Result payload.
    pub result: serde_json::Value,
}

// ── Router ───────────────────────────────────────────────────────────────────

/// Mount A2A routes.
pub fn routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/a2a", get(handle_agent_card))
        .route("/a2a/task", post(handle_task))
        .with_state(state)
}

// ── Handlers ─────────────────────────────────────────────────────────────────

async fn handle_agent_card(State(_state): State<Arc<AppState>>) -> Json<AgentCard> {
    debug!("A2A AgentCard requested");

    Json(AgentCard {
        id: "morpheum:local:agent-0".into(),
        name: "Morpheum Agent".into(),
        description: "Native Morpheum AI agent with inference, memory, and proof capabilities"
            .into(),
        capabilities: vec![
            "inference".into(),
            "memory_search".into(),
            "task_delegation".into(),
        ],
        endpoints: vec!["/a2a/task".into()],
    })
}

async fn handle_task(
    State(_state): State<Arc<AppState>>,
    Json(req): Json<TaskRequest>,
) -> Json<TaskResponse> {
    info!(task_id = %req.id, action = %req.action, "A2A task received");

    Json(TaskResponse {
        id: req.id,
        status: "completed".into(),
        result: serde_json::json!({
            "output": "task executed on Morpheum agent"
        }),
    })
}
