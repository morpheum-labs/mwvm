//! MWVM Swarm Example
//!
//! Demonstrates how to launch and coordinate multiple agents using the
//! high-level `mwvm-orchestrator` crate.
//!
//! Run with:
//! ```bash
//! cargo run -p swarm-example
//! ```

use mwvm_orchestrator::prelude::*;
use std::env;

#[tokio::main]
async fn main() {
    // ── Observability ────────────────────────────────────────────────────
    tracing_subscriber::fmt()
        .with_env_filter(env::var("RUST_LOG").unwrap_or_else(|_| "info".into()))
        .init();

    info!("Starting MWVM Swarm Example");

    // ── Minimal valid WASM ──────────────────────────────────────────────
    //
    // In production you would load your compiled agent WASM from disk.
    // The 8-byte header is the smallest valid WASM 1.0 module.
    let minimal_wasm: &[u8] = &[
        0x00, 0x61, 0x73, 0x6D, // magic: \0asm
        0x01, 0x00, 0x00, 0x00, // version: 1
    ];

    // ── Build the swarm ─────────────────────────────────────────────────
    //
    // The `SwarmBuilder` uses the fluent builder pattern.
    // `build()` is synchronous — it creates the engine, then instantiates
    // each agent's WASM runtime in a tight loop.
    let swarm = Swarm::builder()
        .agent_count(5)
        .base_wasm(minimal_wasm)
        .build()
        .expect("failed to build swarm");

    info!(
        agent_count = swarm.len(),
        "Swarm created successfully"
    );

    // ── Subscribe to the broadcast topic ────────────────────────────────
    //
    // The `MessageBus` supports topic-based pub/sub with RAII subscriptions.
    let (_subscription, rx) = swarm.bus().subscribe("broadcast");

    // ── Broadcast a message ─────────────────────────────────────────────
    //
    // `broadcast()` is async and publishes to the "broadcast" topic.
    swarm
        .broadcast(b"Hello from the swarm example!".to_vec())
        .await
        .expect("broadcast failed");

    info!("Broadcast message sent to all subscribers");

    // ── Receive the broadcast ───────────────────────────────────────────
    //
    // Since we subscribed before publishing, we should receive the message.
    match rx.recv_async().await {
        Ok(event) => info!(?event, "Received broadcast event"),
        Err(e) => warn!(%e, "No event received"),
    }

    // ── Send a targeted message ─────────────────────────────────────────
    //
    // You can also send messages to individual agents by ID.
    let (_agent_sub, agent_rx) = swarm.bus().subscribe("agent:0");

    swarm
        .send_to(0, b"Hello agent 0!".to_vec())
        .await
        .expect("send_to failed");

    match agent_rx.recv_async().await {
        Ok(event) => info!(?event, "Agent 0 received targeted message"),
        Err(e) => warn!(%e, "Agent 0 did not receive message"),
    }

    info!("Swarm example completed successfully");
}
