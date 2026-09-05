<!-- morpheum-workspace v2026-09-05 — shared blocks synced by sync.sh; edit prose freely -->
# mwvm

The Morpheum WASM VM: an off-chain runtime (wasmtime) for building and testing AI agents
with the **exact same WASM bytecode** that executes on-chain, plus dev-time capabilities
(local inference, batching, persistent memory, swarms) and MCP/A2A/DID/x402 gateway
servers.

**This repo is PUBLIC on GitHub.** Everything committed here is public content.

## Status — read before trusting existing code

This repo is **stale relative to the rest of the org** (no commits since 2026-03, no CI,
edition 2021 while sibling repos have moved on). Treat existing code as possibly outdated
against the on-chain VM; re-verify parity before building on it.

## The contract: behavioral parity with the on-chain VM

The entire value of mwvm is that agent bytecode behaves identically off-chain and
on-chain. `scripts/verify-parity.sh` is the most critical check in the project;
`crates/mwvm-tests/tests/parity.rs` pins it. Any change to `mwvm-core`'s engine, linker,
memory, or simulation runs parity before anything else. Dev-time conveniences must never
alter observable execution semantics.

## Layout

- `crates/mwvm-core` — wasmtime engine, host functions, linker, memory, batcher
- `crates/mwvm-sdk` — high-level agent/runtime API; `crates/mwvm-orchestrator` — swarms
- `crates/mwvm-gateway` — axum MCP/A2A/DID/x402 servers; `crates/mwvm-cli` — default binary
- `crates/mwvm-tests` — parity/integration/gateway e2e + fixtures; `examples/`

## Commands `[host]` (builds on the host)

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
scripts/verify-parity.sh        # the gate that matters
cargo run --example minimal_agent
```

There is no CI — every gate you run locally is the only gate that ran; list them in the PR.

<!-- framework:begin ripple -->
## Cross-repo ripple

- Depends on siblings: `../morpheum-primitives` only.
- Dependents: none.
- The binding contract is behavioral **parity with mormcore's AgentCore VM** — the same
  WASM bytecode must behave identically here and on-chain. `scripts/verify-parity.sh` is
  the gate; any engine/linker/memory change runs it.
<!-- framework:end ripple -->

## Verification

- `scripts/verify-parity.sh` + `scripts/verify-core.sh`, then the standard cargo gates.
  No CI exists to catch what you skip.
