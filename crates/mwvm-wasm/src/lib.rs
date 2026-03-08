//! MWVM WASM Bindings — TypeScript / browser interop layer.
//!
//! Compiled by `wasm-pack` into `mwvm_wasm.wasm` + JS glue.
//! All public functions are exported with `#[wasm_bindgen]` and provide
//! a thin, type-safe bridge for JS/TS consumers.
//!
//! **Note**: When targeting `wasm32-unknown-unknown` the full runtime
//! (wasmtime) is **not** available. This binding is intended for use
//! with a running MWVM gateway — it serialises requests to JSON-RPC
//! and sends them over HTTP. For local execution, use the native SDK.

#![forbid(unsafe_code)]
#![warn(clippy::all, clippy::pedantic, clippy::nursery, missing_docs)]
// `#[wasm_bindgen]` functions cannot be `const fn` and `#[must_use]` is
// meaningless for functions exported to JavaScript.
#![allow(
    clippy::must_use_candidate,
    clippy::missing_const_for_fn,
    clippy::missing_errors_doc
)]

use wasm_bindgen::prelude::*;

// =============================================================================
// Initialisation
// =============================================================================

/// Called automatically when the WASM module is loaded.
#[wasm_bindgen(start)]
pub fn init() {
    // Better panic messages in the browser console.
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

// =============================================================================
// Public WASM API — lightweight protocol types
// =============================================================================

/// A JSON-RPC request destined for the MWVM gateway.
#[wasm_bindgen]
pub struct McpToolCall {
    name: String,
    args_json: String,
}

#[wasm_bindgen]
impl McpToolCall {
    /// Create a new tool call.
    #[wasm_bindgen(constructor)]
    pub fn new(name: String, args_json: String) -> Self {
        Self { name, args_json }
    }

    /// Tool name.
    #[wasm_bindgen(getter)]
    pub fn name(&self) -> String {
        self.name.clone()
    }

    /// Serialised JSON arguments.
    #[wasm_bindgen(getter)]
    pub fn args_json(&self) -> String {
        self.args_json.clone()
    }

    /// Encode as a JSON-RPC request body ready to POST to `/mcp`.
    #[wasm_bindgen]
    pub fn to_json_rpc(&self) -> String {
        serde_json::json!({
            "method": "tools/call",
            "params": {
                "name": self.name,
                "arguments": serde_json::from_str::<serde_json::Value>(&self.args_json)
                    .unwrap_or(serde_json::Value::Null)
            }
        })
        .to_string()
    }
}

/// Build a `tools/list` JSON-RPC request body.
#[wasm_bindgen(js_name = toolsListRequest)]
pub fn tools_list_request() -> String {
    serde_json::json!({ "method": "tools/list", "params": {} }).to_string()
}

/// Parse a hex string into raw bytes (convenience for JS callers).
#[wasm_bindgen(js_name = hexToBytes)]
pub fn hex_to_bytes(hex_str: &str) -> Result<Vec<u8>, JsValue> {
    hex::decode(hex_str).map_err(|e| JsValue::from_str(&format!("invalid hex: {e}")))
}

/// Encode raw bytes as a hex string.
#[wasm_bindgen(js_name = bytesToHex)]
pub fn bytes_to_hex(bytes: &[u8]) -> String {
    hex::encode(bytes)
}
