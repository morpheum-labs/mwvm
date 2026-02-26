# MWVM Architecture – Draft 1

**Document Type**: Foundational Architecture  
**Version**: Draft 1 (February 2026)  
**Status**: Design Document  
**Compatible with**: Morpheum 2.0 9-Step DAG Consensus (Mormcore), Object-Centric MVCC + Block-STM, Flash Path, Frosty Epochs, Step-8 Recovery, Constitutional Amendment, KYA/DID Delegation, Bucket-as-Service (BaS).

---

## Document Design

### Purpose

This document defines the **conceptual architecture** of the Morpheum WASM VM (MWVM). It serves as the foundational blueprint that all version drafts (draft2–draft11) and supporting documents (io, storage, keyhost) build upon. It answers:

- **What** MWVM is and why it exists
- **How** its major components fit together
- **Where** to find detailed specifications

### Scope

| In Scope | Out of Scope |
|----------|--------------|
| High-level component diagram | Implementation code or pseudocode |
| Core design principles | Full Host API signatures (→ [keyhost.md](./keyhost.md)) |
| Execution model overview | Load/write/execute details (→ [io.md](./io.md)) |
| State model rationale | Storage mechanics (→ [storage.md](./storage.md)) |
| 9-step DAG integration points | Version-specific features (→ draft5–draft11) |

### Audience

- Architects evaluating MWVM for integration
- Developers new to the MWVM ecosystem
- Contributors needing the "big picture" before diving into specs

### Document Relationships

```
archeciture.md (this doc)
    ├── io.md          — Load/write/execute, race prevention, MVCC + Block-STM
    ├── storage.md     — WASM storage model, object-centric vs KV
    ├── keyhost.md     — Host API (43+ functions)
    ├── draft1.md      — WASM feasibility in DAG, testbeds, pen-testing
    ├── draft2.md      — MWVM v1.0 high-level design
    └── draft11-v2.6.md — Current production spec (v2.6)
```

---

## 1. Executive Summary

**MWVM** is the production-ready WebAssembly smart contract VM for the Morpheum blockchain. It is designed for:

- **DAG-native execution** — Causal snapshots, Block-STM parallelism, Flash-path fast finality
- **Object-centric state** — Versioned objects with MVCC; no global shared mutable state
- **Gasless + deposit model** — Refundable storage deposits instead of execution gas
- **Agentic-first** — Idempotency keys, safe retries, multi-agent workflows
- **Host-mediated security** — All I/O via sandboxed Host API; WASM = pure compute
- **KYA/DID delegation** — Scoped, revocable agent authorization via Verifiable Credentials

### Core Philosophy: "Host is God, WASM is Pure Compute"

- **WASM module** = transient linear memory only (no persistent state, no syscalls, no randomness).
- **Every** interaction with the outside world goes through the Host API (sandboxed, deterministic).
- **Core protocol primitives** (multisig, CLAMM, bucket/perp, staking, bank transfers, etc.) are built-in native infrastructure.
- Access to native features is provided **only** through safe, high-level wrapper functions with KYA/VC delegation and resource quotas.

---

## 2. Architectural Overview

### High-Level Component Stack

```
┌─────────────────────────────────────────────────────────────────────────┐
│                        Mormcore Runtime                                  │
├─────────────────────────────────────────────────────────────────────────┤
│  Consensus Pipeline (9 Steps)                                             │
│  ├── Step 1: Ingress + MAV                                               │
│  ├── Step 2: Blocklace Issue                                              │
│  ├── Steps 3–5: Waves (Propose / Endorse / Ratify)                        │
│  ├── Step 6: Frosty Epochs                                                │
│  ├── Step 7: Finality / Staple                                            │
│  ├── Step 8: Accountability / Recovery                                    │
│  └── Step 9: Constitutional Amendment                                    │
├─────────────────────────────────────────────────────────────────────────┤
│  Msg Router (post-Step 7 / Flash path)                                    │
│       │                                                                   │
│       ▼                                                                   │
│  ┌─────────────────────────────────────────────────────────────────────┐ │
│  │  MWVM Executor (deterministic, sharded)                              │ │
│  │  ├── Scheduler: DAG-Aware Block-STM + MVCC                           │ │
│  │  ├── Object Store (versioned, RocksDB cache + TimescaleDB history)   │ │
│  │  └── Host Import Layer (sandboxed calls from WASM)                   │ │
│  └─────────────────────────────────────────────────────────────────────┘ │
│       │                                                                   │
│       ▼                                                                   │
│  WASM Module (transient linear memory only)                               │
└─────────────────────────────────────────────────────────────────────────┘
```

