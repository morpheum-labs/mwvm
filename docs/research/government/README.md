# Morpheum Governance Documentation

Design and constitutional amendment proposals for **Morpheum governance** — hybrid native/WASM model, Step 9 amendments, and agentic policy activation.

---

## Overview

Morpheum governance follows a **hybrid architecture**:

- **Core governance** (Step 9 constitutional amendments, global parameters, high-authority voting) remains **native infrastructure** — audited, gasless, integrated with consensus and slashing.
- **Application-level policies** (sub-DAOs, custom bucket templates, CLAMM fee curves) are implemented as **WASM smart contracts**, proposed and ratified via native governance with KYA/VC DID delegation.

This balances security, performance, decentralization, and agentic innovation while aligning with the "Host is God" philosophy.

---

## Idea Developments (Pretext)

Conceptual foundations for Web 4.0, autonomous agents, and Level 5 society — read [prelogue.md](./pretext/prelogue.md) first.

| Document | Description |
|----------|-------------|
| [prelogue.md](./pretext/prelogue.md) | **Web 4.0 + agent taxonomy** — Web evolution (1.0→4.0), autonomous agent types, autonomy levels (1–5) |
| [2026-level.md](./pretext/2026-level.md) | Agentic L1 networks — AgentLayer, A2A landscape, development level (2026) |
| [level-5-society.md](./pretext/level-5-society.md) | Level 5 + decentralized blockchain — AI jurisdiction, ethics, symbiotic civilization |
| [agent-body-law.md](./pretext/agent-body-law.md) | How agents "abide" by laws — legal personality, autonomy vs outlaw landscape |
| [hybrid-soiety.md](./pretext/hybrid-soiety.md) | Hybrid human–AI society |
| [government-comparison.md](./pretext/government-comparison.md) | AI government frameworks comparison |
| [ethos-framework.md](./pretext/ethos-framework.md) | ETHOS — federated civilization, constitutional veto |
| [ethos-arch.md](./pretext/ethos-arch.md) | ETHOS architecture |
| [cai-framework.md](./pretext/cai-framework.md) | CAI framework |
| [cai-arch.md](./pretext/cai-arch.md) | CAI architecture |

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

| Mechanism | Governance Backing |
|-----------|-------------------|
| Safe Mode (emergency pause) | MORP-GOV-001, gov-params |
| Constitutional params (quotas, fees) | gov-params, MORP-GOV-001 |
| Insurance fund | bucket-as-insurance, BA-OVERLAP-PENALTY-001 |
| VC scoping & A2A quotas | MORP-GOV-2026-02, MORP-GOV-2026-03 |
| Anti-overlap penalties | BA-OVERLAP-PENALTY-001 |
| Hybrid governance | design, hypbrid-governance |
