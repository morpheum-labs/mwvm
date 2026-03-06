# Recursive Risk Controls — Depth Limiter & Skin-in-the-Game

**Version**: 1.0  
**Date**: 05 March 2026  
**Status**: Design  
**Source**: mwvm/docs/bucket-as-service (recursive-risk-control, depth-limiter, skin-in-game-leverage)

## 1. Recursive Risk Overview

Users can build nested/meta-buckets and carry-trade loops (Bucket A → borrow → Bucket B → borrow → Bucket C). Without controls this creates systemic cascade risk (2022 UST/3AC-style events).

**We do NOT ban recursion** — we **meter and monetize it with $MORM** so deeper recursion = more $MORM demand + higher protocol revenue.

## 2. Recursive Risk Control Stack

| Control Layer | Mechanism | $MORM Integration |
|---------------|-----------|-------------------|
| **Depth Limiter** | Hard cap at **max 4 nesting levels** per bucket family | Each level after #1 requires +2% base $MORM lock |
| **Escalating Skin-in-the-Game** | Level 0: 1.5%; Level 1: 3.5%; Level 2: 5.5%; Level 3: 7.5%; Level 4: 9.5% | All locked $MORM is first-loss buffer — absorbs first 25% drawdown |
| **Effective Leverage Cap** | Global family cap = **3.5×** total recursive multiplier | If breached, protocol auto-sells $MORM buffer first, then forces partial deleverage |
| **Insurance Fund Backstop** | 25% of bucket performance fee + 100% of liquidation penalties | Fund used after $MORM buffer exhausted |

## 3. Depth Limiter Mechanics

### Core Concepts

- **Bucket Family / Tree**: All buckets that reference each other (via strategy target, collateral source, borrow-from) form a single tree
- **Root Bucket** = Level 0 (simple yield bucket, no parent)
- **Child Bucket** = depth = parent.depth + 1
- **Tree ID**: Immutable `keccak256(rootBucketAddress + creationNonce)` stored at root creation

### Enforcement Points

- **Bucket Creation**: Factory checks depth before deployment
- **Deposit / Borrow / Leverage**: Every interaction that would extend tree re-checks depth
- **Meta-Bucket Registration**: If user tries to wrap Level-4 bucket into new one → immediate revert
- **Governance Override**: veMORM holders can vote to temporarily raise/lower max depth per tree

### Why Exactly 4 Levels?

- Level 1–2: Basic carry (80% of expected TVL)
- Level 3: Meta-vaults + tranche products
- Level 4: Full institutional carry desks
- Beyond 4: Risk of hidden 8–10× leverage cascades

## 4. Escalating Skin-in-the-Game

| Depth Level | Required Locked $MORM (% of AUM) | Who Must Lock | First-Loss Priority |
|-------------|----------------------------------|---------------|---------------------|
| **0 (Root)** | 1.5% | Creator only | Absorbs first 25% of tree drawdown |
| **1** | 3.5% (+2%) | Creator + top 5 depositors | Same |
| **2** | 5.5% (+2%) | Creator + top 10 depositors | Same |
| **3** | 7.5% (+2%) | Creator + all leveraged users | Same |
| **4 (max)** | 9.5% (+2%) | Mandatory across entire tree | Same |

## 5. Implementation Notes

- Lock held in **veMORM** (time-weighted, non-transferable during lock)
- Recalculated every 24h based on current AUM (via Morpheum native oracle)
- If lock falls below requirement → new deposits paused until topped up
- All locked $MORM is **tree-specific first-loss capital**

## 6. $MORM Valuation Impact

1. **Exponential Lock Demand** — At $1B recursive TVL, avg depth ~2.5 → ~45–70M $MORM locked
2. **Fee Amplification** — Performance fees tiered by depth: Level 0 = 12%, Level 4 = 22%
3. **Incentive Alignment** — Creators must lock real capital → less reckless looping
4. **Anti-Cascade** — Even in 60% drawdown, losses stop at tree's $MORM buffer

## 7. Transparency & UX

- Public **Recursion Explorer**: visual tree graph per Tree ID, live depth meter, $MORM locked, risk score
- Deposit UI: "This action would move you to Depth 3 → requires additional 2% $MORM lock"
- API: `GET /bucket/{address}/depth`

## Related Documents

- [03-bucket-as-service.md](03-bucket-as-service.md) — BaS rule set
- [../bucket-as-service/depth-limiter.md](../bucket-as-service/depth-limiter.md) — Depth limiter source
- [../bucket-as-service/skin-in-game-leverage.md](../bucket-as-service/skin-in-game-leverage.md) — Skin-in-game source
