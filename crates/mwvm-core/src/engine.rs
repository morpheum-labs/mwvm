//! MWVM Engine — the core WASM execution container.
//!
//! [`EngineBuilder`] is the only way to obtain a [`MwvmEngine`].
//! The engine owns the wasmtime `Engine`, a pre-configured `Linker` with all
//! host functions registered, and shared state (memory, batcher) wrapped in `Arc`.

use std::sync::Arc;

use wasmtime::{Engine, Linker, Module, Store};

use crate::batcher::ContinuousBatcher;
use crate::linker::{register_all_hosts, HostRegistry};
use crate::memory::LocalMemory;
use crate::{MwvmError, Result};

/// Immutable configuration snapshot captured at build time.
#[derive(Debug, Clone)]
pub struct EngineConfig {
    /// Enable continuous-batching inference.
    pub model_serving: bool,
    /// Enable TEE/zkML simulation.
    pub tee_simulation: bool,
    /// Max concurrent WASM instances per engine.
    pub max_concurrent_instances: usize,
}

impl Default for EngineConfig {
    fn default() -> Self {
        Self {
            model_serving: true,
            tee_simulation: cfg!(feature = "tee-simulation"),
            max_concurrent_instances: 1024,
        }
    }
}

/// Per-instance store context passed to every WASM host function via wasmtime.
pub struct StoreContext {
    /// Shared memory backend.
    pub memory: Arc<LocalMemory>,
    /// Optional batching engine.
    pub batcher: Option<Arc<ContinuousBatcher>>,
}

/// Fluent builder for [`MwvmEngine`].
#[must_use]
#[derive(Debug, Default)]
pub struct EngineBuilder {
    config: EngineConfig,
}

impl EngineBuilder {
    /// Create a new builder with production defaults.
    pub fn new() -> Self {
        Self::default()
    }

    /// Enable local model-serving (continuous batching).
    pub const fn with_model_serving(mut self) -> Self {
        self.config.model_serving = true;
        self
    }

    /// Enable TEE/zkML simulation mode.
    pub const fn with_tee_simulation(mut self) -> Self {
        self.config.tee_simulation = true;
        self
    }

    /// Set maximum concurrent WASM instances.
    pub const fn with_max_instances(mut self, n: usize) -> Self {
        self.config.max_concurrent_instances = n;
        self
    }

    /// Build the engine. Host functions are registered here (once) so the
    /// linker can be shared immutably across all subsequent instantiations.
    ///
    /// # Errors
    ///
    /// Returns [`MwvmError::Wasm`] if the wasmtime engine or linker
    /// cannot be created.
    pub fn build(self) -> Result<MwvmEngine> {
        let mut wasm_config = wasmtime::Config::new();
        wasm_config.cranelift_opt_level(wasmtime::OptLevel::Speed);

        let engine = Engine::new(&wasm_config).map_err(MwvmError::wasm)?;

        let memory = Arc::new(LocalMemory::new());
        let batcher = if self.config.model_serving {
            Some(Arc::new(ContinuousBatcher::new()))
        } else {
            None
        };

        let registry = HostRegistry::new(memory.clone(), batcher.clone());

        let mut linker = Linker::<StoreContext>::new(&engine);
        register_all_hosts(&mut linker, &registry)?;

        Ok(MwvmEngine {
            inner: Arc::new(EngineInner {
                wasmtime: engine,
                linker,
                memory,
                batcher,
                config: self.config,
            }),
        })
    }
}

/// The core WASM engine. Cheap to clone (`Arc` internally).
#[derive(Clone)]
pub struct MwvmEngine {
    inner: Arc<EngineInner>,
}

struct EngineInner {
    wasmtime: Engine,
    linker: Linker<StoreContext>,
    memory: Arc<LocalMemory>,
    batcher: Option<Arc<ContinuousBatcher>>,
    config: EngineConfig,
}

impl MwvmEngine {
    /// Create a new agent runtime from compiled WASM bytes.
    ///
    /// # Errors
    ///
    /// Returns [`MwvmError::Wasm`] if compilation or instantiation fails.
    pub fn create_agent_runtime(&self, wasm_bytes: &[u8]) -> Result<AgentRuntime> {
        let module = Module::new(&self.inner.wasmtime, wasm_bytes).map_err(MwvmError::wasm)?;

        let mut store = Store::new(
            &self.inner.wasmtime,
            StoreContext {
                memory: self.inner.memory.clone(),
                batcher: self.inner.batcher.clone(),
            },
        );

        let instance = self
            .inner
            .linker
            .instantiate(&mut store, &module)
            .map_err(MwvmError::wasm)?;

        Ok(AgentRuntime {
            instance,
            store,
            engine: self.clone(),
        })
    }

    /// Shared persistent memory (for direct use in tests or swarms).
    #[must_use]
    pub fn memory(&self) -> &LocalMemory {
        &self.inner.memory
    }

    /// Current engine configuration (read-only).
    #[must_use]
    pub fn config(&self) -> &EngineConfig {
        &self.inner.config
    }
}

/// A running WASM agent instance.
pub struct AgentRuntime {
    instance: wasmtime::Instance,
    store: Store<StoreContext>,
    engine: MwvmEngine,
}

impl AgentRuntime {
    /// Call any exported WASM function by name.
    ///
    /// # Errors
    ///
    /// Returns [`MwvmError::Wasm`] if the function cannot be found or
    /// the call traps.
    pub fn call<Params, Results>(
        &mut self,
        func_name: &str,
        args: Params,
    ) -> Result<Results>
    where
        Params: wasmtime::WasmParams,
        Results: wasmtime::WasmResults,
    {
        let func = self
            .instance
            .get_typed_func::<Params, Results>(&mut self.store, func_name)
            .map_err(MwvmError::wasm)?;
        func.call(&mut self.store, args).map_err(MwvmError::wasm)
    }

    /// Access the owning engine.
    #[must_use]
    pub const fn engine(&self) -> &MwvmEngine {
        &self.engine
    }
}
