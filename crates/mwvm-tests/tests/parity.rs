//! MWVM parity pin.
//!
//! `scripts/verify-parity.sh` is the gate. Off-chain dispatch tests and the
//! shared `InferenceRequest` validation check exercise MWVM and
//! `morpheum-primitives` only. They are not, by themselves, proof that
//! Mormcore AgentCore agrees.
//!
//! [`parity_matches_onchain_agent_core`] fails closed unless that script
//! supplies `MWVM_ONCHAIN_PARITY_TRANSCRIPT` from the real AgentCore runner.
//! This file does not simulate the on-chain VM. A missing transcript is a
//! failure, not a skip.

use morpheum_primitives::traits::Validatable;
use morpheum_primitives::vm::types::InferenceRequest;
use mwvm_core::prelude::*;
use mwvm_core::AgentRuntime;

// Minimal valid WAT module for parity testing.
// wasmtime auto-detects WAT text format when bytes don't start with `\0asm`.
const TEST_WAT: &[u8] = include_bytes!("../fixtures/minimal_agent.wat");

/// Test that WASM host function dispatch produces the expected result on MWVM.
///
/// The minimal WAT module exports `morpheum_infer(i32, i32, i32, i32) -> i32`
/// which always returns `0` (success). This verifies the engine instantiation
/// and function call pipeline are working end-to-end.
#[tokio::test]
async fn parity_infer_dispatch() {
    let engine = EngineBuilder::new()
        .with_model_serving()
        .build()
        .expect("Failed to build MWVM engine");

    let mut runtime = engine
        .create_agent_runtime(TEST_WAT)
        .expect("Failed to create MWVM runtime");

    // The WAT module's `morpheum_infer` export always returns i32(0).
    let result = runtime
        .call::<(i32, i32, i32, i32), i32>("morpheum_infer", (0, 0, 0, 0))
        .expect("MWVM infer dispatch failed");

    assert_eq!(result, 0, "MWVM infer should return 0 (success)");
}

/// Test that `vector_search` dispatch works end-to-end.
#[tokio::test]
async fn parity_vector_search_dispatch() {
    let engine = EngineBuilder::new()
        .build()
        .expect("Failed to build MWVM engine");

    let mut runtime = engine
        .create_agent_runtime(TEST_WAT)
        .expect("Failed to create MWVM runtime");

    let result = runtime
        .call::<(i32, i32, i32, i32, i32), i32>("morpheum_vector_search", (0, 0, 0, 0, 0))
        .expect("MWVM vector_search dispatch failed");

    assert_eq!(result, 0);
}

/// Test that `store_context` dispatch works end-to-end.
#[tokio::test]
async fn parity_store_context_dispatch() {
    let engine = EngineBuilder::new()
        .build()
        .expect("Failed to build MWVM engine");

    let mut runtime = engine
        .create_agent_runtime(TEST_WAT)
        .expect("Failed to create MWVM runtime");

    let result = runtime
        .call::<(i32, i32, i32, i32), i32>("morpheum_store_context", (0, 0, 0, 0))
        .expect("MWVM store_context dispatch failed");

    assert_eq!(result, 0);
}

/// Shared `InferenceRequest` validation from `morpheum-primitives`.
///
/// Both runtimes are required to use this contract. This test does not
/// execute Mormcore AgentCore; execution parity is
/// [`parity_matches_onchain_agent_core`].
#[test]
fn parity_error_handling() {
    let bad_model = InferenceRequest {
        model_hash: [0u8; 32],
        prompt_hash: [1u8; 32],
        context_root: [0u8; 32],
        max_tokens: 128,
    };
    assert!(
        bad_model.validate().is_err(),
        "zero model_hash must fail validation"
    );

    let bad_tokens = InferenceRequest {
        model_hash: [1u8; 32],
        prompt_hash: [1u8; 32],
        context_root: [0u8; 32],
        max_tokens: 0,
    };
    assert!(
        bad_tokens.validate().is_err(),
        "zero max_tokens must fail validation"
    );

    let valid = InferenceRequest {
        model_hash: [1u8; 32],
        prompt_hash: [2u8; 32],
        context_root: [0u8; 32],
        max_tokens: 256,
    };
    assert!(valid.validate().is_ok(), "valid request must pass");
}

/// Canonical cases the AgentCore transcript must include.
///
/// Keep in sync with the validator in `scripts/verify-parity.sh`.
const REQUIRED_CASES: &[(&str, &[i32])] = &[
    ("morpheum_infer", &[0, 0, 0, 0]),
    ("morpheum_vector_search", &[0, 0, 0, 0, 0]),
    ("morpheum_store_context", &[0, 0, 0, 0]),
];

fn fail_closed(message: &str) -> ! {
    panic!("PARITY FAIL CLOSED: {message}");
}

