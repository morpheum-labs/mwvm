/**
 * Type declarations for the wasm-pack generated module.
 *
 * `wasm-pack build` produces `mwvm_wasm.js` + `mwvm_wasm.d.ts` in the output
 * directory. This stub satisfies the TypeScript compiler _before_ the first
 * build and mirrors the public API exported from `lib.rs`.
 *
 * **Note**: `wasm_bindgen` converts Rust `snake_case` to JavaScript `camelCase`
 * by default, so getters and methods use camelCase here.
 */

/** Initialise the WASM module. Must be called once before using any export. */
export default function init(): Promise<void>;

/** A JSON-RPC tool-call request destined for the MWVM gateway. */
export class McpToolCall {
    constructor(name: string, argsJson: string);
    readonly name: string;
    readonly argsJson: string;
    toJsonRpc(): string;
}

/** Build a `tools/list` JSON-RPC request body. */
export function toolsListRequest(): string;

/** Parse a hex string into raw bytes. */
export function hexToBytes(hexStr: string): Uint8Array;

/** Encode raw bytes as a hex string. */
export function bytesToHex(bytes: Uint8Array): string;
