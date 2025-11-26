//! QRAP WebAssembly Visualizer
//!
//! Provides WebAssembly bindings for visualizing quantum simulations in the browser.

use qrap_core::{
    grover::{GroverSimulator, SingleTargetOracle},
    runtime::{QrapRuntime, RuntimeConfig},
};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

/// Initialize panic hook for better error messages
#[wasm_bindgen(start)]
pub fn init() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

/// Log a message to the browser console
#[wasm_bindgen]
pub fn log(message: &str) {
    web_sys::console::log_1(&message.into());
}

/// Grover simulation result for JavaScript
#[derive(Serialize, Deserialize)]
pub struct JsGroverResult {
    pub target: usize,
    pub found: usize,
    pub success: bool,
    pub iterations: u64,
    pub num_qubits: usize,
    pub probability: f64,
}

/// Simulation step data for visualization
#[derive(Serialize, Deserialize)]
pub struct SimulationStep {
    pub step: usize,
    pub probabilities: Vec<f64>,
    pub target_probability: f64,
}

/// WASM wrapper for Grover simulation
#[wasm_bindgen]
pub struct WasmGroverSimulator {
    simulator: GroverSimulator,
    target: usize,
}

#[wasm_bindgen]
impl WasmGroverSimulator {
    /// Create a new Grover simulator
    #[wasm_bindgen(constructor)]
    pub fn new(num_qubits: usize) -> Self {
        let simulator = GroverSimulator::new(num_qubits);
        WasmGroverSimulator {
            simulator,
            target: 0,
        }
    }

    /// Get the number of qubits
    #[wasm_bindgen(getter)]
    pub fn num_qubits(&self) -> usize {
        self.simulator.num_qubits()
    }

    /// Get the search space size
    #[wasm_bindgen(getter)]
    pub fn search_space_size(&self) -> usize {
        self.simulator.search_space_size()
    }

    /// Get optimal number of iterations
    pub fn optimal_iterations(&self) -> u64 {
        self.simulator.optimal_iterations()
    }

    /// Set the target for the search
    pub fn set_target(&mut self, target: usize) {
        self.target = target % self.simulator.search_space_size();
    }

    /// Run the Grover simulation and return result as JsValue
    pub fn run(&mut self) -> JsValue {
        let oracle = SingleTargetOracle::new(self.target, self.simulator.search_space_size());
        let result = self.simulator.run(&oracle);

        let js_result = JsGroverResult {
            target: result.target,
            found: result.found,
            success: result.success,
            iterations: result.iterations,
            num_qubits: result.num_qubits,
            probability: result.probability,
        };

        serde_wasm_bindgen::to_value(&js_result).unwrap_or(JsValue::NULL)
    }

    /// Run simulation with specific iterations
    pub fn run_with_iterations(&mut self, iterations: u64) -> JsValue {
        let oracle = SingleTargetOracle::new(self.target, self.simulator.search_space_size());
        let result = self.simulator.run_with_iterations(&oracle, iterations);

        let js_result = JsGroverResult {
            target: result.target,
            found: result.found,
            success: result.success,
            iterations: result.iterations,
            num_qubits: result.num_qubits,
            probability: result.probability,
        };

        serde_wasm_bindgen::to_value(&js_result).unwrap_or(JsValue::NULL)
    }

    /// Get current probability distribution
    pub fn get_probabilities(&self) -> Vec<f64> {
        self.simulator.get_probabilities()
    }

    /// Get probability of specific state
    pub fn get_probability(&self, index: usize) -> f64 {
        self.simulator.get_probability(index)
    }
}

/// WASM wrapper for Runtime
#[wasm_bindgen]
pub struct WasmRuntime {
    runtime: QrapRuntime,
}

#[wasm_bindgen]
impl WasmRuntime {
    /// Create a new runtime
    #[wasm_bindgen(constructor)]
    pub fn new(num_qubits: usize) -> Result<WasmRuntime, JsValue> {
        let config = RuntimeConfig::new(num_qubits);
        let runtime = QrapRuntime::new(config)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Ok(WasmRuntime { runtime })
    }

    /// Start the runtime
    pub fn start(&mut self) -> Result<(), JsValue> {
        self.runtime
            .start()
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }

    /// Stop the runtime and return elapsed time in seconds
    pub fn stop(&mut self) -> Result<f64, JsValue> {
        let duration = self
            .runtime
            .stop()
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Ok(duration.as_secs_f64())
    }

    /// Reset the runtime
    pub fn reset(&mut self) {
        self.runtime.reset();
    }

    /// Get current state as string
    #[wasm_bindgen(getter)]
    pub fn state(&self) -> String {
        format!("{:?}", self.runtime.state())
    }

    /// Get execution count
    #[wasm_bindgen(getter)]
    pub fn execution_count(&self) -> u64 {
        self.runtime.execution_count()
    }
}

/// Run a benchmark and return results as JsValue
#[wasm_bindgen]
pub fn run_benchmark(num_qubits: usize, num_runs: usize) -> JsValue {
    let results = qrap_core::grover::run_benchmark(num_qubits, num_runs);

    let js_results: Vec<JsGroverResult> = results
        .into_iter()
        .map(|r| JsGroverResult {
            target: r.target,
            found: r.found,
            success: r.success,
            iterations: r.iterations,
            num_qubits: r.num_qubits,
            probability: r.probability,
        })
        .collect();

    serde_wasm_bindgen::to_value(&js_results).unwrap_or(JsValue::NULL)
}

/// Calculate success rate from benchmark results
#[wasm_bindgen]
pub fn calculate_success_rate(results: JsValue) -> f64 {
    let js_results: Result<Vec<JsGroverResult>, _> = serde_wasm_bindgen::from_value(results);

    match js_results {
        Ok(results) => {
            if results.is_empty() {
                return 0.0;
            }
            let successes = results.iter().filter(|r| r.success).count();
            successes as f64 / results.len() as f64
        }
        Err(_) => 0.0,
    }
}

/// Get version string
#[wasm_bindgen]
pub fn version() -> String {
    qrap_core::VERSION.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wasm_grover_simulator() {
        let mut simulator = WasmGroverSimulator::new(4);
        assert_eq!(simulator.num_qubits(), 4);
        assert_eq!(simulator.search_space_size(), 16);
    }

    #[test]
    fn test_wasm_runtime() {
        let runtime = WasmRuntime::new(4);
        assert!(runtime.is_ok());
    }
}
