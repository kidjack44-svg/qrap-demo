//! Runtime Tests
//!
//! Integration tests for the QRAP runtime module.

use qrap_core::runtime::{QrapRuntime, RuntimeConfig, RuntimeError, RuntimeState};
use std::time::Duration;

#[test]
fn test_runtime_default_config() {
    let config = RuntimeConfig::default();

    assert_eq!(config.num_qubits, 4);
    assert_eq!(config.max_execution_time, Duration::from_secs(60));
    assert!(!config.verbose);
}

#[test]
fn test_runtime_custom_config() {
    let config = RuntimeConfig::new(8)
        .with_max_execution_time(Duration::from_secs(120))
        .with_verbose(true);

    assert_eq!(config.num_qubits, 8);
    assert_eq!(config.max_execution_time, Duration::from_secs(120));
    assert!(config.verbose);
}

#[test]
fn test_runtime_creation_valid() {
    let config = RuntimeConfig::new(4);
    let runtime = QrapRuntime::new(config);

    assert!(runtime.is_ok());
    let rt = runtime.unwrap();
    assert_eq!(rt.state(), RuntimeState::Ready);
}

#[test]
fn test_runtime_creation_zero_qubits() {
    let config = RuntimeConfig::new(0);
    let runtime = QrapRuntime::new(config);

    assert!(runtime.is_err());
    match runtime.unwrap_err() {
        RuntimeError::ConfigurationError(msg) => {
            assert!(msg.contains("greater than 0"));
        }
        _ => panic!("Expected ConfigurationError"),
    }
}

#[test]
fn test_runtime_creation_too_many_qubits() {
    let config = RuntimeConfig::new(25);
    let runtime = QrapRuntime::new(config);

    assert!(runtime.is_err());
    match runtime.unwrap_err() {
        RuntimeError::ConfigurationError(msg) => {
            assert!(msg.contains("exceed 20"));
        }
        _ => panic!("Expected ConfigurationError"),
    }
}

#[test]
fn test_runtime_lifecycle_normal() {
    let mut runtime = QrapRuntime::new(RuntimeConfig::default()).unwrap();

    // Initial state should be Ready
    assert_eq!(runtime.state(), RuntimeState::Ready);
    assert_eq!(runtime.execution_count(), 0);

    // Start the runtime
    assert!(runtime.start().is_ok());
    assert_eq!(runtime.state(), RuntimeState::Running);
    assert_eq!(runtime.execution_count(), 1);

    // Stop the runtime
    let elapsed = runtime.stop();
    assert!(elapsed.is_ok());
    assert_eq!(runtime.state(), RuntimeState::Completed);
}

#[test]
fn test_runtime_multiple_executions() {
    let mut runtime = QrapRuntime::new(RuntimeConfig::default()).unwrap();

    for i in 1..=5 {
        runtime.start().unwrap();
        runtime.stop().unwrap();
        assert_eq!(runtime.execution_count(), i);
    }
}

#[test]
fn test_runtime_start_from_completed() {
    let mut runtime = QrapRuntime::new(RuntimeConfig::default()).unwrap();

    // Complete one execution
    runtime.start().unwrap();
    runtime.stop().unwrap();
    assert_eq!(runtime.state(), RuntimeState::Completed);

    // Should be able to start again from Completed state
    assert!(runtime.start().is_ok());
    assert_eq!(runtime.state(), RuntimeState::Running);
}

#[test]
fn test_runtime_double_start_error() {
    let mut runtime = QrapRuntime::new(RuntimeConfig::default()).unwrap();

    // First start should succeed
    assert!(runtime.start().is_ok());

    // Second start should fail
    let result = runtime.start();
    assert!(result.is_err());
    match result.unwrap_err() {
        RuntimeError::StateTransitionError(msg) => {
            assert!(msg.contains("Running"));
        }
        _ => panic!("Expected StateTransitionError"),
    }
}

#[test]
fn test_runtime_stop_without_start_error() {
    let mut runtime = QrapRuntime::new(RuntimeConfig::default()).unwrap();

    // Stop without start should fail
    let result = runtime.stop();
    assert!(result.is_err());
    match result.unwrap_err() {
        RuntimeError::StateTransitionError(msg) => {
            assert!(msg.contains("Ready"));
        }
        _ => panic!("Expected StateTransitionError"),
    }
}

#[test]
fn test_runtime_reset() {
    let mut runtime = QrapRuntime::new(RuntimeConfig::default()).unwrap();

    // Start and stop
    runtime.start().unwrap();
    runtime.stop().unwrap();
    assert_eq!(runtime.state(), RuntimeState::Completed);

    // Reset
    runtime.reset();
    assert_eq!(runtime.state(), RuntimeState::Ready);
}

#[test]
fn test_runtime_elapsed_time() {
    let mut runtime = QrapRuntime::new(RuntimeConfig::default()).unwrap();

    runtime.start().unwrap();

    // Small delay to ensure elapsed time is measurable
    std::thread::sleep(Duration::from_millis(10));

    let elapsed = runtime.stop().unwrap();
    assert!(elapsed.as_millis() >= 10);
}

#[test]
fn test_runtime_config_getters() {
    let config = RuntimeConfig::new(6)
        .with_max_execution_time(Duration::from_secs(30))
        .with_verbose(true);

    let runtime = QrapRuntime::new(config).unwrap();
    let retrieved_config = runtime.config();

    assert_eq!(retrieved_config.num_qubits, 6);
    assert_eq!(retrieved_config.max_execution_time, Duration::from_secs(30));
    assert!(retrieved_config.verbose);
}

#[test]
fn test_runtime_boundary_qubits() {
    // Test minimum valid qubits
    let config = RuntimeConfig::new(1);
    assert!(QrapRuntime::new(config).is_ok());

    // Test maximum valid qubits
    let config = RuntimeConfig::new(20);
    assert!(QrapRuntime::new(config).is_ok());
}

#[test]
fn test_runtime_check_timeout_not_exceeded() {
    let mut runtime = QrapRuntime::new(
        RuntimeConfig::new(4).with_max_execution_time(Duration::from_secs(60)),
    )
    .unwrap();

    runtime.start().unwrap();
    assert!(runtime.check_timeout().is_ok());
    runtime.stop().unwrap();
}
