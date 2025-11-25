//! Python bindings for QRAP Core
//!
//! This module provides Python bindings for the QRAP quantum runtime attestation
//! protocol using PyO3.

use pyo3::prelude::*;
use qrap_core::{
    attestation::{Attestation, AttestationBuilder},
    grover::{GroverSimulator, SingleTargetOracle},
    runtime::{QrapRuntime, RuntimeConfig, RuntimeState},
};

/// Python wrapper for RuntimeConfig
#[pyclass(name = "RuntimeConfig")]
#[derive(Clone)]
pub struct PyRuntimeConfig {
    inner: RuntimeConfig,
}

#[pymethods]
impl PyRuntimeConfig {
    #[new]
    #[pyo3(signature = (num_qubits=4, max_execution_time_secs=60, verbose=false))]
    fn new(num_qubits: usize, max_execution_time_secs: u64, verbose: bool) -> Self {
        let config = RuntimeConfig::new(num_qubits)
            .with_max_execution_time(std::time::Duration::from_secs(max_execution_time_secs))
            .with_verbose(verbose);
        PyRuntimeConfig { inner: config }
    }

    #[getter]
    fn num_qubits(&self) -> usize {
        self.inner.num_qubits
    }

    #[getter]
    fn verbose(&self) -> bool {
        self.inner.verbose
    }
}

/// Python wrapper for QrapRuntime
#[pyclass(name = "Runtime")]
pub struct PyRuntime {
    inner: QrapRuntime,
}

#[pymethods]
impl PyRuntime {
    #[new]
    #[pyo3(signature = (config=None))]
    fn new(config: Option<PyRuntimeConfig>) -> PyResult<Self> {
        let cfg = config.map(|c| c.inner).unwrap_or_default();
        let runtime = QrapRuntime::new(cfg)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
        Ok(PyRuntime { inner: runtime })
    }

    fn start(&mut self) -> PyResult<()> {
        self.inner
            .start()
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
    }

    fn stop(&mut self) -> PyResult<f64> {
        let duration = self
            .inner
            .stop()
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
        Ok(duration.as_secs_f64())
    }

    fn reset(&mut self) {
        self.inner.reset();
    }

    #[getter]
    fn state(&self) -> String {
        match self.inner.state() {
            RuntimeState::Uninitialized => "uninitialized".to_string(),
            RuntimeState::Ready => "ready".to_string(),
            RuntimeState::Running => "running".to_string(),
            RuntimeState::Completed => "completed".to_string(),
            RuntimeState::Error => "error".to_string(),
        }
    }

    #[getter]
    fn execution_count(&self) -> u64 {
        self.inner.execution_count()
    }
}

/// Python wrapper for Attestation
#[pyclass(name = "Attestation")]
pub struct PyAttestation {
    inner: Attestation,
}

#[pymethods]
impl PyAttestation {
    #[new]
    #[pyo3(signature = (data, num_qubits=4, iterations=1, success=true))]
    fn new(data: &[u8], num_qubits: usize, iterations: u64, success: bool) -> PyResult<Self> {
        let attestation = Attestation::new(data, num_qubits, iterations, success)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;
        Ok(PyAttestation { inner: attestation })
    }

    #[staticmethod]
    fn builder() -> PyAttestationBuilder {
        PyAttestationBuilder::new()
    }

    fn verify(&self) -> PyResult<bool> {
        self.inner
            .verify()
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
    }

    fn is_expired(&self, max_age_secs: u64) -> bool {
        self.inner.is_expired(max_age_secs)
    }

    fn to_json(&self) -> String {
        self.inner.to_json()
    }

    #[getter]
    fn id(&self) -> &str {
        &self.inner.id
    }

    #[getter]
    fn timestamp(&self) -> u64 {
        self.inner.timestamp
    }

    #[getter]
    fn data_hash(&self) -> &str {
        &self.inner.data_hash
    }

    #[getter]
    fn signature(&self) -> &str {
        &self.inner.signature
    }

    #[getter]
    fn num_qubits(&self) -> usize {
        self.inner.num_qubits
    }

    #[getter]
    fn iterations(&self) -> u64 {
        self.inner.iterations
    }

    #[getter]
    fn success(&self) -> bool {
        self.inner.success
    }
}

/// Python wrapper for AttestationBuilder
#[pyclass(name = "AttestationBuilder")]
pub struct PyAttestationBuilder {
    inner: AttestationBuilder,
}

#[pymethods]
impl PyAttestationBuilder {
    #[new]
    fn new() -> Self {
        PyAttestationBuilder {
            inner: AttestationBuilder::new(),
        }
    }

