# MWVM System — Overview

**Version**: 1.0  
**Date**: 05 March 2026  
**Status**: Design

## Executive Summary

The MWVM (Morpheum WASM VM) ecosystem enables **agent-issued structural products** and **hybrid governance** on the Morpheum DAG. The architecture rests on three pillars:

1. **Host is God** — WASM = pure compute; all I/O via sandboxed Host API; native primitives never exposed raw
2. **Safe Wrappers** — High-level functions (deploy_bucket_product, bank_transfer, place_limit_order) enforce KYA/VC, quotas, atomicity
3. **Hybrid Governance** — Core (Step 9) native; application-level policies in WASM with native ratification

## Scope


| Component             | Responsibility                                                                              |
| --------------------- | ------------------------------------------------------------------------------------------- |
| **Host API**          | 43+ core functions; safe wrappers for native primitives; capability + version checks        |
| **KYA/VC**            | did_validate, vc_verify, check_delegation_scope, revoke_vc; scoped claims per operation     |
| **BaS**               | deploy_bucket_product, list_bucket_for_sale, buy_bucket; position/asset/mix-backed products |
| **Governance**        | Step 9 constitutional amendments; WASM policy contracts with native ratification            |
| **Recursive Risk**    | Depth limiter (max 4), skin-in-the-game, effective leverage cap                             |
| **Overlap Penalties** | Economic disincentives for WASM duplicating native; Mormtest guidance                       |


## Key Concepts

### Host is God, WASM is Pure Compute

- WASM module = transient linear memory only (no persistent state, no syscalls, no randomness)
- Every interaction with the outside world goes through the Host API (sandboxed, gas-metered, deterministic)
- Core protocol primitives (multisig, CLAMM, bucket/perp core, staking, bank transfers, order placement) are built-in native
- These are **never exposed raw** — access only through safe wrappers with KYA/VC and quotas

### Bucket-as-Service (BaS)

- Agents deploy bucket products via `deploy_bucket_product` with KYA/VC claims
- Product types: **Position-Backed** (perp portfolios), **Asset-Backed** (stable yield), **Mix-Backed** (hybrid)
- List on secondary/P2P with immutable health snapshot; trade at cash, premium, or discount
- Atomic escrow for every sale; insurance fund for misrepresentation; reputation gating

### Hybrid Governance

- **Native**: Constitutional amendments, global params, supermajority voting, slashing, emergency pause
- **WASM**: Sub-DAOs, custom bucket templates, CLAMM fee curves, agent-specific policies
- WASM policies proposed by agents with KYA/VC; activated only after native Step 9 ratification

### Recursive Risk Controls

- **Depth Limiter**: Max 4 nesting levels per bucket family; Tree ID immutable at root
- **Escalating Skin-in-the-Game**: Level 0: 1.5% locked $MORM; each level +2%; first-loss buffer
- **Effective Leverage Cap**: Global family cap ~3.5× total recursive multiplier

## Design Principles

1. **Delegation-first** — All bucket creation/sale flows through KYA/VC with scoped claims
2. **Atomic escrow** — buy_bucket locks payment, verifies health, transfers bucket, releases payment in one atomic step
3. **Immutable health snapshots** — Every listing and purchase records margin, positions, risk ratio
4. **Economic penalties** — Misrepresentation → slashing + reputation ban + insurance payout; spam → deposit burn
5. **Constitutional tunability** — All params Step 9 amendable; safe mode for emergency pause

## $MORM Value Drivers


| Mechanism                  | Impact                                                             |
| -------------------------- | ------------------------------------------------------------------ |
| Creation/listing/sale fees | Paid in $MORM; burn 50%, insurance 30%, treasury 20%               |
| Staking                    | Unlock quotas, verified badges, fee discounts                      |
| Overlap penalties          | Fees for duplicating native; 50% burn, 35% insurance, 15% treasury |
| Recursive skin-in-the-game | Deeper buckets lock more $MORM; first-loss buffer                  |
| Treasury buybacks          | Surplus from insurance fund                                        |


## Related Documents

- [00-mwvm-business-scope.md](00-mwvm-business-scope.md) — Business scope and boundary
- [02-architecture.md](02-architecture.md) — System architecture and data flow
- [10-scope-boundary.md](10-scope-boundary.md) — Locked In/Out matrix

