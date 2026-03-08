//! `mwvm run` — Run a single agent from a compiled WASM file.

use std::path::PathBuf;

use clap::Parser;
use tracing::info;

/// Command-line arguments for `mwvm run`.
#[derive(Parser, Debug)]
pub struct RunCmd {
    /// Path to the compiled WASM agent binary.
    #[arg(required = true, value_name = "WASM_FILE")]
    wasm_path: PathBuf,

    /// Maximum tokens to generate.
    #[arg(long, default_value_t = 512)]
    max_tokens: u32,
}

impl RunCmd {
    /// Execute the command.
    pub async fn execute(self) -> anyhow::Result<()> {
        info!("loading agent from {}", self.wasm_path.display());

        let wasm_bytes = tokio::fs::read(&self.wasm_path).await?;

        let mut agent = mwvm_sdk::Agent::builder()
            .wasm_bytes(wasm_bytes)
            .build()
            .map_err(|e| anyhow::anyhow!("{e}"))?;

        info!("agent loaded — calling _start");

        // Attempt to call the standard WASM entry-point if exported.
        match agent.runtime_mut().call::<(), ()>("_start", ()) {
            Ok(()) => info!("_start returned successfully"),
            Err(e) => info!("_start not found or failed: {e}"),
        }

        Ok(())
    }
}
