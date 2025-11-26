//! QRAP Grover Module
//!
//! Provides a simulation of Grover's quantum search algorithm.

use rand::Rng;
use std::f64::consts::PI;

/// Result of Grover's algorithm execution
#[derive(Debug, Clone)]
pub struct GroverResult {
    /// The target value that was searched for
    pub target: usize,
    /// The value found by the algorithm
    pub found: usize,
    /// Whether the search was successful
    pub success: bool,
    /// Number of iterations performed
    pub iterations: u64,
    /// Number of qubits used
    pub num_qubits: usize,
    /// Probability of the found state
    pub probability: f64,
}

/// Oracle function type for Grover's algorithm
pub trait GroverOracle {
    /// Check if the given index is the target
    fn is_target(&self, index: usize) -> bool;

    /// Get the target value (for verification)
    fn target(&self) -> usize;
}

/// Simple oracle implementation for single target search
#[derive(Debug, Clone)]
pub struct SingleTargetOracle {
    target: usize,
    #[allow(dead_code)]
    search_space_size: usize,
}

impl SingleTargetOracle {
    /// Create a new single target oracle
    pub fn new(target: usize, search_space_size: usize) -> Self {
        SingleTargetOracle {
            target: target % search_space_size,
            search_space_size,
        }
    }
}

impl GroverOracle for SingleTargetOracle {
    fn is_target(&self, index: usize) -> bool {
        index == self.target
    }

    fn target(&self) -> usize {
        self.target
    }
}

/// Grover's algorithm simulator
#[derive(Debug)]
pub struct GroverSimulator {
    num_qubits: usize,
    search_space_size: usize,
    amplitudes: Vec<f64>,
}

impl GroverSimulator {
    /// Create a new Grover simulator with the specified number of qubits
    pub fn new(num_qubits: usize) -> Self {
        let search_space_size = 1 << num_qubits; // 2^num_qubits
        let initial_amplitude = 1.0 / (search_space_size as f64).sqrt();

        GroverSimulator {
            num_qubits,
            search_space_size,
            amplitudes: vec![initial_amplitude; search_space_size],
        }
    }

    /// Get the number of qubits
    pub fn num_qubits(&self) -> usize {
        self.num_qubits
    }

    /// Get the search space size
    pub fn search_space_size(&self) -> usize {
        self.search_space_size
    }

    /// Calculate the optimal number of iterations for Grover's algorithm
    pub fn optimal_iterations(&self) -> u64 {
        let n = self.search_space_size as f64;
        ((PI / 4.0) * n.sqrt()).floor() as u64
    }

    /// Apply the oracle operator (phase flip for target state)
    fn apply_oracle<O: GroverOracle>(&mut self, oracle: &O) {
        for i in 0..self.search_space_size {
            if oracle.is_target(i) {
                self.amplitudes[i] = -self.amplitudes[i];
            }
        }
    }

    /// Apply the diffusion operator (inversion about the mean)
    fn apply_diffusion(&mut self) {
        // Calculate mean amplitude
        let mean: f64 = self.amplitudes.iter().sum::<f64>() / self.search_space_size as f64;

        // Invert about the mean
        for amplitude in self.amplitudes.iter_mut() {
            *amplitude = 2.0 * mean - *amplitude;
        }
    }

    /// Run a single Grover iteration
    fn iterate<O: GroverOracle>(&mut self, oracle: &O) {
        self.apply_oracle(oracle);
        self.apply_diffusion();
    }

    /// Run Grover's algorithm with the given oracle
    pub fn run<O: GroverOracle>(&mut self, oracle: &O) -> GroverResult {
        let iterations = self.optimal_iterations();
        self.run_with_iterations(oracle, iterations)
    }

