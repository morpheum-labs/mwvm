//! `mwvm test` — Run the MWVM test suite.

use std::process::Command;

use clap::Parser;
use tracing::{error, info};

/// Command-line arguments for `mwvm test`.
#[derive(Parser, Debug)]
pub struct TestCmd {
    /// Run only parity tests.
    #[arg(long)]
    parity: bool,

    /// Run only integration tests.
    #[arg(long)]
    integration: bool,

    /// Run all tests (default).
    #[arg(long)]
    all: bool,

    /// Pass `--nocapture` to show stdout.
    #[arg(long)]
    verbose: bool,
}

impl TestCmd {
    /// Execute the command.
    pub async fn execute(self) -> anyhow::Result<()> {
        info!("starting MWVM test suite");

        let run_all = self.all || (!self.parity && !self.integration);
        let mut all_passed = true;

        let packages: Vec<(&str, &[&str])> = if run_all {
            vec![
                ("mwvm-core", &[][..]),
                ("mwvm-sdk", &[]),
                ("mwvm-orchestrator", &[]),
                ("mwvm-gateway", &[]),
                ("mwvm-tests", &["--test", "parity"]),
                ("mwvm-tests", &["--test", "integration"]),
            ]
        } else {
            let mut pkgs = Vec::new();
            if self.parity {
                pkgs.push(("mwvm-tests", &["--test", "parity"][..]));
            }
            if self.integration {
                pkgs.push(("mwvm-tests", &["--test", "integration"][..]));
            }
            pkgs
        };

        for (pkg, extra) in &packages {
            info!("testing {pkg}");
            let mut cmd = Command::new("cargo");
            cmd.arg("test").arg("-p").arg(pkg);
            cmd.args(*extra);
            if self.verbose {
                cmd.args(["--", "--nocapture"]);
            }
            match cmd.status() {
                Ok(s) if s.success() => info!("{pkg} passed"),
                Ok(_) => {
                    error!("{pkg} failed");
                    all_passed = false;
                }
                Err(e) => {
                    error!("could not run cargo test for {pkg}: {e}");
                    all_passed = false;
                }
            }
        }

        if all_passed {
            info!("all tests passed");
            Ok(())
        } else {
            anyhow::bail!("some tests failed")
        }
    }
}
