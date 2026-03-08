#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")" || exit 1

echo "🔨 Generating MWVM test fixtures..."

# Check for wat2wasm (from WebAssembly Binary Toolkit)
if ! command -v wat2wasm &> /dev/null; then
    echo "❌ wat2wasm not found. Install via:"
    echo "   brew install wabt          # macOS"
    echo "   apt install wabt           # Ubuntu/Debian"
    echo "   or download from https://github.com/WebAssembly/wabt"
    exit 1
fi

wat2wasm minimal_agent.wat -o minimal_agent.wasm --enable-all

echo "✅ Generated minimal_agent.wasm ($(wc -c < minimal_agent.wasm) bytes)"
echo "✅ Fixtures ready for parity and integration tests"