### Role of Each Layer

| Layer | Role |
|-------|------|
| **WASM** | Pure compute engine. All I/O, state, concurrency, and ordering via host imports. |
| **Host** | Provides deterministic sandbox, tracks reads/writes, enforces DAG causal order + final total order, performs atomic commits only on staples. |
| **Scheduler** | DAG-aware Block-STM: optimistic parallel execution of non-conflicting txs; re-execute only conflicts + dependents. |
| **Object Store** | Versioned objects with ownership and capability checks. No global shared mutable state. |
| **9-Step Consensus** | Unchanged. WASM VM is a new "module handler" plugged into the Msg Router after Step 7 (or Flash path). |

---

## 3. Core Design Principles

### 3.1 Object-Centric State

Every piece of persistent state is a **versioned object**:

| Field | Type | Description |
|-------|------|--------------|
| ID | Hash | Unique global identifier |
| Owner | Account / Contract ID | Capability-based access |
| Version | u64 | Monotonically increasing; tx declares expected version → reject stale |
| Data | Bytes | Serialized struct / custom format |
| Capabilities | List\<Cap\> | Read-only, mutable, transfer, etc. |
| Parent DAG Refs | List\<Hash\> | Causal predecessors for snapshot materialization |

**No global shared mutable state** → races impossible by construction.

### 3.2 DAG-Native Execution

| Feature | How It Works | Benefit |
|---------|--------------|---------|
| Causal Snapshot Materialization | `host_get_dag_context()` + exact versioned snapshot | Deterministic execution on partial-order DAG |
| Block-STM Scheduler | Tx declares objects; non-conflicting txs run in parallel | Maximized parallelism; minimal re-execution |
| Flash Path | Non-conflicting objects bypass waves | Sub-3δ finality for low-congestion txs |
| Frosty / Step-8 | Bounded rollback on guilt cert | ≤2Δ* revert; safe prefix preserved |

### 3.3 Host-Mediated Security

- All WASM → host calls are capability-checked, version-checked, and deterministic.
- Native protocol features (multisig, CLAMM, bank, orders, etc.) are **never exposed raw**.
- Access via **Safe Native Infrastructure Wrappers** with KYA/VC delegation and resource quotas.
- Optional overlays: ZK proof, TEE enclave, FHE for confidential compute.

### 3.4 Gasless + Deposit Model

- **No execution gas** — Refundable storage deposit (1 $MORPH / 100 KB).
- Constitutional parameters (Step 9) govern quotas, rate limits, and caps.
- See [cost.md](../cost/cost.md) for deployment design.

### 3.5 Agentic-First

- **Idempotency keys** — Agent-generated; host rejects duplicates; safe retries.
- **Per-object version** — Same-account parallel submissions; no single global nonce bottleneck.
- **KYA/DID delegation** — Scoped, revocable agent authorization via Verifiable Credentials.

---

## 4. Execution Model

### Load / Write / Execute Flow

| Step | WASM Module | Host Runtime |
|------|-------------|--------------|
| **Load** | Calls `object_read(id, expected_version)` | Returns snapshot consistent with DAG predecessors |
| **Execute** | Runs contract logic; all I/O via host imports | Tracks reads/writes; deterministic sandbox |
| **Write** | Calls `object_write(id, new_data, new_version)` | Buffers; commits atomically only after finality |

