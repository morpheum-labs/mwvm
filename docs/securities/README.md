# MWVM Security Documentation

Security reviews, permission models, and safe-access patterns for the Morpheum WASM VM (MWVM).

---

## Overview

MWVM security is built on:

- **Host is God** — WASM = pure compute; all I/O via sandboxed Host API
- **Object-centric MVCC + capability** — Version checks, ownership, transient memory
- **KYA/DID + VC delegation** — Scoped, revocable agent permissions (v2.4)
- **Native-only protocol features** — Multisig, full CLAMM, staking core remain built-in; not exposed to WASM (see [mormcore multisign do-not-support-vm](../../mormcore/docs/multisign/do-not-support-vm.md))

---

## Document Index

| Document | Description |
|----------|-------------|
| [vm-security-review.md](./vm-security-review.md) | **Host API security review** — Category-by-category risk assessment (object, DAG context, idempotency, oracle, crosschain, staking, KYA), recommended countermeasures, permission model |
| [security-concern-agents.md](./security-concern-agents.md) | **Safe access for agents** — CLAMM/ReClamm access via native Msg calls, KYA/VC delegation, multisig, reputation gating; never raw from WASM |
| [overlap-features.md](./overlap-features.md) | **Overlapping features in agentic WASM** — Economic penalties, Mormtest guidance, governance backstop; discourages duplication of native primitives |
| [review-v2.4-1.md](./review-v2.4-1.md) | MWVM v2.4 spec — Native-only clarification, KYA delegation group, Host API 43+ functions |
| [review-2.4-2.md](./review-2.4-2.md) | MWVM v2.4 spec — Full security review + permission model incorporated |

---

## Quick Reference

| Topic | Start Here |
|-------|------------|
| Host API risk levels & mitigations | [vm-security-review.md](./vm-security-review.md) |
| Agent/contract access to CLAMM | [security-concern-agents.md](./security-concern-agents.md) |
| Native vs VM boundary | [review-v2.4-1.md](./review-v2.4-1.md) §1, §3 |
| Overlap penalties & agent guidance | [overlap-features.md](./overlap-features.md) |
