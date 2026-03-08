# MWVM System Design — Design Documentation

**Version**: 1.0  
**Date**: 08 March 2026  
**Status**: Design  
**Source**: Aligned with `mwvm/crates` implementation

This directory contains the system design documentation for the Morpheum WASM VM (MWVM) ecosystem. The design reflects **exactly** the structure and responsibilities of the mwvm crates: a portable off-chain WASM runtime, SDK, orchestrator, gateway, and bindings for Morpheum AI agents.

## Document Index

### Crate-Aligned Design (Primary)

| Document | Description |
|----------|-------------|
| [00-mwvm-business-scope.md](00-mwvm-business-scope.md) | Business scope, core philosophy, crate responsibilities |
| [01-overview.md](01-overview.md) | Executive summary, key concepts, design principles |
| [02-architecture.md](02-architecture.md) | System architecture, crate layout, host functions, data flow |
| [09-module-structure.md](09-module-structure.md) | Crate structure, native vs WASM scope, integration points |
| [10-scope-boundary.md](10-scope-boundary.md) | Scope boundary and responsibility matrix |
| [11-mwvm-vs-mormcore-vm.md](11-mwvm-vs-mormcore-vm.md) | MWVM vs Mormcore VM responsibilities, shared primitives |
| [12-native-agent-core.md](12-native-agent-core.md) | Native Agent Core — VM/runtime optimized for AI agents |
| [13-mwvm-architecture-flow.md](13-mwvm-architecture-flow.md) | Architecture and execution flows |
| [14-mwvm-implementation-plan.md](14-mwvm-implementation-plan.md) | Phased rollout, design patterns |

### Mormcore Ecosystem Context (Integration Scope)

| Document | Description |
|----------|-------------|
| [03-bucket-as-service.md](03-bucket-as-service.md) | BaS rule set (Mormcore integration) |
| [04-governance.md](04-governance.md) | Hybrid governance (Mormcore integration) |
| [05-recursive-risk.md](05-recursive-risk.md) | Recursive risk controls (Mormcore integration) |
| [06-cost-deployment.md](06-cost-deployment.md) | Deployment costs (Mormcore integration) |
| [07-plugin-vm-extensibility.md](07-plugin-vm-extensibility.md) | Hook vs Pluggable patterns (Mormcore integration) |
| [08-securities-testing.md](08-securities-testing.md) | Security model, agentic testing (Mormcore integration) |

## Quick Reference

- **Core Philosophy**: Portable off-chain WASM runtime; shared primitives with Mormcore
- **Crates**: mwvm-core, mwvm-sdk, mwvm-orchestrator, mwvm-gateway, mwvm-cli, mwvm-wasm, mwvm-tests
- **Host Functions**: infer, store_context, vector_search, zkml_verify, tee_verify, actor_messaging
- **Gateways**: MCP, A2A, DID resolver, x402
- **CLI Commands**: run, swarm, gateway, test
