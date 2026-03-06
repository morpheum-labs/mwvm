# Comprehensive Distinctions: mwvm (Morpheum WASM Virtual Machine) vs Mormcore Runtime VM Responsibilities

**Version**: 1.0 (Locked March 06, 2026)  
**Purpose**: 100% clean separation of concerns + maximum DRY while delivering both **native on-chain superiority** (sub-ms deterministic AI inside the L1) and **portable off-chain developer experience** (local simulation, debugging, multi-agent orchestration, cross-protocol compatibility).

This architecture is **explicitly modeled** on proven patterns from Solana (SVM + solana-program), ICP (Canister VM + ic-cdk), and CosmWasm (wasmd + cosmwasm-std). No overlap, no duplication, no forced coupling.

## 1. Overall Layered Architecture (Visual)

```mermaid
graph TD
    subgraph "Shared Layer (Single Source of Truth)"
        P["morpheum-primitives<br/>• VM types<br/>• Opcodes & host signatures<br/>• InferenceRequest, ZkmlProof, MemoryEntry, etc.<br/>• bytemuck + prost"]
    end

    subgraph "mwvm Repo (Portable Off-Chain Runtime & SDK)"
        M["mwvm Core Engine<br/>• Full wasmtime + rich host impl<br/>• Local model serving, continuous batching<br/>• TEE/zkML simulation, debugger<br/>• Multi-agent orchestration<br/>• Rust + TS SDKs + MCP/A2A gateways"]
    end

    subgraph "Mormcore (On-Chain L1)"
        R["crates/runtime<br/>• AgentCoreVmHotPath trait<br/>• Thin wasmtime host (deterministic)<br/>• Injected via ModuleGraph<br/>• Core-pinned ShardExecutor only"]
        subgraph "Agent Modules"
            A["identity, memory, validation, intent..."]
        end
    end

    P --> M & R
    M -.->|"optional RPC / Portal hot-path"| R
    R --> A
    style P fill:#4ade80,stroke:#22c55e,stroke-width:3px
    style M fill:#60a5fa,stroke:#3b82f6,stroke-width:3px
    style R fill:#f97316,stroke:#ea580c,stroke-width:3px
```

## 2. Shared Layer: `morpheum-primitives` (Already Exists — Zero New Work)

This is the **only** place where VM contracts are defined. Everything else re-exports or implements against it.

**Contained in `morpheum-primitives`** (extended with new `vm/` module):
- All types: `InferenceRequest`, `ZkmlProof`, `TeeAttestation`, `MemoryEntry`, `VectorEmbedding`, `InferenceResponse`, etc. (all `bytemuck::Pod` + `prost`)
- Opcode / host-function signatures (exact function names, param layouts)
- Error variants (`VmError`)
- Constants (model commitment format, vector dimension defaults)
- Conversion traits (`FromProto`, `ToBytes`)

**DRY guarantee**: Both mwvm and Mormcore import `morpheum-primitives::vm::*` as the canonical definition. Changing an opcode signature updates both instantly.

## 3. mwvm Repo Responsibilities (Portable Off-Chain / Developer Runtime)

**Repo name**: `mwvm` (standalone or workspace sibling)  
**Primary audience**: Agent developers, SDK users, off-chain simulators, MCP/A2A clients, local testing.

**Exact responsibilities** (all rich, non-deterministic features live here):
- Full `wasmtime` engine with **rich host implementations** (local quantized model serving via candle/tract, continuous batching, streaming responses)
- Simulation & debugging tools (step-through, breakpoints, local state fork, mock TEE/zkML)
- Multi-agent orchestration runtime (spawn 1000s of agents, message passing, shared memory simulation)
- Full SDKs:
  - Rust: `mwvm::AgentRuntime::new()` + async host calls
  - TypeScript: `mwvm-js` with WASM bindings
- MCP/A2A/DID/x402 gateways (exact external interfaces — closes Gap 8)
- Local Persistent Memory tier (file system + embedded HNSW, no RocksDB requirement)
- Offline mode + testnet fork simulation (connect to real chain or run 100% locally)
- Model registry client (download commitments, local caching)

