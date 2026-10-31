# Gateway MCP

`mwvm gateway` binds an HTTP server and answers `POST /mcp` with the tool list `infer` and `search_memory`.

## Sub-features

- `gateway-listen` binds the address given by `--bind` and logs that it is listening.
- `gateway-tools-list` returns HTTP 200 and a JSON body whose tool names include `infer` and `search_memory`.

## How to get to it (user POV)

- From the repo root, run `cargo run -p mwvm-cli -- gateway --wasm-path <file.wasm> --bind 127.0.0.1:<port>`.
- Send `POST /mcp` with `content-type: application/json` and body `{"method":"tools/list","params":{}}`.

## Driving it with verify-mwvm

Preconditions:

- `launch` printed `ready` and `doctor` prints `doctor ok` for this run.
- `../morpheum-primitives/Cargo.toml` is present.
- Port `8080` is not used. The helper picks a free `127.0.0.1` port.

- **Start the gateway.** Run `.cursor/skills/verify-mwvm/bin/verify-mwvm drive gateway-mcp`. The helper writes an 8-byte WASM file, chooses a port, and runs `cargo run -p mwvm-cli -- gateway --wasm-path <file> --bind 127.0.0.1:<port>` in a tmux session.
- **Ready.** The session stays up until the transcript contains `MWVM Gateway listening`. A log line alone is not the proof.
- **List tools.** The helper sends the POST above. `http.status` is `200`. `http.body` contains `"name":"infer"` and `"name":"search_memory"`.
- **Stop.** The helper kills that tmux session after the body is saved. `cleanup` kills it again if it is still up.
- **Proof.** `evidence/<run_id>/proof.txt` has `feature=gateway-mcp` and `result=PASS`. `commands.txt` records the port.

## Gotchas

- The flag is `--bind`, not `--port`. `--wasm-path` is required even though this build reads the file and does not instantiate it.
- The default bind is `0.0.0.0:8080`. Do not use it for verification. Two drives would fight over one port, and the lock already forbids two launches sharing `target/`.
- Axum's JSON extractor requires `content-type: application/json`. The quickstart `curl` omits that header.
- `tools/call` answers `{"status":"ok"}` without running inference. A `tools/list` body is the proof for this feature. It does not prove `basic-agent`.
- The helper waits for `MWVM Gateway listening` on stdout, which is where `fmt::layer()` wrote it in the probe. Cargo's notes stay on stderr.
