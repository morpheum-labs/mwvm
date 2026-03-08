# MWVM Implementation Plan

**Version**: 1.0  
**Date**: 08 March 2026  
**Status**: Design  
**Source**: Aligned with `mwvm/crates`

## 1. Strategy & Design Principles

### Clean Architecture Layers

| Layer | Crates | Responsibility |
|-------|--------|----------------|
| **Domain** | morpheum-primitives | Single source of truth |
| **Core** | mwvm-core | Engine, hosts, memory — no CLI/gateway logic |
| **Application** | mwvm-sdk, mwvm-orchestrator | High-level APIs |
| **Adapters** | mwvm-gateway, mwvm-cli, mwvm-wasm | External interfaces |
| **Tests** | mwvm-tests, examples | Verification, parity, E2E |

### SOLID

- **Single Responsibility:** One module = one concern (e.g., host/infer only handles inference)
- **Open/Closed:** New hosts added via registration; no core changes
- **Liskov Substitution:** Implementations satisfy shared traits
- **Interface Segregation:** Small traits (MemoryBackend, etc.)
- **Dependency Inversion:** Depend on abstractions; Arc for shared state

### DRY

- Shared morpheum-primitives for types and signatures
- Re-exports from core; no duplication
- Common error types; bytemuck for zero-copy

## 2. Crate Implementation Order

| Phase | Crates | Verification |
|-------|--------|---------------|
| **0** | Workspace, morpheum-primitives extension | Workspace builds |
| **1** | mwvm-core | Engine + all hosts work |
| **2** | mwvm-sdk, mwvm-orchestrator | SDK and swarm tests pass |
| **3** | mwvm-gateway, mwvm-cli | Gateway and CLI tests pass |
| **4** | mwvm-wasm, examples | Bindings and examples run |
| **5** | mwvm-tests | Parity, integration, gateway E2E pass |
| **6** | Mormcore integration | Parity verified end-to-end |

## 3. Key Design Patterns

| Pattern | Usage |
|---------|-------|
| **Builder** | EngineBuilder, AgentBuilder, GatewayBuilder, SwarmBuilder |
| **Registry** | HostRegistry; register_all_hosts binds hosts to linker |
| **Repository** | LocalMemory — clean memory abstraction |
| **Facade** | Agent, SdkRuntime — simple public API |
| **Strategy** | SimulationMode — offline, fork, replay |
| **Observer** | MessageBus — topic-based publish/subscribe |

## 4. Fixtures

**mwvm-tests/fixtures:**
- minimal_agent.wat — Human-readable WAT; exports host stubs
- minimal_agent.wasm — Compiled; committed for CI
- generate-fixtures.sh — Compiles WAT to WASM
- README.md — Documentation

## 5. Verification

- Every phase ends with verification (build, test)
- Parity tests compare MWVM vs Mormcore on same WASM
- Integration tests cover full stack (core → SDK → orchestrator → gateway)
- Gateway E2E uses axum-test for real HTTP

## 6. Quality Gates

- No unsafe code (forbid(unsafe_code))
- Clippy + rustfmt
- Tracing for observability
- Comprehensive error types (thiserror)

## Related Documents

- [02-architecture.md](02-architecture.md) — System architecture
- [09-module-structure.md](09-module-structure.md) — Crate structure
- [13-mwvm-architecture-flow.md](13-mwvm-architecture-flow.md) — Execution flows
