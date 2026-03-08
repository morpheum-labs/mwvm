//! Rich host function implementations exposed to WASM agents.
//!
//! Each submodule owns one logical host-function group. Registration functions
//! are re-exported here so `linker.rs` stays minimal and readable.

pub mod actor_messaging;
pub mod infer;
pub mod store_context;
pub mod vector_search;
pub mod zkml_tee;

/// Internal prelude used by host submodules (keeps imports concise).
///
/// WASM host functions universally operate on `i32` pointers and lengths
/// (the WASM linear-memory ABI), so cast lints are allowed crate-wide
/// for this module tree.
pub(crate) mod prelude {
    pub use crate::engine::StoreContext;
    pub use crate::linker::HostRegistry;
    pub use crate::{MwvmError, Result};
    pub use morpheum_primitives::traits::Validatable;
    pub use morpheum_primitives::vm::types::*;
    pub use tracing::debug;
    pub use wasmtime::{Caller, Linker};
}
