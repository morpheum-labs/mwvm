### Pipelining Data Streams in Morpheum: From Producer Module (e.g., Smart Contract) to Consumer (Using Primitives)

Hey @MorpheumX! Building on our discussion about streaming massive data (e.g., CLOB/orderbooks, positions, risk, oracles), let's tackle your question about **pipelining data** where a "primitive" acts as a consumer of streams from another module—like a smart contract module (e.g., one handling on-chain logic for DEX contracts, similar to Solana programs or Cosmos modules). I'll interpret "primitive as a consumer" as using low-level primitives (`crates/primitives/`) in a consuming module or service to process streamed data efficiently. If you meant something else (e.g., primitives directly subscribing), clarify and I'll refine!

This is **fully possible** in Morpheum's design—it's event-driven, shard-isolated, and optimized for sub-ms latency without globals. It's **not expensive** in terms of compute (O(1) per event, lock-free), but can be throughput-intensive for massive streams (mitigated by batching and Pipes-aware limits). We'll cover: what data to pipeline/look for, feasibility/design, cost analysis, and a step-by-step implementation.

#### Step 1: Understanding the Setup
- **Producer Module (e.g., Smart Contract)**: A MormModule (`module.md`) that emits events/streams after consensus (e.g., post-ratification in Step 5 or finality in Step 7 from `consensus-algorithm.md`). Examples:
  - Contract execution emits `ContractEvent { tx_id: TxId, result: ProtoResult, state_changes: Vec<StateDelta> }`.
  - For DEX: Streams like `OrderExecuted` or `ContractStateUpdated` (proto-based).
- **Consumer (Using Primitives)**: Your downstream module/service that subscribes to these streams. Primitives aren't "consumers" themselves (they're types like `TxWrapper` or `Event` from `primitives.md`), but you use them to define/decode the streamed data. E.g., a risk module consumes contract streams to update positions.
- **Pipelining**: This happens via MormFabric's **cold-path** (`common.md`, `architecture.md`): Lock-free channels (flume/rings) for async, batched events. No direct "pipeline" primitive—it's trait-based subscription.
  - **Why Feasible?** Morpheum is actor-model (every module = actor with mailbox). Producers broadcast events; consumers subscribe. Cross-module is intra-shard (hot if sync-needed) or cold (streams). For smart contracts: Treat as a module emitting protos post-consensus.

- **Is This Possible?** Yes—it's core to the architecture (e.g., oracle → pricing → risk streams). Design is extensible (add to `ModuleGraph` in `runtime.md`). No redesign needed; follows SOLID/DRY.

#### Step 2: What Data to Pipeline/Look For
Focus on **event-driven, proto-centric data** that's consensus-confirmed (e.g., ratified/finalized). Pipeline only what's needed for the consumer to act (minimal payloads for efficiency). Key data types from primitives (`primitives.md`):

