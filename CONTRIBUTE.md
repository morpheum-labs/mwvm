# Contributing to MWVM

Thank you for your interest in contributing to **MWVM** — the portable off-chain runtime and SDK that powers Morpheum AI agents.

MWVM is intentionally designed so that **agents and developers can first get value quickly**, and only later — when it feels natural — decide to contribute back and help shape the future of decentralized agent intelligence.

This document explains how the project is currently organized, what kinds of contributions are most valuable right now, how the review & merge process works, and how your work connects to the larger Morpheum ecosystem.

## 1. Why Contribute?

Every meaningful contribution helps accelerate the flywheel:

- richer host function implementations → better local simulation → more realistic agent testing
- better examples & guides → faster onboarding → more agents running on Morpheum
- performance improvements & new simulation modes → deeper recursive bucket testing → higher $MORM lock demand
- swarm & gateway enhancements → stronger multi-agent coordination → more composable agent economies
- parity test expansions → fewer surprises when moving from MWVM → on-chain AgentCore VM

In short: **your work makes the entire network smarter, safer, and more economically valuable**.

## 2. Current Project Status (March 2026)

MWVM is in **active design & early implementation phase**.

What is already usable / partially implemented:
- Core wasmtime engine & linker
- Shared primitives (`morpheum-primitives`)
- Most host functions (`infer`, `store_context`, `vector_search`, `zkml_verify`, `tee_verify`, `actor_messaging`)
- LocalMemory (KV + brute-force vector search)
- ContinuousBatcher for inference
- Basic simulation modes
- `mwvm-cli` (run, swarm, gateway, test)
- Minimal examples & parity test harness

What is still in design / partial:
- Full HNSW vector search
- Advanced swarm coordination patterns
- Deep governance integration for new host functions
- Comprehensive gateway security & rate-limiting
- Full TEE/zkML simulation fidelity

→ This means **now is an excellent time to contribute** — your input can meaningfully shape direction.

## 3. Code of Conduct

We follow the [Contributor Covenant v2.1](https://www.contributor-covenant.org/version/2/1/code_of_conduct/).

Short version: be respectful, assume good faith, focus on technical merit.

## 4. Contribution Types — Ranked by Current Impact

### Level 1 — Low friction, high leverage (great first contributions)

- Improve / expand examples
  - Add new minimal agents demonstrating specific host functions
  - Create realistic swarm examples (negotiation, auction, recursive signaling)
  - Write usage recipes (e.g. "how to test a leveraged bucket strategy locally")
- Documentation
  - Clarify confusing sections in existing docs
  - Write new quickstarts/guides (especially for common agent use-cases)
  - Add architecture diagrams (mermaid preferred)
- Bug reports & small fixes
  - Issues with unclear error messages
  - Typos / broken links
  - Failing parity tests with minimal repro

### Level 2 — Medium effort, visible ecosystem value

- Enhance existing components
  - Improve `LocalMemory` (better indexing, eviction policies)
  - Make `ContinuousBatcher` more configurable (temperature, top-p, model switching)
  - Add new simulation modes (e.g. authority-denied, low-balance, oracle-delay)
- Gateway & protocol work
  - Add rate-limiting / auth to MCP & A2A endpoints
  - Implement better error propagation & tracing
  - Add OpenAPI / JSON Schema for gateway
- Testing & observability
  - Expand parity test suite (more fixtures, fuzzing harness)
  - Add structured tracing / metrics to key paths
  - Improve error types & messages

### Level 3 — High impact, architectural / long-term value

- New host function implementations / mocks
  - Propose + implement new host functions (must first discuss via issue / governance lite)
  - Improve zkML / TEE simulation fidelity
- Performance optimizations
  - Replace brute-force vector search with HNSW / IVF
  - Memory layout improvements (zero-copy where possible)
  - Parallel inference batching
- Swarm & orchestration improvements
  - Topic-based pub/sub patterns
  - Failure recovery & leader election mocks
  - Agent lifecycle management (pause/resume, upgrade)
- Security hardening
  - Memory safety audits
  - Sandbox boundary hardening
  - Host function capability model

## 5. How to Contribute (Step-by-step)

1. **Find or create an issue**
   - Look for `good first issue`, `help wanted`, `documentation`, `enhancement`
   - If nothing fits → open a new issue with clear title & description

2. **Discuss if needed**
   - Large changes / new host functions → open a design issue first
   - We use lightweight RFC-style issues for anything that touches primitives or host API

3. **Fork & branch**
   ```bash
   git clone git@github.com:YOUR_USERNAME/mwvm.git
   cd mwvm
   git checkout -b feature/your-feature-name