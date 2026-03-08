//! MCP Server — Model Context Protocol endpoints.
//!
//! Exposes Morpheum agents as native tools to Claude, Cursor, VS Code,
//! and any MCP-compatible client via JSON-RPC.
//!
//! **Endpoints**:
//! - `POST /mcp` — JSON-RPC dispatch (tools/list, tools/call)

use std::sync::Arc;

use axum::{
    extract::State,
    routing::post,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use tracing::{debug, info};

use crate::gateway::AppState;

// ── Protocol types (defined locally — no external MCP crate dependency) ──────

/// A JSON-RPC–style MCP request envelope.
#[derive(Debug, Deserialize)]
pub struct McpRequest {
    /// JSON-RPC method name (e.g. `"tools/list"`, `"tools/call"`).
    pub method: String,
    /// Arbitrary JSON params.
    #[serde(default)]
    pub params: serde_json::Value,
}

/// A JSON-RPC–style MCP response envelope.
#[derive(Debug, Serialize)]
pub struct McpResponse {
    /// `null` on error.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,
    /// `null` on success.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<McpError>,
}

/// JSON-RPC error object.
#[derive(Debug, Serialize)]
pub struct McpError {
    /// Numeric error code.
    pub code: i32,
    /// Human-readable message.
    pub message: String,
}

/// MCP tool definition (returned in `tools/list`).
#[derive(Debug, Serialize)]
pub struct ToolDef {
    /// Tool name.
    pub name: String,
    /// Human-readable description.
    pub description: String,
    /// JSON Schema for the tool's input.
    pub input_schema: serde_json::Value,
}

// ── Router ───────────────────────────────────────────────────────────────────

/// Mount MCP routes.
pub fn routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/mcp", post(handle_mcp))
        .with_state(state)
}

// ── Handlers ─────────────────────────────────────────────────────────────────

async fn handle_mcp(
    State(_state): State<Arc<AppState>>,
    Json(req): Json<McpRequest>,
) -> Json<McpResponse> {
    debug!(method = %req.method, "MCP request");

    match req.method.as_str() {
        "tools/list" => {
            let tools = vec![
                ToolDef {
                    name: "infer".into(),
                    description: "Run inference on the agent's registered model".into(),
                    input_schema: serde_json::json!({
                        "type": "object",
                        "properties": {
                            "model_hash": { "type": "string" },
                            "prompt_hash": { "type": "string" },
                            "max_tokens": { "type": "integer" }
                        },
                        "required": ["model_hash", "prompt_hash"]
                    }),
                },
                ToolDef {
                    name: "search_memory".into(),
                    description: "Vector search over the agent's persistent memory".into(),
                    input_schema: serde_json::json!({
                        "type": "object",
                        "properties": {
                            "query": { "type": "array", "items": { "type": "number" } },
                            "k": { "type": "integer" }
                        },
                        "required": ["query"]
                    }),
                },
            ];
            Json(McpResponse {
                result: Some(serde_json::json!({
                    "tools": serde_json::to_value(tools).unwrap_or_default()
                })),
                error: None,
            })
        }
        "tools/call" => {
            info!("MCP tools/call — delegating to engine");
            // In production this would parse the tool name + args, create an
            // AgentRuntime, and execute the corresponding host function.
            Json(McpResponse {
                result: Some(serde_json::json!({ "status": "ok" })),
                error: None,
            })
        }
        other => Json(McpResponse {
            result: None,
            error: Some(McpError {
                code: -32601,
                message: format!("method not found: {other}"),
            }),
        }),
    }
}
