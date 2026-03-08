#!/usr/bin/env bash
# =============================================================================
# MWVM Parity Verification Script
#
# Verifies perfect behavioral parity between:
#   • MWVM (off-chain rich runtime)
#   • Mormcore AgentCore VM (on-chain deterministic runtime)
#
# This is the most critical test in the entire MWVM project.
# =============================================================================

set -euo pipefail

# Go to project root
cd "$(dirname "$0")/.." || { echo "❌ Failed to cd to project root"; exit 1; }

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

print_step() {
    echo -e "${BLUE}➤${NC} $1"
}

print_success() {
    echo -e "${GREEN}✅ $1${NC}"
}

print_error() {
    echo -e "${RED}❌ $1${NC}"
}

echo -e "${BLUE}═══════════════════════════════════════════════════════════════════════════════${NC}"
echo -e "${BLUE}🚀 MWVM PARITY VERIFICATION${NC}"
echo -e "${BLUE}   (MWVM off-chain vs Mormcore on-chain AgentCore VM)${NC}"
echo -e "${BLUE}═══════════════════════════════════════════════════════════════════════════════${NC}"
echo

print_step "Running parity test suite..."

# Run the dedicated parity tests with full output for visibility
cargo test -p mwvm-tests --test parity --all-features -- --nocapture

echo
print_success "PARITY VERIFICATION PASSED"
echo -e "${GREEN}MWVM and Mormcore AgentCore VM produce identical results.${NC}"
echo
echo -e "${BLUE}═══════════════════════════════════════════════════════════════════════════════${NC}"
echo -e "${GREEN}All parity checks completed successfully.${NC}"
echo -e "${BLUE}═══════════════════════════════════════════════════════════════════════════════${NC}"
echo

exit 0