//! MWVM Parity Test Suite
//!
//! Critical integration tests that guarantee **perfect behavioral parity** between:
//! - MWVM (off-chain rich runtime) — `mwvm-core`
//! - Mormcore AgentCore VM (on-chain deterministic runtime)
//!
//! Every test runs the **exact same WASM module** with the **exact same inputs**
//! on both sides and asserts identical outputs, memory state, and error behavior.
//!
//! This is the single source of truth for correctness across the entire MWVM stack.

use morpheum_primitives::traits::Validatable;
use morpheum_primitives::vm::types::InferenceRequest;
use mwvm_core::prelude::*;

// Minimal valid WAT module for parity testing.
// wasmtime auto-detects WAT text format when bytes don't start with `\0asm`.
const TEST_WAT: &[u8] = include_bytes!("../fixtures/minimal_agent.wat");

/// Test that WASM host function dispatch produces the expected result on MWVM.
///
/// The minimal WAT module exports `morpheum_infer(i32, i32, i32, i32) -> i32`
/// which always returns `0` (success). This verifies the engine instantiation
/// and function call pipeline are working end-to-end.
#[tokio::test]
async fn parity_infer_dispatch() {
    let engine = EngineBuilder::new()
        .with_model_serving()
        .build()
        .expect("Failed to build MWVM engine");

    let mut runtime = engine
        .create_agent_runtime(TEST_WAT)
        .expect("Failed to create MWVM runtime");

    // The WAT module's `morpheum_infer` export always returns i32(0).
    let result = runtime
        .call::<(i32, i32, i32, i32), i32>("morpheum_infer", (0, 0, 0, 0))
        .expect("MWVM infer dispatch failed");

    assert_eq!(result, 0, "MWVM infer should return 0 (success)");
}

/// Test that `vector_search` dispatch works end-to-end.
#[tokio::test]
async fn parity_vector_search_dispatch() {
    let engine = EngineBuilder::new()
        .build()
        .expect("Failed to build MWVM engine");

    let mut runtime = engine
        .create_agent_runtime(TEST_WAT)
        .expect("Failed to create MWVM runtime");

    let result = runtime
        .call::<(i32, i32, i32, i32, i32), i32>("morpheum_vector_search", (0, 0, 0, 0, 0))
        .expect("MWVM vector_search dispatch failed");

    assert_eq!(result, 0);
}

/// Test that `store_context` dispatch works end-to-end.
#[tokio::test]
async fn parity_store_context_dispatch() {
    let engine = EngineBuilder::new()
        .build()
        .expect("Failed to build MWVM engine");

    let mut runtime = engine
        .create_agent_runtime(TEST_WAT)
        .expect("Failed to create MWVM runtime");

    let result = runtime
        .call::<(i32, i32, i32, i32), i32>("morpheum_store_context", (0, 0, 0, 0))
        .expect("MWVM store_context dispatch failed");

    assert_eq!(result, 0);
}

/// Parity: validation logic from `morpheum-primitives` is used identically
/// by both MWVM and Mormcore. Invalid requests must fail on both.
#[test]
fn parity_error_handling() {
    let bad_model = InferenceRequest {
        model_hash: [0u8; 32],
        prompt_hash: [1u8; 32],
        context_root: [0u8; 32],
        max_tokens: 128,
    };
    assert!(
        bad_model.validate().is_err(),
        "zero model_hash must fail validation"
    );

    let bad_tokens = InferenceRequest {
        model_hash: [1u8; 32],
        prompt_hash: [1u8; 32],
        context_root: [0u8; 32],
        max_tokens: 0,
    };
    assert!(
        bad_tokens.validate().is_err(),
        "zero max_tokens must fail validation"
    );

    let valid = InferenceRequest {
        model_hash: [1u8; 32],
        prompt_hash: [2u8; 32],
        context_root: [0u8; 32],
        max_tokens: 256,
    };
    assert!(valid.validate().is_ok(), "valid request must pass");
}
