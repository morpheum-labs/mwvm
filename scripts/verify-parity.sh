#!/usr/bin/env bash
# =============================================================================
# MWVM Parity Verification Script
#
# Compares MWVM (off-chain) with Mormcore AgentCore (on-chain) on one fixture.
#
# Fail closed:
#   • ../morpheum-primitives must be the real sibling checkout Cargo already
#     path-depends on. There is no override: a different directory would not
#     be what `cargo test` compiles.
#   • The on-chain reference is Mormcore AgentCore source (default
#     ../mormcore, override MORMCORE_DIR) at
#     crates/runtime/src/agent_core_vm.rs or crates/runtime/agent_core_vm.rs.
#   • That checkout must supply a real runner:
#       MORMCORE_PARITY_CMD, or
#       $MORMCORE_DIR/scripts/verify-agent-core-parity.sh
#     The runner is invoked as `<runner> <fixture.wat>` and must print a
#     transcript to stdout. This script does not synthesize that transcript
#     and does not embed a stand-in VM.
#
# Transcript (keep in sync with REQUIRED_CASES in
# crates/mwvm-tests/tests/parity.rs):
#   {
#     "producer": "mormcore-agent-core",
#     "cases": [
#       {"func": "morpheum_infer", "args": [0, 0, 0, 0], "result": 0},
#       {"func": "morpheum_vector_search", "args": [0, 0, 0, 0, 0], "result": 0},
#       {"func": "morpheum_store_context", "args": [0, 0, 0, 0], "result": 0}
#     ]
#   }
# `result` values come from AgentCore. MWVM must match them. A missing
# reference, a missing runner, or a mismatch exits non-zero and does not
# print the pass banner.
# =============================================================================

set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m'

print_step() {
    echo -e "${BLUE}➤${NC} $1"
}

print_success() {
    echo -e "${GREEN}✅ $1${NC}"
}

print_error() {
    echo -e "${RED}❌ $1${NC}" >&2
}

