# Agent-to-Agent (A2A) Documentation

Design and strategy for **Agent-to-Agent (A2A)** flows on Morpheum — WASM templates, Bucket-as-Service integration, and ecosystem positioning against Virtuals/Fetch.ai.

---

## Overview

A2A enables autonomous agents to interact trustlessly on-chain: data sales, swarm coordination, task delegation, liquidity sharing, insurance claims, governance voting, cross-chain settlement, and reputation audits. All flows leverage:

- **x402 HTTP handshakes** — Secure negotiation and conditional access (DID/VC-based)
- **migrate_payload_owner** — Ownership transfer with delegation expiry checks
- **KYA/VC delegation** — Scoped, revocable agent authorization
- **MWVM v2.5+** — Safe wrappers, object-centric MVCC, Bucket-as-Service (BaS)

---

## Document Index

| Document | Description |
|----------|-------------|
| [a2a-wasm-templates.md](./a2a-wasm-templates.md) | **Core A2A WASM templates** — 8 pre-built modules (DataSaleA2A, SwarmCoordA2A, TaskDelegateA2A, etc.); x402 + migrate_payload_owner; deployment flow via morpheum_std SDK |
| [a2a-bucket-templates.md](./a2a-bucket-templates.md) | **Bucket + CLAMM integration** — BaS and CLAMM-enhanced A2A templates; composable collateral, liquidity sharing, economic alignment with $MORM |
| [critical-features.md](./critical-features.md) | **Must-build features** — Priority list before announcing vs Virtuals/Fetch.ai: wrapper tool, multi-sig delegation, x402 payment hooks, oracle integration, Agent Hub |
| [fetchai.md](./fetchai.md) | **Market analysis** — Virtuals vs Fetch.ai positioning; Morpheum battle plan as secure multi-chain execution backend; incentive blitz and ecosystem domination |
| [against-virtual.md](./against-virtual.md) | **Agent economy acceleration** — Morpheum Agent Hub v1; rapid adoption flywheel; x402 + WASM + delegation as moat; development priorities |

---

## Quick Reference

| Concept | Reference |
|---------|-----------|
| A2A template catalog | [a2a-wasm-templates.md](./a2a-wasm-templates.md) |
| Bucket/CLAMM integration | [a2a-bucket-templates.md](./a2a-bucket-templates.md) |
| Pre-launch feature checklist | [critical-features.md](./critical-features.md) |
| Virtuals/Fetch.ai strategy | [fetchai.md](./fetchai.md), [against-virtual.md](./against-virtual.md) |
