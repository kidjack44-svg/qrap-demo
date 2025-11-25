#!/bin/bash
# QRAP Demo - Launch Jupyter Notebooks

set -e

echo "========================================"
echo "  Launching QRAP Demo Notebooks"
echo "========================================"

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(dirname "$SCRIPT_DIR")"

cd "$ROOT_DIR"

# Check if Python bindings are available
echo ""
echo "Checking dependencies..."
echo "------------------------"

if python3 -c "import qrap_python" 2>/dev/null; then
    echo "✓ QRAP Python bindings available"
else
    echo "⚠ QRAP Python bindings not installed"
    echo ""
    echo "To install Python bindings:"
    echo "  cd python && maturin develop && cd .."
    echo ""
    read -p "Continue without bindings? [y/N] " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        exit 1
    fi
fi

# Check for Jupyter
if ! command -v jupyter &> /dev/null; then
    echo "❌ Jupyter not found"
    echo "  Install with: pip install jupyterlab"
    exit 1
fi

echo "✓ Jupyter available"

# Launch Jupyter
echo ""
echo "Launching Jupyter Lab..."
echo "-------------------------"
cd notebooks
jupyter lab --no-browser
