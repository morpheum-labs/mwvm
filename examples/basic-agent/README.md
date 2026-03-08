# Basic Agent Example

The **recommended starting template** for building Morpheum AI agents with MWVM.

This minimal example shows how to:
- Create an agent using the high-level `mwvm-sdk`
- Run native inference
- Store results in persistent memory
- Compile to WASM and call from JavaScript/TypeScript

---

## Quick Start

### 1. Build the agent

```bash
cd examples/basic-agent
cargo build --target wasm32-unknown-unknown --release
```

### 2. Run the example

You can run it directly from Rust (for testing) or load the generated WASM in the browser example.

```bash
# Run via the CLI (recommended)
cargo run
```

Or use the main MWVM CLI from the workspace root:

```bash
# From the workspace root
cargo run -p mwvm-cli -- run examples/basic-agent/target/wasm32-unknown-unknown/release/basic_agent.wasm
```

---

## What This Example Demonstrates

- Using `Agent::builder()` to create an agent
- Calling `agent.infer()` with model and prompt hashes
- Storing output using `agent.store_context()`
- Exporting WASM-callable functions via `wasm_bindgen`
- Proper error handling and browser console support

---
