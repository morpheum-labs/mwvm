### Precise Analysis: Comparing Old vs. New "Hyperlane Integration Architecture Overview" Design Docs

This analysis compares the old design doc (internal adaptation, no real Hyperlane network participation) with the new one (native Go ports enabling real participation without a VM). The old design assumes a fully internal simulation (e.g., router gRPC to morphcore handlers for "cross-chain" without external relayers). The new design shifts to hybrid: Internal flows plus ports for real relayer connectivity (e.g., event emission in morphcore, incoming acceptance in router).

The docs are nearly identical in structure/format (same sections, tables, charts). Differences are minimal and targeted—mostly additions for non-VM ports and real network ties. No deletions; ~5-10% new content. Below, I break down exact differences by section, then summarize codebase impacts: "What has not been done/needs to be done" (new features for real integration) and "What was done/needs to be changed" (modifications to existing impl).

#### Differences by Section
- **Introduction**:
    - **Old**: "The integration leverages Hyperlane's core components directly within Morpheum's two node types: morphcore (validator nodes) and router (client interface/load balancer)."
    - **New**: Added "without requiring a traditional virtual machine (VM) in Morpheum's system. This is achieved by porting Hyperlane logic as native Go modules (similar to Hyperlane's Cosmos SDK module and Solana Sealevel/SVM integrations), allowing Morpheum to connect to the real Hyperlane network via permissionless relayers."
        - **Diff Type**: Addition for non-VM emphasis and real network connectivity.

    - **Old**: Assumptions are "Go-based runtime compatibility with non-EVM patterns like Cosmos/Solana integrations".
    - **New**: Expanded to "Go-based runtime compatibility with non-EVM patterns like Cosmos/Solana integrations", with added paragraph on ZK (unchanged from old) but contextualized for non-VM.

- **Morpheum Architecture Recap**:
    - No differences; text and table identical.

- **Hyperlane Components and Morpheum Adaptations**:
    - **Mailbox Subsection**:
        - **Old**: "Ported as Go handlers in the `x/hyperlane` module (router for dispatch submissions/relaying; morphcore for process/verification and atomic delivery)."
        - **New**: Added "running directly in Morpheum's runtime without a VM. Optional ZK: If zkMode enabled, router encrypts dispatch payloads; morphcore processes ZK proofs in handlers before delivery to shielded accounts in x/zk."
            - **Diff Type**: Addition for non-VM and ZK details.

        - **Old Comparison**: "Ties to Morpheum's handlers for atomicity (e.g., process + route in one tx); adapted from Solidity to Go for non-EVM, using Protobuf instead of ABI."
        - **New**: Added "No VM needed—ports like Hyperlane's Cosmos SDK module embed logic in Go handlers."
            - **Diff Type**: Addition referencing Cosmos SDK for non-VM port.

    - **ISMs Subsection**:
        - **Old**: "Ported as keepers in `x/hyperlane` (morphcore handlers for verification, e.g., Verify in MsgProcessMessage)."
        - **New**: Added "No VM—embedded as Go keepers, similar to Cosmos SDK module."
            - **Diff Type**: Addition for non-VM.

        - **New Comparison**: Added "Custom MorphDAG-ISM adapts to Morpheum's DAG without VM, using Go simulations [from custom-ism-design-and-specification.md]; real uses Solidity contracts. Benefits: Faster <50ms tiering in native runtime; differences: Ports enable non-EVM fit, like Solana's Rust ISMs on Sealevel/SVM."
            - **Diff Type**: New sentence referencing Solana non-VM ports.

    - **Warp Routes Subsection**:
        - **Old**: No dedicated subsection (implied in other parts).
        - **New**: Full new subsection with Traditional Role, Integration/Adaptation, Explanation, and Comparison.
            - **Diff Type**: Addition to detail Warp Routes, emphasizing non-VM Go ports.

