//! Minimal MWVM Agent Example
//!
//! Demonstrates:
//! - Building an [`MwvmEngine`] via the [`EngineBuilder`] (fluent builder pattern)
//! - Loading a compiled WASM module into the engine
//! - Creating an [`AgentRuntime`] and attempting to call an exported function
//! - Interacting with the engine's shared persistent memory
//!
//! Run with: `cargo run -p basic-agent`

use mwvm_sdk::prelude::*;

fn main() {
    // ── Observability ────────────────────────────────────────────────────
    tracing_subscriber::fmt()
        .with_env_filter("info")
        .init();

    info!("Starting basic-agent example");

    // ── Build the engine ─────────────────────────────────────────────────
    //
    // `EngineBuilder` is the single entry-point for creating an engine.
    // Enable local model-serving (continuous batching) by default.
    let engine = EngineBuilder::new()
        .with_model_serving()
        .build()
        .expect("failed to build MwvmEngine");

    info!(
        model_serving = engine.config().model_serving,
        "Engine created"
    );

    // ── Load a minimal valid WASM module ────────────────────────────────
    //
    // In production you would load your compiled agent bytes from disk.
    // Here we use the smallest valid WASM module (the 8-byte header).
    let minimal_wasm: &[u8] = &[
        0x00, 0x61, 0x73, 0x6D, // magic: \0asm
        0x01, 0x00, 0x00, 0x00, // version: 1
    ];

    let mut runtime = engine
        .create_agent_runtime(minimal_wasm)
        .expect("failed to instantiate WASM module");

    info!("AgentRuntime instantiated successfully");

    // ── Exercise the memory backend ─────────────────────────────────────
    //
    // The engine exposes a shared `LocalMemory` store.  Any agent running
    // inside this engine shares the same memory namespace.
    let memory = engine.memory();

    memory
        .store(b"example:key", b"hello from basic-agent".to_vec())
        .expect("failed to store value");

    let value = memory
        .load(b"example:key")
        .expect("failed to load value");

    info!(
        value = value.as_deref().map(|v| std::str::from_utf8(v).unwrap_or("<non-utf8>")),
        "Read back from LocalMemory"
    );

    assert_eq!(value.as_deref(), Some(b"hello from basic-agent".as_slice()));
    info!("Memory round-trip verified");

    // ── Vector search demo ──────────────────────────────────────────────
    //
    // LocalMemory also supports cosine-similarity vector search.
    let vec_memory = LocalMemory::with_dimension(3);

    vec_memory.insert_vector(vec![1.0, 0.0, 0.0]).expect("insert failed");
    vec_memory.insert_vector(vec![0.0, 1.0, 0.0]).expect("insert failed");
    vec_memory.insert_vector(vec![0.9, 0.1, 0.0]).expect("insert failed");

    let results = vec_memory.search(&[1.0, 0.0, 0.0], 2);
    info!(top_k = results.len(), "Vector search completed");
    for (rank, result) in results.iter().enumerate() {
        info!(rank = rank + 1, id = result.id, score = result.score, "search result");
    }

    // ── Call a WASM export (if present) ─────────────────────────────────
    //
    // The minimal module has no exports, so this demonstrates graceful
    // error handling when a function is not found.
    match runtime.call::<(), ()>("_start", ()) {
        Ok(()) => info!("_start returned successfully"),
        Err(e) => info!(%e, "Expected: _start not found in minimal module"),
    }

    info!("basic-agent example completed successfully");
}
