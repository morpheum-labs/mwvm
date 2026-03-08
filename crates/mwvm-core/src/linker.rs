//! Host function registration system.
//!
//! [`HostRegistry`] is the central shared-state container injected into every
//! WASM store context. [`register_all_hosts`] is the single public entry-point
//! called once during [`crate::engine::EngineBuilder::build`] to bind all rich
//! Rust host implementations to the wasmtime `Linker`.

use std::sync::Arc;

use tracing::debug;
use wasmtime::Linker;

use morpheum_primitives::vm::opcodes::HOST_NAMESPACE;

use crate::batcher::ContinuousBatcher;
use crate::engine::StoreContext;
use crate::memory::LocalMemory;
use crate::Result;

/// Shared services available to every WASM host function.
///
/// Cheap to clone — all fields are `Arc`-wrapped.
#[derive(Clone)]
pub struct HostRegistry {
    /// Persistent local memory + vector index.
    pub memory: Arc<LocalMemory>,
    /// Optional continuous batching engine.
    pub batcher: Option<Arc<ContinuousBatcher>>,
}

impl HostRegistry {
    /// Create a new registry (called once during engine build).
    #[must_use]
    pub const fn new(memory: Arc<LocalMemory>, batcher: Option<Arc<ContinuousBatcher>>) -> Self {
        Self { memory, batcher }
    }
}

/// Registers **all** MWVM host functions into the wasmtime [`Linker`].
///
/// Called exactly once during [`crate::engine::EngineBuilder::build`].
/// Adding a new host function requires one additional call here.
///
/// # Errors
///
/// Returns an error if any host function cannot be registered with wasmtime.
pub fn register_all_hosts(
    linker: &mut Linker<StoreContext>,
    registry: &HostRegistry,
) -> Result<()> {
    debug!(namespace = HOST_NAMESPACE, "registering MWVM host functions");

    crate::host::infer::register(linker, registry)?;
    crate::host::store_context::register(linker, registry)?;
    crate::host::vector_search::register(linker, registry)?;
    crate::host::zkml_tee::register_zkml(linker, registry)?;
    crate::host::zkml_tee::register_tee(linker, registry)?;
    crate::host::actor_messaging::register(linker, registry)?;

    Ok(())
}
