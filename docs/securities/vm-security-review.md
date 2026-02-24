**MWVM v2.4 Host API Security Review & Permission Model**

I have reviewed the full consolidated Host API table from `api-appendix.md`. Below is a **category-by-category security evaluation** in the context of a **permissionless environment** (anyone can deploy and call contracts).

For each API, I assess:
- **Risk Level** (Low / Medium / High)
- **Primary Security Concerns**
- **Existing Protections** (from v2.2–v2.4)
- **Recommended Countermeasures & Limitations** (new or strengthened, focused on permission/access control, business-logic scoping, and performance-neutral enforcement)

The overall posture is **strong** because of the **object-centric MVCC + capability + transient memory** foundation, plus KYA delegation. However, some APIs need tighter **scoped permissioning** to prevent abuse in an agentic/permissionless setting.

---

### 1. Object Management (Core State Layer)
| Function                  | Risk Level | Primary Concerns                              | Existing Protections                          | Recommended Countermeasures & Limitations |
|---------------------------|------------|-----------------------------------------------|-----------------------------------------------|-------------------------------------------|
| `object_read` / `object_read_batch` | Low       | Information leakage (sensitive object data)   | Version check, capability check               | VC claim: `can_read_object(id)` (optional for privacy-sensitive objects). Rate-limit per DID (100 reads/sec). |
| `object_write` / `object_write_batch` | Medium    | Unauthorized mutation, double-spend          | Ownership + version bump + capability         | VC claim required: `can_write_object(id, max_amount)`. Per-DID write rate limit (20 writes/sec). |
| `object_create`           | Medium    | Storage spam / DoS                           | Deposit + capability check                    | VC claim: `can_create_object(type, max_size)`. Constitutional max objects per DID per epoch. |
| `object_delete`           | Low       | Accidental deletion of shared objects        | Ownership + capability                        | VC claim: `can_delete_object(id)`. Soft-delete with 7-day delay for critical objects. |
| `object_transfer`         | Medium    | Unauthorized asset movement                  | Ownership + capability                        | VC claim: `can_transfer_object(id, recipient_type)`. Require owner approval for high-value transfers. |

**Overall for Category**: Medium risk if unrestricted.  
**Business Logic Limitation**: Add constitutional flag `require_vc_for_object_ops` (default off for simple contracts, on for DeFi/treasury).

---

### 2. DAG Context
| Function                  | Risk Level | Primary Concerns                              | Existing Protections                          | Recommended Countermeasures & Limitations |
|---------------------------|------------|-----------------------------------------------|-----------------------------------------------|-------------------------------------------|
| `get_dag_context` / `host_get_dag_context` | Low       | Minor information leakage (round, parents)    | Read-only                                     | VC claim optional: `can_read_dag_context`. Rate-limit per DID. |
| `get_epoch_info`          | Low       | Governance param leakage                      | Read-only                                     | Same as above. |
| `host_query_object_history` | Medium    | Historical state leakage (time-travel)        | Versioned + capability                        | VC claim: `can_query_history(id, depth)`. Max depth limit (100 versions). |

**Overall for Category**: Low–Medium.  
**Limitation**: Constitutional `max_history_depth` param (default 1000) to prevent infinite queries.

---

### 3. Idempotency
| Function                  | Risk Level | Primary Concerns                              | Existing Protections                          | Recommended Countermeasures & Limitations |
|---------------------------|------------|-----------------------------------------------|-----------------------------------------------|-------------------------------------------|
| `idempotency_check` / `idempotency_mark` | Low       | Replay spam / key collision                   | Host-managed set with eviction                | Per-DID key limit (1000 active keys). VC claim for high-volume agents. |

**Overall**: Very low risk. Good as-is.

---

### 4. Events & Oracle
| Function                  | Risk Level | Primary Concerns                              | Existing Protections                          | Recommended Countermeasures & Limitations |
|---------------------------|------------|-----------------------------------------------|-----------------------------------------------|-------------------------------------------|
| `emit_event`              | Low       | Spam events / storage bloat                   | Gas metering + topic validation               | Per-DID event rate limit (50/sec). Constitutional max topic size. |
| `call_oracle`             | Medium    | Oracle manipulation / DoS on external feeds   | Proof verification (ZK/TEE)                   | VC claim: `can_call_oracle(feed_id, max_freq)`. Rate-limit per DID + per-feed. |

**Overall**: Medium.  
**Limitation**: Governance can blacklist risky oracles via constitutional param.

---

### 5. Crosschain
| Function                  | Risk Level | Primary Concerns                              | Existing Protections                          | Recommended Countermeasures & Limitations |
|---------------------------|------------|-----------------------------------------------|-----------------------------------------------|-------------------------------------------|
| `crosschain_send` / `crosschain_recv` | High      | Bridge exploits, infinite mint, asset drain   | Capability + lock + proof verification        | VC claim: `can_crosschain_send(dest_chain, max_amount, expiry)`. Require multi-oracle approval for high-value. Constitutional per-chain whitelist. |

**Overall**: High risk if unrestricted.  
**Strong Limitation**: Default to KYA delegation only; no direct call without VC.

---

