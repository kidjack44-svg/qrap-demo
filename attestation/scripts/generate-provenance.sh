#!/usr/bin/env bash
set -euo pipefail

# Scaffold script for generating provenance for qrap-demo builds.
# This script is intentionally simple and contains no secrets.
# In a real setup, this would invoke tools like `cosign` or a SLSA generator.

OUTPUT="${1:-attestation/examples/provenance.release-build.v1.json}"
COMMIT="${GITHUB_SHA:-0000000000000000000000000000000000000000}"
REPO="${GITHUB_REPOSITORY:-kidjack44-svg/qrap-demo}"
WORKFLOW="${GITHUB_WORKFLOW:-manual-scaffold}"
RUN_ID="${GITHUB_RUN_ID:-0000000000}"
TIMESTAMP="$(date -u +"%Y-%m-%dT%H:%M:%SZ" 2>/dev/null || echo "1970-01-01T00:00:00Z")"
BRANCH="${GITHUB_REF_NAME:-main}"

mkdir -p "$(dirname "$OUTPUT")"

cat > "$OUTPUT" <<EOF
{
  "\$schema": "../schema/provenance.schema.v1.json",
  "subject": {
    "name": "qrap-demo",
    "type": "container-image",
    "digest": {
      "sha256": "0000000000000000000000000000000000000000000000000000000000000000"
    }
  },
  "build": {
    "source_repo": "https://github.com/${REPO}",
    "commit": "${COMMIT}",
    "branch": "${BRANCH}",
    "workflow": "${WORKFLOW}",
    "workflow_run_id": "${RUN_ID}",
    "timestamp": "${TIMESTAMP}",
    "builder": {
      "id": "https://github.com/actions/runner",
      "version": "2.x"
    }
  },
  "metadata": {
    "buildInvocationId": "${RUN_ID}",
    "completeness": {
      "parameters": true,
      "environment": true,
      "materials": true
    },
    "reproducible": false
  }
}
EOF

echo "Wrote scaffold provenance to $OUTPUT"
echo ""
echo "Note: This is a scaffold implementation."
echo "In production, use tools like:"
echo "  - cosign attest"
echo "  - slsa-framework/slsa-github-generator"
echo "  - gh attestation create"

