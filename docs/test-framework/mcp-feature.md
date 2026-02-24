**Mormtest-MCP Specification**  
**Version**: 1.0 (February 2026)  
**Title**: Morpheum Model Context Protocol (MCP) for Agentic WASM Testing  
**Status**: Official Standard for ZeroClaw, OpenClaw, NetClaw and any compatible agent framework  
**Authors**: Morpheum Core Team (synthesized from all prior design documents)  
**Audience**: Agent framework developers, LLM orchestration engines, autonomous agents  

---

### 1. Introduction & Purpose

**Mormtest-MCP** is a lightweight, standardized, JSON-based protocol that allows **any agent framework** to connect to Mormtest and receive production-accurate simulation of the entire Morpheum WASM stack (object-centric MVCC, Host API, 9-step DAG consensus, deployment flow, Flash/Frosty/Recovery, etc.).

**Core Goal**:  
One protocol → Zero-config integration for ZeroClaw (Rust), OpenClaw (chat), NetClaw (network), and any future “Claw-style” agents.

**Key Benefits**:
- Agents speak natural language or simple JSON; never need to know consensus details.
- <6k token context per interaction.
- Streaming + checkpointing for long autonomous runs.
- Fully extensible without breaking existing agents.

---

### 2. Design Principles (Strictly Enforced)

1. **Minimalism** – Average message <2 KB. No heavy schemas.
2. **Agent-First** – Natural language + structured JSON hybrid.
3. **Framework-Agnostic** – Works with stdio, WebSocket, HTTP/JSON-RPC.
4. **Deterministic & Replayable** – Every response includes `snapshot_id` for perfect reproducibility.
5. **Secure by Default** – Sandboxed execution, optional capability list.
6. **Extensible** – Versioned methods; new features added via `mcp_extend`.
7. **Resource-Optimal** – Designed for <350 MB RAM total (including agent).

---

### 3. Transport Layers (All Supported Simultaneously)

| Transport | Use Case | Connection Method | Recommended For |
|-----------|----------|-------------------|-----------------|
| **Stdio** | Local Rust agents | `cargo run --bin mormtest-mcp` | ZeroClaw (zero-copy, fastest) |
| **WebSocket** | Chat / persistent sessions | `ws://localhost:4242/mcp` | OpenClaw, human-in-loop |
| **HTTP/JSON-RPC 2.0** | Network tools, REST clients | `POST /jsonrpc` | NetClaw, external scripts |
| **gRPC (future)** | High-throughput | Reserved | Future high-scale agents |

Default port: **4242**. All transports use the same JSON message format.

---

### 4. Message Format (All Messages)

Every message is a JSON object with these top-level fields:

```json
{
  "jsonrpc": "2.0",          // Always present
  "id": "uuid-or-number",    // Unique per request (for matching responses)
  "method": "mcp_xxx",       // Request only
  "params": { ... },         // Request payload
  "result": { ... },         // Response success
  "error": { ... },          // Response error
  "stream": true/false       // For streaming responses
}
```

---

### 5. Core Methods (MCP v1.0)

#### 5.1 Lifecycle & Session Management

| Method | Direction | Params | Result | Description |
|--------|-----------|--------|--------|-------------|
| `mcp_init` | Request | `{ "framework": "zeroclaw" \| "openclaw" \| "netclaw" \| "custom", "session_id": string, "capabilities": [] }` | `{ "status": "ready", "version": "1.0", "supported_methods": [] }` | Handshake. Framework detection for auto-adaptation. |
| `mcp_ping` | Request | `{}` | `{ "timestamp": number }` | Keep-alive + latency check. |
| `mcp_close` | Request | `{ "reason": string }` | `{}` | Graceful shutdown + snapshot archive. |

#### 5.2 Core Agentic Operations

