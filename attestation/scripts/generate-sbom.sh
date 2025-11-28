#!/usr/bin/env bash
set -euo pipefail

# Scaffold script for generating SBOM for qrap-demo builds.
# This script is intentionally simple and contains no secrets.
# In a real setup, this would invoke tools like `syft`, `trivy`, or `cyclonedx-cli`.

OUTPUT="${1:-attestation/examples/sbom.web-client.v1.json}"
SUBJECT_NAME="${SUBJECT_NAME:-qrap-demo}"
SUBJECT_VERSION="${SUBJECT_VERSION:-0.1.0}"
TIMESTAMP="$(date -u +"%Y-%m-%dT%H:%M:%SZ" 2>/dev/null || echo "1970-01-01T00:00:00Z")"

mkdir -p "$(dirname "$OUTPUT")"

cat > "$OUTPUT" <<EOF
{
  "\$schema": "../schema/sbom.schema.v1.json",
  "subject": {
    "name": "${SUBJECT_NAME}",
    "version": "${SUBJECT_VERSION}",
    "type": "application"
  },
  "sbom_version": "1.0.0",
  "generated_at": "${TIMESTAMP}",
  "generator": {
    "name": "qrap-demo-scaffold",
    "version": "0.1.0"
  },
  "packages": [
    {
      "name": "qrap-core",
      "version": "0.1.0",
      "type": "cargo",
      "license": "MIT",
      "purl": "pkg:cargo/qrap-core@0.1.0",
      "direct": true
    }
  ],
  "relationships": []
}
EOF

echo "Wrote scaffold SBOM to $OUTPUT"
echo ""
echo "Note: This is a scaffold implementation."
echo "In production, use tools like:"
echo "  - syft"
echo "  - trivy sbom"
echo "  - cyclonedx-cli"
echo "  - cargo-sbom"

