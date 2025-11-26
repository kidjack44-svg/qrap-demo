# QRAP Demo Validation Checklist

This checklist is used to validate the QRAP (Quantum Runtime Attestation Protocol) demo implementation.

## Core Library Validation

### Runtime Module
- [ ] Runtime can be created with valid configuration
- [ ] Runtime rejects invalid qubit counts (0 or >20)
- [ ] Runtime state transitions work correctly (Ready → Running → Completed)
- [ ] Runtime tracks execution count properly
- [ ] Runtime reset functionality works
- [ ] Timeout detection works correctly

### Attestation Module
- [ ] Attestations can be generated for arbitrary data
- [ ] Attestation hashes are consistent for same data
- [ ] Attestation signatures are verifiable
- [ ] Tampered attestations fail verification
- [ ] Attestation expiry detection works
- [ ] JSON serialization produces valid output

### Grover Module
- [ ] Simulator correctly initializes with uniform amplitude distribution
- [ ] Optimal iteration count follows O(√N) formula
- [ ] Oracle correctly identifies target states
- [ ] Diffusion operator inverts about mean
- [ ] Success rate is significantly better than random guessing
- [ ] Probability distribution concentrates on target state

## Integration Tests

### Cross-Module Integration
- [ ] Runtime can execute Grover simulations
- [ ] Attestations can be generated for Grover results
- [ ] End-to-end workflow produces valid attestations

### Python Bindings
- [ ] All core types are exposed to Python
- [ ] Python API matches Rust API semantics
- [ ] Error handling works correctly across FFI boundary
- [ ] Builder patterns work in Python

### WASM Bindings
- [ ] Grover simulator works in browser
- [ ] Runtime works in browser
- [ ] Serialization to JavaScript types works
- [ ] Benchmark functions work correctly

## Performance Validation

### Baseline Comparison
- [ ] 4-qubit success rate ≥ 90%
- [ ] 6-qubit success rate ≥ 90%
- [ ] 8-qubit success rate ≥ 90%
- [ ] Execution time scales as expected

### Memory Usage
- [ ] No memory leaks in simulation
- [ ] Reasonable memory footprint for large qubit counts

## Documentation Validation

### README
- [ ] Quick start instructions are accurate
- [ ] Installation steps work
- [ ] Example code runs successfully

### Notebooks
- [ ] Runtime attestation notebook runs without errors
- [ ] Grover benchmark notebook runs without errors
- [ ] Visualizations render correctly

## Build & CI Validation

### Build Scripts
- [ ] `build_all.sh` completes successfully
- [ ] `run_tests.sh` completes successfully
- [ ] Docker build completes successfully

### CI Pipeline
- [ ] All tests pass in CI
- [ ] Code coverage meets threshold
- [ ] No clippy warnings

---

## Sign-off

| Validator | Date | Status |
|-----------|------|--------|
| | | |

## Notes

*Add any validation notes or observations here.*
