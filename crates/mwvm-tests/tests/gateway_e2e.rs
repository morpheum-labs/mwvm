//! MWVM Gateway End-to-End Test Suite
//!
//! Real HTTP integration tests for the full gateway stack.
//! Spins up the actual `Gateway` with all protocols enabled and performs
//! live HTTP calls using `axum_test::TestServer`.
//!
//! Tests: MCP, A2A, DID resolver, and x402 payment enforcement.
//! No mocks — every request hits real routes and real handler code.

use axum_test::TestServer;
use mwvm_core::prelude::*;
use serde_json::json;

/// Build a real gateway router with all protocols enabled (for HTTP tests).
fn build_test_gateway_router() -> axum::Router {
    let engine = EngineBuilder::new()
        .with_model_serving()
        .build()
        .expect("Failed to build engine");

    mwvm_gateway::Gateway::builder()
        .bind(([127, 0, 0, 1], 0).into())
        .engine(engine)
        .enable_mcp(true)
        .enable_a2a(true)
        .enable_did(true)
        .enable_x402(true)
        .build()
        .expect("Failed to build gateway")
        .into_router()
}

/// End-to-end test: Full gateway with all protocols responding to HTTP calls.
#[tokio::test]
async fn gateway_full_e2e() {
    let server = TestServer::new(build_test_gateway_router());

    // ── 1. MCP tools/list ──
    let mcp_response: axum_test::TestResponse = server
        .post("/mcp")
        .json(&json!({
            "method": "tools/list",
            "params": {}
        }))
        .await;

    assert_eq!(mcp_response.status_code(), 200);
    let body: serde_json::Value = mcp_response.json();
    assert!(body["result"]["tools"].is_array());

    // ── 2. A2A AgentCard ──
    let a2a_response: axum_test::TestResponse = server.get("/a2a").await;
    assert_eq!(a2a_response.status_code(), 200);
    let card: serde_json::Value = a2a_response.json();
    assert!(card["id"].is_string());

    // ── 3. DID Document ──
    let did_response: axum_test::TestResponse = server.get("/did/agent-123").await;
    assert_eq!(did_response.status_code(), 200);
    let did_doc: serde_json::Value = did_response.json();
    assert!(did_doc["id"].as_str().unwrap().contains("did:morpheum"));

    // ── 4. x402 Payment Required (no payment header) ──
    let x402_response: axum_test::TestResponse = server
        .post("/x402/pay")
        .json(&json!({
            "model_hash": "0000000000000000000000000000000000000000000000000000000000000000",
            "prompt_hash": "1111111111111111111111111111111111111111111111111111111111111111"
        }))
        .await;

    assert_eq!(x402_response.status_code(), 402);
}

/// Test gateway with only MCP enabled (selective protocol test).
#[tokio::test]
async fn gateway_mcp_only() {
    let engine = EngineBuilder::new().build().expect("Failed to build engine");

    let router = mwvm_gateway::Gateway::builder()
        .bind(([127, 0, 0, 1], 0).into())
        .engine(engine)
        .enable_mcp(true)
        .enable_a2a(false)
        .enable_did(false)
        .enable_x402(false)
        .build()
        .expect("Failed to build gateway")
        .into_router();

    let server = TestServer::new(router);

    let response: axum_test::TestResponse = server
        .post("/mcp")
        .json(&json!({"method": "tools/list", "params": {}}))
        .await;

    assert_eq!(response.status_code(), 200);
}