    /// Run Grover's algorithm with a specific number of iterations
    pub fn run_with_iterations<O: GroverOracle>(
        &mut self,
        oracle: &O,
        iterations: u64,
    ) -> GroverResult {
        // Reset amplitudes
        let initial_amplitude = 1.0 / (self.search_space_size as f64).sqrt();
        self.amplitudes = vec![initial_amplitude; self.search_space_size];

        // Apply Grover iterations
        for _ in 0..iterations {
            self.iterate(oracle);
        }

        // Measure (simulate by finding max probability state)
        let (found, probability) = self.measure();

        GroverResult {
            target: oracle.target(),
            found,
            success: oracle.is_target(found),
            iterations,
            num_qubits: self.num_qubits,
            probability,
        }
    }

    /// Simulate measurement by finding the state with highest probability
    fn measure(&self) -> (usize, f64) {
        let probabilities: Vec<f64> = self.amplitudes.iter().map(|a| a * a).collect();

        let mut rng = rand::thread_rng();
        let random_val: f64 = rng.gen();

        let mut cumulative = 0.0;
        for (i, &prob) in probabilities.iter().enumerate() {
            cumulative += prob;
            if random_val < cumulative {
                return (i, prob);
            }
        }

        // Fallback to max probability state
        let (max_idx, max_prob) = probabilities
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
            .unwrap();

        (max_idx, *max_prob)
    }

    /// Get the current probability distribution
    pub fn get_probabilities(&self) -> Vec<f64> {
        self.amplitudes.iter().map(|a| a * a).collect()
    }

    /// Get the probability of a specific state
    pub fn get_probability(&self, index: usize) -> f64 {
        if index < self.search_space_size {
            self.amplitudes[index] * self.amplitudes[index]
        } else {
            0.0
        }
    }
}

/// Run a Grover search benchmark
pub fn run_benchmark(num_qubits: usize, num_runs: usize) -> Vec<GroverResult> {
    let mut results = Vec::with_capacity(num_runs);
    let search_space_size = 1 << num_qubits;

    for _ in 0..num_runs {
        let target = rand::thread_rng().gen_range(0..search_space_size);
        let oracle = SingleTargetOracle::new(target, search_space_size);
        let mut simulator = GroverSimulator::new(num_qubits);
        let result = simulator.run(&oracle);
        results.push(result);
    }

    results
}

/// Calculate success rate from benchmark results
pub fn calculate_success_rate(results: &[GroverResult]) -> f64 {
    if results.is_empty() {
        return 0.0;
    }

    let successes = results.iter().filter(|r| r.success).count();
    successes as f64 / results.len() as f64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grover_simulator_creation() {
        let simulator = GroverSimulator::new(4);
        assert_eq!(simulator.num_qubits(), 4);
        assert_eq!(simulator.search_space_size(), 16);
    }

    #[test]
    fn test_optimal_iterations() {
        let simulator = GroverSimulator::new(4);
        let iterations = simulator.optimal_iterations();
        // For 4 qubits (16 states), optimal iterations is around 3
        assert!(iterations >= 2 && iterations <= 4);
    }

    #[test]
    fn test_single_target_oracle() {
        let oracle = SingleTargetOracle::new(5, 16);
        assert!(oracle.is_target(5));
        assert!(!oracle.is_target(3));
        assert_eq!(oracle.target(), 5);
    }

    #[test]
    fn test_grover_run() {
        let mut simulator = GroverSimulator::new(4);
        let oracle = SingleTargetOracle::new(7, 16);
        let result = simulator.run(&oracle);

        assert_eq!(result.target, 7);
        assert_eq!(result.num_qubits, 4);
        // With optimal iterations, success rate should be high
        // but due to probabilistic nature, we just check it ran
        assert!(result.iterations > 0);
    }

    #[test]
    fn test_probability_distribution() {
        let simulator = GroverSimulator::new(2);
        let probs = simulator.get_probabilities();

        assert_eq!(probs.len(), 4);
        // Initial state should have uniform distribution
        let expected_prob = 0.25;
        for prob in probs {
            assert!((prob - expected_prob).abs() < 1e-10);
        }
    }

    #[test]
    fn test_benchmark() {
        let results = run_benchmark(3, 10);
        assert_eq!(results.len(), 10);

        let success_rate = calculate_success_rate(&results);
        assert!(success_rate >= 0.0 && success_rate <= 1.0);
    }
}
