#!/bin/bash
# QRAP Demo Entrypoint Script

set -e

echo "========================================"
echo "  QRAP Demo - Quantum Runtime Attestation Protocol"
echo "========================================"
echo ""

# Check for Python bindings
if python3 -c "import qrap_python" 2>/dev/null; then
    echo "✓ QRAP Python bindings available"
else
    echo "⚠ QRAP Python bindings not installed"
    echo "  Run 'maturin develop' in the python/ directory to install"
fi

echo ""
echo "Starting application..."
echo ""

# Execute the command passed to the container
exec "$@"
