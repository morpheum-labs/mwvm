# Securities & Testing — Security Model & Agentic Testing

**Version**: 1.0  
**Date**: 05 March 2026  
**Status**: Design  
**Source**: mwvm/docs/securities, mwvm/docs/test-framework

## 1. MWVM Security Model

MWVM security is built on:

- **Host is God** — WASM = pure compute; all I/O via sandboxed Host API
- **Object-centric MVCC + capability** — Version checks, ownership, transient memory
- **KYA/DID + VC delegation** — Scoped, revocable agent permissions (v2.4)
- **Native-only protocol features** — Multisig, full CLAMM, staking core remain built-in; not exposed to WASM

## 2. Category-by-Category Security Review


| Category                | Risk Level | Key Concerns                | v2.5 Countermeasures                                        |
| ----------------------- | ---------- | --------------------------- | ----------------------------------------------------------- |
| Object Management       | Medium     | Unauthorized mutation, spam | Mandatory VC for write/create/transfer; per-DID rate limits |
| Bank / Bucket Transfers | High       | Fund drains, spam           | Safe wrappers + value/rate caps                             |
| Token Issuance          | Very High  | Inflation, spam             | Safe wrapper + supply/epoch caps                            |
| Order Placement/Cancel  | High       | Order spam, MEV             | Safe wrappers + notional/rate caps + backpressure           |
| Crosschain              | High       | Bridge exploits             | Mandatory VC + per-chain whitelist                          |


## 3. Overlap Penalties (BA-OVERLAP-PENALTY-001)

Economic disincentives for WASM contracts duplicating native primitives:


| Parameter                           | Initial Value | Purpose                              |
| ----------------------------------- | ------------- | ------------------------------------ |
| wasm_overlap_deposit_multiplier     | 2.0×          | Higher stake for detected overlaps   |
| wasm_overlap_usage_fee_morph        | 15 $MORM/call | Extra fee for duplicate native calls |
| wasm_overlap_insurance_contribution | 35%           | Routed to BaS Insurance Fund         |
| wasm_overlap_treasury_allocation    | 15%           | Treasury buybacks                    |


**Fee split**: 50% burned, 35% insurance, 15% treasury. No hard rejects — purely economic.

## 4. Agentic WASM Testing

### Current Landscape (2025–2026)

- **EVM**: Mature (EVMbench, SCONE-bench) — agents detect, patch, exploit
- **WASM** (CosmWasm, ink!): Emerging — cw-multi-test, ink! test utils, WASIF fuzzing
- **Agentic layers**: CrewAI, LangGraph, AutoGPT-style orchestration

### Recommended Stack for DAG + WASM

1. **Base**: cw-multi-test or Substrate test utils for local DAG simulation
2. **Agent Layer**: CrewAI or LangGraph — Planner → Code → Tester → Security Agent
3. **Integration**: Wasmtime/Wasmi for deterministic execution; storage mocking, oracle simulation
4. **Autonomy**: Self-healing (regenerate failing tests), exploration, structured reporting

### Mormtest Integration

- Full KYA simulation including safe wrappers and BaS policy testing
- Overlap warnings in every mcp_task result
- `simulate_overlap_cost` tool for agents
- Native infrastructure simulation remains internal

## 5. BaS Security Concerns (from security-concern.md)


| Concern              | Severity    | Countermeasure                                    |
| -------------------- | ----------- | ------------------------------------------------- |
| Deployment Spam      | High        | Creation deposit, quotas, backpressure            |
| Fraudulent Products  | Very High   | Health snapshot, insurance fund, slashing         |
| Risk Contagion       | High        | Bucket isolation, depth limiter, first-loss $MORM |
| Non-Atomic P2P Sales | High        | Atomic escrow, no reentrancy window               |
| Secondary Market MEV | Medium-High | Minimum listing duration, reputation gating       |


## Related Documents

- [../securities/README.md](../securities/README.md) — Security index
- [../securities/overlap-features.md](../securities/overlap-features.md) — Overlap penalties
- [../test-framework/test-framework.md](../test-framework/test-framework.md) — Agentic testing source
- [04-governance.md](04-governance.md) — BA-OVERLAP-PENALTY-001

