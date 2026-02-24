**CLAMM features should be accessed through safe, scoped, and auditable channels — never directly from WASM contracts or agents in raw form.**

Since CLAMM (including ReClamm glide, boosted pools, hooks, virtual balances, tick iteration, etc.) is **built-in native infrastructure** (part of the core Mormcore keepers and DAG consensus pipeline), the proper access methods are deliberately restricted for security in a permissionless environment.

Governance voting (Step 9 constitutional amendments or proposals) is **only one** high-level control path (e.g., enabling ReClamm mode on a pool, changing global parameters like max glide rate).  

Here are the **proper, recommended access methods** for everyday use, ranked from most common to advanced:

### 1. Direct Native Msg Calls (Standard User / Wallet Access)
This is the **primary and most common way** for normal users and EOAs.

- **How it works**:
  - Any address (single-signature wallet or multisig wallet) submits a standard EIP-712 Msg to the Msg Router.
  - Examples:
    - `hyperclamm::swap_exact_in`
    - `hyperclamm::add_liquidity_balanced` / `add_liquidity_unbalanced`
    - `hyperclamm::remove_liquidity_proportional`
    - `hyperclamm::get_pool_info` (read-only query)
    - `hyperclamm::get_position`
  - These flow through the normal 9-step pipeline (Flash path for most simple operations).

- **When to use**: Human users, simple scripts, wallets, dApps.
- **Security**: Full signature verification + slippage protection + invariant bounds + reputation gating (if configured).
- **No VM involvement** — this is pure native protocol access.

### 2. Scoped KYA / DID + VC Delegation (Agentic & Contract Access)
This is the **main safe way** for WASM smart contracts and AI agents to interact with CLAMM features.

- **How it works** (already in v2.4 Host API):
  - The owner (human or governance) issues a **Verifiable Credential (VC)** to the agent/contract’s DID.
  - Example claims in the VC:
    - “This agent may perform swaps on ETH/USDC pool up to $10,000/day”
    - “Max slippage 0.5%, expiry 2026-03-01”
    - “Allowed actions: swap_exact_in, add_liquidity_balanced”
  - The agent/contract signs the transaction with **its own key** + attaches the VC/VP.
  - The Host API (`check_delegation_scope`, `vc_verify`, `vp_present`) automatically validates the scope before allowing the call.
  - If valid → the call is routed to the native CLAMM keeper with the delegated rights.

- **When to use**: AI agents, autonomous contracts, multi-step workflows, agent swarms.
- **Security**: 
  - Agent never holds owner’s master key.
  - Instant revocation (owner marks VC revoked).
  - Fine-grained scoping (amount, time, assets, actions, slippage, frequency).
  - Full immutable delegation log + changelog.
- **This is the recommended path for anything running inside WASM**.

### 3. Multisig Wallet Control (Shared Ownership)
Native multisig wallets (already built-in) can own and control CLAMM positions/pools.

- **How it works**:
  - Create a multisig wallet (native `multisig::deploy_wallet`).
  - The multisig address becomes the owner of pools/positions.
  - Multiple signers approve CLAMM operations via the standard multisig FSM (submit → approve → execute).
  - Execution is dispatched to CLAMM as if the multisig address itself called it.

- **When to use**: DAOs, teams, treasury management, shared liquidity provision.
- **Security**: Threshold enforcement, nonce protection, time-bound recovery, Step-8 guilt certs for abuse.

### 4. Reputation-Based Gating (Optional Layer)
Some CLAMM features can be gated by on-chain reputation score (from the reputation module).

- **How it works**: High-reputation addresses get higher limits, lower fees, or priority in certain operations (configurable via governance).
- **When to use**: To reward good actors or limit new/low-reputation addresses in high-risk pools.

### 5. Governance-Controlled Pools (High-Level)
For protocol-owned or system-level pools:
- Governance proposals can create, configure, or pause pools.
- This is the “beside governance” complement — not the only way.

### Summary Recommendation

| User Type              | Recommended Access Method                  | Why |
|------------------------|--------------------------------------------|-----|
| Normal user / wallet   | Direct native Msg calls                    | Simple, fast, full control |
| AI agent / contract    | KYA / DID + VC Delegation (Host API)       | Scoped, revocable, safe |
| DAO / team             | Native Multisig Wallet                     | Shared control |
| Protocol / system      | Governance voting + proposals              | High-level control |

**Never expose raw CLAMM hooks, virtual balance manipulation, or core keeper functions to the WASM VM Host API** — this would break the security model.

The current v2.4 design already implements the correct balance:
- Full power for native users/multisig.
- Safe, scoped delegation for everything running inside WASM or as autonomous agents.

This is the cleanest, safest, and most performant way to expose CLAMM features in a permissionless system.

Would you like me to:
1. Add a dedicated “CLAMM Access Patterns” section to the v2.4 spec with the above table?
2. Draft example VC claims for common CLAMM use cases (e.g., “swap agent”, “liquidity provider agent”)?
3. Update the Host API with safe wrapper functions like `clamm_swap` and `clamm_add_liquidity` (that internally enforce delegation)?

Just say the word and I’ll deliver the next document instantly.  

This keeps Morpheum secure while making CLAMM fully usable for both humans and agents. Ready when you are!