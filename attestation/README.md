# Attestation for qrap-demo

This directory contains public, non-secret scaffolding for build and runtime
attestations for the `qrap-demo` repository.

## Overview

QRAP Demo (Quantum Runtime Attestation Protocol) uses attestations to provide
cryptographic proof of build provenance and software composition for:

- **Container images**: Docker images built from this repository
- **WASM artifacts**: WebAssembly builds for the visualizer
- **Python wheels**: PyO3-based Python bindings
- **Release bundles**: Compiled binaries and packages

## Layout

- `schema/` — JSON schemas for provenance, SBOM, and verification result
  documents used by qrap-demo.
- `examples/` — Example attestation documents (using fake commits/digests).
- `policies/` — Human-readable and machine-readable policy for required
  attestations in different environments.
- `scripts/` — Helper scripts for generating and verifying attestations.
  These scripts do not contain secrets and expect keys/tokens from the
  environment or CI.
- `ci/` — Reusable CI step snippets for generating and verifying attestations.

## Security Notice

No private keys, tokens, or other secrets are stored in this directory.
All scripts expect credentials to be provided via environment variables
(e.g., `COSIGN_KEY`, `GITHUB_TOKEN`) or CI secret injection.

## Attestation Types

### Provenance

Provenance attestations document how artifacts were built:
- Source repository and commit
- Build workflow and timestamp
- Builder identity and configuration

### SBOM (Software Bill of Materials)

SBOM attestations list all dependencies:
- Direct and transitive dependencies
- Version information
- License details

### Verification Results

Verification result documents capture the outcome of attestation verification:
- Subject artifact
- Checks performed
- Overall status

## Usage

### Generate Provenance

```bash
./attestation/scripts/generate-provenance.sh [output-path]
```

### Verify Provenance

```bash
./attestation/scripts/verify-provenance.sh [attestation-path]
```

### Generate SBOM

```bash
./attestation/scripts/generate-sbom.sh [output-path]
```

### Verify SBOM

```bash
./attestation/scripts/verify-sbom.sh [sbom-path]
```

## CI Integration

Reusable workflow steps are available in `ci/`:
- `steps-generate-provenance.yml` — Generate provenance in CI
- `steps-verify-provenance.yml` — Verify provenance in CI

## Related Documentation

- [Attestation Policy](policies/attestation-policy.md)
- [SLSA Framework](https://slsa.dev/)
- [Sigstore/Cosign](https://docs.sigstore.dev/)
- [SPDX SBOM](https://spdx.dev/)
