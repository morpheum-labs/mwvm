# Hyperlane Flowcharts and Diagrams

## Introduction
This document unifies all key flowcharts and diagrams from Morpheum's Hyperlane integration, building on developed documents like the "Hyperlane Integration Architecture Overview," "Custom ISM Design and Specification (MorphDAG-ISM and Variants)," "Token Bridging and Crediting Implementation Guide," and "Performance Optimizations and Security Analysis." It includes corrected Mermaid diagrams with syntax fixes for parseability (e.g., double-quoted labels with specials like percentages or parentheses to avoid token mismatches), ensuring visual clarity for developers and auditors.

Diagrams distinguish between **morphcore** (validator nodes for consensus-critical tasks like verification and atomic commits) and **router** (gateway for external/client tasks like relaying and pre-processing). They cover overall flows, ISM subgraphs, token bridging in/out, and optional ZK paths. Each step labels the corresponding Hyperlane component (e.g., "Hyperlane Relayers") for real integration ties.

Assumptions: Go-based repo; diagrams use Mermaid v10+ syntax; tests assume standard tools (e.g., go test, Foundry if hybrid). For real Hyperlane, all charts include external relayer steps (e.g., event emission for pickup, delivery acceptance).

## Unified Collection of Diagrams
All diagrams use corrected syntax (e.g., |"Label with % ()"| for parseability). Each includes node distinctions (morphcore/router), Hyperlane component labels, and real network ties (e.g., relayer arrows).

### 1. Overall MorphDAG-BFT Flow with Hyperlane Integration
This diagram shows the end-to-end consensus flow, embedding Hyperlane (e.g., message as DAG event). Morphcore handles core steps; router offloads external. Real ties: Relayers watch/ deliver.

Mermaid Chart: Overall MorphDAG-BFT Flow (with Hyperlane Ties)

```mermaid
graph TD
    subgraph "Router Nodes (Client Gateway - Relayers/Hooks/IGP Adaptations - Hyperlane Relayers/Hooks/IGP)"
        A["External Event or Client Submit (e.g., Bridge Request - Hyperlane Relayers)"] --> B["Relay/Batch/Quote IGP (Router - Relayers Adaptation - Hyperlane Relayers; <5ms)"]
        B --> C["gRPC Forward to Morphcore (Router - Load Balancing - Hyperlane Hooks)"]
    end
    subgraph "Morphcore Nodes (Consensus & Execution - Mailbox/ISMs/Warp Routes/Validators Adaptations - Hyperlane Mailbox/ISMs/Warp Routes/Validators)"
        C --> D["Receive & Classify Message (Morphcore - Routing ISM Adaptation - Hyperlane Routing ISM)"]
        D --> E["Verify Authenticity (Morphcore - ISMs Adaptation - Hyperlane ISMs; <50ms)"]
        E --> F["Mint/Credit & Deduct Fees (Morphcore - Warp Routes/IGP Adaptation - Hyperlane Warp Routes/IGP)"]
        F --> G["Atomic Route to Clob/Bucket (Morphcore - Hooks/Interchain Accounts Adaptation - Hyperlane Hooks/Interchain Accounts)"]
        G --> H["Shard DAG Extension & Quorum Check (Morphcore - Validators Adaptation - Hyperlane Validators)"]
        H --> I["Commit & Broadcast (Morphcore - Watchtowers Monitoring Adaptation - Hyperlane Watchtowers)"]
    end
    J["External Relayers Watch Events (Hyperlane Relayers)"] -.-> A
    K["External Relayers Deliver Incoming (Hyperlane Relayers)"] -.-> B
    I -.-> J
    L["Optional ZK: Generate Proof Off-Chain"] -.-> A
    M["Optional ZK: Encrypt/Relay Proof (Router - Mixnets Adaptation - Hyperlane Hooks)"] -.-> B
    N["Optional ZK: Verify Proof <1ms (Morphcore - ZK-ISM Adaptation in x/zk - Custom Hyperlane ZK-ISM)"] -.-> E
    O["Optional ZK: Shielded Mint/Route (Morphcore - Warp Routes Adaptation in x/zk - Hyperlane Warp Routes)"] -.-> F
    E -->|Fail| P["Abort & Router Resubmit (<0.1% Failure - Hyperlane Watchtowers)"]
```

### 2. ISM Verification Subgraph
From ISM spec, showing composition (e.g., Aggregation for combos). Morphcore-exclusive (handlers/keepers). Real ties: Relayers provide metadata.

Mermaid Chart: ISM Verification Subgraph (Morphcore-Only)

