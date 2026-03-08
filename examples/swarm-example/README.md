# Swarm Example

Demonstrates how to launch and coordinate **multiple agents** using the MWVM orchestrator.

This example shows:
- Spawning many agents from a single WASM template
- Broadcasting messages to the entire swarm
- Running in simulation mode (`Offline`, `Fork`, or `Replay`)
- Graceful shutdown with Ctrl+C

---

## Quick Start

### 1. Build the example

```bash
cd examples/swarm-example
cargo build --release
```

### 2. Run the swarm

```bash
cargo run
```

Or from the workspace root:

```bash
cargo run -p swarm-example
```

The swarm will start with 20 agents (configurable) and keep running until you press **Ctrl+C**.

---

## What This Example Demonstrates

- Using `Swarm::builder()` with a custom agent count
- Loading the same WASM template for every agent
- Broadcasting messages to all agents via `swarm.broadcast()`
- Switching simulation modes (`Offline` is used by default)
- Clean async shutdown handling

---

## Project Structure

```
swarm-example/
├── Cargo.toml
├── src/
│   └── main.rs          # Main swarm orchestration logic
└── README.md
```

---

## Customization Ideas

- Change `agent_count` in `main.rs` to spawn 100+ agents
- Switch to `SimulationMode::Fork` for isolated testing
- Add custom message handling logic inside each agent
- Integrate with the gateway for external communication

---

## Next Steps

1. Try the browser example to see agents running in the web
2. Combine this with the gateway to expose your swarm via MCP/A2A
3. Explore the full orchestrator API in `crates/mwvm-orchestrator`

---
