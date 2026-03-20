//! # `mwvm-svm` — SVM Developer Tooling for Morpheum
//!
//! Provides local development and testing infrastructure for SVM (Solana VM)
//! programs targeting Morpheum. Developers can:
//!
//! - Access Morpheum native modules via system program CPI definitions
//! - Test programs locally with a simulated bank/x402 environment
//! - Validate CPI account metadata before deployment
//!
//! ## Architecture
//!
//! - **`bindings`**: Morpheum system program IDs and CPI instruction
//!   definitions. Programs call these via cross-program invocation (CPI)
//!   to interact with native Morpheum modules (bank mint/burn, x402 settlement).
//! - **`mock_env`**: Simulated Morpheum environment for local testing
//! - **`testing`**: Helpers for program testing workflows
//!
//! ## Usage
//!
//! In an SVM program's test harness:
//!
//! ```toml
//! [dev-dependencies]
//! mwvm-svm = { path = "../../mwvm/crates/mwvm-svm" }
//! ```
//!
//! In tests:
//!
//! ```rust,ignore
//! use mwvm_svm::bindings::{MorpheumCpi, PROGRAM_MORPHEUM_MINT};
//! use mwvm_svm::mock_env::MockSvmApp;
//!
//! let mut app = MockSvmApp::new();
//! app.execute_cpi(MorpheumCpi::MintTo {
//!     recipient: "mormPK1abc...".into(),
//!     asset_index: 1,
//!     amount: 1_000_000,
//! }).unwrap();
//! ```

pub mod bindings;
pub mod mock_env;
pub mod testing;
