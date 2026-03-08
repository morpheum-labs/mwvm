//! Swarm — multi-agent orchestration runtime.
//!
//! Manages a pool of WASM agent runtimes backed by a single shared
//! [`MwvmEngine`] and routes messages between them via the [`MessageBus`].

use std::sync::Arc;

use dashmap::DashMap;
use tokio::sync::Mutex;
use tracing::info;

use mwvm_sdk::MwvmEngine;

use crate::message_bus::{Event, MessageBus};
use crate::{OrchestratorError, Result};

/// A single entry in the swarm — holds the agent runtime for future call dispatch.
#[allow(dead_code)]
struct AgentSlot {
    runtime: Mutex<mwvm_sdk::mwvm_core::AgentRuntime>,
}

// =============================================================================
// SwarmBuilder
// =============================================================================

/// Fluent builder for [`Swarm`].
#[must_use]
#[derive(Default)]
pub struct SwarmBuilder {
    agent_count: usize,
    base_wasm: Option<Vec<u8>>,
}

impl SwarmBuilder {
    /// Create a new builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Number of agents to spawn (required, must be > 0).
    pub const fn agent_count(mut self, count: usize) -> Self {
        self.agent_count = count;
        self
    }

    /// Base WASM bytes used to instantiate every agent (required).
    pub fn base_wasm(mut self, wasm: impl Into<Vec<u8>>) -> Self {
        self.base_wasm = Some(wasm.into());
        self
    }

    /// Build the swarm.
    ///
    /// # Errors
    ///
    /// Returns [`OrchestratorError::Config`] if `base_wasm` is missing or
    /// `agent_count` is zero, or [`OrchestratorError::Sdk`] if engine or
    /// runtime creation fails.
    #[allow(clippy::cast_possible_truncation)]
    pub fn build(self) -> Result<Swarm> {
        let wasm = self
            .base_wasm
            .ok_or_else(|| OrchestratorError::Config("base_wasm is required".into()))?;

        if self.agent_count == 0 {
            return Err(OrchestratorError::Config(
                "agent_count must be > 0".into(),
            ));
        }

        let engine = mwvm_sdk::EngineBuilder::new()
            .with_model_serving()
            .build()
            .map_err(mwvm_sdk::SdkError::Core)?;

        let agents: DashMap<u64, Arc<AgentSlot>> = DashMap::with_capacity(self.agent_count);

        for i in 0..self.agent_count {
            let runtime = engine
                .create_agent_runtime(&wasm)
                .map_err(mwvm_sdk::SdkError::Core)?;
            agents.insert(
                i as u64,
                Arc::new(AgentSlot {
                    runtime: Mutex::new(runtime),
                }),
            );
        }

        info!(count = self.agent_count, "swarm created");

        Ok(Swarm {
            engine,
            agents: Arc::new(agents),
            bus: MessageBus::new(),
        })
    }
}

// =============================================================================
// Swarm
// =============================================================================

/// Multi-agent swarm runtime.
///
/// Cheap to clone (all state is `Arc`-wrapped).
#[derive(Clone)]
pub struct Swarm {
    engine: MwvmEngine,
    agents: Arc<DashMap<u64, Arc<AgentSlot>>>,
    bus: MessageBus,
}

impl Swarm {
    /// Start building a new swarm.
    pub fn builder() -> SwarmBuilder {
        SwarmBuilder::new()
    }

    /// Send a message to a specific agent.
    ///
    /// # Errors
    ///
    /// Returns an error if the publish operation fails.
    pub async fn send_to(&self, agent_id: u64, payload: Vec<u8>) -> Result<()> {
        self.bus
            .publish(
                format!("agent:{agent_id}"),
                Event::AgentMessage {
                    from: 0,
                    to: Some(agent_id),
                    payload,
                },
            )
            .await
    }

    /// Broadcast a message to all agents.
    ///
    /// # Errors
    ///
    /// Returns an error if the publish operation fails.
    pub async fn broadcast(&self, payload: Vec<u8>) -> Result<()> {
        self.bus.broadcast(payload).await
    }

    /// Number of agents in the swarm.
    #[must_use]
    pub fn len(&self) -> usize {
        self.agents.len()
    }

    /// Whether the swarm has no agents.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.agents.is_empty()
    }

    /// The shared engine backing all agents.
    #[must_use]
    pub const fn engine(&self) -> &MwvmEngine {
        &self.engine
    }

    /// The swarm's message bus.
    #[must_use]
    pub const fn bus(&self) -> &MessageBus {
        &self.bus
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn swarm_builder_requires_wasm() {
        let res = Swarm::builder().agent_count(1).build();
        assert!(res.is_err());
    }

    #[test]
    fn swarm_builder_requires_nonzero_agents() {
        let res = Swarm::builder()
            .base_wasm(b"\x00asm\x01\x00\x00\x00")
            .agent_count(0)
            .build();
        assert!(res.is_err());
    }
}