- **Integration Blueprint**:
    - **Old**: "Hyperlane embeds into Morpheum as a native Go module (e.g., via hyperlane-morpheum repo adaptations), treating messages as DAG events."
    - **New**: Changed to "Hyperlane integrates natively into Morpheum's Go runtime without a VM, using module-based ports (e.g., like Cosmos SDK module [web:10,11,13]) for Mailbox/ISMs, enabling connection to external relayers for real cross-chain."
        - **Diff Type**: Rewrite for non-VM and real relayer focus.

    - **Sharded DAG-BFT Flows**: Added "External relayers connect for true cross-chain" in description.
        - **Diff Type**: Addition for real integration.

    - **Mermaid Chart (Message Flow)**: Added dotted lines for relayers (e.g., "J[External Relayers] -.-> A").
        - **Diff Type**: Visual addition for relayer ties.

    - **Router gRPC Handling**: Unchanged.

- **System Designs for Cross-Chain Messaging and Token Bridging**:
    - **Cross-Chain Messaging**: Added "supports external relayer deliveries" in Process.
        - **Diff Type**: Addition for real integration.

    - **Token Bridging**: Added dotted lines for relayers in Mermaid (e.g., "J[External Relayers] -.-> B").
        - **Diff Type**: Visual addition.

- **Assumptions and Tradeoffs**: No differences.

- **Optimizations for Security, Robustness, Performance**: No differences (including supplements).

- **Conclusion**:
    - **Old**: "This integration optimizes Hyperlane for Morpheum's dual-node setup, embedding critical logic in morphcore for security while leveraging router for performance. Optional ZK enables robust, secure, performant anonymity (sound proofs, <1ms verify, fallback paths) without mandatory use. Next: Prototype in repo, test bounds."
    - **New**: Added "without a traditional VM, embedding critical logic in morphcore for security while leveraging router for performance and enabling real network connectivity via native Go ports."
        - **Diff Type**: Addition for non-VM and real connectivity.

#### Summary of Codebase Impacts (Exact Diff for Implementation Changes)
Assuming the codebase is fully implemented per the old doc (internal adaptation without real Hyperlane ties), here's the exact diff for transitioning to the new design (real integration via non-VM Go ports). This focuses on what's missing (not done) and what's existing but needs change.

- **What Has Not Been Done and Needs to Be Done** (New Features for Real Integration):
    - **Native Go Ports for Relayer Compatibility**: Implement Protobuf event emission in morphcore handlers (e.g., during dispatch/process) using Hyperlane-standard formats (version, nonce, domains, etc.) for external relayers to pick up [not in old; add to `x/hyperlane` module, ~100 LOC].
    - **Router Acceptance of Relayer Deliveries**: Add gRPC endpoints in router to receive incoming messages/metadata from external relayers, forward to morphcore for verification [not in old; ~50 LOC in router.go, with auth checks].
    - **Warp Routes Subsection Impl**: Add dedicated handlers for token types (Collateral/Synthetic) in `x/hyperlane`, with event emission for relayers [not in old; ~150 LOC, tying to bank minting].
    - **ZK Ties to Real Network**: Extend ZK paths to include relayer-compatible proof metadata emission (e.g., nullifiers in events) [partial in old; add ~50 LOC in x/zk].
    - **Testing for Real Relayers**: Add integration tests simulating external relayers (e.g., using Hyperlane testnets) [not in old; ~200 LOC in tests].

- **What Was Done but Needs to Be Changed** (Modifications to Existing Impl):
    - **Event Emission in Morphcore**: Modify existing broadcast (e.g., in HandleMsgReceive) to include Hyperlane-compatible events (add fields like relayerId) [old has internal broadcast; change ~20 LOC to emit for relayers].
    - **Router Relaying**: Update relay.go to accept/format incoming from external relayers (add relayerSignature validation) [old is internal batching; change ~30 LOC for real deliveries].
    - **ISM Keepers**: Add relayer metadata processing (e.g., validatorSignatures from external) in Verify funcs [old is internal; change ~40 LOC per ISM].
    - **Charts/Code Sketches**: Update Mermaids to include relayer arrows/labels (e.g., "External Relayers Transport"); add relayerId in code [old charts internal; change ~10 lines per chart].
    - **ZK Handling**: Link zkMode/proof to relayer metadata (e.g., encrypt for external transport) [old is internal; change ~20 LOC in router/morphcore].

Total Estimated Effort: ~600 LOC additions/changes; 2-4 weeks for a small team, focusing on relayer tests for true integration. This upgrades from adaptation to real participation without VM.