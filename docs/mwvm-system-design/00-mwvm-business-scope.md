# Business Scope of the MWVM Ecosystem

**Version**: 1.0  
**Date**: 08 March 2026  
**Status**: Design  
**Source**: Aligned with `mwvm/crates`

## 1. Core Business Objective

MWVM (Morpheum WASM VM) is the **portable off-chain WASM runtime and SDK** for Morpheum AI agents. It provides:

- **Rich WASM execution** — Full wasmtime engine with AI host functions (inference, vector search, persistent memory, TEE/zkML simulation)
- **Developer experience** — Rust SDK, TypeScript bindings, CLI, multi-agent orchestration
- **Protocol gateways** — MCP, A2A, DID resolver, x402 — for interoperability with Claude, Google ADK, ERC-8004 tools
- **Behavioral parity** — Shared primitives with Mormcore; agents compile once and run locally or on-chain

## 2. Crate Ownership

| Crate | Sole Owner Of |
|-------|---------------|
| **mwvm-core** | Engine, linker, host functions, LocalMemory, batcher, simulation |
| **mwvm-sdk** | Agent, AgentBuilder, SdkRuntime, SdkConfig — high-level facade |
| **mwvm-gateway** | MCP, A2A, DID resolver, x402 — HTTP endpoints |
| **mwvm-orchestrator** | Swarm, MessageBus — multi-agent coordination |
| **mwvm-cli** | run, swarm, gateway, test commands |
| **mwvm-wasm** | TypeScript/WASM bindings for gateway clients |
| **mwvm-tests** | Parity, integration, gateway E2E tests |

## 3. Boundary with Mormcore

MWVM is **off-chain only**. It never touches consensus, sharding, or hot-path injection. Mormcore hosts the thin deterministic AgentCore VM for on-chain execution. Both share `morpheum-primitives` for types, opcodes, and host signatures.

| MWVM Owns | Mormcore Owns |
|-----------|---------------|
| Local simulation, debugging, orchestration | Consensus, state transitions, validation |
| MCP/A2A/DID/x402 gateways | On-chain AgentPortal hot-path |
| Rich host implementations (local inference, batching) | Thin deterministic host (sub-100 µs) |
| Developer SDK, CLI, TypeScript bindings | ShardExecutor, ModuleGraph |

## 4. Core Scope (Implemented in Crates)

| Category | Included? | Detail |
|----------|-----------|--------|
| WASM Engine | YES | wasmtime, linker, host registration |
| Host Functions | YES | infer, store_context, vector_search, zkml_tee, actor_messaging |
| Persistent Memory | YES | LocalMemory — KV store + brute-force vector search |
| Continuous Batching | YES | ContinuousBatcher for inference |
| Simulation | YES | SimulationMode, Simulator — fork, replay, offline |
| SDK | YES | Agent, AgentBuilder, SdkRuntime |
| Gateways | YES | MCP, A2A, DID, x402 |
| Orchestrator | YES | Swarm, MessageBus |
| CLI | YES | run, swarm, gateway, test |
| TypeScript Bindings | YES | mwvm-wasm — McpToolCall, tools_list_request, hex utils |
| Parity Tests | YES | Same WASM on MWVM and Mormcore; identical results |

## 5. Non-Functional Requirements

- **Portable** — Runs locally, in CI, or against testnet; no chain dependency for core flows
- **Deterministic replay** — Simulation mode matches on-chain behavior (shared primitives)
- **Zero unsafe** — All crates forbid unsafe code
- **Observable** — Tracing throughout; structured logging

## 6. Out-of-Scope (for MWVM Crates)

- **On-chain execution** — Belongs to Mormcore
- **Bucket-as-Service, KYA/VC, governance** — Mormcore ecosystem (see 03–08)
- **Raw native protocol access** — MWVM provides host functions; native infra is Mormcore

## 7. Integration Points

- **morpheum-primitives** — Single source of truth for VM types, opcodes, host signatures
- **Mormcore** — Optional RPC/Portal hot-path for hybrid local ↔ on-chain testing
- **MCP/A2A/DID/x402** — Standard protocols; gateways expose canonical endpoints

---

**Bottom line**: MWVM is the **developer runtime** for Morpheum AI agents — rich, portable, and parity-aligned with on-chain execution.