```mermaid
graph TD
    subgraph "Morphcore Nodes &#40;x/hyperlane Handlers - ISMs Adaptations - Hyperlane ISMs&#41;"
        A["Incoming gRPC Message from Router &#40;from External Relayers - Hyperlane Relayers&#41;"] --> B["Select ISM via Routing &#40;Morphcore - Routing ISM Adaptation - Hyperlane Routing ISM&#41;"]
        B --> C["Default: Multisig ISM &#40;m-of-n Validator Sigs; Morphcore Keeper - Hyperlane Multisig ISM&#41;"]
        B --> D["Aggregation ISM &#40;m-of-n Combos; Morphcore Handler - Hyperlane Aggregation ISM&#41;"]
        D --> E["Sub: Merkle Root Multisig &#40;Batched; Morphcore Keeper - Hyperlane Merkle Root Multisig ISM&#41;"]
        D --> F["Sub: Message ID Multisig &#40;Single; Morphcore Keeper - Hyperlane Message ID Multisig ISM&#41;"]
        D --> G["Sub: Optimistic ISM &#40;Fraud Window; Morphcore with Eventbus - Hyperlane Optimistic ISM&#41;"]
        D --> H["Sub: MorphDAG-ISM &#40;DAG Sim; Morphcore Handler - Custom Hyperlane ISM&#41;"]
        D --> I["Sub: ZK-ISM &#40;Proof Verify &lt;1ms; Optional in x/zk - Custom Hyperlane ZK-ISM&#41;"]
        C --> J
        G --> J
        H --> J
        I --> J
        J{"Verify? &#40;Morphcore - Quorum/Bounds Check - Hyperlane Validators&#41;"}
        J -->|Yes| K["Proceed to Mint/Route &#40;Morphcore - Atomic STM - Hyperlane Warp Routes&#41;"]
        J -->|No| L["Abort Tx &amp; Emit Error &#40;Morphcore - Resubmit to Router - Hyperlane Watchtowers&#41;"]
    end
    M["External Relayers Provide Metadata &#40;Hyperlane Relayers&#41;"] -.-> A
    K --> N["Emit Proof for Relayers &#40;Morphcore - Mailbox Adaptation - Hyperlane Mailbox&#41;"]
    N --> M
```

### 3. Token Bridging Flow
From token guide, detailed with steps/locations/components. Router for relay; morphcore for verify/mint/route. Real ties: Relayers transport.

Mermaid Chart: Token Bridging In Flow (Detailed with Steps/Locations/Components)

```mermaid
graph TD
    subgraph "Origin Chain (External - Not in Morpheum)"
        A1["Step 1: Lock/Burn Token (User/Origin Mailbox - Hyperlane Mailbox)"] --> A2["Step 2: Dispatch Message (Mailbox Adaptation - Hyperlane Mailbox)"]
    end
    A2 --> B["External Relayers Transport (Hyperlane Relayers)"]
    B --> C["Step 3: Router - Receive from Relayers & Batch (Relayers Adaptation - Hyperlane Relayers; <5ms)"]
    C --> D["Step 4: Router - Quote 105% Deduction & Check Non-Meme (IGP/Hooks Adaptation - Hyperlane IGP/Hooks)"]
    D --> E["Step 5: gRPC Forward to Morphcore (Router - Load Balancing)"]
    subgraph "Morphcore Nodes (Validator - ISMs/Mailbox/Warp Routes/Validators Adaptations - Hyperlane ISMs/Mailbox/Warp Routes/Validators)"
        E --> F["Step 6: Verify Authenticity (ISMs Adaptation - Hyperlane ISMs; <50ms)"]
        F --> G["Step 7: Dynamic Register if New (Warp Routes Adaptation - Hyperlane Warp Routes; O(1))"]
        G --> H["Step 8: Mint uint256 to Temp (Mailbox Process Adaptation - Hyperlane Mailbox)"]
        H --> I["Step 9: Deduct 105% (IGP Adaptation - Hyperlane IGP)"]
        I --> J["Step 10: Atomic Route to Clob/Bucket (Hooks Adaptation - Hyperlane Hooks/Interchain Accounts)"]
        J --> K["Step 11: Trigger DEX Liquidation if Needed (Morphcore - Eventbus - Hyperlane Interchain Accounts)"]
        K --> L["Step 12: Commit & Broadcast Event for Relayers (Validators/Watchtowers Adaptation - Hyperlane Validators/Watchtowers)"]
    end
    F -->|Fail| M["Error: Abort & Resubmit (<0.1%; Morphcore Handler - Hyperlane Watchtowers)"]
    N["Optional ZK: Origin - Generate Proof (~100ms; User-Side)"] -.-> A2
    O["Optional ZK: Router - Encrypt/Relay Proof (Hooks Adaptation - Hyperlane Hooks)"] -.-> C
    P["Optional ZK: Morphcore - Verify Proof <1ms (ZK-ISM - Custom Hyperlane ZK-ISM)"] -.-> F
    Q["Optional ZK: Morphcore - Shielded Mint (Warp Routes Adaptation - Hyperlane Warp Routes)"] -.-> H
    L --> B["External Relayers for Further Flows (Hyperlane Relayers)"]
```

