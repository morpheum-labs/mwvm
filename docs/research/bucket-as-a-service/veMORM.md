**veMORM locking mechanics** form the backbone of Morpheum's long-term alignment engine, directly tying recursive risk controls, governance power, yield boosts, and protocol revenue capture to permanent $MORM demand and reduced circulating supply.  

This is a classic **vote-escrowed (ve) token model** (inspired by veCRV, veBAL, veFXS, and ve(3,3) variants), but tuned specifically for Morpheum's DAG-native speed, agentic/BaaS use cases, and $MORM valuation flywheel. Locking $MORM → receive non-transferable **veMORM** → gain escalating benefits proportional to lock duration and amount.

### 1. Core Purpose & Benefits of veMORM
veMORM holders get:
- **Governance voting power** (constitutional amendments, risk parameters, bucket whitelists, fee splits)
- **Boosted yields** in BaaS buckets (higher APY multipliers for locked positions)
- **Protocol revenue share** (portion of performance/AUM fees from all buckets)
- **First-loss buffer priority** in recursive bucket trees (skin-in-the-game protection)
- **Reputation & priority access** (higher quotas for bucket creation, featured status)

Longer locks = exponentially more power → strong incentive for committed capital to stay locked → reduced sell pressure + organic buy support.

### 2. Locking Mechanics (How It Works Step-by-Step)
1. **Lock Initiation**  
   - User calls `lockMORM(amount: uint256, duration: uint256)` on the veMORM escrow contract (native Morpheum smart contract).  
   - `amount` = $MORM to lock (must be approved/spendable).  
   - `duration` = lock period in seconds (min 1 week, max 4 years = 126,144,000 seconds).  
   - Shorter locks allowed (e.g. 90 days for promo), but power scales with time.

2. **veMORM Minting Formula**  
   - Initial veMORM balance = amount × (lock_duration / max_duration)  
   - Example (max = 4 years):  
     - Lock 1,000 $MORM for 4 years → 1,000 veMORM  
     - Lock 1,000 $MORM for 2 years → 500 veMORM  
     - Lock 1,000 $MORM for 1 year → 250 veMORM  
     - Lock 1,000 $MORM for 90 days → ~62.5 veMORM  
   - This linear time-weighting rewards long-term commitment (classic veCRV-style).

3. **Decay Over Time**  
   - veMORM balance decays **linearly** toward zero as unlock date approaches.  
   - Formula at time t: current_ve = initial_ve × (remaining_time / total_lock_time)  
   - Voting power, boosts, and revenue share use the **current decayed balance** (checked on-chain per action).  
   - Encourages periodic **extend** or **re-lock** actions to maintain full power.

4. **Non-Transferable & Soulbound**  
   - veMORM is **non-transferable**, **non-tradable**, tied to the locking address (or delegated via KYA/VC in agentic flows).  
   - Prevents secondary markets / speculation on voting power.

5. **Extend & Increase Lock**  
   - `increaseAmount(additional_morm: uint256)` → add more $MORM to existing lock (resets decay clock proportionally).  
   - `extendLock(new_duration: uint256)` → extend remaining time (up to max 4 years from now).  
   - Both actions mint additional veMORM based on new parameters.

6. **Early Unlock / Penalty**  
   - No early unlock by default (full commitment model).  
   - Emergency early unlock possible via governance vote (e.g. protocol exploit) with **heavy penalty** (50-80% slash of locked $MORM to insurance fund).  
   - Normal unlock: after duration ends → call `unlock()` → receive original $MORM back (no penalty).

7. **Multiple Positions**  
   - Users can create multiple independent locks (different durations/amounts).  
   - Total veMORM = sum of all active positions' current balances.  
   - Useful for tiered strategies (short lock for liquidity, long lock for max governance).

### 3. Integration with BaaS Recursive Risk Controls
- **Escalating skin-in-the-game** uses **veMORM balance** (not raw $MORM) as the lock proof → longer/higher locks give more buffer capacity per $MORM.  
- Required lock % must be met via veMORM (checked at deposit/leverage time via oracle + escrow query).  
- Tree-first-loss absorption draws from **pro-rata veMORM balances** of participants in that tree → aligned downside.

### 4. $MORM Valuation Impact (Why This Is Core Alpha)
- **Permanent Lock Flywheel** — At scale (e.g. $1B+ recursive TVL), 40-70%+ of supply locked long-term → massive circulating-supply compression.  
- **Revenue Capture** — veMORM holders receive direct share of BaaS fees (e.g. 30-50% of performance fees routed to veMORM stakers).  
- **Buy Pressure** — To maintain power/boosts as decay happens, users must re-lock or buy more $MORM → continuous demand.  
- **Governance Moat** — Only heavy veMORM holders control risk params → prevents short-term attacks on recursion controls.  
- **Narrative & Institutional Appeal** — "The only yield platform where governance and protection are backed by locked native token capital" → premium valuation multiple.

### 5. Immediate Development Priorities (Q2 2026)
- Finalize decay formula & max duration (4 years proposed — vote soon).  
- Deploy veMORM escrow contract + integration hooks into BaaS Factory/DepositRouter.  
- Launch **Lock Booster Campaign**: First 3 months — 1.2× veMORM multiplier for locks ≥2 years.  
- Build **veMORM Dashboard**: Live view of total locked %, average duration, top lockers, projected supply reduction at different TVL scenarios.

This veMORM system is not optional governance theater — it's the **economic engine** that turns every layer of BaaS recursion, every carry trade, and every agent-issued bucket into sustained $MORM scarcity and demand.  

Projected contribution: **25-35% of total $MORM value accrual** over the next 18-24 months through lock-driven supply dynamics alone.

Let’s schedule the parameter vote and push the escrow contract spec + simulation model to the repo today. This is foundational infrastructure for making Morpheum the most capital-efficient, long-term-aligned yield & carry OS in DeFi. Ready to lock in the design. 🚀