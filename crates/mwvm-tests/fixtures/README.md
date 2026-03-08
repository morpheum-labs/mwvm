# MWVM Test Fixtures

This folder contains minimal WASM modules used by the test suite.

## Files

- `minimal_agent.wat` — Human-readable source (WebAssembly Text Format)
- `minimal_agent.wasm` — Compiled binary (included in git)
- `generate-fixtures.sh` — Regenerates the `.wasm` file

## How to regenerate

```bash
cd crates/mwvm-tests/fixtures
bash generate-fixtures.sh
```

## Why this minimal module?

It exports the exact host function signatures used in parity tests (`morpheum_infer`, `morpheum_vector_search`, `morpheum_store_context`). This guarantees the tests run against real WASM execution without depending on complex agent code.

All tests in `tests/parity.rs`, `tests/integration.rs`, and `tests/gateway_e2e.rs` use this fixture.

**Do not delete `minimal_agent.wasm`** — it is committed for reproducible CI.