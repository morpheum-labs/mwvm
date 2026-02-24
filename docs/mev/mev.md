**Comparison of MEV (Maximal Extractable Value) in WASM-based VMs vs. EVM**

MEV refers to the profit validators/block producers can extract by reordering, including, or censoring transactions in a block — most famously sandwich attacks, arbitrage, liquidations, and front-running in DEXes.

The **virtual machine itself** (EVM vs. WASM) is **not the primary driver** of MEV. MEV is mostly shaped by:

- Mempool visibility & transaction ordering rules
- Block production mechanism (PoW/PoS/DAG, leader selection, private mempools)
- Gas/fee market design
- Application-layer logic (especially AMMs/DEXes)
- Network latency & propagation

That said, there are **meaningful indirect differences** between typical **EVM chains** (Ethereum mainnet, most L2s) and **WASM-based chains** (NEAR, Polkadot parachains, CosmWasm chains, etc.). Below is a structured comparison.

### 1. Core Architectural Differences That Influence MEV Surface

| Aspect                        | EVM (Ethereum-style)                                      | WASM (NEAR, CosmWasm, Polkadot ink!, etc.)                          | MEV Impact Winner          |
|-------------------------------|------------------------------------------------------------|---------------------------------------------------------------------|----------------------------|
| **Execution model**           | Stack-based, 256-bit words, very high-level gas ops        | Register-based, near-native speed, dynamic metering                 | WASM (faster = shorter windows) |
| **State access**              | Global account storage (SLOAD/SSTORE), reentrancy possible | Host-mediated (object/KV), usually stricter access control         | WASM (harder reentrancy)   |
| **Typical gas metering**      | Fixed per-opcode (very predictable but expensive)          | Dynamic / weight-based (more flexible, often cheaper for complex ops) | WASM (less economic pressure on tight orders) |
| **Reentrancy vulnerability**  | Very common (The DAO, many others)                         | Much rarer (sandbox + host functions, no direct calls in many impl.) | WASM                       |
| **Execution speed**           | Slow (interpreters/JIT still lag native)                   | 10–100× faster in many benchmarks (Wasmtime, Wasmer)                | WASM (reduces profitable window) |
| **Parallel execution**        | Almost none on L1 (serial)                                 | Often supported (sharding, object models, Block-STM style)          | WASM (dilutes sequential MEV) |

### 2. MEV Vectors — Head-to-Head

| MEV Type                      | EVM Prevalence & Ease                                      | WASM Prevalence & Ease                                              | Why the difference? |
|-------------------------------|------------------------------------------------------------|---------------------------------------------------------------------|---------------------|
| **Sandwich attacks**          | Extremely common on Ethereum L1 & many L2 DEXes            | Much less common / profitable on most WASM chains                   | Faster execution + sharding + sometimes fair-ordering reduce reorder window |
| **Arbitrage (cross-DEX / cross-chain)** | Very high (Flashbots, builders compete heavily)            | High, but often lower per-opportunity due to speed & lower fees     | WASM chains usually have lower latency & cheaper complex logic |
| **Liquidations**              | Common in lending protocols                                | Similar, but faster execution can make bots compete more fiercely   | Neutral / slight WASM advantage |
| **Front-running oracle updates** | Very common                                                | Similar, but many WASM chains use more robust oracles (Band, Supra) | Neutral |
| **Transaction spam / priority gas auction** | Severe on Ethereum L1 (pre-PBS)                            | Usually milder (dynamic weights, sharding)                          | WASM |
| **Censorship / inclusion MEV** | High (builders can exclude txs)                            | Depends on consensus (many WASM chains use BFT or DAG → harder)     | WASM (in many cases) |

### 3. Real-World Observations (2025–2026 Context)

- **Ethereum (EVM)** — Still has the **largest absolute MEV** because it has:
  - The highest TVL and DEX volume
  - Public mempool (until PBS matures)
  - Builder market with intense competition
  - → MEV extraction is in the **billions of USD/year**

- **NEAR (WASM)** — Nightshade sharding + fast finality + chunk producers → **much lower sandwich MEV** than equivalent TVL on Ethereum. Speed reduces profitable reorder windows.

- **CosmWasm chains** (Osmosis, Juno, etc.) — IBC creates cross-chain arb, but **per-chain MEV is lower** due to:
  - CometBFT finality (~1–2 s)
  - No global public mempool in the same way
  - Often lower overall volume per chain

- **Polkadot parachains (ink! / WASM)** — Parachain slot + shared security → MEV is **mostly local to parachain**. Relay chain validators have limited ability to extract cross-parachain MEV.

### 4. Bottom Line — Which Has More/Less MEV?

| Dimension                     | Winner (Lower MEV) | Explanation |
|-------------------------------|--------------------|-------------|
| **MEV per $1 TVL**            | WASM chains        | Faster execution + often better ordering / sharding dilute opportunities |
| **Absolute MEV extraction**   | EVM (Ethereum ecosystem) | Simply because of massive liquidity & public mempool |
| **Ease of sandwiching**       | WASM               | Shorter profitable windows + parallel execution in many implementations |
| **Economic incentive to extract** | EVM                | Higher gas volatility + expensive complex ops create more pressure |
| **Long-term trend (2026+)**   | WASM direction     | As chains adopt PBS, private mempools, encrypted mempools — difference shrinks, but WASM speed advantage remains |

**In summary (2026 view)**  
- The **EVM ecosystem** currently sees **far more MEV in absolute terms** — mostly because it has far more economic activity.
- **WASM-based VMs** tend to exhibit **structurally lower MEV per unit of activity** thanks to faster execution, more flexible metering, stricter state access patterns, and — in many implementations — better parallelism or finality properties.

If your chain (like Morpheum) combines **WASM execution** with **DAG/blocklace structure**, **Flash fast-path**, **object-centric dependencies**, and **fair VRF leadership**, you are positioned on the **low-MEV side** of the spectrum — even compared to other WASM chains.

Would you like a deeper dive into any specific vector (e.g. sandwich math on fast vs slow VMs), or how to add MEV mitigation primitives to the Morpheum Host API?