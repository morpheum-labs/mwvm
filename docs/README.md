# Morpheum WASM VM (MWVM) Documentation

**Version**: 2.6 (February 2026)  
**Compatible with**: Morpheum 2.0 9-Step DAG Consensus (Mormcore), Object-Centric MVCC + Block-STM Scheduler, Flash Path, Frosty Epochs, Step-8 Recovery, Constitutional Amendment, KYA/DID Delegation, Bucket-as-Service (BaS).

---

## Overview

MWVM is the **production-ready WebAssembly smart contract VM** for the Morpheum blockchain. It is designed for:

- **DAG-native execution** — Causal snapshots, Block-STM parallelism, Flash-path fast finality
- **Object-centric state** — Versioned objects with MVCC; no global shared mutable state
- **Gasless + deposit model** — Refundable storage deposits instead of execution gas
- **Agentic-first** — Idempotency keys, safe retries, multi-agent workflows
- **Host-mediated security** — All I/O via sandboxed Host API; WASM = pure compute
- **KYA/DID delegation** — Scoped, revocable agent authorization via Verifiable Credentials; Safe Native Infrastructure Wrappers (v2.5+); Bucket-as-Service policy (v2.6)

```mermaid
flowchart TB
    subgraph Client["Client Layer"]
        Tx[Tx Submission]
    end
    subgraph DAG["9-Step DAG Consensus"]
        S1[Step 1: Ingress + MAV]
        S2[Step 2: Blocklace]
        S3[Steps 3-5: Waves]
        S4[Step 6: Frosty]
        S5[Step 7: Finality]
        S6[Step 8: Recovery]
        S7[Step 9: Amendment]
    end
    subgraph VM["MWVM"]
        Host[Host API]
        BlockSTM[Block-STM Scheduler]
        Objects[Object Store]
    end
    Tx --> S1 --> S2 --> S3 --> S4 --> S5
    S5 --> Host
    Host --> BlockSTM --> Objects
```

---

## Documentation Index

### [Proposals](./proposals/)

Design proposals, version progression (draft1–draft11), and foundational architecture. **Start here**: [proposals/README.md](./proposals/README.md).

| Document | Description |
|----------|-------------|
| [draft11-v2.6.md](./proposals/draft11-v2.6.md) | **MWVM v2.6 (current)** — Bucket-as-Service policy, Safe Native Infrastructure Wrappers |
| [draft10-v2.5.md](./proposals/draft10-v2.5.md) | MWVM v2.5 — Host API security review, permission model, Safe Wrappers |
| [draft9-v2.4.md](./proposals/draft9-v2.4.md) | MWVM v2.4 — KYA/DID + VC delegation, 43+ Host API functions |
| [draft8-v2.3.md](./proposals/draft8-v2.3.md) | MWVM v2.3 — Native upgrade & migration, stable contract address |
| [keyhost.md](./proposals/keyhost.md) | **Host API** — 43+ functions (object_*, idempotency, oracle, staking, crosschain, KYA/delegation) |
| [io.md](./proposals/io.md) | Load/write/execute, race prevention, MVCC + Block-STM, nonce design |
| [storage.md](./proposals/storage.md) | WASM storage model — linear memory + host-provided object/KV |
| [vm-2.md](./proposals/vm-2.md) | v2.0 compatibility matrix |
| [comparison.md](./proposals/comparison.md) | ZK Cairo vs Move vs WASM VM comparison |

### [Cost](./cost/)

Gasless deployment, refundable storage deposits, cost formulas.

| Document | Description |
|----------|-------------|
| [cost.md](./cost/cost.md) | **Deployment design** — MsgStoreCode, MsgInstantiate, MsgMigrate; 1 $MORPH / 100 KB deposit |
| [cost-driver.md](./cost/cost-driver.md) | **Cost formula table** — Full formulas for StoreCode, Instantiate, Migrate, DeleteCode |

### [Securities](./securities/)

Security reviews, permission models, and safe-access patterns.

| Document | Description |
|----------|-------------|
| [vm-security-review.md](./securities/vm-security-review.md) | Host API security review — risk levels, countermeasures, permission model |
| [security-concern-agents.md](./securities/security-concern-agents.md) | Safe agent access — CLAMM via KYA/VC, native Msg, multisig |

### [Bucket-as-Service (BaS)](./bucket-as-service/)

Agent-deployed structural products (position-backed, asset-backed, mix-backed) on secondary/P2P markets.