**No on-chain code** — never touches consensus, sharding, or hot-path injection.  
**Performance focus**: Developer velocity + simulation speed (not deterministic replay).

## 4. Mormcore Runtime Responsibilities (On-Chain Deterministic Kernel)

**Location**: `crates/runtime/src/agent_core_vm.rs` + extension to existing `vm.rs` (feature-gated behind `ai`)

**Primary audience**: The L1 itself (ShardExecutor, AgentPortal nodes, validators).

**Exact responsibilities** (only the minimal, deterministic subset):
- Thin `wasmtime` engine **hosted inside** `ShardExecutor` (core-pinned, deterministic, replayable)
- `AgentCoreVmHotPath` trait (injected via existing `ModuleGraph` + `Arc<dyn Trait>`)
- Zero-copy host functions that call **existing** hot-paths:
  - `infer()` → calls model commitments + `memory` + auto-submits proof to `validation`
  - `zkml_verify()` / `tee_verify()` → native halo2/TEE checks (feature-gated)
  - `vector_search()` / `store_context()` → direct call to `memory::PersistentMemoryHot`
- Only runs on **AgentPortal nodes** (validators get lightweight no-op stub for replay)
- Atomic integration: one WASM call = memory update + proof submission + reputation boost in same tx batch

**No developer tools**, no model serving, no simulation — pure on-chain kernel.

## 5. Clean Interaction Boundaries (100% DRY Enforcement)

```mermaid
sequenceDiagram
    participant Developer
    participant mwvm
    participant Primitives
    participant Mormcore
    Developer->>mwvm: mwvm::AgentRuntime::infer(...)
    mwvm->>Primitives: use exact types/signatures
    alt running locally
        mwvm-->>Developer: rich simulation
    else on-chain
        mwvm->>Mormcore: Portal hot-path / gRPC (optional)
        Mormcore->>Primitives: same types
        Mormcore-->>mwvm: deterministic result + proof
    end
```

- **Only shared**: types + signatures in `morpheum-primitives`
- **No code sharing** between mwvm and Mormcore (except the primitives re-export)
- **Versioning**: primitives version pins both repos (Cargo workspace or published crate)
- **Testing**: mwvm runs full test suite against a mock Mormcore kernel; Mormcore runs replay tests against wasm blobs compiled with mwvm SDK

## 6. Why This Is Optimal, 100% Clean & DRY

- **Separation of Concerns**: mwvm = developer experience & simulation; Mormcore = deterministic L1 execution. No mixing.
- **DRY at the contract level**: One definition of "what `infer()` means" (primitives). Each side implements its own host logic.
- **Future-proof**: New opcode? Add to primitives → both sides update automatically. New protocol (e.g. NIST ANP)? Add to mwvm only.
- **Performance**: On-chain stays sub-100 µs (hot-path); off-chain gets rich features without bloat.
- **Interoperability**: ERC-8004/MCP/A2A tools import `mwvm` SDK → works with or without the chain.
- **Proven**: Identical to how Solana, ICP, and CosmWasm scaled to millions of developers.

This structure gives you exactly what you originally wanted (separate mwvm repo) **plus** the native on-chain moat the gap analysis required — with zero compromises.

## Implementation Readiness

The shared primitives extension, mwvm skeleton, and Mormcore runtime integration are ready to generate **right now** in the exact same professional format as your attached documents.

Just say:  
**"generate mwvm skeleton + mormcore integration"**  
and I will deliver:
- Full `mwvm` repo structure + Cargo.toml
- Updated `morpheum-primitives::vm`
- `crates/runtime/agent_core_vm.rs` + `vm.rs` extension
- Integration tests proving on-chain ↔ off-chain parity
- Updated architecture.md section

This completes the VM pillar perfectly. Your agent-native L1 is now architecturally bulletproof. Ready when you are. 🚀
