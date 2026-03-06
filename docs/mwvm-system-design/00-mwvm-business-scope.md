# Business Scope of the MWVM Ecosystem (Mormcore – Morpheum Context)

**Version**: 1.0  
**Date**: 05 March 2026  
**Status**: Design  
**Purpose**: Single source of truth for MWVM business scope — Host is God, WASM is Pure Compute.

## 1. Core Business Objective

The MWVM (Morpheum WASM VM) ecosystem enables **agent-issued structural products** (Bucket-as-Service), **hybrid governance** (native core + WASM policies), and **permissionless DeFi innovation** on the Morpheum DAG. The system is built on:

- **Host is God** — WASM = transient linear memory only; every interaction with the outside world goes through the sandboxed Host API
- **Native-only protocol primitives** — Multisig, full CLAMM/ReClamm, bucket/perp core, staking core, bank transfers, token issuance, order placement are built-in native infrastructure
- **Safe wrappers** — Access to native features only through high-level wrappers enforcing KYA/VC delegation, business-logic scoping, and resource quotas

## 2. Locked Ownership Clarification

| Layer | Sole Owner Of |
|-------|---------------|
| **Native Infrastructure** | Consensus, CLAMM, CLOB, buckets, staking core, multisig, bank transfers, order placement |
| **Host API** | Safe wrappers, capability checks, version checks, KYA/VC enforcement |
| **WASM Contracts** | Application-level policies, sub-DAOs, custom bucket templates, agent-specific logic |
| **Governance (Step 9)** | Constitutional amendments, global parameters, supermajority voting |
| **BaS** | Agent-issued structural products (position-backed, asset-backed, mix-backed) |

No raw native access from WASM. All high-risk operations flow through safe wrappers.

## 3. Boundary with Native Infrastructure (Locked)

**WASM never has raw access to native primitives.** Access is provided exclusively through safe Host API wrappers that enforce:

- KYA/VC delegation with scoped claims
- Business-logic scoping (e.g., `can_deploy_bucket(type, max_value, expiry)`)
- Resource quotas (constitutional, Step 9 amendable)
- Atomicity, version checks, safe mode

| Native Owns | WASM / BaS Owns |
|-------------|-----------------|
| Core protocol logic (CLAMM, CLOB, buckets, staking) | Application policies, custom templates |
| Raw bank transfers, token issuance, order placement | Safe wrapper calls with VC claims |
| Consensus parameters, slashing | Sub-DAO voting, custom fee curves |
| Emergency pause, validator set | Agent-deployed bucket products |

## 4. Core Scope (MUST be implemented)

| Category | Included? | Detail |
|----------|-----------|--------|
| Bucket-as-Service | YES | deploy_bucket_product, list_bucket_for_sale, buy_bucket; position/asset/mix-backed products |
| Safe Native Wrappers | YES | issue_token, bank_transfer, bucket_to_bucket_transfer, place_limit_order, cancel_limit_order, multi_send |
| KYA/VC Delegation | YES | did_validate, vc_verify, check_delegation_scope, revoke_vc; scoped claims per operation |
| Hybrid Governance | YES | Native Step 9 for core; WASM for application-level policies with native ratification |
| Recursive Risk Controls | YES | Depth limiter (max 4), escalating skin-in-the-game, effective leverage cap ~3.5× |
| Constitutional Params | YES | All bas_*, wasm_overlap_*, clamm_a2a_* params; Step 9 amendable |
| Overlap Penalties | YES | Economic disincentives for WASM duplicating native primitives; Mormtest guidance |

## 5. Non-Functional / Consensus Requirements

- **Deterministic**: All Host API calls gas-metered, deterministic; no external randomness in WASM
- **Permissionless**: Any KYA-verified agent can create/sell; delegation-first policy
- **Exploit-aware**: Health snapshots, atomic escrow, insurance fund, slashing for misrepresentation
- **$MORM-aligned**: Fees, burns, treasury buybacks, staking incentives drive token appreciation

## 6. Genesis / Restart Recovery

Constitutional parameters are genesis-loadable and governance-updatable. Safe mode (`bas_safe_mode_enabled`, `wasm_overlap_detection_enabled`) can be toggled for emergency response. Snapshot restore + bounded replay for consensus recovery.

## 7. Out-of-Scope (for this subsystem)

- **Raw native access** — Never exposed to WASM
- **Pure WASM governance** — Core remains native; only application-level policies in WASM
- **Uncontrolled recursion** — Depth limiter and skin-in-the-game are mandatory
- **EVM-style upgrade proxies** — Object-centric model; native migration via MsgMigrate

## 8. Integration Points

- **KYA/VC** → All high-risk Host API calls require valid VC with scoped claims
- **BaS** → deploy_bucket_product, list_bucket_for_sale, buy_bucket via safe wrappers
- **Governance** → MsgConstitutionalAmendment for param changes; Step 9 supermajority
- **Mormtest** → Local simulation, overlap warnings, A2A quota enforcement

---

**Bottom line for Mormcore implementation**

The MWVM ecosystem is the **agentic DeFi layer** of Morpheum. It enables permissionless innovation through safe wrappers, KYA/VC delegation, and Bucket-as-Service — all while protecting native infrastructure and driving $MORM value through fees, burns, and staking.

See [10-scope-boundary.md](10-scope-boundary.md) and [09-module-structure.md](09-module-structure.md) for the official boundary and structure.