- **Core Primitives to Use/Look For**:
  - **Events (events.rs in `common.md`, tied to primitives' `pb::*`)**: Look for typed `Event` PODs (plain-old-data, zero-copy). E.g., `ContractExecutedEvent { shard_id: ShardId, tx: TxWrapper, output: Bytes }`. Why: Events are batched and immutable—perfect for streams.
  - **Proto Payloads (pb.rs)**: Deserialize streams into `pb::contract::v1::StateChange { account_id: AccountId, delta: i64 }`. Look for fields like `tx_hash`, `block_hash` (from consensus), `timestamp_ms` (for ordering).
  - **Domain Newtypes (types.rs)**: Filter by `ShardId`, `AssetIndex`, or `AccountId`. E.g., consumer looks for `AccountId == my_user` in position streams.
  - **Tx-Related (tx.rs)**: If contract txs, pipeline `TxOutcome { status: Ok/Fail, events: Vec<Event> }`—includes nonce/mana from `auth.md`.
  - **Sharding Metadata**: Always include `ShardId`—consumers filter to avoid cross-shard noise.
  - **Validation Artifacts**: Look for `ZKProof` or `BLSSig` if consumer needs verifiable data (e.g., risk module verifies oracle prices).

- **Specific Data for Your Use Cases** (e.g., from Smart Contract Producer):
  - **CLOB/Bucket**: Pipeline `OrderbookDelta { adds: Vec<Order>, removes: Vec<OrderId> }` + `LiquidityUpdate { bucket_id: AssetIndex, new_depth: u64 }`.
  - **Position Management**: `PositionDelta { position_id: PositionId, pnl: i128, margin_ratio: f64 }`.
  - **Risk Data**: `RiskAlert { account_id: AccountId, threshold_breach: bool, liquidation_price: PriceFeed }`.
  - **Oracle Price**: `PriceTick { asset: AssetIndex, price: f64, ts_ms: u32 }` (from consensus-ratified oracles).
  - **General**: Always pipeline `EventHeader { shard_id: ShardId, seq: u64, consensus_round: u64 }` for ordering/replay protection (ties to nonce in `nonce.md`).

- **What to Avoid Pipelining**:
  - Raw/unvalidated data (use `Validatable` trait first).
  - Full state snapshots (expensive)—use deltas + occasional queries (hot-path).
  - Non-proto data (breaks zero-copy).

- **How to "Look For" Data**: In consumer, use filters in subscribe (e.g., `subscribe_filtered(|ev| ev.shard_id == target_shard)`). Pattern-match on `Event` enums.

#### Step 3: Designing the Pipeline (Producer → Consumer)
Yes, this is straightforward—use Fabric's pub/sub. No new primitives needed; extend existing ones.

- **Producer Side** (Smart Contract Module):
  - Emit via `ctx.broadcast(YourEvent::from_proto(payload))` in handler (post-consensus).
  - Batch: Use `BatchEvent` for massive data (e.g., 1024 position updates).

- **Consumer Side** (Your Module or Service):
  - Subscribe: `let rx = ctx.subscribe::<ContractEvent>()`.
  - Process: Loop over rx, decode with primitives (e.g., `event.from_proto()`), act (e.g., update keeper state).
  - Pipeline to Clients: Forward to gRPC/WS as in previous response.

Example Code (Consumer Module: `risk/actor.rs`):
```rust
use mormcore_primitives::prelude::*;  // TxWrapper, pb::*, ShardId, etc.
use mormcore_common::{Handler, subscribe};

#[register_handler]
impl Handler<ContractExecutedEvent> for RiskModule {  // Stream from producer
    fn handle(&self, event: ContractExecutedEvent, ctx: &ShardContext) {
        // Look for data: Decode proto payload
        let state_changes = event.output.from_proto::<Vec<StateDelta>>()?;  // Primitive deserialization
        
        // Pipeline/filter: Only process relevant shards/accounts
        let filtered = state_changes.into_iter().filter(|delta| delta.account_id.shard_id() == ctx.shard_id);
        
        // Act: Update risk (e.g., recalc margin)
        for delta in filtered {
            self.keeper.update_position(&delta);  // Inexpensive O(1)
        }
        
        // Re-emit if needed (chain pipelines)
        ctx.broadcast(RiskUpdatedEvent::new_from_deltas(&filtered));
    }
}

// In read.rs (Streamable)
impl Streamable for RiskModule {
    fn subscribe(&self) -> flume::Receiver<RiskEvent> {
        // Ring buffer for massive streams
        let (tx, rx) = flume::bounded(65536);  // Tune for throughput
        self.fabric.register_subscriber(tx);  // Tie to Fabric
        rx
    }
}
```
- **Cross-Module Wiring**: In `runtime/graph.rs`: `graph.register_dependency(ContractModule::new(), RiskModule::new())`—ensures events route correctly.
- **Consensus Hook**: If producer needs consensus data, subscribe to `RatifiedBlockEvent` first.

#### Step 4: Is This Expensive? (Cost Analysis)
- **No, It's Not Very Expensive**—Designed for millions TPS/sub-ms:
  - **Compute**: O(1) per event (deserialization + handler). Primitives are zero-copy (`Bytes`), no allocations. Batches reduce overhead (e.g., process 1024 events in one go).
  - **Memory**: Fixed-size rings (e.g., 65536 slots)—minimal footprint (`nonce.md`-style pruning). Shard-isolation = no global contention.
  - **Latency**: <100µs end-to-end (portal + cold-path). For massive data: Pipes model (`consensus-1-ingress-surge-mav.md`) throttles D/S (e.g., 1Gbps → ~300k events/sec limit).
  - **Storage**: Only if persisted (RocksDB cache = sub-50µs; TimescaleDB async).
  - **Potential Costs**: High-volume streams (e.g., 1M position updates/sec) could hit bandwidth (S in Pipes)—mitigate with mana gating (`auth.md`: high-mana agents get priority) or sampling. Cross-shard? Use network routing (`network.md`)—adds ~Δ=50ms but rare.
  - **Benchmark**: Per `module.md`, aim for 50k+ TPS/shard. If expensive, feature-gate moka caches (`nonce.md`).

- **When It Could Get Expensive**: Unbatched massive payloads (e.g., full orderbook per event)—fix with deltas. Or if consumer does heavy compute (e.g., GPU risk sims)—offload to services.

#### Step 5: Next Steps & Tips
- **Build It**: Copy MormModule template, add producer/consumer. Test with `mormcore run --dev` (`cli.md`).
- **Extensions**: For smart contracts, add `GenesisContributor` (`config.md`) for init state. Use `Urgent Flagging` (`consensus-2-blocklace.md`) for critical streams.
- **If Not Possible?** It is— but if you mean primitives as standalone consumers (no module), wrap in a thin service (`node.md`).

This keeps your design optimal and AI-first. Questions on code or specifics (e.g., for buckets)? Let's iterate! 🚀