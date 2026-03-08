# MWVM — Module Structure

**Version**: 1.0  
**Date**: 08 March 2026  
**Status**: Design  
**Source**: Aligned with `mwvm/crates`

## 1. Crate Hierarchy

| Layer | Crates | Responsibility |
|-------|--------|----------------|
| **Core** | mwvm-core | Engine, host functions, memory, batcher, simulation |
| **Application** | mwvm-sdk, mwvm-orchestrator | High-level APIs, swarm, message bus |
| **Adapters** | mwvm-gateway, mwvm-cli, mwvm-wasm | HTTP gateways, CLI, TypeScript bindings |
| **Tests** | mwvm-tests | Parity, integration, gateway E2E |

## 2. mwvm-core Structure

| Module | Purpose |
|--------|---------|
| engine | MwvmEngine, EngineBuilder, EngineConfig, AgentRuntime, StoreContext |
| linker | HostRegistry, register_all_hosts |
| host | infer, store_context, vector_search, zkml_tee, actor_messaging |
| memory | LocalMemory — KV + brute-force vector search |
| batcher | ContinuousBatcher |
| simulation | SimulationMode, Simulator |
| error | MwvmError |

## 3. mwvm-sdk Structure

| Module | Purpose |
|--------|---------|
| agent | Agent, AgentBuilder |
| runtime | SdkRuntime |
| config | SdkConfig |

Re-exports EngineBuilder, LocalMemory, MwvmEngine, SimulationMode, Simulator from core.

## 4. mwvm-gateway Structure

| Module | Purpose |
|--------|---------|
| gateway | Gateway, GatewayBuilder, GatewayConfig, AppState |
| mcp_server | MCP routes |
| a2a_server | A2A routes |
| did_resolver | DID routes |
| x402_handler | x402 routes |

## 5. mwvm-orchestrator Structure

| Module | Purpose |
|--------|---------|
| swarm | Swarm, SwarmBuilder, AgentSlot |
| message_bus | MessageBus, Event, SystemEvent, Subscription |

## 6. mwvm-cli Structure

| Command | Purpose |
|---------|---------|
| run | Run single agent from WASM file |
| swarm | Launch multi-agent swarm |
| gateway | Start MCP/A2A/DID/x402 gateway |
| test | Run MWVM test suite |

## 7. mwvm-wasm Structure

| Export | Purpose |
|--------|---------|
| McpToolCall | JSON-RPC request for tools/call |
| tools_list_request | JSON-RPC request for tools/list |
| hex_to_bytes | Parse hex to bytes |
| bytes_to_hex | Encode bytes as hex |

Note: mwvm-wasm targets wasm32-unknown-unknown; wasmtime does not run in browser. Bindings serialize requests to the gateway over HTTP.

## 8. Integration Points

| Interface | Purpose |
|-----------|---------|
| morpheum-primitives::vm | Types, opcodes, host signatures |
| morpheum-primitives::MemoryBackend | LocalMemory implements this trait |
| morpheum-primitives::Validatable | InferenceRequest validation |

## 9. Design Principle

**Thin facades, heavy core.** All logic lives in mwvm-core. SDK, gateway, orchestrator, and CLI are thin wrappers. mwvm-wasm is a lightweight client for gateway communication.

## Related Documents

- [02-architecture.md](02-architecture.md) — System architecture
- [10-scope-boundary.md](10-scope-boundary.md) — Scope boundary matrix
