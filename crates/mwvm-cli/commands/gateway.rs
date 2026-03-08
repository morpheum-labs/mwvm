//! `mwvm gateway` — Start the unified MCP/A2A/DID/x402 gateway.

use std::path::PathBuf;

use clap::Parser;
use tracing::info;

/// Command-line arguments for `mwvm gateway`.
#[derive(Parser, Debug)]
pub struct GatewayCmd {
    /// Path to the compiled WASM agent binary.
    #[arg(required = true, long, value_name = "WASM_FILE")]
    wasm_path: PathBuf,

    /// Bind address.
    #[arg(long, default_value = "0.0.0.0:8080")]
    bind: String,

    /// Enable MCP endpoints.
    #[arg(long, default_value_t = true)]
    mcp: bool,

    /// Enable A2A endpoints.
    #[arg(long, default_value_t = true)]
    a2a: bool,

    /// Enable DID resolver.
    #[arg(long, default_value_t = true)]
    did: bool,

    /// Enable x402 handler.
    #[arg(long, default_value_t = true)]
    x402: bool,
}

impl GatewayCmd {
    /// Execute the command.
    pub async fn execute(self) -> anyhow::Result<()> {
        info!("loading agent from {}", self.wasm_path.display());

        let _wasm_bytes = tokio::fs::read(&self.wasm_path).await?;

        let engine = mwvm_sdk::EngineBuilder::new()
            .with_model_serving()
            .build()
            .map_err(|e| anyhow::anyhow!("{e}"))?;

        // We don't instantiate the WASM here — the gateway will create
        // runtimes on demand using the shared engine.
        let bind_addr: std::net::SocketAddr = self
            .bind
            .parse()
            .map_err(|e| anyhow::anyhow!("invalid bind address '{}': {e}", self.bind))?;

        let gateway = mwvm_gateway::Gateway::builder()
            .bind(bind_addr)
            .engine(engine)
            .enable_mcp(self.mcp)
            .enable_a2a(self.a2a)
            .enable_did(self.did)
            .enable_x402(self.x402)
            .build()
            .map_err(|e| anyhow::anyhow!("{e}"))?;

        info!("MWVM Gateway listening on http://{bind_addr}");
        gateway.serve().await.map_err(|e| anyhow::anyhow!("{e}"))?;

        Ok(())
    }
}
