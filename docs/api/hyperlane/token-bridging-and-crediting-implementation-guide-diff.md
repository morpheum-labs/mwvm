### Precise Analysis: Comparing Old vs. New "Token Bridging and Crediting Implementation Guide" Design Docs

This analysis compares the old design doc (internal simulation of token bridging via router/morphcore without real Hyperlane network ties) with the new one (native Go ports enabling real participation via relayer-compatible events/deliveries without a VM). The old focuses on internal steps (e.g., router batching to morphcore minting without external relayers). The new adds real integration: Steps/code/charts include relayer transport, event emission for pickup, and metadata handling.

Docs are identical in structure (same sections, headings, format, including Mermaid chart, components list, steps, code sketches, tables). Differences are content additions/rewrites (~15-20% new text) for non-VM/real ties, plus expansions (e.g., relayer mentions in every step, new bridging out chart). No deletions or reordering. Below: overview, section diff, and codebase impacts (what's missing/needs adding; what's existing/needs changing). Assumptions: Codebase fully implements old (internal simulation without relayers); new requires ~400-600 LOC changes for real integration.

#### Overview of Differences
- **Core Shift**: Old is pure adaptation (internal gRPC/minting, no external). New is hybrid: Internal + relayer ties (e.g., "accepts from external relayers" in steps, event emission in code/charts).
- **Length/Extensiveness**: New is longer due to relayer details (e.g., "Hyperlane Ties" added to every step) and new bridging out chart (requested addition).
- **Charts**: New adds bridging out chart; both charts have relayer arrows/labels (e.g., "External Relayers Transport"); old has one chart without relayers.
- **Code Sketches**: New adds comments for relayer ties (e.g., "emit for relayers if outgoing" in relay.go); old has none.
- **ZK**: New expands ZK paths with relayer ties (e.g., "Encrypt/Relay Proof"); old is internal.
- **Real Integration Focus**: New mentions "For real Hyperlane" in intro, assumptions, components, and every step (~10 times); old has none.

#### Detailed Section-by-Section Diff
- **Introduction**:
    - Old: "Process: Router relays; morphcore verifies/mints/deducts/routes."
    - New: Added "Router relays/accepts from relayers; morphcore verifies/mints/deducts/routes."
        - **Diff Type**: Addition (half sentence) for relayer acceptance.

    - Old/New: Optimizations identical, but new adds "For real Hyperlane, handlers emit events for external relayers and process relayer deliveries." at end.
        - **Diff Type**: Addition (1 sentence) for real ties.

- **Overview of Token Bridging Mechanics**:
    - Old: "Process: Router relays; morphcore verifies/mints/deducts/routes."
    - New: Added "Router relays/accepts from relayers; morphcore verifies/mints/deducts/routes."
        - **Diff Type**: Addition for relayers.

    - Chart: New adds relayer arrows/labels (e.g., "N[External Relayers Transport (Hyperlane Relayers)] -.-> C"); old has none.
        - **Diff Type**: Visual additions (2 lines) for relayers.

- **Components Involved**:
    - Old: "Router: Relays, quotes deductions (ties to Relayers/IGP/Hooks)."
    - New: Added "For real Hyperlane, accepts from external relayers."
        - **Diff Type**: Addition (half sentence) for relayers.

    - Old/New: Other components identical, but new adds "For real Hyperlane, eventbus emits for relayers/validators/watchtowers." at end.
        - **Diff Type**: Addition (half sentence) for real emission.

- **Step-by-Step Implementation Guide** (Per Step):
    - Common Diff: Each step adds "Hyperlane Ties" subsection (e.g., in Step 3: "Accepts incoming batches from external relayers; formats for morphcore.").
        - **Diff Type**: New subsection per step (7 total additions, ~50 words each) for relayer ties.

    - Step 3 Code: New adds comment "// For internal; emit for relayers if outgoing" in relay.go.
        - **Diff Type**: Addition (1 line) for emission.

    - Other Steps/Code: Unchanged, but new context implies relayer handling (e.g., "Processes relayer metadata" in Step 4 ties).

- **Dynamic Registration/Crediting/Minting/Value-Based Deductions/Error Handling/Repo Module Usage/DEX Relevance**: Identical, but new adds "Hyperlane Ties" sentences (e.g., in Dynamic Registration: "Schemas compatible for relayer mints on other chains.").
    - **Diff Type**: Additions (1 sentence each, 4 total) for real ties.

- **Mermaid Chart**: New adds relayer arrows/labels (e.g., "N["External Relayers Transport (Hyperlane Relayers)"] -.-> C"); old has none.
    - **Diff Type**: Visual additions (2 lines).

- **Optimizations for Security, Robustness, Performance**: Identical.

- **Table: Optimization Bounds**: Identical.

- **Conclusion**: Added "For real Hyperlane, all steps support relayer ties."
    - **Diff Type**: Addition (half sentence) for real ties.

#### Codebase Impacts
Assuming implementation is complete per old doc (internal relay/mint without real relayers), here's the exact diff for new design.

- **What Has Not Been Done and Needs to Be Done** (New for Real Integration):
    - Relayer Acceptance in Router: Add gRPC endpoints to receive deliveries from external relayers (e.g., SubmitFromRelayer RPC; ~100 LOC in router.go with validation) [not in old; forwards to morphcore].
    - Event Emission in Morphcore: Add Protobuf event emission in handlers (e.g., after commit in Step 9; ~50 LOC in x/hyperlane, matching Hyperlane formats) [not in old; for relayer pickup].
    - "Hyperlane Ties" Impl: Add relayer metadata parsing in steps (e.g., signatures in Step 4 Verify; ~40 LOC per relevant step, 160 total) [not in old; for real proofs].
    - Bridging Out Flow: Implement out flow (e.g., burn/emit in morphcore; ~150 LOC in handlers for Warp out) [not in old; enables bidirectional].
    - ZK Relayer Ties: Add mixnet support in router for ZK proofs (~50 LOC) [not in old; for real anonymous relay].
    - Tests for Real Relayers: Add integration tests with mock relayers (~100 LOC) [not in old; verifies interop].

- **What Was Done but Needs to Be Changed** (Modifications to Existing):
    - Router Relay: Update relay.go to accept external relayer batches (add relayerSignature check; ~30 LOC) [old internal; change for incoming].
    - Morphcore Verify/Mint: Add relayer metadata processing in handlers (e.g., external signatures in Verify; ~40 LOC) [old internal; change for real proofs].
    - Commit/Broadcast: Modify to include Hyperlane-compatible events (add relayerId; ~20 LOC) [old internal; change for emission].
    - Charts in Doc: Update Mermaids with relayer lines/labels (e.g., "External Relayers Transport"; ~10 lines) [old internal; change for real flows].
    - ZK Paths: Link to relayers (e.g., encrypt for external transport in relay.go; ~20 LOC) [old internal; change for real anonymity].

Total Effort: ~600 LOC changes; 2-4 weeks, prioritizing relayer endpoints for true integration without VM. This upgrades from adaptation to real participation.