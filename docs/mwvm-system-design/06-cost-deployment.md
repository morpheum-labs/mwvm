# Cost & Deployment — WASM Deployment Design

**Version**: 1.0  
**Date**: 05 March 2026  
**Status**: Design  
**Source**: mwvm/docs/cost

## 1. Deployment Overview

Morpheum is **gasless** (no execution fees). To prevent storage spam, use a **refundable storage deposit** model (like NEAR/Substrate). Deployment follows a two-phase pattern:

1. **Store Code**: Upload WASM bytecode as reusable, versioned object (immutable after finality)
2. **Instantiate Contract**: Create runtime instance from stored code with initial state and address
3. **Optional Migrate**: Upgrades via MsgMigrate

## 2. Deployment Tx Types


| Tx Type            | Description                               | Metadata                                         |
| ------------------ | ----------------------------------------- | ------------------------------------------------ |
| **MsgStoreCode**   | Upload WASM bytecode as code object       | bytecode, code_hash, deposit_amount, permissions |
| **MsgInstantiate** | Create contract instance from stored code | code_id, init_msg, admin, label                  |
| **MsgMigrate**     | Upgrade instance to new code              | contract_addr, new_code_id, migrate_msg          |


## 3. Consensus Integration


| Consensus Step       | Deployment Interaction                                                          |
| -------------------- | ------------------------------------------------------------------------------- |
| **1. Ingress + MAV** | Validate bytecode (size, no malicious ops); compute deposit; MAV groups deploys |
| **2. Blocklace**     | Tx as DAG vertex; deposit covers storage                                        |
| **3–5. Waves**       | Parallel validation if non-conflicting                                          |
| **6. Frosty**        | On stall: EC/SC prioritizes pending deploys                                     |
| **7. Finality**      | Atomic commit: store bytecode, deduct deposit, emit event                       |
| **8. Recovery**      | Bounded rollback reverts code object creation                                   |
| **9. Amendment**     | Tx can amend deployment params (max size, deposit rate) via supermajority       |


## 4. Suggested Costs


| Item                        | Recommendation                           | Rationale                               |
| --------------------------- | ---------------------------------------- | --------------------------------------- |
| **Base Deposit Rate**       | 1 $MORPH per 100KB bytecode (refundable) | Matches NEAR; incentivizes optimization |
| **Example: 50KB**           | 0.5 $MORPH locked                        |                                         |
| **Example: 500KB**          | 5 $MORPH                                 | Unoptimized penalized                   |
| **Instantiation/Migration** | ~0.01 $MORPH flat                        | Initial state object                    |
| **Testnet**                 | Free/zero deposit for dev                |                                         |


**USD Estimate (2026)**: At $5/MORPH, small deploy ~$2.50; large ~$25.

## 5. Security & Robustness

- **Validation**: Host scans bytecode for safety (no host escapes, valid WASM)
- **Capabilities**: Stored code objects have perms (public vs restricted instantiate)
- **Recovery**: Step-8 guilt certs slash spammers; rollback reverts deposits
- **Agentic**: Idempotency keys prevent duplicate deploys

## 6. Constitutional Parameters

Deployment params (max bytecode size, deposit rate) are Step 9 amendable. Changes effective at epoch boundary.

## Related Documents

- [../cost/cost.md](../cost/cost.md) — Full deployment design source
- [02-architecture.md](02-architecture.md) — Host API, safe wrappers

