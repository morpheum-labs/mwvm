//! `mwvm swarm` — Launch a multi-agent swarm.

use std::path::PathBuf;

use clap::Parser;
use tracing::info;

/// Command-line arguments for `mwvm swarm`.
#[derive(Parser, Debug)]
pub struct SwarmCmd {
    /// Path to the compiled WASM agent binary (template for all agents).
    #[arg(required = true, long, value_name = "WASM_FILE")]
    wasm_path: PathBuf,

    /// Number of agents to spawn.
    #[arg(long, default_value_t = 10)]
    count: usize,
}

impl SwarmCmd {
    /// Execute the command.
    pub async fn execute(self) -> anyhow::Result<()> {
        info!(
            "spawning swarm of {} agents from {}",
            self.count,
            self.wasm_path.display()
        );

        let wasm_bytes = tokio::fs::read(&self.wasm_path).await?;

        let swarm = mwvm_orchestrator::Swarm::builder()
            .agent_count(self.count)
            .base_wasm(wasm_bytes)
            .build()
            .map_err(|e| anyhow::anyhow!("{e}"))?;

        info!(agents = swarm.len(), "swarm started — press Ctrl+C to stop");
        tokio::signal::ctrl_c().await?;
        info!("shutting down");

        Ok(())
    }
}