Mermaid Chart: Token Bridging Out Flow (Detailed with Steps/Locations/Components)

```mermaid
graph TD
    subgraph "Router Nodes (Gateway - Relayers Adaptation - Hyperlane Relayers)"
        A["Step 1: Client Submits Outgoing Request"] --> B["Step 2: Quote 105% & Pre-Validate (IGP/Hooks Adaptation - Hyperlane IGP/Hooks; <5ms)"]
        B --> C["Step 3: gRPC to Morphcore (Router - Load Balancing)"]
    end
    subgraph "Morphcore Nodes (Validator - ISMs/Mailbox/Warp Routes/Validators Adaptations - Hyperlane ISMs/Mailbox/Warp Routes/Validators)"
        C --> D["Step 4: Burn/Lock Token (Warp Routes Adaptation - Hyperlane Warp Routes)"]
        D --> E["Step 5: Generate Msg ID/Nonce (Mailbox Adaptation - Hyperlane Mailbox)"]
        E --> F["Step 6: Apply Post-Hooks (Hooks Adaptation - Hyperlane Hooks/Interchain Accounts)"]
        F --> G["Step 7: Emit Event for External Relayers (Mailbox Adaptation - Hyperlane Mailbox)"]
    end
    G --> H["External Relayers Transport to Destination (Hyperlane Relayers)"]
    H --> I["Step 8: Destination - Verify & Mint (Destination ISM/Warp Routes - Hyperlane ISMs/Warp Routes)"]
    J["Optional ZK: Client - Generate Proof (~100ms)"] -.-> A
    K["Optional ZK: Router - Encrypt Payload (Hooks Adaptation - Hyperlane Hooks)"] -.-> B
    L["Optional ZK: Morphcore - Shielded Burn <1ms (ZK-ISM - Custom Hyperlane ZK-ISM)"] -.-> D
    M["Monitor Fraud (Morphcore - Watchtowers Adaptation - Hyperlane Watchtowers)"] -.-> G
    E -->|Fail| N["Error: Abort & Resubmit (<0.1% - Hyperlane Watchtowers)"]
```

### 4. ZK Anonymity Sub-Flow (Optional)
Standalone subgraph for ZK paths. Morphcore for verify/mint; router for relay. Real ties: Relayers handle encrypted proofs.

Mermaid Chart: ZK Anonymity Sub-Flow (Morphcore/Router Distinctions)

```mermaid
graph TD
    subgraph "Router Nodes (Gateway - Relayers Adaptation - Hyperlane Relayers)"
        A["User Submits ZK-Enabled Request (zkMode Flag)"] --> B["Encrypt Payload & Proof (Router - Mixnets/Hooks Adaptation - Hyperlane Hooks; <10ms)"]
        B --> C["Relay to Morphcore (Router - gRPC - Hyperlane Hooks)"]
    end
    subgraph "Morphcore Nodes (Validator - ISMs/ZK-ISM/Warp Routes Adaptations - Hyperlane ISMs/Warp Routes)"
        C --> D["Decrypt & Verify ZK Proof <1ms (Morphcore - ZK-ISM in x/zk Handler - Custom Hyperlane ZK-ISM)"]
        D --> E{Valid?}
        E -->|Yes| F["Mint to Shielded Pool with Nullifier (Morphcore - Warp Routes in x/zk - Hyperlane Warp Routes; Atomic)"]
        F --> G["Anonymous Route to Bucket/Clob (Morphcore - Hooks Adaptation - Hyperlane Hooks/Interchain Accounts)"]
        G --> H["Commit Tx & Emit for Relayers (Morphcore - Validators Adaptation - Hyperlane Validators)"]
        E -->|No| I["Fallback to Non-ZK Path (Morphcore Handler)"]
    end
    I --> H
    J["External Relayers Deliver Encrypted Proofs (Hyperlane Relayers)"] -.-> B
    H --> J
```

## Conclusion
These diagrams and protocols provide clear, parseable visuals with morphcore/router distinctions and Hyperlane component labels, ensuring developers/auditors can verify the integration. For real Hyperlane, all charts include external relayer steps (e.g., emission/pickup, delivery acceptance).