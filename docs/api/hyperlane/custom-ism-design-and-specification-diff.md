### Precise Analysis: Comparing Old vs. New "Custom ISM Design and Specification" Design Docs

This analysis compares the old design doc (internal adaptation of ISMs as Go keepers in morphcore, without real Hyperlane network ties) with the new one (native Go ports enabling real participation via relayer-compatible metadata/events, without a VM). The old focuses on simulating ISMs internally for Morpheum's atomic verification (e.g., keepers calling sub-keepers sequentially). The new adds real integration: Keepers process relayer metadata and emit proofs/events for external relayers/validators.

Docs are nearly identical in structure (same sections, ISM list, charts, code sketches). Differences are content additions/rewrites (~15% new text) for non-VM/real ties, plus minor expansions (e.g., relayer integration in each ISM). No deletions or reordering. Below: overview, section diff, and codebase impacts (what's missing/needs adding; what's existing/needs changing). Assumptions: Codebase fully implements old (internal keepers without relayers); new requires ~300-500 LOC changes for real integration.

#### Overview of Differences
- **Core Shift**: Old is pure adaptation (internal calls, no external). New is hybrid: Internal + relayer ties (e.g., "accepts relayer-provided signatures" in each ISM, event emission in charts/code).
- **Length/Extensiveness**: New is longer due to relayer details (e.g., "For real Hyperlane, accepts relayer-provided signatures" added to every ISM) and updated chart with relayer arrows.
- **Charts**: New adds relayer lines/labels (e.g., "from External Relayers - Hyperlane Relayers"); old has none.
- **Code Sketches**: New unchanged, but context implies relayer additions (e.g., in Verify).
- **ZK/Security**: Unchanged, but new ties ZK to relayers (e.g., "accepts relayer proofs").
- **Real Integration Focus**: New mentions "For real Hyperlane" 8 times per ISM; old has none.

#### Detailed Section-by-Section Diff
- **Introduction**:
    - Old: "ISMs are integrated as part of the `x/hyperlane` module in morphcore nodes, running in module handlers (e.g., during message processing) without altering the DAG-BFT consensus pipeline."
    - New: Added "For real Hyperlane integration, keepers emit verification metadata in standard formats for external relayers."
        - **Diff Type**: Addition (1 sentence) for real relayer ties.

    - Old/New: Optimizations, Solana/multi-sig, assumptions identical, but new contextualizes for real network.

    - Chart: New adds relayer arrows/labels (e.g., "A["Incoming Message via Router (from External Relayers - Hyperlane Relayers)"]"); old has "A["Incoming Message via Router"]".
        - **Diff Type**: Visual additions (3 lines) for relayers.

- **Overview of Hyperlane ISMs in Morpheum**:
    - Old: "ISMs are ported to Go as keepers in `x/hyperlane` (morphcore handlers for verification). They run as pre-checks in tx processing (e.g., HandleMsgReceive verifies before minting/routing), without consensus involvement."
    - New: Added "For real Hyperlane, keepers process relayer-delivered metadata and emit results for relayer proofs."
        - **Diff Type**: Addition (1 sentence) for relayers.

- **Detailed ISMs in Morpheum** (Per ISM Subsection):
    - Common Diff: Each ISM's "Specifics" adds "For real Hyperlane, accepts relayer-provided signatures and emits verification proof." (e.g., in Multisig).
        - **Diff Type**: Addition (1 sentence per ISM, 7 total) for relayer ties.

    - Common Diff: Each "Morpheum Adaptation" adds "Relayer integration: Processes incoming metadata from external relayers." (e.g., in Aggregation).
        - **Diff Type**: Addition (half sentence per ISM).

    - MorphDAG-ISM Code: Unchanged.

    - ZK-ISM: "Specifics" adds "For real Hyperlane, accepts relayer proofs; emits verified nullifier."
        - **Diff Type**: Addition for relayers.

- **Composition and Building Guide**:
    - Composition: Added "For real Hyperlane, aggregation processes relayer sub-metadata and emits combined proofs."
        - **Diff Type**: Addition (half sentence).

    - Building in Repo: Added "For real integration: Ensure keepers emit Hyperlane-standard metadata (e.g., via Protobuf schemas matching real ISMs) for relayer compatibility."
        - **Diff Type**: Addition (1 sentence) for real metadata.

    - Chart: New adds relayer arrows/labels (e.g., "A["Message from External Relayers (Hyperlane Relayers)"]"); old has "A["Message"]".
        - **Diff Type**: Visual additions (4 lines) for relayers.

- **Optimizations for Security, Robustness, Performance**: Identical.

- **Table: ISM Bounds**: Identical.

- **Conclusion**: Added "For real Hyperlane, all ISMs tie to relayer metadata and events."
    - **Diff Type**: Addition (half sentence) for real ties.

#### Codebase Impacts
Assuming implementation is complete per old doc (internal keepers without real relayer ties), here's the exact diff for new design.

- **What Has Not Been Done and Needs to Be Done** (New for Real Integration):
    - Relayer Metadata Processing: Add code in each ISM keeper to handle external relayer-provided signatures/proofs (e.g., in Verify func; ~30 LOC per ISM, 210 total) [not in old; enables real deliveries].
    - Event Emission for Proofs: Add emission of verification proofs/events in keepers (e.g., after successful Verify; ~50 LOC in x/hyperlane, using Protobuf standards) [not in old; for relayer claims].
    - Building Guide Impl: Add Protobuf schema matching for real ISMs in repo (e.g., extend ism_config with relayer fields; ~100 LOC) [not in old; for compatibility].
    - Chart/Code Tests: Add relayer sim in unit tests (e.g., mock external metadata; ~100 LOC) [not in old; verifies real interop].

- **What Was Done but Needs to Be Changed** (Modifications to Existing):
    - ISM Specifics/Adaptation: Update descriptions in code comments/docs to include relayer ties (e.g., "Accepts relayer signatures" in Verify; ~10 LOC per ISM, 70 total) [old internal; change for real proofs].
    - Aggregation Keeper: Modify sequential calls to process relayer sub-metadata (add parsing; ~40 LOC in aggregation.go) [old internal; change for real combos].
    - Charts in Doc: Update Mermaids with relayer lines (e.g., add "from External Relayers"; ~5 lines per chart, 10 total) [old internal; change for real flows].
    - ZK-ISM Code: Link to relayer proofs (e.g., accept relayer-delivered zkProof in Verify; ~20 LOC) [old internal; change for real anonymity].

Total Effort: ~500 LOC changes; 1-3 weeks for a developer, prioritizing relayer metadata for true integration without VM. This upgrades from adaptation to real participation.