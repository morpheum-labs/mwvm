# MWVM Governance — Hybrid Model & Constitutional Amendments

**Version**: 1.0  
**Date**: 05 March 2026  
**Status**: Design  
**Source**: mwvm/docs/government

## 1. Core Design: Hybrid Governance

**Recommendation**: Hybrid Model — Core constitutional governance (Step 9) remains **native infrastructure**. Application-level policies, sub-DAOs, and executable policy contracts are implemented as **WASM smart contracts**, proposed and activated through native governance with KYA/VC DID delegation.

### Why Pure WASM Governance Is Not Suitable


| Approach             | Pros                                                   | Cons                                                                                            |
| -------------------- | ------------------------------------------------------ | ----------------------------------------------------------------------------------------------- |
| **Entirely on WASM** | High flexibility, agentic composability, easy upgrades | High security risk, deployment spam, audit burden, potential malicious high-authority contracts |
| **Entirely Native**  | Highest security, performance, consensus integration   | Too rigid for agentic/custom policies, slower innovation                                        |


### Why Hybrid Is Optimal

- **Core (Native)**: Constitutional amendments, supermajority voting, global params (CLAMM glide, CLOB fee tiers, bucket whitelist, quotas, insurance rules)
- **Application (WASM)**: Sub-DAOs, custom bucket templates, CLAMM fee curves, agent-specific policies
- WASM policies proposed by agents with KYA/VC; activated only after native Step 9 ratification

## 2. Native vs WASM Scope

### Native Governance (Core Infrastructure)

- Consensus & consensus parameters
- Global system parameters (resource quotas, rate limits, deposits)
- Core DeFi primitives: CLAMM, CLOB, Bucket, Multisig, Staking
- Token economics: $MORM issuance, burn rates, buyback mechanisms
- High-authority actions: Emergency pause, validator set, slashing, oracle whitelists

### WASM-Governed (Application-Level)

- Sub-DAOs and community governance contracts
- Custom bucket product templates
- Agent-specific voting logic and policy execution
- Custom CLAMM hook templates or dynamic fee curves
- Structured product policies (e.g., auto-rebalancing rules)
- Application-level treasury for sub-DAOs
- Reputation and KYA policy extensions

## 3. Safe Integration Mechanism

- WASM policy contracts deployed via safe wrapper (`deploy_policy_contract`)
- Proposed by agents with KYA/VC DID delegation and scoped claims
- Ratified by native Step 9 vote (supermajority)
- Activated through safe native wrappers with limited authority
- All interactions logged immutably with reputation impact

## 4. Constitutional Amendment Proposals

### MORP-GOV-001: Bucket-as-Service Launch

- Activates BaS; constitutional params (deposits, fees, quotas)
- Safe mode templates
- Parameters: bas_creation_deposit_morph, bas_listing_fee_*, bas_max_products_per_did_per_epoch, etc.

### MORP-GOV-2026-02: CLAMM A2A VC Claims

- VC claims: clamm_a2a_swap, clamm_a2a_liquidity_add, clamm_a2a_liquidity_remove, clamm_a2a_hook_trigger
- Quotas: clamm_a2a_max_agents_per_did, clamm_a2a_rate_limit_per_agent
- Stake for "Verified A2A Agent" badge

### MORP-GOV-2026-03: Bucket A2A VC Claims

- Claims: can_deploy_bucket, can_trade_bucket, can_rebalance_bucket, can_claim_insurance_bucket
- Quotas: max_buckets_per_did_epoch, max_a2a_trades_per_bucket

### BA-OVERLAP-PENALTY-001: Overlap Penalties

- Economic disincentives for WASM duplicating native primitives
- Parameters: wasm_overlap_deposit_multiplier, wasm_overlap_usage_fee_morph, wasm_overlap_insurance_contribution
- Fee split: 50% burned, 35% insurance, 15% treasury

## 5. Amendment Process

- **Tx Type**: MsgConstitutionalAmendment
- **Threshold**: ≥66.67% supermajority of active validators
- **Timelock**: 7 days for major changes (fee increases >20%, quota changes >50%)
- **Emergency**: 51% for emergency proposals (with justification and sunset clause)

## 6. $MORM Value Drivers

- Governance participation: Stake $MORM for voting power
- Proposal fees: Paid in $MORM (burn + treasury)
- Policy execution: WASM contracts pay small $MORM fees for activation
- Staking incentives: Unlock higher proposal quotas, "Verified Proposer" badges
- Treasury buybacks: Governance fees fund $MORM buybacks/burns

## Related Documents

- [design.md](../government/design.md) — Why hybrid (not pure WASM)
- [hypbrid-governance.md](../government/hypbrid-governance.md) — Native vs WASM scope
- [MORP-GOV-001.md](../government/MORP-GOV-001.md) — BaS launch
- [BA-OVERLAP-PENALTY-001.md](../government/BA-OVERLAP-PENALTY-001.md) — Overlap penalties

