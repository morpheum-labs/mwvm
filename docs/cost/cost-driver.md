Here is a **full cost formula table** for **Morpheum WASM smart contract deployment**, based on the gasless + refundable storage deposit model previously recommended for your chain.

This table covers:
- **MsgStoreCode** (uploading the immutable WASM bytecode → code object)
- **MsgInstantiate** (creating a contract instance from stored code + initial state)
- **MsgMigrate** (upgrading an existing contract)
- **MsgDeleteCode** (cleanup unused code, full refund if no instances remain)

All amounts are in **$MORPH** (native token).  
The model is **refundable** (deposit locked, not burned), **spam-resistant**, and **amendable via Step 9 constitutional tx**.

### Core Parameters (Constitutional Defaults – Amendable)
- **Base storage rate**: 1 $MORPH per 100 000 bytes (100 KB) of **compressed bytecode** (zstd recommended/required)
- **Minimum code deposit**: 0.1 $MORPH (prevents dust spam for tiny files)
- **Compression**: Mandatory zstd (or equivalent); deposit calculated after compression
- **Instantiation flat fee**: 0.02 $MORPH (covers object creation + small initial state)
- **Migrate flat fee**: 0.05 $MORPH (slightly higher due to potential state migration logic)
- **Flash path eligibility**: Yes for non-conflicting deploys (no waves needed → faster inclusion)
- **Refund policy**: Full code deposit refund on successful MsgDeleteCode (if zero instances remain); no refund on instantiation/migrate fees

### Full Cost Formula Table

| Action                  | Formula / Calculation                                                                 | Typical Real-World Size (optimized Rust WASM) | Deposit / Fee ($MORPH) | Approx. USD Equivalent (at $MORPH = $5) | Notes / Examples |
|-------------------------|---------------------------------------------------------------------------------------|-----------------------------------------------|-------------------------|------------------------------------------|------------------|
| **MsgStoreCode** (upload bytecode) | max( min_deposit, ceil(compressed_bytes / 100_000) × 1 )                             | 40–80 KB (minimal / counter)                 | 0.4 – 0.8              | $2.00 – $4.00                           | Hello world / basic counter |
|                         |                                                                                       | 80–150 KB (token / cw20-like)                | 0.8 – 1.5              | $4.00 – $7.50                           | Fungible token standard |
|                         |                                                                                       | 120–250 KB (DEX / AMM core logic)            | 1.2 – 2.5              | $6.00 – $12.50                          | Order book or simple AMM |
|                         |                                                                                       | 200–400 KB (complex DeFi / staking + restaking) | 2.0 – 4.0           | $10.00 – $20.00                         | Treasury + governance features |
|                         |                                                                                       | 400–700 KB (heavy: oracle integration, multi-call agentic) | 4.0 – 7.0     | $20.00 – $35.00                         | Full-featured protocol |
| **MsgInstantiate** (create instance) | flat_instantiate + (initial_state_bytes / 1_000_000) × 0.01 (optional state scaling) | Typical: 1–10 KB initial state               | 0.02 – 0.03            | $0.10 – $0.15                           | Most contracts: just ~0.02 |
| **MsgMigrate** (upgrade contract) | flat_migrate + (migration_delta_bytes / 1_000_000) × 0.01 (if state change large)   | Typical migration (no big state rewrite)     | 0.05 – 0.10            | $0.25 – $0.50                           | Admin-only; capability check required |
| **MsgDeleteCode** (cleanup unused code) | -0 × code_deposit (full refund if no instances)                                      | Any size                                      | - (full refund)        | + full amount back                      | Incentivizes removing dead code |
| **Total for fresh deploy** (store + instantiate) | MsgStoreCode deposit + 0.02 instantiate                                      | 100 KB optimized                             | 1.02                   | ~$5.10                                  | Very common case |
|                         |                                                                                       | 250 KB optimized                             | 2.52                   | ~$12.60                                 | Medium complexity DEX |
| **Dust / spam protection** | min_deposit enforced; Pipes rate-limiting in Step 1 applies separately               | <10 KB malicious tiny file                   | 0.1 (minimum)          | $0.50                                   | Prevents thousands of 1-byte spam txs |

### Additional Notes on the Model
- **Compression mandatory** — deposit calculated **after** zstd compression (reduces cost 40–70% vs raw bytecode). Host validates and rejects if not compressed or if compression ratio suspiciously low (anti-abuse).
- **No gas for execution** — Morpheum remains fully gasless; only storage deposit + tiny flat fees.
- **Flash-path advantage** — Non-conflicting code uploads (different shards or no hot-object overlap) finalize in <150 ms via Flash → very fast deploys.
- **State rent (future extension)** — If you later want ongoing storage fees for large contract state (like NEAR), add a small per-byte-per-epoch charge (amendable). Current model is **pure one-time deposit** for simplicity.
- **USD sensitivity** — At $MORPH = $2 → $10 range:
  - Small contract: $0.20–$1.00
  - Medium: $0.50–$2.50
  - Heavy: $2–$7
  → Keeps Morpheum **developer-friendly** compared to Ethereum L1 (~$10–$150+), similar to NEAR/Polkadot but gasless.

This table can be directly turned into constitutional config objects (passed via Step 9 amendment tx) for live tuning without forking.

If you'd like:
- A version with different base rates (e.g. 1 $MORPH / 150 KB)
- Inclusion of optional per-epoch state rent column
- Pseudocode / JSON schema for the constitutional config object
- Comparison column vs NEAR / Polkadot / CosmWasm real costs (2026 estimates)

Just let me know!