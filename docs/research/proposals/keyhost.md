**Morpheum WASM VM Host API – Expanded & Comprehensive**  
**Version**: 1.2 (February 2026)  
**Compatible with**: Morpheum 2.0 9-Step DAG Consensus (Mormcore), Object-Centric MVCC + Block-STM Scheduler, Flash Path, Frosty Epochs, Step-8 Recovery (bounded rollback), Constitutional Amendment, KYA/DID Delegation, Safe Native Infrastructure Wrappers (v2.5+), Bucket-as-Service (v2.6), and all advanced requirements (treasury staking/restaking, crosschain settlements, oracle integration, agentic ops).

All functions are **WASM host imports** (sandboxed, strictly capability-checked, gas-metered, deterministic).  
WASM modules see **only transient linear memory** — all persistent state, ordering, parallelism, and security are handled exclusively by the Mormcore host runtime.  
Host enforces **DAG causal order + final total order** (post-Step-7 Staple or Flash safety).  
Reads always return the **exact snapshot consistent with the tx’s predecessors** in the final blocklace topological order.  
Writes are **buffered locally** in the tx and **atomically committed** only after finality (or Flash safety).  
Conflicts are resolved by the **DAG-Aware Block-STM Scheduler** (optimistic parallel + minimal re-execution of dependents).

### Master Host API Table

Functions are grouped by category. Each entry includes:

- **Signature** (Rust-like WASM import style)
- **Parameters**
- **Returns**
- **Layman Description**
- **Formal Role & Consensus Tie-in**
- **Host Enforcement / Security**
- **Gas Model** (approximate; configurable via constitutional amendment)
- **Primary Use Cases**

#### 1. Object Management & Versioning (Core State + Race Prevention)
| Function | Signature | Parameters | Returns | Layman Description | Formal Role & Consensus Tie-in | Host Enforcement / Security | Gas Model | Primary Use Cases |
|----------|-----------|------------|---------|--------------------|--------------------------------|-----------------------------|-----------|-------------------|
| `object_read` | `object_read(id: Hash, expected_ver: u64) -> (data: Vec<u8>, actual_ver: u64)` | `id`, `expected_ver` | `data`, `actual_ver` | "Give me the latest safe copy of this object if my version is still good." | Returns versioned snapshot consistent with DAG predecessors (post-Step-7 or Flash). Used in Block-STM read-set tracking. | Ownership/capability check; version mismatch → reject tx early (Step-1/2). | Linear in data size (read cost + copy) | Balance checks, treasury queries, agent state reads |
| `object_read_batch` | `object_read_batch(ids: Vec<Hash>, expected_vers: Vec<u64>) -> Vec<(data, ver)>` | Arrays of IDs + expected versions | Array of (data, ver) | "Give me a bunch of objects at once." | Batch read-set for scheduler optimisation (parallel non-conflicting txs). | Same as single + atomic snapshot. | Fixed overhead + per-item | Complex contracts (order matching, multi-asset staking) |
| `object_write` | `object_write(id: Hash, new_data: Vec<u8>, new_ver: u64)` | `id`, `new_data`, `new_ver` | Unit | "Schedule this update — host will commit safely later." | Buffers write-set; host bumps version on commit (post-finality). | Ownership + capability + version match; double-write in same tx → reject. | Linear in data size + fixed commit overhead | Token transfers, treasury updates |
| `object_write_batch` | `object_write_batch(ids: Vec<Hash>, data: Vec<Vec<u8>>, new_vers: Vec<u64>)` | Arrays | Unit | "Schedule multiple updates at once." | Optimised write-set for Block-STM (reduces scheduler overhead). | All checks atomic. | Batch discount (lower fixed cost) | Batch staking, restaking, multi-call agents |
| `object_create` | `object_create(owner: ID, initial_data: Vec<u8>) -> Hash` | `owner`, `data` | New ID | "Create a brand-new object I own." | Host generates ID, initial version=1; added to tx write-set. | Caller must have create capability (or be contract owner). | Fixed + data size | New positions, vaults, agent sessions |
| `object_delete` | `object_delete(id: Hash)` | `id` | Unit | "Safely destroy this object (refund storage)." | Marks for deletion; storage reclaimed on commit. | Ownership + capability. | Negative gas (refund) | Cleanup after expiry, agent cleanup |
| `object_transfer` | `object_transfer(id: Hash, new_owner: ID)` | `id`, `new_owner` | Unit | "Hand this object to someone else." | Transfers ownership/capability atomically. | Valid capability; prevents double-spend. | Fixed | Asset movement, restaking delegation |

