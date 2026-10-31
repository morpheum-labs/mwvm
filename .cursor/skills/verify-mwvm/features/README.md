# MWVM verification map

This directory is the source for verifying MWVM's public cargo entrypoints. Read the index, then follow one feature file.

## Baseline preconditions

- `../morpheum-primitives/Cargo.toml` exists. `doctor` fails while it is missing. This skill does not write that crate. A compile shim at that path can load the workspace and is not a parity proof.
- `rustc` is `1.85.0` or newer. The manifest says `1.82`, and current `blake3` resolution needs edition 2024.
- `cargo`, `git`, `tmux`, `curl`, `python3`, and `sha256sum` are on `PATH`.
- Run `launch`, then `doctor`, and require `doctor ok` for this run id.
- One verification run owns the workspace `target/` directory. A second `launch` exits while the lock at `/tmp/verify-mwvm/lock` is held.
- Examples do not use a data directory. Gateway drives bind `127.0.0.1` on a free port.

## Driving conventions

- Start from a finished `launch` unless the feature file says otherwise.
- Treat every command as literal.
- Run drives through `.cursor/skills/verify-mwvm/bin/verify-mwvm drive <feature>`.
- `cleanup` deletes the tmux session and `/tmp` state for the run. It leaves `evidence/<run_id>/` in place.
- A feature that fails to build stays failed. Record the compiler output. Do not mark it passed through a different program.

## Proof and skip reporting

- Capture the command, stdout, stderr, and exit code.
- For `gateway-mcp`, also capture the HTTP status and body.
- Record the feature id, entry command, and git head in `proof.txt`.
- An unreachable entry records `result=FAIL` and the cargo transcript. It is not verified by another feature.
- `cargo test`, including `mwvm-tests`, is not a substitute for these drives.

## Feature entry contract

Each feature file has an H1, one paragraph, then four H2 sections in order: `Sub-features`, `How to get to it (user POV)`, `Driving it with verify-mwvm`, and `Gotchas`.

## Features

- [Basic agent](./basic-agent.md) builds an engine, instantiates a minimal WASM module, round-trips a memory key, and runs a vector search.
- [Swarm example](./swarm-example.md) spawns five agents, broadcasts one payload, and sends one targeted message.
- [CLI run](./cli-run.md) loads a WASM file with `mwvm run` and reports a missing `_start` export.
- [Gateway MCP](./gateway-mcp.md) serves `POST /mcp` `tools/list` on a private port.