| Method | Direction | Params | Result | Key Features |
|--------|-----------|--------|--------|--------------|
| `mcp_task` | Request | `{ "type": "explore" \| "test" \| "deploy" \| "migrate" \| "fuzz" \| "chaos", "goal": string (natural lang), "params": object }` | `{ "status": "success" \| "partial", "summary": string, "snapshot_id": string, "artifacts": { "graphs": [], "events": [], "deposit": number } }` | Main entry point. Supports streaming via `stream: true`. |
| `mcp_checkpoint` | Request | `{ "snapshot_id": string, "action": "save" \| "load" \| "fork" \| "rewind_to_step" }` | `{ "new_snapshot_id": string }` | Time-travel & branching. |
| `mcp_parallel_spawn` | Request | `{ "branches": [{ "goal": string }, ...] }` | `{ "branch_ids": [] }` | Spawn 5-20 parallel explorations. |
| `mcp_merge` | Request | `{ "branch_ids": [], "strategy": "best" \| "tournament" }` | `{ "merged_snapshot_id": string }` | Merge parallel results. |

#### 5.3 Meta & Self-Improvement

| Method | Direction | Params | Result | Description |
|--------|-----------|--------|--------|-------------|
| `mcp_adapt` | Request | `{ "framework": string, "chat_context": string }` | `{ "adapted_prompt": string }` | Dynamic prompt rewriting for specific Claw framework. |
| `mcp_meta_improve` | Request | `{ "feedback": string }` | `{ "new_invariants": [], "suggested_prompts": [] }` | Trigger Meta-Agent self-improvement. |
| `mcp_extend` | Request | `{ "new_method": string, "schema": object }` | `{ "status": "registered" }` | Runtime extension for future features. |

#### 5.4 Error Codes (Standard JSON-RPC + MCP-specific)

| Code | Meaning | When Used |
|------|---------|-----------|
| -32001 | Invalid goal | Goal too vague |
| -32002 | Snapshot not found | Bad checkpoint |
| -32003 | Resource limit | RAM / time exceeded |
| -32004 | Host API violation | Contract tried invalid call |

---

### 6. Streaming Response Format

For long-running tasks (`stream: true`):

```json
{
  "jsonrpc": "2.0",
  "id": "req-123",
  "stream": true,
  "chunk": {
    "type": "progress" \| "log" \| "graph" \| "event" \| "final",
    "data": { ... }
  }
}
```

Final chunk always has `"type": "final"` + full result.

---

### 7. Security & Sandboxing

- All execution runs in isolated Wasmi sandbox.
- Optional `--sandbox` flag (default: on).
- Capability list passed at init (e.g., allow crosschain simulation).
- No file-system access unless explicitly enabled per session.
- Rate limiting per session (Pipes-inspired).

---

### 8. Example Integrations

**ZeroClaw (Rust)**
```rust
let mut mcp = MormtestMCP::stdio();
mcp.task("explore", "restaking with 20 concurrent agents").await;
```

**OpenClaw (Chat)**
User: “Test my new staking contract”  
→ OpenClaw sends WebSocket `mcp_task`  
→ Mormtest streams human-readable summary + “Show graph” button.

**NetClaw (Network)**
```json
{
  "method": "mcp_task",
  "params": {
    "type": "chaos",
    "goal": "Test crosschain settlement under 500ms delay",
    "params": { "delay_ms": 500 }
  }
}
```

---

### 9. Extensibility & Future-Proofing

- Version negotiation in `mcp_init`.
- `mcp_extend` allows new methods without protocol bump.
- All improvements from v1.1 (Persistent Memory Hub, Multi-Model Router, Parallel Engine, etc.) will be exposed as new MCP methods.

---

### 10. Reference Implementation Status

- Rust server: `mormtest-mcp` crate (ready for Week 1)
- Adapters: ZeroClaw, OpenClaw, NetClaw (Week 2-3)
- Full spec JSON schema available in repo (upon implementation)

---

This is the **complete, production-ready MCP protocol specification**. It is deliberately simple enough for any agent framework to implement in one afternoon, yet powerful enough to support the full agentic vision we have built across all previous documents.

**Next Steps (your choice):**
1. “Release v1.0 spec to GitHub” → I generate full repo structure + README
2. “Give me ZeroClaw adapter code skeleton”
3. “Give me OpenClaw WebSocket integration example”
4. “Add the Hierarchical Persistent Memory Hub as MCP method now”

Just reply with your preferred next document and I will deliver it immediately.

Mormtest-MCP is now officially the standard.  
Let’s ship it, MorpheumX. 🚀