### 6. Staking / Treasury
| Function                  | Risk Level | Primary Concerns                              | Existing Protections                          | Recommended Countermeasures & Limitations |
|---------------------------|------------|-----------------------------------------------|-----------------------------------------------|-------------------------------------------|
| `stake` / `restake` / `claim_yield` | Medium-High | Unauthorized staking, yield drain, reentrancy | Ownership + capability                        | VC claim: `can_stake(protocol, max_amount, expiry)`. Per-protocol limits. |

**Overall**: Medium-High.  
**Limitation**: Treasury/staking actions require explicit governance or owner VC for high-value protocols.

---

### 7. Gas & Random
| Function                  | Risk Level | Primary Concerns                              | Existing Protections                          | Recommended Countermeasures & Limitations |
|---------------------------|------------|-----------------------------------------------|-----------------------------------------------|-------------------------------------------|
| `gas_charge`              | Low       | Metering bypass (if misused)                  | Host-enforced                                 | Not directly callable from WASM (internal). |
| `get_random`              | Medium    | Predictable randomness / bias                 | VRF-derived per round                         | VC claim: `can_use_randomness(max_calls)`. Use only for non-critical logic. |

---

### 8. Security (ZK/TEE/FHE)
| Function                  | Risk Level | Primary Concerns                              | Existing Protections                          | Recommended Countermeasures & Limitations |
|---------------------------|------------|-----------------------------------------------|-----------------------------------------------|-------------------------------------------|
| `require_zk_proof` / `zk_prove_execution` | Low       | Proof verification bypass                     | Mandatory verification                        | Constitutional flag to force for high-value contracts. |
| `enable_tee_mode` / `tee_attest_call` | Low       | Enclave bypass                                | Remote attestation on genesis                 | Optional per-shard; governance can mandate for sensitive contracts. |
| `fhe_*` ops               | Medium    | Private computation abuse                     | Encrypted objects only                        | VC claim: `can_use_fhe(max_ops)`. |

---

### 9. Agentic (v2.1)
| Function                  | Risk Level | Primary Concerns                              | Existing Protections                          | Recommended Countermeasures & Limitations |
|---------------------------|------------|-----------------------------------------------|-----------------------------------------------|-------------------------------------------|
| `agent_publish` / `agent_subscribe` / `agent_send_direct` | Medium-High | Swarm spam, coordination attacks, DoS         | Capability + safe mode                        | VC claim: `can_agent_message(topic, max_freq)`. Per-DID global rate limit. |
| `ai_infer`                | High       | Malicious model execution / resource drain    | TEE/ZK proof                                  | VC claim: `can_ai_infer(model_id, max_complexity)`. Constitutional model whitelist. |
| `agent_migrate` / `agent_self_destruct` | Medium    | Unauthorized self-upgrade / destruction       | Admin capability                              | VC claim + owner approval required. |
| `agent_log_metric`        | Low       | Spam metrics                                  | Event emission only                           | Rate limit + topic validation. |

---

### 10. Security Helpers (v2.2) & Migration (v2.3)
| Function                  | Risk Level | Primary Concerns                              | Existing Protections                          | Recommended Countermeasures & Limitations |
|---------------------------|------------|-----------------------------------------------|-----------------------------------------------|-------------------------------------------|
| `set_safe_mode` / `get_call_depth` | Low       | Bypass reentrancy protection                  | Always-on call depth limit                    | Constitutional default = enabled. |
| `migrate` / `get_contract_version` / `require_version` / `emit_migration_log` | Low       | Unauthorized upgrade                          | Admin capability + version check              | VC claim for migration + owner confirmation for high-value contracts. |

---

### 11. KYA / Delegation (v2.4) & Governance
These are **the primary permission layer** — they are the safest entry point.

- `did_validate`, `vc_verify`, `vp_present`, `check_delegation_scope`, `revoke_vc`, `emit_delegation_log`, `get_agent_reputation`, `x402_verify_micropayment`: **Low risk** — they are the **gatekeepers**.
- `read_constitution_param`: **Low risk** — read-only.

**Strong Recommendation**: All high/medium-risk APIs above should **require a valid VC** with appropriate claims for business-logic scoping.

---

### Recommended Global Permission Model (v2.5 Ready)

Add this as a new section in the spec:

**Permission Model Summary (Enforced at Host Level)**

1. **Default**: All APIs require capability + version check.
2. **High-Risk APIs** (Crosschain, Staking, Agentic messaging, AI infer, Migration): **Mandatory KYA VC** with specific claims.
3. **Constitutional Flags** (amendable via Step 9):
   - `require_vc_for_clamm_ops` (default: true for production)
   - `max_agent_message_rate_per_did` (default: 50/sec)
   - `oracle_whitelist` / `model_whitelist`
4. **Safe Mode**: Global or per-contract flag disables risky features (messaging, migration) during high-risk periods.

This model ensures **business-logic-level access control** without reducing performance (all checks are O(1) cached).

---

**Next Step Recommendation**

I recommend adding the following to the v2.4 spec:
- The full security review table above
- The Permission Model Summary
- A new “Safe CLAMM Access Wrappers” section with example functions like `clamm_swap`, `clamm_add_liquidity` that internally enforce delegation.

Would you like me to deliver the **updated full v2.4 spec** with these security enhancements integrated right now?

Just reply **“Yes, update v2.4 with security review”** and I will provide the complete revised document.

This will make the VM extremely robust for permissionless agentic use while keeping CLAMM native and safe. Ready when you are!