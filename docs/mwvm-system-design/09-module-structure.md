# MWVM — Module Structure & Native vs WASM Scope

**Version**: 1.0  
**Date**: 05 March 2026  
**Status**: Design  
**Source**: mwvm/docs/government (hypbrid-governance, design)

## 1. Native Infrastructure (Never Exposed Raw)

These modules and business logic must stay native. Exposing them to WASM would create catastrophic risk in a permissionless environment.


| Module                 | Owned Logic                                                                                       |
| ---------------------- | ------------------------------------------------------------------------------------------------- |
| **Consensus**          | Flash path thresholds, wave quorum, Frosty epoch rules, Step 8 rollback, VRF params               |
| **Global System**      | Resource quotas, rate limits, max objects per DID, deposit rates, storage fees                    |
| **CLAMM**              | Glide rate (τ), centeredness margin (m), volatility σ, max hooks, virtual balance caps, fee tiers |
| **CLOB**               | Matching engine params, order rate limits, backpressure, MEV resistance                           |
| **Bucket System**      | Allowed collateral types, liquidation thresholds, insurance fund rules                            |
| **Multisig**           | Global threshold limits, recovery time bounds                                                     |
| **Staking / Treasury** | Reward rates, restaking, treasury allocation                                                      |
| **Token Economics**    | $MORM issuance, burn rates, buyback mechanisms                                                    |
| **High-Authority**     | Emergency pause, validator set, slashing, oracle whitelists, constitutional amendments            |


## 2. WASM-Governed (Application-Level)

These are on WASM for flexibility, agentic innovation, and decentralized experimentation.


| Module              | Owned Logic                                           |
| ------------------- | ----------------------------------------------------- |
| Sub-DAOs            | Community governance contracts                        |
| Bucket Templates    | Custom position/asset/mix-backed rules and parameters |
| Agent Voting        | Custom voting logic and policy execution              |
| CLAMM Hooks         | Custom hook templates or dynamic fee curves           |
| Structured Products | Auto-rebalancing rules for mix-backed buckets         |
| Sub-DAO Treasury    | Application-level treasury management                 |
| Reputation/KYA      | Custom scoring rules, policy extensions               |


## 3. Safe Wrapper Integration

- WASM policy contracts deployed via `deploy_policy_contract`
- Proposed by agents with KYA/VC DID delegation and scoped claims
- Ratified by native Step 9 vote (supermajority)
- Activated through safe native wrappers that call core modules with limited authority
- All interactions logged immutably with reputation impact

## 4. Example Flow

1. Agent deploys WASM policy contract proposing new CLAMM fee curve
2. Submits proposal via KYA/VC
3. Native governance votes and ratifies
4. Policy activated via safe wrapper that updates CLAMM parameters with scoped limits

## 5. Key Integration Points


| Interface               | Purpose                                        |
| ----------------------- | ---------------------------------------------- |
| read_constitution_param | Read bas_*, wasm_overlap_*, clamm_a2a_* params |
| check_delegation_scope  | Validate VC claims before wrapper execution    |
| vc_verify               | Verify VC signature and expiry                 |
| revoke_vc               | Instant revocation; immutable changelog        |


## 6. Benefits of Hybrid Structure


| Benefit            | How Achieved                                                        |
| ------------------ | ------------------------------------------------------------------- |
| Security           | Core native and protected; WASM sandboxed and scoped                |
| Performance        | Core logic gasless and optimized                                    |
| Decentralization   | Agents innovate freely at application level                         |
| $MORM Appreciation | Governance fees, staking, treasury buybacks from WASM activity      |
| Scalability        | Native handles global rules; WASM handles high-volume agentic logic |


## 7. Design Principle

**Keep core primitives (CLAMM, CLOB, buckets, staking, multisig, token issuance) under native governance. Allow WASM for application-level policies and agent-created products.**

## Related Documents

- [04-governance.md](04-governance.md) — Hybrid governance, constitutional amendments
- [02-architecture.md](02-architecture.md) — Host API, safe wrappers
- [../government/hypbrid-governance.md](../government/hypbrid-governance.md) — Source

