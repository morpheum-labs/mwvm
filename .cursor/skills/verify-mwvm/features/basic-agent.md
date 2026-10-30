# Basic agent

Basic agent is the recommended starting example. It builds an engine with model serving, instantiates an 8-byte WASM module, stores and loads one memory key, runs a 3-dimensional vector search, and logs a missing `_start` export without failing the process.

## Sub-features

- `engine-create` builds an engine with `model_serving=true`.
- `wasm-instantiate` instantiates the embedded minimal module.
- `memory-roundtrip` stores `example:key` and reads `hello from basic-agent` back.
- `vector-search` inserts three vectors and searches with top-k 2.
- `missing-start` logs that `_start` is not exported and still exits 0.

## How to get to it (user POV)

- From the repo root, run `cargo run -p basic-agent`.
- From `examples/basic-agent`, run `cargo run`.

## Driving it with verify-mwvm

Preconditions:

- `launch` printed `ready` and `doctor` prints `doctor ok` for this run.
- `../morpheum-primitives/Cargo.toml` is present.
- No other verification run holds `/tmp/verify-mwvm/lock`.

- **Run the example.** Run `.cursor/skills/verify-mwvm/bin/verify-mwvm drive basic-agent`. The helper's session runs `cargo run -p basic-agent`. Exit code `0`.
- **Engine.** The transcript contains `Engine created` and `model_serving=true`.
- **Instantiate.** The transcript contains `AgentRuntime instantiated successfully`.
- **Memory.** The transcript contains `hello from basic-agent` and `Memory round-trip verified`.
- **Search.** The transcript contains `Vector search completed` and `top_k=2`.
- **Missing export.** The transcript contains `_start not found in minimal module` and `basic-agent example completed successfully`.
- **Proof.** `evidence/<run_id>/proof.txt` has `feature=basic-agent` and `result=PASS`. `cargo-run.exit` is `0`.

## Gotchas

- `README.md` tells you to run `cargo run --example minimal_agent`. That example target does not exist. The package name is `basic-agent`.
- `EngineBuilder` with model serving calls `tokio::spawn` inside `ContinuousBatcher::new`. A `fn main` without a runtime panics with `there is no reactor running`. The example uses `#[tokio::main]` for that reason. A panic before `basic-agent example completed successfully` is a failed drive.
- Search the combined stdout and stderr transcript. Cargo writes `Running` to stderr. In the proof run the example's `INFO` lines were on stdout.
- The stored value is only in process memory. There is no file to re-read. The success line is printed after the example's own `assert_eq!` on the loaded bytes.
- The log field is `value="hello from basic-agent"`, not a `Some(...)` debug print.
- A sibling crate that merely compiles is not `scripts/verify-parity.sh`.
