# MWVM Test Fixtures

This folder contains minimal WASM modules used by the test suite.

## Files

- `minimal_agent.wat` — Human-readable source (WebAssembly Text Format)
- `minimal_agent.wasm` — Compiled binary (included in git)
- `generate-fixtures.sh` — Regenerates the `.wasm` file

## How to regenerate

```bash
cd crates/mwvm-tests/fixtures
./generate-fixtures.sh