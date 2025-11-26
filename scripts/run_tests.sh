#!/bin/bash
# QRAP Demo - Run All Tests

set -e

echo "========================================"
echo "  Running QRAP Demo Tests"
echo "========================================"

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(dirname "$SCRIPT_DIR")"

cd "$ROOT_DIR"

# Run core library tests
echo ""
echo "Running core library tests..."
echo "-----------------------------"
cargo test --package qrap-core

# Run integration tests
echo ""
echo "Running integration tests..."
echo "----------------------------"
cargo test --test attestation_tests
cargo test --test runtime_tests

# Run all tests with verbose output
echo ""
echo "Running all tests (verbose)..."
echo "------------------------------"
cargo test --all -- --nocapture 2>&1 | head -100

echo ""
echo "========================================"
echo "  All Tests Passed!"
echo "========================================"