**Execution timing**: Always **post-finality** (after Step 7 staple or Flash safety). Host materializes exact state snapshot. Result = sequential execution in final total order (even if parallel execution occurred).

### Block-STM + MVCC

1. **Tx declares objects** it will touch (read-only vs mutable) in metadata.
2. **Scheduler** builds dependency DAG from object accesses.
3. **Executes** non-conflicting txs in parallel (optimistic).
4. **On conflict**: Re-execute only conflicting tx and dependents.
5. **Guarantee**: Final result = sequential execution in DAG consensus order.

See [io.md](./io.md) for full load/write/execute flow and race prevention.

---

## 5. State Model

### Storage Backend

- **RocksDB** — Fast KV cache, per-shard.
- **TimescaleDB** — Historical snapshots for replay/auditing.
- **Sharding** — Objects grouped by asset pair / shard key → parallel execution across Mormcore shards.

### Object-Centric vs Plain KV

MWVM uses **object-centric** storage (not raw key-value):

- **Eliminates races** — Per-object versioning; no global shared mutable state.
- **Aligns with DAG** — Reads return snapshots consistent with tx's DAG predecessors.
- **Supports agentic** — Idempotency keys enable safe retries.

See [storage.md](./storage.md) for comparison with CosmWasm, NEAR, Substrate.

---

## 6. Security Architecture

### Host API Sandbox

- All host functions are **sandboxed**, **capability-checked**, and **version-checked**.
- WASM has no direct access to filesystem, network, or randomness (except host-provided VRF seed).

### Permission Model (v2.5+)

- **Baseline**: All APIs require capability + version check.
- **High/Medium-Risk APIs**: Mandatory KYA VC with explicit business-logic claims.
- **Constitutional Flags** (Step 9 amendable): Rate limits, value caps, quotas.
- **Safe Mode**: Global/per-contract flag disables high-risk functions.

### Optional Overlays

| Feature | Integration |
|---------|-------------|
| **ZK Verifiability** | Optional zkWASM wrapper; host can require ZK proof for high-stakes calls |
| **TEE** | VM instance runs inside Intel SGX/AWS Nitro enclave (optional per-shard) |
| **FHE** | Host provides FHE ops as imports; contracts compute on ciphertext |

---

## 7. Integration with 9-Step DAG Consensus

| Consensus Step | MWVM Interaction |
|----------------|------------------|
| **1. Ingress + MAV** | Signature + ZK-proof validation. Object declarations extracted. Urgent flag → Flash path. |
| **2. Blocklace Issue** | Tx becomes vertex in blocklace. Flash mode → immediate small block for non-conflicting txs. |
| **3–5. Waves** | Scheduler builds dependency DAG. Optimistic parallel execution. |
| **6. Frosty Epochs** | On stuck: re-scheduling of pending object txs. Simplex fallback serializes conflicts. |
| **7. Finality/Staple** | Atomic commit: bump versions, persist writes, emit events. |
| **8. Accountability/Recovery** | On guilt cert: bounded rollback; re-execute only affected dependents. |
| **9. Constitutional Amendment** | Amendment tx can mutate config objects (e.g., max object size, gas table, VM params). |

---

## 8. Where to Go Next

| Need | Document |
|------|----------|
| Current production spec | [draft11-v2.6.md](./draft11-v2.6.md) |
| Host API reference (43+ functions) | [keyhost.md](./keyhost.md) |
| Load/write/execute, race prevention | [io.md](./io.md) |
| WASM storage model | [storage.md](./storage.md) |
| WASM feasibility, testbeds, pen-testing | [draft1.md](./draft1.md) |
| MWVM v1.0 high-level design | [draft2.md](./draft2.md) |
| VM choice (ZK Cairo / Move / WASM) | [comparison.md](./comparison.md) |
| SDK architecture | [sdk-opensource.md](./sdk-opensource.md) |

---

## Document History

| Version | Date | Changes |
|---------|------|---------|
| Draft 1 | February 2026 | Initial architecture document design |