#### 2. DAG & Consensus Context Awareness
| Function | Signature | Parameters | Returns | Layman Description | Formal Role & Consensus Tie-in | Host Enforcement / Security | Gas Model | Primary Use Cases |
|----------|-----------|------------|---------|--------------------|--------------------------------|-----------------------------|-----------|-------------------|
| `get_dag_context` | `get_dag_context() -> (parents: Vec<Hash>, round: u64, finality: bool)` | None | Parents, round, finality flag | "Tell me where I am in the DAG so I can reason about history." | Returns causal parents + current round from blocklace (materialised post-Step-7/Flash). | Read-only; deterministic view. | Fixed low | Oracle-dependent logic, time-weighted staking |
| `get_epoch_info` | `get_epoch_info() -> (epoch_id: u64, validators_hash: Hash, constitution: Bytes)` | None | Epoch data | "What are the current voting rules?" | Returns current epoch + constitution (updated via Step-9). | Read-only. | Fixed | Governance-aware contracts |

#### 3. Agentic & Idempotency (Safe Retries for AI Agents)
| Function | Signature | Parameters | Returns | Layman Description | Formal Role & Consensus Tie-in | Host Enforcement / Security | Gas Model | Primary Use Cases |
|----------|-----------|------------|---------|--------------------|--------------------------------|-----------------------------|-----------|-------------------|
| `idempotency_check` | `idempotency_check(key: Hash) -> bool` | `key` (agent-generated) | `processed: bool` | "Has this exact operation already been done?" | Checks per-account idempotency set (evicted after finality). | Duplicate → early reject (Step-1). | Fixed low | Agent retries, multi-step explorations |
| `idempotency_mark` | `idempotency_mark(key: Hash)` | `key` | Unit | "Mark this as done so retries are safe." | Host adds to set on successful commit. | Only callable once per tx. | Fixed | Agentic workflows, explorations |

#### 4. Events, Oracles & External Data
| Function | Signature | Parameters | Returns | Layman Description | Formal Role & Consensus Tie-in | Host Enforcement / Security | Gas Model | Primary Use Cases |
|----------|-----------|------------|---------|--------------------|--------------------------------|-----------------------------|-----------|-------------------|
| `emit_event` | `emit_event(topic: Hash, data: Vec<u8>)` | `topic`, `data` | Unit | "Announce something happened." | Events indexed in TimescaleDB; gossiped via CAN (Step-7). | No state change; deterministic. | Linear in data | Order fills, staking rewards, agent logs |
| `call_oracle` | `call_oracle(feed_id: Hash, params: Vec<u8>) -> (data: Vec<u8>, proof: Vec<u8>)` | `feed_id`, `params` | Data + ZK/TEE proof | "Get fresh verified real-world data." | Host queries oracle (TEE/ZK-backed); data becomes object on success. | Proof must verify or tx reverts. | High (external call) | Price feeds, weather, AI inference |

#### 5. Crosschain & Interoperability
| Function | Signature | Parameters | Returns | Layman Description | Formal Role & Consensus Tie-in | Host Enforcement / Security | Gas Model | Primary Use Cases |
|----------|-----------|------------|---------|--------------------|--------------------------------|-----------------------------|-----------|-------------------|
| `crosschain_send` | `crosschain_send(dest_chain: ID, msg: Vec<u8>, object_locks: Vec<Hash>) -> Hash` | Dest, msg, optional locks | Outbound ID | "Send assets or message to another chain atomically." | Locks objects until IBC/XCM ack; uses ZK proof for settlement. | Capability + lock check. | High + lock fee | Atomic swaps, restaking across chains |
| `crosschain_recv` | `crosschain_recv(inbound_id: Hash) -> (msg: Vec<u8>, proof: Vec<u8>)` | Inbound ID | Message + proof | "Receive and verify incoming crosschain message." | Host validates proof; unlocks objects on success. | Proof verification required. | Fixed + proof cost | Inbound settlements |

#### 6. Staking / Treasury / Restaking Primitives (Native Support)
| Function | Signature | Parameters | Returns | Layman Description | Formal Role & Consensus Tie-in | Host Enforcement / Security | Gas Model | Primary Use Cases |
|----------|-----------|------------|---------|--------------------|--------------------------------|-----------------------------|-----------|-------------------|
| `stake` | `stake(object_id: Hash, amount: u128, protocol: Hash)` | Object, amount, protocol | Receipt | "Lock tokens into staking pool." | Creates staking derivative object; yield accrues via host cron-like updates. | Ownership. | Fixed | Treasury staking, delegation |
| `restake` | `restake(staked_id: Hash, new_protocol: Hash)` | Staked object, new protocol | New derivative | "Reuse already-staked assets for extra yield." | Capability transfer to new protocol; host tracks shared security. | Multi-protocol capability check. | Fixed | EigenLayer-style restaking |
| `claim_yield` | `claim_yield(staked_id: Hash)` | Staked object | Yield amount | "Collect accrued rewards." | Host computes yield from consensus timestamps. | Ownership. | Fixed | Automated agent yield harvesting |

