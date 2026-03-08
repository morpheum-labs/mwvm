/**
 * MWVM WASM SDK — High-level TypeScript API
 *
 * This is the **recommended public interface** for browser / Node.js consumers.
 * It provides clean, typed, async methods over the raw WASM bindings generated
 * by `wasm-pack` from the Rust `mwvm-wasm` crate.
 *
 * Usage:
 * ```ts
 * import { initMwvm, McpToolCall, hexToBytes, bytesToHex } from "mwvm-wasm";
 *
 * await initMwvm();
 *
 * const call = new McpToolCall("morpheum_infer", '{"max_tokens": 512}');
 * const body = call.to_json_rpc();
 *
 * const response = await fetch("/mcp", {
 *   method: "POST",
 *   headers: { "Content-Type": "application/json" },
 *   body,
 * });
 * ```
 */

import init, {
    McpToolCall,
    toolsListRequest,
    hexToBytes,
    bytesToHex,
} from "./mwvm_wasm";

// =============================================================================
// Shared primitive types (mirrors morpheum-primitives in TypeScript)
// =============================================================================

/** 32-byte hex-encoded hash (64 characters). */
export type Hash32 = string;

/** Inference request parameters matching the on-chain `InferenceRequest`. */
export interface InferenceRequest {
    /** Blake3 hash of the registered model (32-byte hex string). */
    modelHash: Hash32;
    /** Blake3 hash of the prompt (32-byte hex string). */
    promptHash: Hash32;
    /** Root hash of the agent's context tree (32-byte hex string). */
    contextRoot: Hash32;
    /** Maximum tokens to generate. */
    maxTokens: number;
}

/** zkML proof envelope matching the on-chain `ZkmlProof`. */
export interface ZkmlProof {
    /** Verifier key hash. */
    verifierKeyHash: Hash32;
    /** Public inputs hash. */
    publicInputsHash: Hash32;
    /** Raw proof bytes (hex-encoded). */
    proofData: string;
}

/** TEE attestation envelope matching the on-chain `TeeAttestation`. */
export interface TeeAttestation {
    /** Enclave measurement hash. */
    enclaveHash: Hash32;
    /** Unix timestamp (seconds). */
    timestamp: number;
    /** Raw attestation blob (hex-encoded). */
    attestationData: string;
}

// =============================================================================
// Initialisation
// =============================================================================

/** One-time WASM module initialisation. Call before using any other export. */
export async function initMwvm(): Promise<void> {
    await init();
}

// =============================================================================
// Re-exports from WASM bindings
// =============================================================================

export { McpToolCall, toolsListRequest, hexToBytes, bytesToHex };

// Default export for convenience.
export default initMwvm;
