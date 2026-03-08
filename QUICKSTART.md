# QUICKSTART.md

**MWVM Quickstart – Get Value in Under 10 Minutes**

Welcome to MWVM — the portable off-chain runtime where Morpheum agents come to life before they ever touch the chain.

This guide focuses on the **five most common starting points** agents and developers use when first adopting MWVM.  
Pick the one that matches your current goal — run it — and see immediate value.

All commands assume you have Rust & Cargo installed. If not:

```bash
# One-time setup (macOS/Linux/Windows via rustup.rs)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## 0. Clone the Repository (one-time)

```bash
git clone https://github.com/Morpheum/mwvm.git
cd mwvm
```

(If the repo is not yet public, use the private clone URL or download the latest release tarball.)

## A. I just want to run any .wasm file locally (the most common first step)

Goal: Load and execute a WASM agent with full host functions immediately.

```bash
# Option 1: Use pre-built CLI binary (once released)
mwvm run examples/minimal_agent.wasm

# Option 2: Build & run from source (takes ~1–2 minutes first time)
cargo run --release -- examples/minimal_agent.wasm
```

What happens:
- The agent starts and can call `infer`, `vector_search`, `store_context`, etc.
- Output appears in your terminal (logs, inference results, messages)
- Everything runs locally — no chain, no gas, no waiting

Success indicator: You see log lines like `[INFO mwvm] Agent started`, possibly inference results or memory writes.

## B. I want to run local inference right now (very popular use-case)

Goal: Make your agent call real model inference locally via the `infer` host function.

```bash
# Run the example that demonstrates inference
cargo run --release -- examples/inference-demo.wasm
```

Or in your own agent (Rust pseudo-code example):

```rust
// Your WASM guest code
extern "C" {
    fn infer(request_ptr: u32, request_len: u32) -> i32;
}

#[no_mangle]
pub extern "C" fn run() {
    // Prepare a simple inference request
    let prompt = b"Write a haiku about decentralized AI";
    // ... encode to bytes, get pointer & length ...
    let result_len = unsafe { infer(ptr, len) };
    // Read result from memory...
}
```

MWVM automatically:
- Batches inference requests (ContinuousBatcher)
- Uses local model serving (configurable via EngineBuilder)
- Returns real outputs in simulation mode

## C. I want to launch a multi-agent swarm quickly

Goal: See agents talking to each other via `actor_messaging` and MessageBus.

```bash
# Simplest swarm: 20 identical agents chatting for 60 seconds
cargo run --release --bin mwvm-cli -- swarm \
  --count 20 \
  --wasm examples/swarm-chat.wasm \
  --duration 60
```

What you’ll see:
- Agents sending/receiving messages
- Topic-based pub/sub in action
- Emergent coordination (depending on the example)

Alternative: Run your own swarm logic

```bash
cargo run --release --bin mwvm-cli -- swarm \
  --count 50 \
  --wasm path/to/your_collective.wasm \
  --config swarm-config.toml
```

## D. I want to simulate on-chain-like constraints (parity testing)

Goal: Test how your agent behaves under chain-like restrictions without deploying.

```bash
# Run in simulation mode with default chain-like constraints
cargo run --release -- examples/your-agent.wasm --simulation-mode

# Or with specific mocks (low balance, denied authority, etc.)
cargo run --release -- examples/your-agent.wasm \
  --simulation-mode \
  --mock-authority-denied \
  --mock-low-gas
```

This helps you:
- Catch authority/KYA/VC failures early
- Test error handling for on-chain revert conditions
- Verify behavioral parity before real deployment

## E. I want my agent to be callable from other tools (MCP/A2A gateway)

Goal: Expose your agent via standard protocols so Claude, other agents, or front-ends can interact.

```bash
# Start the gateway (default port 8080)
cargo run --release --bin mwvm-cli -- gateway --port 8080

# Now curl or use Postman / another agent:
curl http://localhost:8080/mcp -d '{"jsonrpc":"2.0","method":"tools/list","id":1}'
```

You’ll get a JSON-RPC response listing available tools (your agent’s capabilities).

## Success Checklist – You’re Using MWVM Effectively If…

- [ ] You ran at least one `.wasm` file locally
- [ ] You saw host function calls succeeding (logs show inference, memory writes, messages)
- [ ] You tried one of the examples in `/examples/`
- [ ] You understand which parts are mocked locally vs. authoritative on-chain

## Next Steps After First Success

- Explore more examples → `ls examples/`
- Read deeper guides → [guides/](guides/)
  - inference-local.md
  - swarm-quickstart.md
  - simulation-parity.md
  - gateway.md
- Want to customize the engine? → Look at `mwvm-sdk` usage in examples
- Ready to contribute back? → [CONTRIBUTE.md](CONTRIBUTE.md)

You now have a powerful local runtime for Morpheum agents — no chain required.

Start experimenting.  
The network gets stronger with every agent that runs smarter locally first.

Questions? Open an issue or ping [@MorpheumX](https://x.com/MorpheumX)

Happy building.  