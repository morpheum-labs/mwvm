 MWVM Examples

Welcome to the official MWVM example gallery!

These ready-to-run examples demonstrate how to use the full MWVM stack — from single agents to multi-agent swarms and browser-based applications.

All examples are **minimal**, **well-documented**, and **production-ready**. They serve as perfect templates for your own projects.

---

## 📋 Available Examples

### 1. [`basic-agent/`](./basic-agent)
**Recommended starting point**

Minimal single-agent template in Rust that compiles to WASM.  
Shows how to:
- Create an agent using `mwvm-sdk`
- Run native inference
- Store data in persistent memory
- Export WASM-callable functions

**Best for**: First-time users and new agent development.

---

### 2. [`swarm-example/`](./swarm-example)
**Multi-agent orchestration demo**

Shows how to launch and coordinate many agents using `mwvm-orchestrator`.  
Features:
- Spawning 20+ agents from one WASM template
- Broadcasting messages to the entire swarm
- Simulation modes (`Offline`, `Fork`, `Replay`)
- Graceful shutdown

**Best for**: Learning swarm coordination and message passing.

---

### 3. [`web-browser-example/`](./web-browser-example)
**Browser + TypeScript demo**

Full React + Vite example using the `mwvm-wasm` TypeScript SDK.  
Demonstrates:
- Loading and running agents directly in the browser
- Calling `infer()`, `vectorSearch()`, and `storeContext()`
- Real-time UI feedback

**Best for**: Web developers and frontend integration.

---

## 🚀 Quick Start

### Run any example

```bash
# 1. Build the basic agent (required by other examples)
cd examples/basic-agent
cargo build --target wasm32-unknown-unknown --release

# 2. Run the swarm example
cd ../swarm-example
cargo run

# 3. Run the browser demo
cd ../web-browser-example
npm install
npm run dev
```

---

## 📖 How to Use These Examples

1. Start with `basic-agent/` — understand the fundamentals
2. Move to `swarm-example/` — learn multi-agent coordination
3. Try `web-browser-example/` — integrate agents into web apps

All examples are designed to be **forked and modified**. Feel free to copy any part into your own projects.

---
