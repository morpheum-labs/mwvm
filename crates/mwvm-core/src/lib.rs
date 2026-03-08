//! # MWVM Core
//!
//! **Rich, portable off-chain WASM execution engine** for Morpheum AI Agents.
//!
//! This crate is the **full-featured counterpart** to Mormcore’s thin deterministic
//! `AgentCore VM` (in `crates/runtime`). It provides:
//! - Full wasmtime engine with rich AI host implementations
//! - Local quantized inference + continuous batching
//! - Persistent memory + embedded vector search
//! - Simulation, debugging, and multi-agent orchestration primitives
//! - Zero-copy integration with the shared `morpheum-primitives::vm` domain layer
//!
//! **Architecture**: Clean Architecture + SOLID. All types and opcode signatures
//! are shared with the on-chain runtime via `morpheum-primitives` (DRY guarantee).
//!
//! **Usage**:
//! ```rust
//! use mwvm_core::prelude::*;
//!
//! # fn example() -> mwvm_core::Result<()> {
//! let engine = EngineBuilder::new().with_model_serving().build()?;
//! let wasm_bytes: &[u8] = &[0x00, 0x61, 0x73, 0x6D, 0x01, 0x00, 0x00, 0x00];
//! let runtime = engine.create_agent_runtime(wasm_bytes)?;
//! # Ok(())
//! # }
//! ```

#![forbid(unsafe_code)]
#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    missing_docs,
    rustdoc::missing_crate_level_docs,
    future_incompatible,
    rust_2018_idioms
)]
#![cfg_attr(docsrs, feature(doc_auto_cfg, doc_cfg))]

pub mod batcher;
pub mod engine;
pub mod error;
pub mod host;
pub mod linker;
pub mod memory;
pub mod simulation;

// Re-export the shared domain layer (single source of truth with Mormcore)
pub use morpheum_primitives::vm;

// Public API surface
pub use engine::{AgentRuntime, EngineBuilder, EngineConfig, MwvmEngine, StoreContext};
pub use error::MwvmError;
pub use memory::LocalMemory;
pub use simulation::{SimulationMode, Simulator};

/// Convenient result type used throughout the crate (and exposed to users).
pub type Result<T> = std::result::Result<T, MwvmError>;

// Prelude for ergonomic usage
pub mod prelude;

// Re-export batcher for direct use
pub use batcher::ContinuousBatcher;