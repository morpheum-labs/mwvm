# Plugin-VM Extensibility — Hook vs Pluggable Patterns

**Version**: 1.0  
**Date**: 05 March 2026  
**Status**: Design  
**Source**: mwvm/docs/plugin-vm

## 1. MWVM Infrastructure Modularization Skillset (MIMS)

A repeatable framework for breaking down infrastructure modules and assigning the right extensibility pattern. Enables **heavy marketing campaigns** (promos, referrals, gamification) and **business-model pivots** (new fee models, tokenomics) to launch in days instead of months.

### 7-Step MIMS Checklist

1. **Module Inventory** — List every component + call graph
2. **Stability & Change-Frequency Rating** — Score 1–5 (how often marketing/BD needs changes)
3. **Impact & Risk Assessment** — Core (high risk) vs Volatile (safe to externalize)
4. **Extensibility Point Discovery** — Identify pre/post/action hooks or logic boundaries
5. **Pattern Assignment** — Hook or Microkernel/Pluggable
6. **API Interface Extraction** — Define Wasm ABI (memory-safe, versioned)
7. **Governance & Versioning Mapping** — Link to DAO voting + backward-compatible default

## 2. Pattern Selector: Hook vs Pluggable


| Step | Question               | Hook favored when…                   | Pluggable favored when…                |
| ---- | ---------------------- | ------------------------------------ | -------------------------------------- |
| 1    | Change target size     | Small, focused modification          | Large, cross-cutting, stateful logic   |
| 2    | Execution timing       | Logic at precise pre-defined moments | Replaces entire vertical/layer         |
| 3    | Frequency & ownership  | Marketing changes weekly–monthly     | BD owns whole experience over quarters |
| 4    | State & data ownership | Almost stateless, small outputs      | Own persistent state, complex config   |
| 5    | Audit & risk surface   | 50–300 LOC, low blast radius         | Larger surface, swappable module       |
| 6    | Performance            | Extremely lightweight (hot path)     | Can afford higher overhead             |


**Quick rule**: ≥4 "Hook" → Hook; ≥4 "Pluggable" → Pluggable; 3–3 split → prefer Hook.

## 3. Marketplace Module Recommendations


| Module                           | Change Frequency | Pattern                   | Rationale                                                         |
| -------------------------------- | ---------------- | ------------------------- | ----------------------------------------------------------------- |
| Fee & Discount Engine            | Very High        | **Hook**                  | Surgical override; marketing deploys new discount logic instantly |
| Reward & Loyalty Calculator      | Very High        | Hook + Pluggable fallback | Hook for quick tweaks; full module for complex loyalty            |
| Campaign & Promo Rule Engine     | Extremely High   | **Pluggable**             | Entire campaign logic in one swappable Wasm module                |
| Compliance & Business Validation | High             | **Hook**                  | Intercept every action; rapid regulatory pivots                   |
| Settlement & Revenue Split       | Medium-High      | **Hook**                  | Post-trade distribution changes without core redeploy             |


## 4. API Interface Examples

- **Hook**: `hook_before_fee_calc(trade_data_ptr, discount_out_ptr) -> i32`
- **Hook**: `hook_after_trade_reward(user_id, volume, reward_out_ptr) -> i32`
- **Pluggable**: `execute_campaign_logic(campaign_id, context_ptr, result_ptr) -> i32`

## 5. $MORM Impact

- **Marketing velocity**: 8–12 campaigns/year instead of 2–3
- **Cost efficiency**: 70–85% savings per change
- **Competitive moat**: Morpheum ships marketing changes weekly at ~$15k vs months + $100k+ elsewhere

## Related Documents

- [../plugin-vm/skills.md](../plugin-vm/skills.md) — Full MIMS source
- [02-architecture.md](02-architecture.md) — Host API, safe wrappers

