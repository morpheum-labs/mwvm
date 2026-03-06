**Storage in WASM-based blockchain virtual machines** refers to how persistent data (state) is handled for smart contracts or runtime modules. Unlike the **Ethereum Virtual Machine (EVM)**, which uses a built-in account-based key-value storage model (256-bit words via opcodes like SLOAD/SSTORE), **pure WebAssembly (WASM)** has no native persistent storage concept. WASM provides **linear memory** — a contiguous, growable byte array for temporary runtime use during execution — but this memory is volatile and cleared after each call/transaction.

In blockchain contexts (e.g., Polkadot/Substrate, CosmWasm in Cosmos, NEAR, MultiversX, **MWVM**), storage is provided externally by the **host blockchain runtime**, not by WASM itself. The contract imports host functions (via WASI-like or custom imports) to read/write persistent state — either a global key-value store (often backed by LevelDB, RocksDB, or similar) or, in MWVM's case, an **object-centric versioned store** with ownership and capability checks.

### Core Storage Mechanics in WASM Blockchains

- **Linear Memory (Runtime/Transient)**  
  WASM's built-in memory model: A single linear byte array (growable via `memory.grow`). Used for stack, heap, local data during execution. Fast but **not persistent** — lost after the call ends. Great for computations, but state must be explicitly saved to host storage.

- **Persistent Storage (Blockchain-Hosted)**  
  Handled by the chain's runtime via imported host functions (e.g., `read`, `write`, `remove`, `iterate`).  
  - Data is stored in the blockchain's global state trie or database.  
  - Contracts pay **storage fees** (e.g., rent/deposit models in some chains) for occupied space.  
  - Keys are often prefixed by contract address/ID to isolate state.

### Comparison Across Major WASM Ecosystems

| Platform / VM          | Storage Model Details                                                                 | Key Features / Abstractions                          | Fees / Costs                                      | Notes / Strengths                                      |
|------------------------|---------------------------------------------------------------------------------------|------------------------------------------------------|---------------------------------------------------|--------------------------------------------------------|
| **CosmWasm** (Cosmos SDK chains) | Key-value store via `cosmwasm_std::Storage` trait. Contracts use `Storage` interface for get/set/remove. | `cw-storage-plus` crate: `Item<T>` (singleton), `Map<K,V>` (keyed collections), prefixed keys. | Gas for reads/writes + storage staking (some chains). | Rust-friendly abstractions; prevents common errors; modular for IBC/cross-chain. |
| **Polkadot / Substrate** (`pallet-contracts`) | Global trie-based storage; contracts access via host functions. ink! uses `Spread` / `Packed` layouts. | ink! storage traits: auto-prefixing, flushing changes. | Weight-based (computational + storage weights). | Deterministic; supports ink! (Rust) + other langs; integrates with XCM/cross-chain. |
| **NEAR Protocol**     | Account-based sharded storage; contracts have prefixed key-value entries. | High-level SDKs (Rust/AssemblyScript); global contracts reuse code to cut costs. | Storage staking: pay for bytes used (refundable on delete). | Sharded for scalability; "rent" model prevents bloat; efficient for large state. |
| **General WASM (e.g., MultiversX, EOSIO forks)** | Linear memory + host imports for key-value ops. | Custom abstractions per chain. | Varies (gas, RAM staking, etc.). | Focus on high TPS; memory-safe sandboxing. |
| **MWVM (Morpheum)** | Object-centric versioned storage via host imports (`object_read`, `object_write`, `object_create`, etc.). No raw KV — all state is versioned objects with ownership/capability. | Object management + MVCC + Block-STM scheduler; DAG causal order; idempotency keys for agents. | Gasless + refundable storage deposit (1 $MORPH / 100 KB); Step-9 amendable. | Object-centric eliminates races by design; integrates with 9-step DAG consensus, Flash path, Frosty epochs. See [io.md](./io.md), [keyhost.md](./keyhost.md). |

### Key Advantages Over EVM Storage
- **Efficiency** — Linear memory + host KV is often faster/cheaper than EVM's 256-bit slots (fewer reads/writes for complex data).
- **Flexibility** — Developers use native Rust structs/maps (via crates like `cw-storage-plus` or ink! storage) instead of manual packing/unpacking.
- **Security** — Sandboxed execution + memory safety (no buffer overflows); prefixes prevent accidental cross-contract access.
- **Cost Model** — Dynamic metering (based on actual ops/storage used) vs. EVM's fixed per-op gas.

### Limitations / Considerations
- **No direct memory persistence** — Must explicitly serialize/deserialize state to host storage → boilerplate but safer.
- **Storage bloat risk** — Chains use deposits/staking/rent to discourage unused data (e.g., NEAR refunds on delete).
- **Determinism** — All storage ops must be deterministic for consensus.

### MWVM: Object-Centric vs Plain KV

MWVM uses an **object-centric** model rather than raw key-value storage. Each piece of state is a versioned object (ID + Owner + Version + Data + Capability). Host functions (`object_read`, `object_write`, `object_create`, `object_delete`, `object_transfer`) enforce ownership and version checks. This design:

- **Eliminates races by construction** — No global shared mutable state; per-object versioning enables optimistic parallel execution (Block-STM).
- **Aligns with DAG consensus** — Reads return snapshots consistent with the tx's DAG predecessors; writes commit atomically after finality.
- **Supports agentic workflows** — Idempotency keys (`idempotency_check`, `idempotency_mark`) enable safe retries for AI agents.

See [io.md](./io.md) for load/write/execute flow and race prevention, and [keyhost.md](./keyhost.md) for the full Host API.

---

In short: WASM provides fast **transient linear memory** for execution, while **persistent storage** is outsourced to the blockchain host via imported functions and abstractions. This makes WASM VMs more performant and developer-friendly for complex apps (especially with Rust ecosystems) compared to EVM's rigid model — ideal for treasury staking, restaking, agentic ops, or cross-chain features where efficient state handling matters. MWVM extends this pattern with an object-centric, MVCC-backed model for maximum parallelism and race-free execution on DAG consensus.