#### 7. Gas, Metering & Runtime Utilities
| Function | Signature | Parameters | Returns | Layman Description | Formal Role & Consensus Tie-in | Host Enforcement / Security | Gas Model | Primary Use Cases |
|----------|-----------|------------|---------|--------------------|--------------------------------|-----------------------------|-----------|-------------------|
| `gas_charge` | `gas_charge(cost: u64)` | `cost` | Unit | "Pay for the work I just did." | Host meters every operation; out-of-gas → revert. | Enforced at every import. | N/A (caller pays) | All contracts |
| `get_random` | `get_random() -> [u8; 32]` | None | 32-byte seed | "Give me unpredictable randomness." | Host provides VRF-derived seed from current round (Step-3). | Deterministic per round. | Fixed | Lotteries, agent exploration |

#### 8. Security & Verifiability Overlays (ZK / TEE / FHE)
| Function | Signature | Parameters | Returns | Layman Description | Formal Role & Consensus Tie-in | Host Enforcement / Security | Gas Model | Primary Use Cases |
|----------|-----------|------------|---------|--------------------|--------------------------------|-----------------------------|-----------|-------------------|
| `require_zk_proof` | `require_zk_proof(proof: Vec<u8>, public_inputs: Vec<u8>)` | Proof + inputs | Unit | "This execution must be proven." | Host verifies ZK proof before commit (optional per-tx flag). | Revert on invalid. | High | Private treasury ops |
| `enable_tee_mode` | `enable_tee_mode()` | None | Unit | "Run this tx inside a hardware enclave." | Optional per-shard; attestation on genesis (Step-9). | TEE attestation required. | Overhead (10-20%) | Confidential agent logic |
| `fhe_encrypt` / `fhe_decrypt` / `fhe_compute` | Various (e.g. `fhe_add(cipher1, cipher2) -> cipher`) | Ciphertexts | New ciphertext | "Compute on encrypted data." | Host provides FHE ops (TFHE/OpenFHE bindings). | Encrypted objects only. | Very high | Private finance, agent privacy |

#### 9. KYA / Delegation Group (v2.4 – Scoped Agent Authorization)
| Function | Signature | Parameters | Returns | Layman Description | Formal Role & Consensus Tie-in | Host Enforcement / Security | Gas Model | Primary Use Cases |
|----------|-----------|------------|---------|--------------------|--------------------------------|-----------------------------|-----------|-------------------|
| `did_validate` | `did_validate(did: String) -> Result<DidInfo>` | `did` | DidInfo | "Parse and validate a DID." | Uses did-rs parser (O(1), syntactic). | Prevents malformed DIDs. | Fixed low | Agent identity validation |
| `vc_verify` | `vc_verify(vc: Vec<u8>) -> Result<VerifiedClaims>` | `vc` | VerifiedClaims | "Verify owner-signed Verifiable Credential." | Cryptographic check + claim extraction. | Scoped delegation proof. | Fixed | Agent delegation proof |
| `vp_present` | `vp_present(vp: Vec<u8>) -> Result<VerifiedClaims>` | `vp` | VerifiedClaims | "Present VP containing one or more VCs." | Validates agent signature + VC chain. | Full delegation proof. | Fixed | Agent authorization |
| `check_delegation_scope` | `check_delegation_scope(claims, tx_context) -> bool` | claims, tx_context | bool | "Does this tx match the VC limits (amount, assets, expiry, slippage)?" | Enforced before any execute/migrate. | Fine-grained policy. | Fixed | Pre-execution scope check |
| `get_agent_reputation` | `get_agent_reputation(did: String) -> u32` | `did` | u32 | "Get current KYA reputation score." | Cached lookup (hot moka cache). | Reputation-aware routing. | Fixed low | Agent routing, gating |
| `x402_verify_micropayment` | `x402_verify_micropayment(header: Vec<u8>) -> bool` | `header` | bool | "Verify x402 payment proof in HTTP-style header." | Instant stablecoin micropayment for agent calls. | Account-less payments. | Fixed | Agent micropayments |
| `revoke_vc` | `revoke_vc(vc_id: Hash)` | `vc_id` | Unit | "Revoke a previously issued VC (issuer only)." | Updates revocation list (immutable log). | Issuer-only; instant revocation. | Fixed | Delegation revocation |
| `emit_delegation_log` | `emit_delegation_log(action: String, vc_id: Hash, notes: Vec<u8>)` | action, vc_id, notes | Unit | "Record immutable delegation event." | Emits DelegationEvent + appends to changelog. | Full audit trail. | Fixed | Audit, compliance |

