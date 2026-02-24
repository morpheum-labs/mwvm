### Hello World WASM Smart Contract Example for Morpheum

Below, I'll develop a simple "Hello World" WASM smart contract example tailored to your Morpheum blockchain (using the MWVM execution layer, object-centric model, and Host API from previous designs). This contract will:

- Be written in Rust (standard for WASM contracts on chains like Morpheum).
- Handle a basic "greet" message: On instantiation, it creates a simple object with a greeting string. On execution, it reads the object and emits an event with "Hello, Morpheum!".
- Use Morpheum's Host API calls (e.g., `object_create`, `object_read`, `emit_event`) for state and events — keeping all persistence host-mediated (transient linear memory only).
- Be deployable via the two-phase process (MsgStoreCode + MsgInstantiate) on a hypothetical Morpheum testnet.

This is a **minimal example** to demonstrate the workflow. In production, add error handling, capabilities, and tests.

#### Step 1: Project Setup and Files
Create a new Rust library crate for the contract. Use `cargo new hello-world --lib` in your terminal.

Here's the full set of example files:

1. **Cargo.toml** (Dependencies and build config)
   ```
   [package]
   name = "hello-world"
   version = "0.1.0"
   edition = "2021"

   [lib]
   crate-type = ["cdylib", "rlib"]  # For WASM compilation

   [dependencies]
   serde = { version = "1.0", default-features = false, features = ["alloc"] }  # For serialization
   morpheum-std = { version = "0.1.0" }  # Hypothetical Morpheum SDK crate (provides Host API wrappers)
   ```

   - `morpheum-std`: Assume this is your chain's SDK (like `cosmwasm-std` or `ink!`). It exports Host API functions like `object_create`, `object_read`, `emit_event`.

2. **src/lib.rs** (Main contract logic)
   ```
   #![no_std]  // For WASM: no standard library
   use morpheum_std::{entry_point, ObjectId, EmitEvent};  // Import SDK types

   // Contract state: A simple greeting object
   const GREETING_OBJECT_KEY: &str = "greeting";

   #[entry_point]  // Morpheum SDK macro for instantiation
   pub fn instantiate(init_msg: &str) -> Result<(), String> {
       // Create initial greeting object
       let greeting = format!("Hello from init: {}", init_msg);
       let obj_id: ObjectId = morpheum_std::object_create(&greeting.as_bytes());
       morpheum_std::object_write(GREETING_OBJECT_KEY.as_bytes(), obj_id.to_bytes());  // Store ID in a key-value like way
       Ok(())
   }

   #[entry_point]  // For execute (e.g., MsgExecuteContract)
   pub fn execute(action: &str) -> Result<(), String> {
       if action == "greet" {
           // Read greeting object
           let obj_id_bytes = morpheum_std::object_read(GREETING_OBJECT_KEY.as_bytes());
           let obj_id: ObjectId = ObjectId::from_bytes(&obj_id_bytes)?;
           let greeting_bytes = morpheum_std::object_read(obj_id, 0);  // Version 0 (latest)
           let greeting = String::from_utf8(greeting_bytes).map_err(|_| "Invalid greeting")?;

           // Emit event
           morpheum_std::emit_event("GreetingEvent", &format!("Hello, Morpheum! {}", greeting));
           Ok(())
       } else {
           Err("Unknown action".to_string())
       }
   }

   #[entry_point]  // Optional query (read-only)
   pub fn query() -> Result<String, String> {
       // Similar to execute, but read-only (no writes)
       let obj_id_bytes = morpheum_std::object_read(GREETING_OBJECT_KEY.as_bytes());
       let obj_id: ObjectId = ObjectId::from_bytes(&obj_id_bytes)?;
       let greeting_bytes = morpheum_std::object_read(obj_id, 0);
       let greeting = String::from_utf8(greeting_bytes).map_err(|_| "Invalid greeting")?;
       Ok(format!("Current greeting: {}", greeting))
   }
   ```

   - This uses hypothetical Morpheum SDK functions (based on your Host API: `object_create`, `object_read`, `object_write`, `emit_event`).
   - No persistent state inside WASM — all via host calls.
   - For a real setup, add versioning checks and error enums.

