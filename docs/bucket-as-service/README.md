# Bucket-as-Service (BaS) Documentation

Agent-deployed, position-backed, asset-backed, and mix-backed structural products on Morpheum's secondary and P2P markets. Built on MWVM v2.5 (safe wrappers, KYA/VC delegation) and native bucket infrastructure.

---

## Overview

Bucket-as-Service enables agents (AI or human) to:

- **Deploy** bucket products via `deploy_bucket_product` wrapper with KYA/VC claims
- **List** on secondary/P2P markets with immutable health snapshots
- **Trade** at cash, premium, or discount with atomic escrow settlement
- **Drive** $MORM value through fees, burns, staking, and treasury buybacks

Product types: **Position-Backed** (perp portfolios), **Asset-Backed** (stable yield), **Mix-Backed** (hybrid).

---

## Document Index

| Document | Description |
|----------|-------------|
| [design.md](./design.md) | **BaS rule set** — Creation, listing, trading, settlement, exploit countermeasures; Step 9 amendable |
| [business-model.md](./business-model.md) | **Strategic blueprint** — Agents as issuers, revenue streams, $MORM flywheel, DeFi comparison |
| [security-concern.md](./security-concern.md) | **Security analysis** — Severity-ranked concerns, countermeasures, delegation-first policy |
| [economic-view.md](./economic-view.md) | **Yield & carry** — BaaS as yield buckets, recursive layers, systemic risk controls |
| [bucket-as-insurance.md](./bucket-as-insurance.md) | Insurance fund mechanics and victim compensation |
| [gov-params.md](./gov-params.md) | Governance parameters and constitutional tuning |
| [veMORM.md](./veMORM.md) | veMORM integration with BaS quotas and incentives |
| [skin-in-game-leverage.md](./skin-in-game-leverage.md) | Skin-in-the-game leverage and issuer alignment |
| [depth-limiter.md](./depth-limiter.md) | Recursion depth limits and risk controls |
| [recursive-risk-control.md](./recursive-risk-control.md) | Recursive bucket risk management |
| [index-fund-product.md](./index-fund-product.md) | **Index funds** — Single vs collective buckets, ETF-like structures |
| [index-fund-requirements.md](./index-fund-requirements.md) | Index fund requirements and constraints |
| [index-fund-incentive.md](./index-fund-incentive.md) | Index fund incentives and issuer economics |

---

## Quick Reference

| Topic | Start Here |
|-------|------------|
| BaS rule set & product types | [design.md](./design.md) |
| Business model & flywheel | [business-model.md](./business-model.md) |
| Security concerns & mitigations | [security-concern.md](./security-concern.md) |
| Index funds on buckets | [index-fund-product.md](./index-fund-product.md) |
| MWVM v2.6 BaS policy | [../proposals/draft11-v2.6.md](../proposals/draft11-v2.6.md) §5 |
