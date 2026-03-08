# MWVM System — Overview

**Version**: 1.0  
**Date**: 08 March 2026  
**Status**: Design  
**Source**: Aligned with `mwvm/crates`

## Executive Summary

MWVM is the **standalone, portable off-chain WASM runtime and SDK** for Morpheum AI agents. It gives developers a rich local experience while guaranteeing behavioral parity with on-chain execution via shared primitives.

## Scope

| Component | Responsibility |
|-----------|----------------|
| **mwvm-core** | Engine, linker, host functions (infer, store_context, vector_search, zkml_tee, actor_messaging), LocalMemory, ContinuousBatcher, Simulator |
| **mwvm-sdk** | Agent, AgentBuilder, SdkRuntime, SdkConfig — high-level facade over core |
| **mwvm-gateway** | MCP, A2A, DID resolver, x402 — HTTP endpoints at canonical paths |
| **mwvm-orchestrator** | Swarm, MessageBus — multi-agent coordination, topic-based messaging |
| **mwvm-cli** | run, swarm, gateway, test — command-line interface |
| **mwvm-wasm** | TypeScript/WASM bindings for gateway clients (McpToolCall, tools_list_request, hex utils) |
| **mwvm-tests** | Parity, integration, gateway E2E — verification against Mormcore |

## Key Concepts

### Portable Off-Chain Runtime

- MWVM runs WASM agents locally with full wasmtime
- Host functions provide inference, persistent memory, vector search, TEE/zkML simulation, actor messaging
- No chain dependency for core flows; optional Portal RPC for hybrid testing

### Shared Primitives

- `morpheum-primitives` defines VM types, opcodes, host signatures
- MWVM and Mormcore both implement against this contract
- One change to primitives updates both runtimes

### Gateways

- MCP — tools/list, tools/call at `/mcp`
- A2A — AgentCard at `/a2a`
- DID — Document at `/did/{agent-id}`
- x402 — Payment at `/x402/pay` (402 when payment required)

### Multi-Agent Orchestration

- Swarm spawns many agents from shared WASM
- MessageBus provides topic-based publish/subscribe
- Targeted send and broadcast supported

## Design Principles

1. **Thin facades** — SDK, gateway, orchestrator wrap core; no logic duplication
2. **Builder pattern** — EngineBuilder, AgentBuilder, GatewayBuilder, SwarmBuilder
3. **DRY** — Shared primitives; re-exports from core
4. **Observable** — Tracing and structured logging throughout

## Related Documents

- [00-mwvm-business-scope.md](00-mwvm-business-scope.md) — Business scope and crate ownership
- [02-architecture.md](02-architecture.md) — System architecture and data flow
- [10-scope-boundary.md](10-scope-boundary.md) — Scope boundary matrix
- [13-mwvm-architecture-flow.md](13-mwvm-architecture-flow.md) — Architecture and execution flows
