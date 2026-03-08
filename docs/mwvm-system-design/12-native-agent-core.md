# Native Agent Core — VM/Runtime for AI Agents

**Version**: 1.0  
**Date**: 08 March 2026  
**Status**: Design  
**Source**: Aligned with `mwvm/crates`

## 1. Overview

The Native Agent Core is the VM and runtime optimized for AI agents. MWVM implements the off-chain side: rich host functions, persistent memory, inference batching, and protocol gateways. Mormcore implements the on-chain side: thin deterministic host for consensus.

## 2. MWVM Host Functions (Implemented)

| Host | Purpose |
|------|---------|
| **infer** | Local inference via ContinuousBatcher; uses InferenceRequest from primitives |
| **store_context** | Store blob under key in LocalMemory |
| **vector_search** | Cosine-similarity search over embeddings |
| **zkml_verify** | Mock zkML verification (simulation) |
| **tee_verify** | Mock TEE attestation (simulation) |
| **actor_messaging** | Send message to agent (actor model) |

## 3. Persistent Memory

**LocalMemory** in mwvm-core provides:
- Key-value store (blake3-hashed keys, 2 KiB max key size)
- Vector store with configurable dimension (default 1536)
- Brute-force cosine-similarity search
- Implements MemoryBackend trait from morpheum-primitives

## 4. Inference

**ContinuousBatcher** in mwvm-core:
- Batches inference requests
- Optional (enabled via EngineBuilder.with_model_serving)
- Uses morpheum-primitives InferenceRequest; validates via Validatable

## 5. TEE/zkML Simulation

**zkml_tee** host module:
- Mock zkML verification for local testing
- Mock TEE attestation for local testing
- Optional (enabled via EngineBuilder.with_tee_simulation)

## 6. Actor Messaging

**actor_messaging** host module:
- Native send(agent_id, message) pattern
- Maps to A2A protocol; agents communicate via signed messages

## 7. Gateways (MCP, A2A, DID, x402)

**mwvm-gateway** exposes:
- MCP at `/mcp` — tools/list, tools/call
- A2A at `/a2a` — AgentCard
- DID at `/did/{agent-id}` — DID Document
- x402 at `/x402/pay` — Payment required (402 when unpaid)

## 8. Performance Targets

| Operation | Target |
|-----------|--------|
| Local inference (batched) | Continuous batching; throughput-focused |
| Vector search | Brute-force; suitable for local dev |
| Gateway latency | Axum; sub-ms routing |

## 9. Roadmap (Conceptual)

- **Phase 0 (Current):** Core engine, hosts, SDK, gateways, orchestrator, CLI, parity tests
- **Phase 1:** HNSW or similar for vector search at scale
- **Phase 2:** Optional gRPC hot-path to Mormcore for hybrid testing
- **Phase 3:** Full zkML/TEE integration (Mormcore)

## Related Documents

- [02-architecture.md](02-architecture.md) — System architecture
- [11-mwvm-vs-mormcore-vm.md](11-mwvm-vs-mormcore-vm.md) — MWVM vs Mormcore
- [13-mwvm-architecture-flow.md](13-mwvm-architecture-flow.md) — Execution flows