3. **Optional: src/schema.rs** (For ABI/JSON schema generation — helps explorers decode events)
   ```
   // Use morpheum-schema crate to generate JSON schemas for messages/events
   use morpheum_std::schema;

   schema::generate_schema!();
   ```

#### Step 2: Compilation to WASM
Compile the Rust code to optimized WASM bytecode (target: wasm32-unknown-unknown).

1. Install Rust WASM target: `rustup target add wasm32-unknown-unknown`
2. Build: `cargo build --release --target wasm32-unknown-unknown`
   - Output: `target/wasm32-unknown-unknown/release/hello-world.wasm` (~200–400 KB raw)
3. Optimize (critical for deposit cost): Use `wasm-opt` from Binaryen or `rust-optimizer` script.
   - Install: `cargo install wasm-opt`
   - Run: `wasm-opt -Oz -o optimized.wasm target/wasm32-unknown-unknown/release/hello-world.wasm`
   - Reduces size to ~50–150 KB (deposit: ~0.5–1.5 $MORPH)

4. Compress (for deposit calc): Use zstd (as per your cost model).
   - `zstd optimized.wasm -o optimized.wasm.zst`
   - Final size: Even smaller (~40–100 KB) → deposit based on this.

#### Step 3: Deployment to Morpheum Testnet
Assume Morpheum has a CLI tool (`morpheumd`) or SDK (like `cosmjs` for Cosmos) for testnet interactions. Testnet is free (zero deposit, as per your cost.md recommendation).

1. **Setup Wallet/CLI**:
   - Install `morpheumd` (hypothetical CLI).
   - Create testnet wallet: `morpheumd keys add mywallet --testnet`
   - Fund with test $MORPH via faucet (e.g., testnet.morpheum.org/faucet).

2. **MsgStoreCode** (Upload bytecode):
   - CLI command: `morpheumd tx wasm store optimized.wasm.zst --from mywallet --testnet --deposit-auto` (auto-calculates deposit based on size; testnet skips deposit).
   - What happens:
     - Tx enters Step 1 (Ingress + MAV validation: checks WASM validity, no host escapes).
     - If non-conflicting (different shard), uses Flash path for fast inclusion.
     - Post-finality (Step 7): Code stored as immutable object.
     - Response: Code ID (e.g., "code_id: 42") + tx hash.
   - Cost: On testnet = 0; mainnet = ~0.5 $MORPH for 50 KB.

3. **MsgInstantiate** (Create instance):
   - CLI command: `morpheumd tx wasm instantiate 42 '{"init_msg": "World!"}' --from mywallet --label "HelloWorldContract" --testnet`
     - `42`: Code ID from previous step.
     - Init JSON: Passed to `instantiate` entry point.
     - Label: Human-readable name for explorer.
   - What happens:
     - Tx routed to MWVM handler post-consensus.
     - Host calls `instantiate`, creates greeting object.
     - Response: Contract address (e.g., "morph1abc...") + events + tx hash.
   - Cost: On testnet = 0; mainnet = 0.02 $MORPH flat.

4. **Verify & Interact**:
   - Query: `morpheumd query wasm contract morph1abc...`
     - Calls `query` entry point → "Current greeting: Hello from init: World!"
   - Execute: `morpheumd tx wasm execute morph1abc... '{"action": "greet"}' --from mywallet --testnet`
     - Emits "GreetingEvent: Hello, Morpheum! Hello from init: World!"
   - Explorer: Use Morpheum testnet explorer (e.g., explorer.testnet.morpheum.org) to view tx, contract state, events.

#### Step 4: Testing & Debugging
- Local: Use Mormtest (your local framework) — `mormtest simulate-deploy optimized.wasm --init '{"init_msg": "World!"}'`
- On testnet: Watch for finality (~1–3 s via Flash or waves); check explorer for object versions/events.

This example is deployable today on similar chains (adapt `morpheum-std` to `cosmwasm-std` for Cosmos testnet testing). For full Morpheum, implement the SDK first.

If you need: full code zip, variations (e.g., with staking), or testnet CLI pseudocode — let me know!