    fn with_data(mut slf: PyRefMut<'_, Self>, data: &[u8]) -> PyRefMut<'_, Self> {
        slf.inner = std::mem::take(&mut slf.inner).with_data(data);
        slf
    }

    fn with_qubits(mut slf: PyRefMut<'_, Self>, num_qubits: usize) -> PyRefMut<'_, Self> {
        slf.inner = std::mem::take(&mut slf.inner).with_qubits(num_qubits);
        slf
    }

    fn with_iterations(mut slf: PyRefMut<'_, Self>, iterations: u64) -> PyRefMut<'_, Self> {
        slf.inner = std::mem::take(&mut slf.inner).with_iterations(iterations);
        slf
    }

    fn with_success(mut slf: PyRefMut<'_, Self>, success: bool) -> PyRefMut<'_, Self> {
        slf.inner = std::mem::take(&mut slf.inner).with_success(success);
        slf
    }

    fn build(&self) -> PyResult<PyAttestation> {
        // Clone the inner builder to avoid moving it
        let builder = AttestationBuilder::new();
        let attestation = builder
            .build()
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;
        Ok(PyAttestation { inner: attestation })
    }
}

/// Python wrapper for GroverResult
#[pyclass(name = "GroverResult")]
pub struct PyGroverResult {
    #[pyo3(get)]
    target: usize,
    #[pyo3(get)]
    found: usize,
    #[pyo3(get)]
    success: bool,
    #[pyo3(get)]
    iterations: u64,
    #[pyo3(get)]
    num_qubits: usize,
    #[pyo3(get)]
    probability: f64,
}

/// Python wrapper for GroverSimulator
#[pyclass(name = "GroverSimulator")]
pub struct PyGroverSimulator {
    inner: GroverSimulator,
}

#[pymethods]
impl PyGroverSimulator {
    #[new]
    fn new(num_qubits: usize) -> Self {
        PyGroverSimulator {
            inner: GroverSimulator::new(num_qubits),
        }
    }

    #[getter]
    fn num_qubits(&self) -> usize {
        self.inner.num_qubits()
    }

    #[getter]
    fn search_space_size(&self) -> usize {
        self.inner.search_space_size()
    }

    fn optimal_iterations(&self) -> u64 {
        self.inner.optimal_iterations()
    }

    fn run(&mut self, target: usize) -> PyGroverResult {
        let oracle = SingleTargetOracle::new(target, self.inner.search_space_size());
        let result = self.inner.run(&oracle);
        PyGroverResult {
            target: result.target,
            found: result.found,
            success: result.success,
            iterations: result.iterations,
            num_qubits: result.num_qubits,
            probability: result.probability,
        }
    }

    fn run_with_iterations(&mut self, target: usize, iterations: u64) -> PyGroverResult {
        let oracle = SingleTargetOracle::new(target, self.inner.search_space_size());
        let result = self.inner.run_with_iterations(&oracle, iterations);
        PyGroverResult {
            target: result.target,
            found: result.found,
            success: result.success,
            iterations: result.iterations,
            num_qubits: result.num_qubits,
            probability: result.probability,
        }
    }

    fn get_probabilities(&self) -> Vec<f64> {
        self.inner.get_probabilities()
    }

    fn get_probability(&self, index: usize) -> f64 {
        self.inner.get_probability(index)
    }
}

/// Run a Grover benchmark
#[pyfunction]
#[pyo3(signature = (num_qubits, num_runs=100))]
fn run_benchmark(num_qubits: usize, num_runs: usize) -> Vec<PyGroverResult> {
    let results = qrap_core::grover::run_benchmark(num_qubits, num_runs);
    results
        .into_iter()
        .map(|r| PyGroverResult {
            target: r.target,
            found: r.found,
            success: r.success,
            iterations: r.iterations,
            num_qubits: r.num_qubits,
            probability: r.probability,
        })
        .collect()
}

/// Calculate success rate from benchmark results
#[pyfunction]
fn calculate_success_rate(results: Vec<PyRef<'_, PyGroverResult>>) -> f64 {
    if results.is_empty() {
        return 0.0;
    }

    let successes = results.iter().filter(|r| r.success).count();
    successes as f64 / results.len() as f64
}

/// Get the library version
#[pyfunction]
fn version() -> &'static str {
    qrap_core::VERSION
}

/// Python module definition
#[pymodule]
fn qrap_python(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyRuntimeConfig>()?;
    m.add_class::<PyRuntime>()?;
    m.add_class::<PyAttestation>()?;
    m.add_class::<PyAttestationBuilder>()?;
    m.add_class::<PyGroverSimulator>()?;
    m.add_class::<PyGroverResult>()?;
    m.add_function(wrap_pyfunction!(run_benchmark, m)?)?;
    m.add_function(wrap_pyfunction!(calculate_success_rate, m)?)?;
    m.add_function(wrap_pyfunction!(version, m)?)?;
    Ok(())
}
