//! QRAP Core Library
//!
//! Quantum Runtime Attestation Protocol core functionality including:
//! - Runtime management and state tracking
//! - Attestation generation and verification
//! - Grover's algorithm simulation

pub mod attestation;
pub mod grover;
pub mod runtime;

pub use attestation::{Attestation, AttestationError, AttestationResult};
pub use grover::{GroverOracle, GroverResult, GroverSimulator};
pub use runtime::{QrapRuntime, RuntimeConfig, RuntimeError, RuntimeState};

/// Library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Initialize the QRAP core library
pub fn init() -> Result<(), RuntimeError> {
    QrapRuntime::new(RuntimeConfig::default()).map(|_| ())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init() {
        assert!(init().is_ok());
    }

    #[test]
    fn test_version() {
        assert_eq!(VERSION, "0.1.0");
    }
}
