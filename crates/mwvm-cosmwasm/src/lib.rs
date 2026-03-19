//! # `mwvm-cosmwasm` — CosmWasm Developer Tooling for Morpheum
//!
//! Provides local development and testing infrastructure for CosmWasm
//! contracts targeting Morpheum. Developers can:
//!
//! - Write contracts using `MorpheumMsg` and `MorpheumQuery` custom types
//! - Test contracts locally with simulated bank/x402 modules
//! - Compile and validate WASM artifacts before deployment
//!
//! ## Architecture
//!
//! - **`bindings`**: `MorpheumMsg` and `MorpheumQuery` type definitions
//!   (identical to `mormcore/crates/modules/wasm/src/types/custom.rs`)
//! - **`mock_env`**: Simulated Morpheum environment for local testing
//! - **`testing`**: Helpers for contract testing workflows
//!
//! ## Usage
//!
//! In a CosmWasm contract's dev-dependencies:
//!
//! ```toml
//! [dev-dependencies]
//! mwvm-cosmwasm = { path = "../../mwvm/crates/mwvm-cosmwasm" }
//! ```
//!
//! In tests:
//!
//! ```rust,ignore
//! use mwvm_cosmwasm::bindings::MorpheumMsg;
//! use mwvm_cosmwasm::mock_env::MockMorpheumApp;
//!
//! let app = MockMorpheumApp::new();
//! // ... upload, instantiate, execute contracts with MorpheumMsg support
//! ```

pub mod bindings;
pub mod mock_env;
pub mod testing;
