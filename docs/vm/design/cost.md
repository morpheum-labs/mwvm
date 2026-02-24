## Morpheum WASM Smart Contract Deployment Design

**Version**: 1.0 (February 2026)  
**Compatible with**: Morpheum 2.0 9-Step DAG Consensus (Mormcore), Object-Centric MVCC + Block-STM Scheduler, Host API (as expanded previously), Flash Path, Frosty Epochs, Step-8 Recovery, and Constitutional Amendment.  

This is a **pure high-level architecture design** for deploying WASM smart contracts on Morpheum (no code, no pseudocode). Deployment is treated as a special type of transaction (Msg) that integrates seamlessly with the existing 9-step consensus pipeline — no changes required to the core mechanics. The process emphasizes **permissionless, gasless efficiency** (aligned with Morpheum's blockless/gasless design), while incorporating **anti-spam measures** via Pipes rate-limiting (Step-1) and **storage deposits** to prevent bloat.  

WASM deployment turns the bytecode into a **versioned object** (as per the object-centric state model), allowing for cheap instantiation, upgrades (migrations), and reuse across shards. All steps are deterministic, parallelizable (non-conflicting deploys via Flash path), and recoverable (bounded rollback in Step-8 reverts only recent objects without affecting finalized code).  

### 1. High-Level Deployment Overview

Deployment follows a **two-phase pattern** (inspired by CosmWasm/ink! but adapted to Morpheum's DAG + object model):  
1. **Store Code**: Upload and store the WASM bytecode as a reusable, versioned object (immutable after finality).  
2. **Instantiate Contract**: Create a runtime instance from stored code, with initial state and address.  

Optional: **Migrate** for upgrades.  

```
External Client → Tx Submission (MsgDeploy / MsgInstantiate)
↓ (Step 1: Ingress + MAV + Validation)
DAG Pipeline (Steps 2-9: Blocklace → Waves → Frosty → Finality → Recovery if needed → Amendment for config changes)
↓ (Post-Finality: Msg Router → WASM VM Handler)
Object Store Commit (RocksDB + TimescaleDB)
↓
Response: Code ID / Contract Address + Events
```

- **Why Two Phases?** Separates immutable code storage (heavy, one-time) from lightweight instantiation (reusable, cheap). Reduces redundancy — multiple instances share the same code object.  
- **Permissionless**: Any account can deploy (subject to Pipes rate-limits and storage deposit).  
- **Sharding**: Code objects sharded by hash (or user-specified key) for parallel deploys. Non-conflicting deploys (different shards) use Flash path for <3δ finality.  
- **Security**: Host validates WASM bytecode (e.g., no invalid ops, size limits via constitutional params). ZK/TEE optional for confidential deploys.  
- **Agentic-Friendly**: Agents can autonomously deploy via idempotency keys + multi-call batches.  

### 2. Deployment Tx Types & Metadata

All deployments are submitted as standard Msgs (with WASM-specific metadata):  

| Tx Type          | Description | Metadata Fields | Consensus Integration |
|------------------|-------------|-----------------|-----------------------|
| **MsgStoreCode** | Uploads WASM bytecode as a new code object. | `bytecode: Vec<u8>`, `code_hash: Hash`, `deposit_amount: u128` (storage fee), `permissions: Capabilities` (e.g., who can instantiate). | Goes through full 9 steps; bytecode becomes versioned object post-Staple (Step-7). Flash if no conflicts. |
| **MsgInstantiate** | Creates a contract instance from stored code. | `code_id: Hash`, `init_msg: Vec<u8>` (constructor args), `admin: ID` (optional migrator), `label: String` (human-readable). | Lighter; host calls WASM init via Block-STM. Atomic with state creation. |
| **MsgMigrate** | Upgrades an instance to new code (version bump). | `contract_addr: ID`, `new_code_id: Hash`, `migrate_msg: Vec<u8>`. | Host checks admin capability; runs migration logic post-finality. Rollback-safe (Step-8). |

- **Tx Validation (Step-1)**: Signature + ZK-proof (deposit sufficiency, bytecode validity). MAV batches similar deploys (e.g., same code). Urgent flag for critical deploys (e.g., protocol upgrades).  
- **Output**: Events emitted (e.g., `CodeStored {id: Hash}`) + response with ID/address.  

### 3. Detailed Deployment Flow (Mapped to 9-Step Consensus)

| Consensus Step | Deployment Interaction |
|----------------|------------------------|
| **1. Ingress + MAV** | Tx reception; validate bytecode (size < max via constitution, no malicious ops). Compute deposit (proportional to size). MAV groups deploys by shard/code_hash for efficiency. Reject if Pipes D > S (anti-spam). |
| **2. Blocklace Issue** | Tx as DAG vertex; pointers to deps (e.g., existing code objects). Balanced check: deposit covers storage. Flash mode for small, non-conflicting deploys (e.g., <100KB code). |
| **3–5. Waves** | Scheduler builds dep graph (code objects isolated). Parallel validation of multiple deploys if non-conflicting. M/L-notarizations include deposit proofs. |
| **6. Frosty Epochs** | On stall: EC/SC prioritizes pending deploys. Simplex fallback serializes if needed, but deploys rarely conflict. |
| **7. Finality/Staple** | Atomic commit: Store bytecode as object (version=1), deduct deposit, emit event. CAN gossip carries code_hash for verification. |
| **8. Accountability/Recovery** | On violation: Bounded rollback reverts code object creation (≤2Δ* ~100ms). No finalized code lost (safe genesis preserves). |
| **9. Constitutional Amendment** | Tx can amend deployment params (e.g., max bytecode size, deposit rate) via supermajority. New genesis propagates updates. |

- **Post-Deployment**: Contract address = hash(code_id + salt + sender). Instances are objects with `code_ref: Hash` + state. Calls use existing Host API (e.g., `object_read` for state).  
- **Upgrades**: MsgMigrate requires admin capability; host runs old → new migration in sandbox, commits new version atomically.  
- **Cleanup**: If code unused (no instances), optional MsgDeleteCode refunds deposit (post-finality).  

### 4. Security & Robustness Features

- **Validation**: Host scans bytecode for safety (e.g., no host escapes, valid WASM). Optional ZK-proof for verifiable deployment (prove no backdoors).  
- **Capabilities**: Stored code objects have perms (e.g., public instantiate vs. restricted).  
- **Recovery**: Step-8 guilt certs slash spammers (e.g., malformed bytecode). Rollback reverts deposits.  
- **Agentic**: Agents deploy via tools (e.g., simulate locally with cw-multi-test equivalent, then submit Tx). Idempotency keys prevent duplicate deploys.  

### 5. Suggested Costs

Morpheum is **gasless** (no execution fees), but to prevent storage spam, use a **refundable storage deposit** model (like NEAR/Substrate). Deposits lock native tokens (e.g., $MORPH) proportional to size, reclaimable on deletion. This aligns with anti-spam (Pipes) and encourages efficient code (optimize to <100KB via rust-optimizer).  

Based on benchmarks from similar WASM chains (as of 2026 data):  
- **CosmWasm (Cosmos)**: Deployment ~0.1-1 native token (e.g., uOSMO) for optimized 100-500KB contract, via gas (but Morpheum adapts to deposit).  
- **NEAR**: ~1 NEAR (~$5-10 USD in 2026) per 100KB, refundable staking.  
- **Polkadot (ink!)**: Storage deposit ~0.1-1 DOT (~$5-20 USD) per 100KB, plus minimal tx fee.  

**Recommendation for Morpheum**:  
- **Base Deposit Rate**: 1 $MORPH per 100KB of bytecode (refundable on code deletion).  
  - Example: 50KB optimized contract = 0.5 $MORPH locked.  
  - 500KB unoptimized = 5 $MORPH (incentivizes optimization).  
- **Instantiation/Migration**: Minimal ~0.01 $MORPH flat (for initial state object).  
- **Rationale**: Matches NEAR's model (efficient, refundable); low barrier (~$1-10 USD assuming $MORPH at $2-10). Adjustable via Step-9 amendment. No burn (unlike NEAR global contracts) to keep sovereign.  
- **USD Estimate (2026)**: At $5/MORPH, small deploy ~$2.50; large ~$25. Testnet: Free/zero deposit for dev.  

This design is **optimal, secure, and cost-effective** — ready for Mormcore implementation. If you want expansions (e.g., full Tx metadata schemas, Rust toolchain integration, or cost formulas), let me know, @MorpheumX!