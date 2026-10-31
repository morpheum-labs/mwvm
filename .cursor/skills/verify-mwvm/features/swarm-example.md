# Swarm example

Swarm example spawns five agents from one minimal WASM module, broadcasts one payload on the `broadcast` topic, and sends one payload to agent 0. The process then exits.

## Sub-features

- `swarm-spawn` builds a swarm of five agents.
- `swarm-broadcast` publishes one payload and receives it on `broadcast`.
- `swarm-send-to` publishes one payload on `agent:0` and receives it.

## How to get to it (user POV)

- From the repo root, run `cargo run -p swarm-example`.
- From `examples/swarm-example`, run `cargo run`.

## Driving it with verify-mwvm

Preconditions:

- `launch` printed `ready` and `doctor` prints `doctor ok` for this run.
- `../morpheum-primitives/Cargo.toml` is present.

- **Run the example.** Run `.cursor/skills/verify-mwvm/bin/verify-mwvm drive swarm-example`. The helper's session runs `cargo run -p swarm-example`. Exit code `0`.
- **Spawn.** The transcript contains `agent_count=5` and `Swarm created successfully`.
- **Broadcast.** The transcript contains `Received broadcast event`, `to: None`, and `payload: [72, 101, 108, 108, 111`. Those bytes are the start of `Hello from the swarm example!`.
- **Targeted send.** The transcript contains `Agent 0 received targeted message` and `to: Some(0)`.
- **Finish.** The transcript contains `Swarm example completed successfully`.
- **Proof.** `evidence/<run_id>/proof.txt` has `feature=swarm-example` and `result=PASS`.

## Gotchas

- `examples/swarm-example/README.md` says the swarm stays up until Ctrl+C and starts 20 agents. The source uses `agent_count(5)` and returns from `main`. Assert 5, and assert that the process exits.
- `mwvm swarm --wasm <file> --count <n>` blocks on Ctrl+C and does not broadcast. That command does not prove this feature.
- The payload is `Debug` of `Vec<u8>`, so the transcript shows decimal bytes, not the UTF-8 sentence.
- Search the combined stdout and stderr transcript. Do not require the `INFO` lines to be on stderr.
