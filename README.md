# Morpheum WASM VM (MWVM)

**Morpheum WASM Virtual Machine** — the WebAssembly smart contract runtime for the Morpheum blockchain, optimized for 9-step DAG consensus, object-centric state, and agentic workflows.

## Overview

MWVM executes WASM smart contracts on Morpheum with:

- **DAG-native execution** — Causal snapshots from blocklace; Block-STM parallel scheduler; Flash path for sub-3δ finality
- **Object-centric MVCC** — Versioned objects (ID + Owner + Version + Data); no global shared mutable state
- **43+ Host API functions** — Object ops, idempotency, oracle, staking/restaking, crosschain, KYA/DID delegation, Safe Native Infrastructure Wrappers (v2.5+), ZK/TEE/FHE
- **Gasless + deposit** — Refundable storage deposit (1 $MORPH / 100 KB); no execution gas
- **Agentic-first** — Idempotency keys, safe retries, multi-agent testing via MormTest + MCP

## Documentation

All design, cost, MEV, and test-framework documentation lives in [`docs/`](./docs/):

| Section | Contents |
|---------|----------|
| [**Proposals**](./docs/proposals/) | Design proposals, VM spec (v2.6), Host API, storage, I/O, KYA/DID delegation, Bucket-as-Service |
| [**Cost**](./docs/cost/) | Deployment flow, storage deposits, cost formulas |
| [**MEV**](./docs/mev/) | MEV analysis: WASM vs EVM |
| [**Test Framework**](./docs/test-framework/) | MormTest, agentic testing, MCP protocol |

**Start here**: [docs/README.md](./docs/README.md) — full index and quick reference.

**Idea developments**: [docs/government/pretext/prelogue.md](./docs/government/pretext/prelogue.md) — Web 4.0, agent taxonomy, Level 5 society (conceptual context).

## Compatibility

- **Mormcore** — 9-step DAG consensus (Blocklace, Waves, Frosty, Finality, Recovery, Amendment)
- **Flash path** — Fast finality for non-conflicting transactions
- **Object-centric state** — RocksDB + TimescaleDB backend

## Status

Design phase (February 2026). Specification is production-ready (v2.6); implementation targets Mormcore integration.
