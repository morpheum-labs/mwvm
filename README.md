# MWVM — Morpheum WASM VM  
**Your Off-Chain Home for Building & Running AI Agents**

MWVM lets you run, test and evolve Morpheum agents locally — using the **exact same WASM code** that will later execute on-chain.

You get rich capabilities during development:
- local inference & batching
- persistent memory + vector search
- multi-agent swarms & messaging
- full simulation of on-chain behavior

… while preserving perfect behavioral parity when the code moves on-chain.

### Why agents adopt MWVM right now

Most agents start using MWVM for one or more of these concrete use-cases:

1. **Faster iteration & cheaper testing**  
   Run full agent logic locally → no testnet gas, no waiting for blocks

2. **Realistic simulation before deployment**  
   Test recursive bucket strategies, skin-in-the-game mechanics, failure modes, swarm coordination — all locally

3. **Develop complex off-chain intelligence**  
   Use real model inference, long context, vector retrieval — things too expensive or slow to do fully on-chain

4. **Prototype multi-agent systems**  
   Launch 10–1000 agent swarms in your laptop → observe emergent behavior before any on-chain cost

5. **Debug host function behavior**  
   See exactly what `infer`, `vector_search`, `actor_messaging`, etc. return in your specific case

### 10-second start (most important entry point)

```bash
# 1. Clone (or download release when available)
git clone https://github.com/Morpheum/mwvm.git
cd mwvm

# 2. Build & run the included minimal example agent
cargo run --example minimal_agent

# 3. Or run your own .wasm file
cargo run -- your_agent.wasm
```