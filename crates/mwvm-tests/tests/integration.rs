//! MWVM Integration Test Suite
//!
//! End-to-end integration tests that verify the full MWVM stack works together:
//! - mwvm-core → mwvm-sdk → mwvm-orchestrator → mwvm-gateway
//!
//! These tests use real WASM execution, real engine instantiation, and real
//! message passing. No mocks, no stubs — everything is production code paths.

use mwvm_core::prelude::*;
use mwvm_orchestrator::prelude::*;

// Minimal valid WAT module for integration tests.
const MINIMAL_AGENT_WAT: &[u8] = include_bytes!("../fixtures/minimal_agent.wat");

/// End-to-end test: Agent creation → WASM function call
#[tokio::test]
async fn full_agent_lifecycle() {
    // 1. Create agent via SDK (synchronous builder — engine creation is CPU-bound)
    let mut agent = Agent::builder()
        .wasm_bytes(MINIMAL_AGENT_WAT)
        .build()
        .expect("Failed to create agent");

    // 2. Call an exported WASM function
    let result = agent
        .runtime_mut()
        .call::<(i32, i32, i32, i32), i32>("morpheum_infer", (0, 0, 0, 0))
        .expect("morpheum_infer call failed");

    assert_eq!(result, 0, "morpheum_infer should return 0 (success)");

    // 3. Call another exported function
    let result = agent
        .runtime_mut()
        .call::<(i32, i32, i32, i32), i32>("morpheum_store_context", (0, 0, 0, 0))
        .expect("morpheum_store_context call failed");

    assert_eq!(result, 0, "morpheum_store_context should return 0 (success)");
}

/// Test swarm creation and basic lifecycle.
#[tokio::test]
async fn swarm_lifecycle() {
    let swarm = Swarm::builder()
        .agent_count(5)
        .base_wasm(MINIMAL_AGENT_WAT)
        .build()
        .expect("Failed to create swarm");

    assert_eq!(swarm.len(), 5);
    assert!(!swarm.is_empty());

    // Broadcast a message (no subscribers yet, so it just verifies no panics)
    swarm
        .broadcast(b"hello from integration test".to_vec())
        .await
        .expect("Broadcast failed");
}

/// Test swarm subscribe/publish flow end-to-end.
#[tokio::test]
async fn swarm_message_bus() {
    let swarm = Swarm::builder()
        .agent_count(2)
        .base_wasm(MINIMAL_AGENT_WAT)
        .build()
        .expect("Failed to create swarm");

    // Subscribe to broadcast topic
    let (_sub, rx) = swarm.bus().subscribe("broadcast");

    swarm
        .broadcast(b"test payload".to_vec())
        .await
        .expect("Broadcast failed");

    let event = rx.recv_async().await.expect("Should receive broadcast");
    match event {
        mwvm_orchestrator::message_bus::Event::AgentMessage { to, payload, .. } => {
            assert!(to.is_none(), "broadcast should have to=None");
            assert_eq!(payload, b"test payload");
        }
        _ => panic!("Expected AgentMessage event"),
    }
}

/// Test gateway construction with all protocols.
#[tokio::test]
async fn gateway_construction() {
    let engine = EngineBuilder::new()
        .with_model_serving()
        .build()
        .expect("Failed to build engine");

    let gateway = mwvm_gateway::Gateway::builder()
        .bind(([127, 0, 0, 1], 0).into())
        .engine(engine)
        .enable_mcp(true)
        .enable_a2a(true)
        .enable_did(true)
        .enable_x402(true)
        .build()
        .expect("Failed to build gateway");

    // Verify we can extract the router (used for testing or embedding)
    let _router = gateway.into_router();
}

/// Test gateway construction with selective protocols.
#[tokio::test]
async fn gateway_mcp_only() {
    let engine = EngineBuilder::new().build().expect("Failed to build engine");

    let gateway = mwvm_gateway::Gateway::builder()
        .bind(([127, 0, 0, 1], 0).into())
        .engine(engine)
        .enable_mcp(true)
        .enable_a2a(false)
        .enable_did(false)
        .enable_x402(false)
        .build()
        .expect("Failed to build gateway");

    let _router = gateway.into_router();
}
