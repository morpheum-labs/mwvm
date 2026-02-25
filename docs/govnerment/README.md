# Morpheum Governance Documentation

Design and constitutional amendment proposals for **Morpheum governance** — hybrid native/WASM model, Step 9 amendments, and agentic policy activation.

---

## Overview

Morpheum governance follows a **hybrid architecture**:

- **Core governance** (Step 9 constitutional amendments, global parameters, high-authority voting) remains **native infrastructure** — audited, gasless, integrated with consensus and slashing.
- **Application-level policies** (sub-DAOs, custom bucket templates, CLAMM fee curves) are implemented as **WASM smart contracts**, proposed and ratified via native governance with KYA/VC DID delegation.

This balances security, performance, decentralization, and agentic innovation while aligning with the "Host is God" philosophy.

---

## Document Index

### Design & Architecture

| Document | Description |
|----------|-------------|
| [design.md](./design.md) | **Core design** — Why pure WASM governance is unsuitable; hybrid model rationale; security model; $MORM value drivers |
| [hypbrid-governance.md](./hypbrid-governance.md) | **Hybrid governance** — Native vs WASM scope; core modules (consensus, CLAMM, CLOB, buckets, staking); safe integration via wrappers and KYA/VC |

### Constitutional Amendment Proposals

| Document | Description |
|----------|-------------|
| [MORP-GOV-001.md](./MORP-GOV-001.md) | **Bucket-as-Service (BaS)** — Launch proposal; constitutional parameters (deposits, fees, quotas); proposal templates |
| [MORP-GOV-2026-02.md](./MORP-GOV-2026-02.md) | **CLAMM A2A** — VC claims and quotas for agent-to-agent CLAMM access (swap, liquidity, hooks); staking incentives |
| [MORP-GOV-2026-03.md](./MORP-GOV-2026-03.md) | **Bucket A2A** — Bucket-specific VC claims and quotas for BaS A2A interactions (deploy, trade, rebalance, insurance) |
| [BA-OVERLAP-PENALTY-001.md](./BA-OVERLAP-PENALTY-001.md) | **Overlap penalties** — Economic disincentives for WASM contracts duplicating native primitives; deposit multipliers, usage fees, burn/buyback split |

---

## Quick Reference

| Concept | Reference |
|---------|-----------|
| Why hybrid (not pure WASM) | [design.md](./design.md) |
| Native vs WASM scope | [hypbrid-governance.md](./hypbrid-governance.md) |
| BaS launch parameters | [MORP-GOV-001.md](./MORP-GOV-001.md) |
| CLAMM agent delegation | [MORP-GOV-2026-02.md](./MORP-GOV-2026-02.md) |
| Bucket agent delegation | [MORP-GOV-2026-03.md](./MORP-GOV-2026-03.md) |
| Anti-overlap economics | [BA-OVERLAP-PENALTY-001.md](./BA-OVERLAP-PENALTY-001.md) |

---

## Fail-Safe Countermeasure Index

These governance proposals back the **fail-safe countermeasures** described in [../bucket-as-service/README.md#10-governance-backing-fail-safe-index](../bucket-as-service/README.md#10-governance-backing-fail-safe-index):

| Mechanism | Govnerment Backing |
|-----------|-------------------|
| Safe Mode (emergency pause) | MORP-GOV-001, gov-params |
| Constitutional params (quotas, fees) | gov-params, MORP-GOV-001 |
| Insurance fund | bucket-as-insurance, BA-OVERLAP-PENALTY-001 |
| VC scoping & A2A quotas | MORP-GOV-2026-02, MORP-GOV-2026-03 |
| Anti-overlap penalties | BA-OVERLAP-PENALTY-001 |
| Hybrid governance | design, hypbrid-governance |
