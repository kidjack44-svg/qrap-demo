#!/usr/bin/env bash
set -euo pipefail

# Scaffold script for verifying SBOM for qrap-demo builds.
# This script is intentionally simple and contains no secrets.
# In a real setup, this would invoke tools for SBOM validation and signature verification.

SBOM="${1:-attestation/examples/sbom.web-client.v1.json}"

echo "Verifying SBOM: $SBOM"
echo ""

if [ ! -f "$SBOM" ]; then
    echo "ERROR: SBOM file not found: $SBOM"
    exit 1
fi

# Basic JSON validation
if ! command -v jq &> /dev/null; then
    echo "WARNING: jq not installed, skipping JSON validation"
else
    if ! jq empty "$SBOM" 2>/dev/null; then
        echo "ERROR: Invalid JSON in SBOM file"
        exit 1
    fi

    echo "Checking SBOM contents..."
    
    # Check required fields
    SUBJECT_NAME=$(jq -r '.subject.name // empty' "$SBOM")
    SUBJECT_VERSION=$(jq -r '.subject.version // empty' "$SBOM")
    PACKAGE_COUNT=$(jq '.packages | length' "$SBOM")
    GENERATED_AT=$(jq -r '.generated_at // empty' "$SBOM")

    if [ -z "$SUBJECT_NAME" ]; then
        echo "ERROR: Missing subject.name"
        exit 1
    fi

    echo "  Subject: $SUBJECT_NAME"
    echo "  Version: ${SUBJECT_VERSION:-"(not specified)"}"
    echo "  Packages: $PACKAGE_COUNT"
    echo "  Generated: ${GENERATED_AT:-"(not specified)"}"
    echo ""

    # List packages
    echo "Package list:"
    jq -r '.packages[] | "  - \(.name)@\(.version) (\(.type // "unknown"))"' "$SBOM"
    echo ""

    # Check for required fields in packages
    MISSING_NAMES=$(jq '[.packages[] | select(.name == null or .name == "")] | length' "$SBOM")
    MISSING_VERSIONS=$(jq '[.packages[] | select(.version == null or .version == "")] | length' "$SBOM")

    if [ "$MISSING_NAMES" -gt 0 ]; then
        echo "WARNING: $MISSING_NAMES package(s) missing name"
    fi

    if [ "$MISSING_VERSIONS" -gt 0 ]; then
        echo "WARNING: $MISSING_VERSIONS package(s) missing version"
    fi
fi

echo ""
echo "Note: This is a scaffold implementation."
echo "In production, use tools like:"
echo "  - cosign verify-attestation (for signed SBOMs)"
echo "  - grype (for vulnerability scanning against SBOM)"
echo "  - sbom-scorecard"
echo ""
echo "Scaffold verification completed."