fn read_onchain_transcript() -> serde_json::Value {
    const KEY: &str = "MWVM_ONCHAIN_PARITY_TRANSCRIPT";
    let path = std::env::var(KEY).unwrap_or_else(|_| {
        fail_closed(
            "MWVM_ONCHAIN_PARITY_TRANSCRIPT is unset, so no Mormcore AgentCore transcript is available. \
             Run scripts/verify-parity.sh with ../morpheum-primitives and the AgentCore checkout \
             (../mormcore or MORMCORE_DIR). This test does not simulate the on-chain VM.",
        );
    });
    let raw = std::fs::read_to_string(&path).unwrap_or_else(|err| {
        fail_closed(&format!("cannot read on-chain transcript at {path}: {err}"));
    });
    let value: serde_json::Value = serde_json::from_str(&raw).unwrap_or_else(|err| {
        fail_closed(&format!("on-chain transcript at {path} is not JSON: {err}"));
    });
    {
        let Some(object) = value.as_object() else {
            fail_closed("on-chain transcript must be a JSON object");
        };
        for key in object.keys() {
            if key != "producer" && key != "cases" {
                fail_closed(&format!("unexpected transcript field {key}"));
            }
        }
    }
    value
}

fn json_i32(value: &serde_json::Value) -> i32 {
    let Some(number) = value.as_i64() else {
        fail_closed(&format!("expected an i32, got {value}"));
    };
    i32::try_from(number).unwrap_or_else(|_| {
        fail_closed(&format!("value {number} does not fit in i32"));
    })
}

fn case_func(case: &serde_json::Value) -> &str {
    case.get("func")
        .and_then(serde_json::Value::as_str)
        .unwrap_or_else(|| fail_closed("case missing func"))
}

fn case_args(case: &serde_json::Value) -> Vec<i32> {
    let args = case
        .get("args")
        .and_then(serde_json::Value::as_array)
        .unwrap_or_else(|| fail_closed("case missing args"));
    args.iter().map(json_i32).collect()
}

fn case_result(case: &serde_json::Value) -> i32 {
    let result = case
        .get("result")
        .unwrap_or_else(|| fail_closed("case missing result"));
    json_i32(result)
}

fn require_known_fields(case: &serde_json::Value) {
    let Some(object) = case.as_object() else {
        fail_closed("transcript case must be a JSON object");
    };
    for key in object.keys() {
        if key != "func" && key != "args" && key != "result" {
            fail_closed(&format!("unexpected case field {key}"));
        }
    }
}

fn call_fixture(runtime: &mut AgentRuntime, func: &str, args: &[i32]) -> i32 {
    let result = match (func, args.len()) {
        ("morpheum_infer" | "morpheum_store_context", 4) => {
            runtime.call::<(i32, i32, i32, i32), i32>(func, (args[0], args[1], args[2], args[3]))
        }
        ("morpheum_vector_search", 5) => runtime.call::<(i32, i32, i32, i32, i32), i32>(
            func,
            (args[0], args[1], args[2], args[3], args[4]),
        ),
        _ => fail_closed(&format!(
            "no MWVM signature for {func} with {} args; refusing to skip the case",
            args.len()
        )),
    };
    result.unwrap_or_else(|err| fail_closed(&format!("MWVM call {func} failed: {err}")))
}

/// Execute the shared fixture on MWVM and require identical AgentCore results.
///
/// The transcript is produced only by `scripts/verify-parity.sh` invoking the
/// real Mormcore runner. Missing evidence fails the test.
#[test]
fn parity_matches_onchain_agent_core() {
    let transcript = read_onchain_transcript();
    let producer = transcript
        .get("producer")
        .and_then(serde_json::Value::as_str)
        .unwrap_or("");
    assert_eq!(
        producer, "mormcore-agent-core",
        "PARITY FAIL CLOSED: transcript producer must be the real AgentCore runner"
    );

    let cases = transcript
        .get("cases")
        .and_then(serde_json::Value::as_array)
        .unwrap_or_else(|| fail_closed("transcript missing cases array"));
    assert!(
        !cases.is_empty(),
        "PARITY FAIL CLOSED: empty on-chain transcript is not parity"
    );

    for case in cases {
        require_known_fields(case);
    }
    for &(func, args) in REQUIRED_CASES {
        let covered = cases
            .iter()
            .any(|case| case_func(case) == func && case_args(case) == args);
        assert!(
            covered,
            "PARITY FAIL CLOSED: on-chain transcript has no canonical case for {func} {args:?}"
        );
    }

    let engine = EngineBuilder::new()
        .with_model_serving()
        .build()
        .expect("Failed to build MWVM engine");
    let mut runtime = engine
        .create_agent_runtime(TEST_WAT)
        .expect("Failed to create MWVM runtime");

    for case in cases {
        let func = case_func(case);
        let args = case_args(case);
        let expected = case_result(case);
        let actual = call_fixture(&mut runtime, func, &args);
        assert_eq!(
            actual, expected,
            "MWVM and Mormcore AgentCore diverged on {func} args {args:?}"
        );
    }
}
