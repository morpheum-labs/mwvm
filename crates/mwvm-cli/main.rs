//! MWVM CLI — Official command-line interface for Morpheum AI agents.
//!
//! ```bash
//! mwvm run agent.wasm
//! mwvm swarm --count 100 --wasm agent.wasm
//! mwvm gateway --wasm agent.wasm --port 8080
//! mwvm test --all
//! ```

#![forbid(unsafe_code)]

mod commands;

use clap::Parser;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

// =============================================================================
// CLI Definition
// =============================================================================

#[derive(Parser)]
#[command(
    name = "mwvm",
    version = env!("CARGO_PKG_VERSION"),
    about = "MWVM — Portable WASM runtime, SDK, orchestrator & gateway for Morpheum AI agents",
    long_about = None,
    arg_required_else_help = true
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Parser)]
enum Commands {
    /// Run a single agent from a WASM file.
    Run(commands::run::RunCmd),
    /// Launch a multi-agent swarm.
    Swarm(commands::swarm::SwarmCmd),
    /// Start the unified MCP/A2A/DID/x402 gateway.
    Gateway(commands::gateway::GatewayCmd),
    /// Run the MWVM test suite.
    Test(commands::test::TestCmd),
}

// =============================================================================
// Main
// =============================================================================

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("mwvm=info,mwvm_core=info")),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Run(cmd) => cmd.execute().await,
        Commands::Swarm(cmd) => cmd.execute().await,
        Commands::Gateway(cmd) => cmd.execute().await,
        Commands::Test(cmd) => cmd.execute().await,
    }
}
