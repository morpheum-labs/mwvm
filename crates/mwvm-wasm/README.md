# mwvm-wasm

**Official WebAssembly + TypeScript SDK for Morpheum AI Agents**

Run high-performance Morpheum agents directly in the browser, Node.js, Next.js, or any JavaScript environment with full support for inference, persistent memory, vector search, and native proofs.

[![npm version](https://img.shields.io/npm/v/mwvm-wasm.svg)](https://www.npmjs.com/package/mwvm-wasm)
[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)

---

## Features

- **Zero-copy inference** — native model execution via the MWVM engine
- **Persistent memory** — `storeContext` + vector search (HNSW)
- **Full MCP & A2A support** — works with Claude, Cursor, Google ADK, LangGraph
- **TypeScript-first API** — excellent autocomplete and type safety
- **Works everywhere** — browsers, Node.js, Deno, Cloudflare Workers

---

## Installation

```bash
npm install mwvm-wasm
# or
yarn add mwvm-wasm
# or
pnpm add mwvm-wasm
```

---

## Quick Start

```ts
import { Agent } from "mwvm-wasm";

async function main() {
  // Load your compiled agent WASM file
  const wasmBytes = await fetch("/agents/my-agent.wasm").then(r => r.arrayBuffer());

  const agent = await Agent.create(new Uint8Array(wasmBytes));

  // Run inference
  const result = await agent.infer({
    modelHash: "0000000000000000000000000000000000000000000000000000000000000000",
    promptHash: "1111111111111111111111111111111111111111111111111111111111111111",
    maxTokens: 512,
  });

  console.log("Inference output:", result);
}

main();
```

---

## API Reference

### `Agent.create(wasmBytes: Uint8Array): Promise<Agent>`

Creates a new agent instance from compiled WASM bytes.

### `agent.infer(options: InferenceOptions): Promise<Uint8Array>`

```ts
interface InferenceOptions {
  modelHash: string;     // 32-byte hex
  promptHash: string;    // 32-byte hex
  maxTokens?: number;    // default 512
}
```

### `agent.vectorSearch(query: Float32Array, k?: number): Promise<VectorSearchResult[]>`

```ts
interface VectorSearchResult {
  score: number;
  id: number;
}
```

### `agent.storeContext(key: string, data: Uint8Array): Promise<void>`

Stores arbitrary data in the agent's persistent memory.

---

## Building from Source

```bash
# From the workspace root
cd crates/mwvm-wasm
npm run build
```

This runs `wasm-pack` and generates the `dist/` folder with:
- `mwvm_wasm.wasm`
- `mwvm_wasm.js`
- `mwvm_wasm.d.ts`

---

## Examples

See the [`examples/`](https://github.com/morpheum/mwvm/tree/main/examples) folder in the main repository:

- `basic-agent/` — Minimal single-agent example
- `web-browser-example/` — React + Vite demo
- `swarm-example/` — Multi-agent orchestration

---

## License

Apache-2.0 © Morpheum Team

---