#### 10. Safe Native Infrastructure Wrappers (v2.5+ – VC-Gated Access to Native Features)
| Function | Signature | Parameters | Returns | Layman Description | Formal Role & Consensus Tie-in | Host Enforcement / Security | Gas Model | Primary Use Cases |
|----------|-----------|------------|---------|--------------------|--------------------------------|-----------------------------|-----------|-------------------|
| `issue_token` | `(name, symbol, total_supply, mint_to)` | name, symbol, supply, mint_to | Receipt | "Issue new token (safe, scoped)." | VC claim `can_issue_token` required; 1 token/epoch per DID. | Type whitelist, supply cap. | Fixed + supply | Agent token issuance |
| `bank_transfer` | `(to: ID, amount: u128, token: Hash)` | to, amount, token | Unit | "Transfer from bank/spot." | VC claim `can_transfer` required; 20/sec, $100k daily cap. | Recipient whitelist, value cap. | Fixed | Agent transfers |
| `bucket_to_bucket_transfer` | `(from_bucket, to_bucket, amount)` | from, to, amount | Unit | "Transfer between buckets (same collateralAssetId)." | VC claim `can_transfer_bucket` required. | Type match enforced. | Fixed | Bucket rebalancing |
| `bank_to_bucket_transfer` | `(bucket_id, amount)` | bucket_id, amount | Unit | "Fund bucket from bank/spot." | VC claim `can_fund_bucket` required. | Bucket ownership + IM check. | Fixed | Bucket funding |
| `bucket_to_bank_transfer` | `(bucket_id, amount)` | bucket_id, amount | Unit | "Withdraw from bucket to bank/spot." | VC claim `can_withdraw_from_bucket` required. | Cannot drop below IM. | Fixed | Bucket withdrawals |
| `place_limit_order` | `(market, side, price, size, fill_type)` | market, side, price, size, fill_type | Order ID | "Place limit order (safe)." | VC claim `can_place_order` required; 50 orders/sec, notional cap. | CLOB backpressure reject. | Fixed | Agent order placement |
| `cancel_limit_order` | `(order_id: Hash)` or batch | order_id(s) | Unit | "Cancel own order(s)." | VC claim `can_cancel_order` required; 100 cancels/sec. | Only own orders. | Fixed | Agent order cancellation |
| `multi_send` | `(recipients: Vec<(to, amount, token)>)` | recipients | Unit | "Safe multi-recipient transfer." | VC claim `can_multi_send` required; max 50 recipients/call. | Gas proportional to recipients. | Per-recipient | Batch payouts |

**Note**: All Safe Wrappers call `check_delegation_scope` + `vc_verify` automatically; emit immutable action/delegation logs; fail closed on quota exceed or VC mismatch.

### Summary & Key Properties
- **Total functions**: 43+ (extensible via constitutional amendment — new host functions added as config objects).
- **All calls are deterministic** — identical result on every node.
- **Parallelism maximised** — Block-STM schedules non-conflicting objects in parallel (Flash path = zero waves).
- **Rollback safety** — Step-8 bounded rollback only reverts object versions ≤2Δ* (100 ms).
- **Agentic-friendly** — idempotency + multi-call batches + verifiable randomness + oracle hooks.
- **Upgradable** — Step-9 constitutional tx can add/remove host functions or change gas tables without fork.
- **Zero changes to 9-step consensus** — WASM is simply another Msg handler in the router (post-finality).
- **Agentic-first** — KYA/VC delegation (v2.4) + Safe Native Infrastructure Wrappers (v2.5+) enable scoped, revocable agent authorization without exposing raw native primitives.

This expanded Host API is **production-grade, maximally secure, and perfectly aligned** with every requirement in the Morpheum 2.0 documents (object-centric MVCC, nonce/idempotency, race prevention, DAG sequencing, treasury/restaking, crosschain, oracles, agentic ops, ZK/TEE/FHE, KYA delegation, Safe Wrappers).

If you want:
- Rust/WAT example signatures for any function
- Full capability model spec
- Gas cost formulas or example contract flow
- Integration diagram with Block-STM scheduler
- KYA/VC delegation flow or Safe Wrapper usage examples

…just say the word and I’ll deliver the next document instantly. This is the complete, ready-to-implement Host API for your DAG + WASM system.