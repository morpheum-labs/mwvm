//! # `mwvm-evm` — EVM Developer Tooling for Morpheum
//!
//! Provides local development and testing infrastructure for Solidity/Vyper
//! contracts targeting Morpheum. Developers can:
//!
//! - Access Morpheum native modules via precompile definitions
//! - Test contracts locally with a simulated bank/x402 environment
//! - Validate precompile calldata encoding before deployment
//!
//! ## Architecture
//!
//! - **`bindings`**: Morpheum precompile address constants and ABI-compatible
//!   call encoding helpers. Contracts call these precompiles to interact
//!   with native Morpheum modules (bank mint/burn, x402 settlement).
//! - **`mock_env`**: Simulated Morpheum environment for local testing
//! - **`testing`**: Helpers for contract testing workflows
//!
//! ## Usage
//!
//! In an EVM contract's test harness:
//!
//! ```toml
//! [dev-dependencies]
//! mwvm-evm = { path = "../../mwvm/crates/mwvm-evm" }
//! ```
//!
//! In tests:
//!
//! ```rust,ignore
//! use mwvm_evm::bindings::{MorpheumPrecompile, PRECOMPILE_MINT};
//! use mwvm_evm::mock_env::MockEvmApp;
//!
//! let mut app = MockEvmApp::new();
//! app.call_precompile(MorpheumPrecompile::MintTo {
//!     recipient: "0xabc...".into(),
//!     asset_index: 1,
//!     amount: 1_000_000,
//! }).unwrap();
//! ```

pub mod bindings;
pub mod mock_env;
pub mod testing;
