# CLI run

`mwvm run` reads a WASM file, builds an agent, and calls the `_start` export when it exists. For an 8-byte module with no exports, it logs that `_start` is missing and exits 0.

## Sub-features

- `cli-load` reads the WASM path and logs `agent loaded`.
- `cli-missing-start` logs the missing `_start` export and still exits 0.

## How to get to it (user POV)

- From the repo root, run `cargo run -p mwvm-cli -- run <path-to.wasm>`.
- After `cargo build -p mwvm-cli`, run `target/debug/mwvm run <path-to.wasm>`.

## Driving it with verify-mwvm

Preconditions:

- `launch` printed `ready` and `doctor` prints `doctor ok` for this run.
- `../morpheum-primitives/Cargo.toml` is present.

- **Run a minimal module.** Run `.cursor/skills/verify-mwvm/bin/verify-mwvm drive cli-run`. The helper writes an 8-byte WASM module under `/tmp/verify-mwvm/runs/<run_id>/minimal.wasm` and runs `cargo run -p mwvm-cli -- run <that file>`. Exit code `0`.
- **Load.** The transcript contains `loading agent from` and that same path, then `agent loaded — calling _start`.
- **Missing export.** The transcript contains `_start not found or failed`.
- **Proof.** `evidence/<run_id>/proof.txt` has `feature=cli-run` and `result=PASS`. The WASM file is scratch. Cleanup may delete it. The transcript stays.

## Gotchas

- `cargo run -- your_agent.wasm` and `cargo run --release -- examples/minimal_agent.wasm` do not select the `run` subcommand. Clap exits with help or an error.
- `--simulation-mode`, `--mock-authority-denied`, and `--mock-low-gas` are not flags on this binary.
- A missing `_start` is a successful run of this minimal module. A missing file is a failure: non-zero exit, no `agent loaded` line.
- Search the combined transcript. In a direct probe, `mwvm run` wrote `INFO` lines to stdout and Cargo wrote its notes to stderr.
- `mwvm run` does not print the memory round-trip from `basic-agent`. Do not treat this drive as that feature.
