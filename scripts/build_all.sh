#!/bin/bash
# QRAP Demo - Build All Components

set -e

echo "========================================"
echo "  Building QRAP Demo Components"
echo "========================================"

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(dirname "$SCRIPT_DIR")"

cd "$ROOT_DIR"

# Build core library
echo ""
echo "Building core library..."
echo "------------------------"
cargo build --release --package qrap-core

# Build Python bindings
echo ""
echo "Building Python bindings..."
echo "---------------------------"
if command -v maturin &> /dev/null; then
    cd python
    maturin build --release
    cd "$ROOT_DIR"
else
    echo "⚠ maturin not found. Skipping Python bindings."
    echo "  Install with: pip install maturin"
fi

# Build WASM visualizer
echo ""
echo "Building WASM visualizer..."
echo "---------------------------"
if rustup target list | grep -q "wasm32-unknown-unknown (installed)"; then
    cargo build --release --target wasm32-unknown-unknown --package qrap-wasm-visualizer
else
    echo "⚠ wasm32-unknown-unknown target not installed. Skipping WASM build."
    echo "  Install with: rustup target add wasm32-unknown-unknown"
fi

echo ""
echo "========================================"
echo "  Build Complete!"
echo "========================================"
