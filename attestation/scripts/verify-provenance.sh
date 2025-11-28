#!/usr/bin/env bash
set -euo pipefail

# Scaffold script for verifying provenance for qrap-demo builds.
# This script is intentionally simple and contains no secrets.
# In a real setup, this would invoke tools like `cosign verify-attestation`.

ATTESTATION="${1:-attestation/examples/provenance.release-build.v1.json}"
EXPECTED_REPO="${EXPECTED_REPO:-https://github.com/kidjack44-svg/qrap-demo}"

echo "Verifying provenance attestation: $ATTESTATION"
echo ""

if [ ! -f "$ATTESTATION" ]; then
    echo "ERROR: Attestation file not found: $ATTESTATION"
    exit 1
fi

# Basic JSON validation
if ! command -v jq &> /dev/null; then
    echo "WARNING: jq not installed, skipping JSON validation"
else
    if ! jq empty "$ATTESTATION" 2>/dev/null; then
        echo "ERROR: Invalid JSON in attestation file"
        exit 1
    fi

    echo "Checking attestation contents..."
    
    # Check required fields
    SUBJECT_NAME=$(jq -r '.subject.name // empty' "$ATTESTATION")
    SOURCE_REPO=$(jq -r '.build.source_repo // empty' "$ATTESTATION")
    COMMIT=$(jq -r '.build.commit // empty' "$ATTESTATION")
    TIMESTAMP=$(jq -r '.build.timestamp // empty' "$ATTESTATION")

    if [ -z "$SUBJECT_NAME" ]; then
        echo "ERROR: Missing subject.name"
        exit 1
    fi

    if [ -z "$SOURCE_REPO" ]; then
        echo "ERROR: Missing build.source_repo"
        exit 1
    fi

    if [ -z "$COMMIT" ]; then
        echo "ERROR: Missing build.commit"
        exit 1
    fi

    if [ -z "$TIMESTAMP" ]; then
        echo "ERROR: Missing build.timestamp"
        exit 1
    fi

    echo "  Subject: $SUBJECT_NAME"
    echo "  Source:  $SOURCE_REPO"
    echo "  Commit:  $COMMIT"
    echo "  Time:    $TIMESTAMP"
    echo ""

    # Check source repo matches expected
    if [ "$SOURCE_REPO" != "$EXPECTED_REPO" ]; then
        echo "WARNING: Source repo does not match expected"
        echo "  Expected: $EXPECTED_REPO"
        echo "  Got:      $SOURCE_REPO"
    fi
fi

echo ""
echo "Note: This is a scaffold implementation."
echo "In production, use tools like:"
echo "  - cosign verify-attestation"
echo "  - gh attestation verify"
echo ""
echo "Scaffold verification completed."
