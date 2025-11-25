# QRAP Demo

[![Rust](https://img.shields.io/badge/rust-1.75+-orange.svg)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

**Quantum Runtime Attestation Protocol** - A Rust-based framework for quantum simulation with cryptographic attestation.

## Overview

QRAP Demo provides:
- **Core Library**: Quantum simulation primitives including Grover's algorithm
- **Runtime Attestation**: Cryptographic proof generation for quantum operations
- **Python Bindings**: PyO3-based Python interface
- **WASM Visualizer**: Browser-based quantum state visualization
- **Jupyter Notebooks**: Interactive demos and benchmarks

## Project Structure

```
qrap-demo/
├── core/                    # Core Rust library
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs           # Library entry point
│       ├── runtime.rs       # Runtime management
│       ├── attestation.rs   # Attestation generation/verification
│       └── grover.rs        # Grover's algorithm simulation
├── python/                  # Python bindings (PyO3)
│   ├── Cargo.toml
│   ├── pyproject.toml
│   └── src/lib.rs
├── wasm-visualizer/         # WebAssembly visualizer
│   ├── Cargo.toml
│   └── src/lib.rs
├── notebooks/               # Jupyter notebooks
│   ├── 01_runtime_attestation.ipynb
│   └── 02_grover_benchmark.ipynb
├── tests/                   # Integration tests
│   ├── attestation_tests.rs
│   └── runtime_tests.rs
├── docker/                  # Docker configuration
│   ├── Dockerfile
│   └── entrypoint.sh
├── scripts/                 # Build and utility scripts
│   ├── build_all.sh
│   ├── run_tests.sh
│   └── launch_notebooks.sh
├── baseline/                # Performance baselines
│   └── grover_ntos_genesis_baseline.log
├── README.md
└── VALIDATION_CHECKLIST.md
```

## Quick Start

### Prerequisites

- Rust 1.75 or later
- Python 3.8+ (for Python bindings)
- Node.js (for WASM development)

### Building

```bash
# Clone the repository
git clone https://github.com/kidjack44-svg/qrap-demo.git
cd qrap-demo

# Build the core library
cargo build --release --package qrap-core

# Run tests
cargo test --all
```

### Python Bindings

```bash
# Install maturin
pip install maturin

# Build and install Python bindings
cd python
maturin develop
cd ..

# Use in Python
python -c "import qrap_python; print(qrap_python.version())"
```

### WASM Build

```bash
# Add WASM target
rustup target add wasm32-unknown-unknown

# Build WASM package
cargo build --release --target wasm32-unknown-unknown --package qrap-wasm-visualizer
```

### Docker

```bash
# Build Docker image
docker build -t qrap-demo -f docker/Dockerfile .

# Run with Jupyter
docker run -p 8888:8888 qrap-demo
```

## Usage Examples

### Rust

```rust
use qrap_core::{QrapRuntime, RuntimeConfig, GroverSimulator, SingleTargetOracle};

// Create and configure runtime
let config = RuntimeConfig::new(4);
let mut runtime = QrapRuntime::new(config)?;

// Run Grover's algorithm
let mut simulator = GroverSimulator::new(4);
let oracle = SingleTargetOracle::new(7, 16);

runtime.start()?;
let result = simulator.run(&oracle);
let elapsed = runtime.stop()?;

println!("Found: {}, Success: {}", result.found, result.success);
```

### Python

```python
import qrap_python as qrap

# Create a Grover simulator
simulator = qrap.GroverSimulator(num_qubits=4)

# Run the search
result = simulator.run(target=7)
print(f"Found: {result.found}, Success: {result.success}")

# Generate attestation
attestation = qrap.Attestation(
    data=b"simulation result",
    num_qubits=4,
    iterations=result.iterations,
    success=result.success
)
print(f"Attestation ID: {attestation.id}")
```

## Algorithm Details

### Grover's Algorithm

Grover's algorithm provides quadratic speedup for unstructured search:

- **Classical**: O(N) queries to find target
- **Quantum**: O(√N) queries to find target

For N = 2^n states:
- Optimal iterations: ⌊(π/4)√N⌋
- Success probability: ~96.9% for 4 qubits

### Attestation

Attestations provide cryptographic proof of quantum operations:

1. **Data Hash**: SHA-256 hash of operation data
2. **Timestamp**: Unix timestamp of attestation creation
3. **Signature**: HMAC-based signature for verification
4. **Metadata**: Qubit count, iterations, success status

## Testing

```bash
# Run all tests
cargo test --all

# Run specific test suite
cargo test --package qrap-core
cargo test --test attestation_tests
cargo test --test runtime_tests

# Run with verbose output
cargo test --all -- --nocapture
```

## Benchmarks

See `baseline/grover_ntos_genesis_baseline.log` for performance baselines.

```bash
# Run benchmarks
cargo bench
```

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Run tests and linting
5. Submit a pull request

## License

MIT License - see LICENSE file for details.