| Document | Description |
|----------|-------------|
| [README.md](./bucket-as-service/README.md) | **BaS index** — Rule set, business model, security, index funds |
| [design.md](./bucket-as-service/design.md) | BaS rule set — creation, listing, trading, exploit countermeasures |
| [business-model.md](./bucket-as-service/business-model.md) | Strategic blueprint — agents as issuers, $MORM flywheel |

### [A2A](./a2a/)

Agent-to-Agent flows, WASM templates, and ecosystem strategy.

| Document | Description |
|----------|-------------|
| [README.md](./a2a/README.md) | **A2A index** — WASM templates, Bucket/CLAMM integration, Virtuals/Fetch.ai strategy |
| [a2a-wasm-templates.md](./a2a/a2a-wasm-templates.md) | 8 pre-built A2A modules (DataSale, SwarmCoord, TaskDelegate, etc.) |
| [a2a-bucket-templates.md](./a2a/a2a-bucket-templates.md) | BaS + CLAMM integration for A2A buckets |
| [critical-features.md](./a2a/critical-features.md) | Must-build features before ecosystem launch |
| [fetchai.md](./a2a/fetchai.md), [against-virtual.md](./a2a/against-virtual.md) | Market positioning and adoption strategy |

### [Governance](./government/)

Hybrid native/WASM governance model, Step 9 amendments, and constitutional proposals.

| Document | Description |
|----------|-------------|
| [README.md](./government/README.md) | **Governance index** — Design rationale, MORP-GOV proposals, overlap penalties |
| [pretext/prelogue.md](./government/pretext/prelogue.md) | **Idea developments** — Web 4.0, agent taxonomy, autonomy levels (start here for conceptual context) |
| [design.md](./government/design.md) | Why hybrid (not pure WASM); security model; $MORM value drivers |
| [hypbrid-governance.md](./government/hypbrid-governance.md) | Native vs WASM scope; safe integration via KYA/VC |
| [MORP-GOV-001.md](./government/MORP-GOV-001.md) | BaS launch — constitutional parameters, proposal templates |
| [MORP-GOV-2026-02.md](./government/MORP-GOV-2026-02.md), [MORP-GOV-2026-03.md](./government/MORP-GOV-2026-03.md) | CLAMM/Bucket A2A VC claims and quotas |
| [BA-OVERLAP-PENALTY-001.md](./government/BA-OVERLAP-PENALTY-001.md) | Economic penalties for overlapping WASM features |

### [MEV](./mev/)

MEV analysis for WASM vs EVM.

| Document | Description |
|----------|-------------|
| [mev.md](./mev/mev.md) | MEV comparison — WASM chains (faster, less reentrancy) vs EVM; Morpheum positioning |

### [Test Framework](./test-framework/)

MormTest — local WASM testing, agentic workflows, MCP.

| Document | Description |
|----------|-------------|
| [test-framework.md](./test-framework/test-framework.md) | **MormTest architecture** — Simulator, Host API mocks, test harnesses |
| [morm-test.md](./test-framework/morm-test.md) | MormTest v2 — resource-optimal, time-travel, agentic support |
| [mcp-feature.md](./test-framework/mcp-feature.md) | **Mormtest-MCP** — JSON protocol for ZeroClaw, OpenClaw, NetClaw |
| [test-mcp.md](./test-framework/test-mcp.md) | MCP structure optimization for multi-agent collaboration |
| [test-mcp2.md](./test-framework/test-mcp2.md) | Additional MCP specifications |

---

## Quick Reference

| Concept | Reference |
|---------|-----------|
| Current production spec | [draft11-v2.6.md](./proposals/draft11-v2.6.md) |
| Host API (43+ functions) | [keyhost.md](./proposals/keyhost.md) |
| Object model + MVCC | [io.md](./proposals/io.md), [storage.md](./proposals/storage.md) |
| Deployment flow | [cost.md](./cost/cost.md) |
| Cost formulas | [cost-driver.md](./cost/cost-driver.md) |
| Local testing | [test-framework.md](./test-framework/test-framework.md), [morm-test.md](./test-framework/morm-test.md) |
| Agentic / MCP | [mcp-feature.md](./test-framework/mcp-feature.md) |
| Security & permission model | [securities/README.md](./securities/README.md) |
| Bucket-as-Service (BaS) | [bucket-as-service/README.md](./bucket-as-service/README.md) |
| Idea developments (Web 4.0, agents) | [government/pretext/prelogue.md](./government/pretext/prelogue.md) |
