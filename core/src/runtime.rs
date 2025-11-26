//! QRAP Runtime Module
//!
//! Provides the core runtime for quantum attestation protocol operations.

use std::time::{Duration, Instant};
use thiserror::Error;

/// Runtime error types
#[derive(Error, Debug)]
pub enum RuntimeError {
    #[error("Runtime initialization failed: {0}")]
    InitializationError(String),

    #[error("Invalid configuration: {0}")]
    ConfigurationError(String),

    #[error("State transition error: {0}")]
    StateTransitionError(String),

    #[error("Timeout error: operation exceeded {0:?}")]
    TimeoutError(Duration),
}

/// Runtime state enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuntimeState {
    /// Runtime is not initialized
    Uninitialized,
    /// Runtime is ready for operations
    Ready,
    /// Runtime is executing an operation
    Running,
    /// Runtime has completed an operation
    Completed,
    /// Runtime encountered an error
    Error,
}

impl Default for RuntimeState {
    fn default() -> Self {
        RuntimeState::Uninitialized
    }
}

/// Runtime configuration
#[derive(Debug, Clone)]
pub struct RuntimeConfig {
    /// Maximum execution time for operations
    pub max_execution_time: Duration,
    /// Number of qubits to simulate
    pub num_qubits: usize,
    /// Enable verbose logging
    pub verbose: bool,
}

impl Default for RuntimeConfig {
    fn default() -> Self {
        RuntimeConfig {
            max_execution_time: Duration::from_secs(60),
            num_qubits: 4,
            verbose: false,
        }
    }
}

impl RuntimeConfig {
    /// Create a new runtime configuration
    pub fn new(num_qubits: usize) -> Self {
        RuntimeConfig {
            num_qubits,
            ..Default::default()
        }
    }

    /// Set maximum execution time
    pub fn with_max_execution_time(mut self, duration: Duration) -> Self {
        self.max_execution_time = duration;
        self
    }

    /// Enable or disable verbose mode
    pub fn with_verbose(mut self, verbose: bool) -> Self {
        self.verbose = verbose;
        self
    }
}

/// QRAP Runtime
#[derive(Debug)]
pub struct QrapRuntime {
    config: RuntimeConfig,
    state: RuntimeState,
    start_time: Option<Instant>,
    execution_count: u64,
}

impl QrapRuntime {
    /// Create a new QRAP runtime with the given configuration
    pub fn new(config: RuntimeConfig) -> Result<Self, RuntimeError> {
        if config.num_qubits == 0 {
            return Err(RuntimeError::ConfigurationError(
                "Number of qubits must be greater than 0".to_string(),
            ));
        }

        if config.num_qubits > 20 {
            return Err(RuntimeError::ConfigurationError(
                "Number of qubits must not exceed 20 for simulation".to_string(),
            ));
        }

        Ok(QrapRuntime {
            config,
            state: RuntimeState::Ready,
            start_time: None,
            execution_count: 0,
        })
    }

    /// Get the current runtime state
    pub fn state(&self) -> RuntimeState {
        self.state
    }

    /// Get the runtime configuration
    pub fn config(&self) -> &RuntimeConfig {
        &self.config
    }

    /// Get the execution count
    pub fn execution_count(&self) -> u64 {
        self.execution_count
    }

    /// Start the runtime
    pub fn start(&mut self) -> Result<(), RuntimeError> {
        match self.state {
            RuntimeState::Ready | RuntimeState::Completed => {
                self.state = RuntimeState::Running;
                self.start_time = Some(Instant::now());
                self.execution_count += 1;
                Ok(())
            }
            _ => Err(RuntimeError::StateTransitionError(format!(
                "Cannot start from state {:?}",
                self.state
            ))),
        }
    }

    /// Stop the runtime
    pub fn stop(&mut self) -> Result<Duration, RuntimeError> {
        match self.state {
            RuntimeState::Running => {
                let elapsed = self.start_time.map(|t| t.elapsed()).unwrap_or_default();
                self.state = RuntimeState::Completed;
                self.start_time = None;
                Ok(elapsed)
            }
            _ => Err(RuntimeError::StateTransitionError(format!(
                "Cannot stop from state {:?}",
                self.state
            ))),
        }
    }

    /// Reset the runtime to ready state
    pub fn reset(&mut self) {
        self.state = RuntimeState::Ready;
        self.start_time = None;
    }

    /// Check if the runtime has exceeded max execution time
    pub fn check_timeout(&self) -> Result<(), RuntimeError> {
        if let Some(start) = self.start_time {
            if start.elapsed() > self.config.max_execution_time {
                return Err(RuntimeError::TimeoutError(self.config.max_execution_time));
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = RuntimeConfig::default();
        assert_eq!(config.num_qubits, 4);
        assert!(!config.verbose);
    }

    #[test]
    fn test_runtime_creation() {
        let runtime = QrapRuntime::new(RuntimeConfig::default());
        assert!(runtime.is_ok());
        let runtime = runtime.unwrap();
        assert_eq!(runtime.state(), RuntimeState::Ready);
    }

    #[test]
    fn test_runtime_invalid_qubits() {
        let config = RuntimeConfig::new(0);
        let runtime = QrapRuntime::new(config);
        assert!(runtime.is_err());
    }

    #[test]
    fn test_runtime_lifecycle() {
        let mut runtime = QrapRuntime::new(RuntimeConfig::default()).unwrap();
        assert_eq!(runtime.state(), RuntimeState::Ready);

        runtime.start().unwrap();
        assert_eq!(runtime.state(), RuntimeState::Running);

        let elapsed = runtime.stop().unwrap();
        assert_eq!(runtime.state(), RuntimeState::Completed);
        assert!(elapsed.as_nanos() > 0);
    }
}