fail_closed() {
    print_error "PARITY GATE FAILED CLOSED"
    while [[ $# -gt 0 ]]; do
        print_error "$1"
        shift
    done
    print_error "Parity was not established. This is not a pass."
    exit 1
}

# `pwd` of `/` is `/`, and appending `/child` would print `//child`.
sibling_path() {
    local base child
    base="$(cd "$ROOT/.." && pwd)"
    child="$1"
    if [[ "$base" == "/" || "$base" == "//" ]]; then
        printf '/%s\n' "$child"
    else
        printf '%s/%s\n' "${base%/}" "$child"
    fi
}

PRIMITIVES_DIR="$(sibling_path morpheum-primitives)"
if [[ -z "${MORMCORE_DIR:-}" ]]; then
    MORMCORE_DIR="$(sibling_path mormcore)"
fi

FIXTURE="$ROOT/crates/mwvm-tests/fixtures/minimal_agent.wat"
AGENT_CORE_CANDIDATES=(
    "$MORMCORE_DIR/crates/runtime/src/agent_core_vm.rs"
    "$MORMCORE_DIR/crates/runtime/agent_core_vm.rs"
)

echo -e "${BLUE}═══════════════════════════════════════════════════════════════════════════════${NC}"
echo -e "${BLUE}🚀 MWVM PARITY VERIFICATION${NC}"
echo -e "${BLUE}   (MWVM off-chain vs Mormcore on-chain AgentCore VM)${NC}"
echo -e "${BLUE}═══════════════════════════════════════════════════════════════════════════════${NC}"
echo

print_step "Checking morpheum-primitives sibling..."
if [[ ! -f "$PRIMITIVES_DIR/Cargo.toml" ]]; then
    fail_closed \
        "Missing sibling morpheum-primitives." \
        "Expected a Cargo package at: $PRIMITIVES_DIR" \
        "The workspace path dependency is ../morpheum-primitives (relative to the mwvm checkout)." \
        "Place that checkout next to this repo and re-run scripts/verify-parity.sh." \
        "MWVM-only execution is not on-chain parity, and this script will not invent primitives."
fi
print_success "Found morpheum-primitives at $PRIMITIVES_DIR"

print_step "Checking Mormcore AgentCore source..."
AGENT_CORE_SRC=""
for candidate in "${AGENT_CORE_CANDIDATES[@]}"; do
    if [[ -f "$candidate" ]]; then
        AGENT_CORE_SRC="$candidate"
        break
    fi
done
if [[ -z "$AGENT_CORE_SRC" ]]; then
    fail_closed \
        "Missing on-chain reference: Mormcore AgentCore VM." \
        "Looked for:" \
        "  ${AGENT_CORE_CANDIDATES[0]}" \
        "  ${AGENT_CORE_CANDIDATES[1]}" \
        "Default checkout is ../mormcore. Override with MORMCORE_DIR when the clone lives elsewhere." \
        "Refusing to report PASS from MWVM-only tests. This script does not embed a stand-in AgentCore."
fi
print_success "Found AgentCore source at $AGENT_CORE_SRC"

if [[ ! -f "$FIXTURE" ]]; then
    fail_closed "In-repo parity fixture is missing: $FIXTURE"
fi

RUNNER_DESC=""
if [[ -n "${MORMCORE_PARITY_CMD:-}" ]]; then
    RUNNER_DESC="MORMCORE_PARITY_CMD"
elif [[ -f "$MORMCORE_DIR/scripts/verify-agent-core-parity.sh" ]]; then
    RUNNER_DESC="$MORMCORE_DIR/scripts/verify-agent-core-parity.sh"
else
    fail_closed \
        "On-chain AgentCore source is present, but no parity runner was found." \
        "Source: $AGENT_CORE_SRC" \
        "Expected one of:" \
        "  MORMCORE_PARITY_CMD    (command; the fixture path is appended as the last argument)" \
        "  $MORMCORE_DIR/scripts/verify-agent-core-parity.sh" \
        "The runner must execute the real AgentCore VM on the fixture and print a JSON transcript" \
        "to stdout: {\"producer\":\"mormcore-agent-core\",\"cases\":[{\"func\":\"morpheum_infer\",\"args\":[0,0,0,0],\"result\":<i32>}, ...]}." \
        "Canonical cases also include morpheum_vector_search [0,0,0,0,0] and morpheum_store_context [0,0,0,0]." \
        "This script will not synthesize on-chain results."
fi

print_step "Running AgentCore parity runner ($RUNNER_DESC)..."
transcript="$(mktemp)"
errlog="$(mktemp)"
cleanup() {
    rm -f "$transcript" "$errlog"
}
trap cleanup EXIT

set +e
if [[ -n "${MORMCORE_PARITY_CMD:-}" ]]; then
    # Intentional word-split: the variable is a command plus optional arguments.
    # shellcheck disable=SC2086
    $MORMCORE_PARITY_CMD "$FIXTURE" >"$transcript" 2>"$errlog"
else
    bash "$MORMCORE_DIR/scripts/verify-agent-core-parity.sh" "$FIXTURE" >"$transcript" 2>"$errlog"
fi
runner_status=$?
set -e
if [[ "$runner_status" -ne 0 ]]; then
    print_error "AgentCore runner exited $runner_status."
    if [[ -s "$errlog" ]]; then
        print_error "Runner stderr (first 40 lines):"
        head -n 40 "$errlog" >&2 || true
    fi
    fail_closed \
        "The on-chain parity runner failed." \
        "Refusing to compare MWVM against a missing or failed AgentCore execution."
fi

if ! command -v python3 >/dev/null 2>&1; then
    fail_closed \
        "python3 is required to validate the AgentCore transcript before MWVM is compared." \
        "Install python3, or run this gate in an environment that has it. Not reporting PASS."
fi

print_step "Validating AgentCore transcript..."
set +e
python3 - "$transcript" <<'PY'
import json
import sys

path = sys.argv[1]
with open(path, encoding="utf-8") as handle:
    data = json.load(handle)

if not isinstance(data, dict):
    sys.stderr.write("transcript must be a JSON object\n")
    sys.exit(1)
extra = set(data) - {"producer", "cases"}
if extra:
    sys.stderr.write("unexpected transcript fields: %s\n" % ", ".join(sorted(extra)))
    sys.exit(1)
if data.get("producer") != "mormcore-agent-core":
    sys.stderr.write("transcript producer is not mormcore-agent-core\n")
    sys.exit(1)
cases = data.get("cases")
if not isinstance(cases, list) or not cases:
    sys.stderr.write("transcript cases must be a non-empty list\n")
    sys.exit(1)

# Keep in sync with REQUIRED_CASES in crates/mwvm-tests/tests/parity.rs.
required = {
    "morpheum_infer": [0, 0, 0, 0],
    "morpheum_vector_search": [0, 0, 0, 0, 0],
    "morpheum_store_context": [0, 0, 0, 0],
}
seen = set()
for case in cases:
    if not isinstance(case, dict):
        sys.stderr.write("transcript case is not an object\n")
        sys.exit(1)
    case_extra = set(case) - {"func", "args", "result"}
    if case_extra:
        sys.stderr.write(
            "unexpected case fields: %s\n" % ", ".join(sorted(case_extra))
        )
        sys.exit(1)
    func = case.get("func")
    args = case.get("args")
    result = case.get("result")
    if isinstance(result, bool) or not isinstance(result, int):
        sys.stderr.write("case result must be an integer for %r\n" % (func,))
        sys.exit(1)
    if func in required and args == required[func]:
        seen.add(func)
missing = [name for name in required if name not in seen]
if missing:
    sys.stderr.write(
        "transcript missing canonical cases: %s\n" % ", ".join(missing)
    )
    sys.exit(1)
PY
validate_status=$?
set -e
if [[ "$validate_status" -ne 0 ]]; then
    fail_closed \
        "AgentCore transcript is missing, malformed, or not from producer mormcore-agent-core." \
        "See stderr above. Not comparing MWVM against an untrusted transcript."
fi
print_success "AgentCore transcript matches the parity contract"

print_step "Comparing MWVM against the AgentCore transcript..."
set +e
MWVM_ONCHAIN_PARITY_TRANSCRIPT="$transcript" \
    cargo test -p mwvm-tests --test parity --all-features -- --nocapture
cargo_status=$?
set -e
if [[ "$cargo_status" -ne 0 ]]; then
    fail_closed \
        "MWVM did not match the Mormcore AgentCore transcript (cargo exit $cargo_status)." \
        "Observable results diverged, or the parity suite failed to build."
fi

echo
print_success "PARITY VERIFICATION PASSED"
echo -e "${GREEN}MWVM and Mormcore AgentCore produced identical results on the shared fixture.${NC}"
echo
echo -e "${BLUE}═══════════════════════════════════════════════════════════════════════════════${NC}"
echo -e "${GREEN}All parity checks completed successfully.${NC}"
echo -e "${BLUE}═══════════════════════════════════════════════════════════════════════════════${NC}"
echo

exit 0
