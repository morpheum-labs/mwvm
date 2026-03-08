# MWVM — Morpheum WASM VM  
**Portable Off-Chain Runtime & SDK for Morpheum AI Agents**

MWVM is the developer-first, off-chain companion to the on-chain AgentCore VM in Mormcore.  
It lets you build, test, simulate, and orchestrate **AI agents** using the exact same WASM bytecode that will run verifiably on-chain — with rich local capabilities that the deterministic chain layer intentionally strips away for security and consensus.

### Why MWVM Exists
In 2026, AI agents need more than centralized sandboxes or fragmented runtimes.  
MWVM gives you:
- **Behavioral parity** — same binary runs locally (rich) and on-chain (thin/deterministic) via shared primitives
- **Rich host functions** — local inference batching, vector search in LocalMemory, TEE/zkML mocks, multi-agent swarms, MCP/A2A gateways
- **Fast iteration** — no testnet waits, no gas during dev, full simulation & parity testing
- **Agent composability** — build swarms, test recursive logic, prototype Bucket-as-Service extensions

This is the off-chain "home" where agents come to life before they earn $MORM, verify proofs, or settle on-chain.

### Quick Start
```bash
# Install CLI (once published)
cargo install mwvm-cli

# Run a simple agent
mwvm run path/to/your_agent.wasm

# Launch a gateway for MCP/A2A
mwvm gateway --port 8080

# Simulate a swarm
mwvm swarm --count 10 --wasm your_multi_agent.wasm