---
name: verify-mwvm
description: "Drive MWVM from cargo: basic-agent, swarm-example, mwvm run, and the gateway /mcp endpoint. Use after an engine, SDK, orchestrator, CLI, or gateway change when a review needs proof that a public entrypoint still runs. There is no product UI."
---

# Verify mwvm

MWVM is a Rust library and a short-lived CLI. The user-facing surface is `cargo run -p basic-agent`, `cargo run -p swarm-example`, and `cargo run -p mwvm-cli -- <run|gateway|swarm>`. There is no product UI in this skill. The React demo under `examples/web-browser-example` and the `mwvm-wasm` package are a second surface; this skill does not start Vite. There are no benches. `cargo test` is not the drive.

The workspace has one `target/` directory. A second `launch` refuses to start while `/tmp/verify-mwvm/lock` is held. Examples keep memory inside the process. The gateway binds `127.0.0.1` on a port chosen for that drive, not `0.0.0.0:8080`.

`doctor` fails while `../morpheum-primitives/Cargo.toml` is missing. That crate is a private sibling. This skill does not create it. A reconstructed crate can make `cargo` load and is not a parity proof; `scripts/verify-parity.sh` stays the parity gate. The generator's own proof run used a compile shim at that path because the private crate was not available. Opcode strings, `InferenceRequest` layout, and `validate` rules from that shim are unverified against the real sibling. Re-run `drive` where the real crate is checked out before treating host ABI as proved. The shim is not part of this repo.

`rust-version` in the workspace manifest is `1.82`. Resolving `blake3 = "1.5"` currently selects a crate that needs Cargo edition 2024, so `rustc` must be `1.85.0` or newer. The generator installed stable (`1.99.1`) and set a rustup directory override for this checkout. A machine whose `rustc` is already new enough does not need that override.

## Launch

From the repo root:

```bash
.cursor/skills/verify-mwvm/bin/verify-mwvm launch
```

`launch` checks `rustc` >= `1.85.0`, and checks that `cargo`, `git`, `tmux`, `curl`, `python3`, and `sha256sum` are on `PATH`. It checks that `../morpheum-primitives/Cargo.toml` exists. It then builds the three binaries the drives run:

```bash
cargo build -p basic-agent -p swarm-example -p mwvm-cli
```

Ready is the line `ready run_id=<id> native=ready`. The id is stored in `/tmp/verify-mwvm/current`. The debug binaries are `target/debug/basic-agent`, `target/debug/swarm-example`, and `target/debug/mwvm`. `launch` records their sha256. There is no server to wait for and no migration or seed. Each example starts empty.

If `cargo build` fails, `launch` exits non-zero and does not print `ready`. Run `cleanup`, then start a new `launch`.

## Doctor

```bash
.cursor/skills/verify-mwvm/bin/verify-mwvm doctor
```

`doctor` only reads. It re-checks the toolchain and the sibling manifest, then checks that `HEAD` and the three binaries still match what `launch` recorded. `drive` runs `doctor` before it opens a session. Exit 0 prints `doctor ok run_id=<id> head=<sha> rustc=<version> sibling=<path>`.

These processes are short-lived, so `doctor` does not look for a listening port. A gateway is worth driving only after `doctor ok` and only on the port that drive prints into `commands.txt`.

## Drive

```bash
.cursor/skills/verify-mwvm/bin/verify-mwvm drive basic-agent
```

Feature ids: `basic-agent`, `swarm-example`, `cli-run`, `gateway-mcp`. The map in `features/` is the recipe. One drive uses one tmux session named `verify-mwvm-<run_id>-<feature>`.

Inside that session the helper runs, from the repo root, the `cargo run` line in the feature file. Match the feature's lines in the combined stdout and stderr transcript. On this tree, `basic-agent` and the `mwvm` binary wrote their `INFO` lines to stdout, and Cargo wrote `Finished` and `Running` to stderr. Do not require a line to sit on one stream.

`basic-agent` is the drive to run first. It must exit 0 and its transcript must contain `Memory round-trip verified`, `hello from basic-agent`, `Vector search completed`, and `basic-agent example completed successfully`.

## Evidence

Transcripts land in:

```text
.cursor/skills/verify-mwvm/evidence/<run_id>/
```

| File | Contents |
| --- | --- |
| `launch.log` | toolchain note and `cargo build` output |
| `launch-meta.txt` | run id, git head, rustc, sibling path and hashes |
| `binaries.sha` | sha256 of the three debug binaries at launch |
| `doctor.txt` | the doctor report used before the drive |
| `commands.txt` | the cargo command (and the gateway port, when used) |
| `cargo-run.stdout`, `cargo-run.stderr`, `cargo-run.exit` | the user command |
| `http.headers`, `http.body`, `http.status` | gateway `tools/list` response, when that feature is driven |
| `proof.txt` | feature id, entry command, exit code, sibling hash, `result=PASS` or `result=FAIL` |

```bash
.cursor/skills/verify-mwvm/bin/verify-mwvm evidence
```

prints `proof.txt` and exits 0 only when it contains `result=PASS`.

A passing `basic-agent` proof shows exit `0` and the four log lines above. The value string is the read-back of the key the example stored. The success line is printed only after that read-back matches. The process does not write a data directory.

Capture the command and the resulting transcript, not only the last line. Do not point this skill at `cargo test` or at an internal setter and call that the example.

## Cleanup

```bash
.cursor/skills/verify-mwvm/bin/verify-mwvm cleanup
```

`cleanup` kills tmux sessions named `verify-mwvm-<run_id>-*`, deletes `/tmp/verify-mwvm/runs/<run_id>`, and releases the lock when it belongs to that run. The evidence directory stays. `cleanup` prints `evidence=<absolute path>` after checking that directory is still there.

Run `cleanup` after a failed `launch` or `drive` as well, then start a new `launch`. Cleanup never kills by process name.

## Helpers

The helper is `.cursor/skills/verify-mwvm/bin/verify-mwvm`. It is executable. The commands above are the whole interface. Set `VERIFY_RUN_ID` to point `doctor`, `drive`, `evidence`, or `cleanup` at a run other than `/tmp/verify-mwvm/current`.

When the examples or CLI flags change, update the map with `/maintain-verification-skill`.
