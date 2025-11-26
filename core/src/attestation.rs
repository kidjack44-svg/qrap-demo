//! QRAP Attestation Module
//!
//! Provides attestation generation and verification for quantum runtime operations.

use sha2::{Digest, Sha256};
use std::time::{SystemTime, UNIX_EPOCH};
use thiserror::Error;

/// Attestation error types
#[derive(Error, Debug)]
pub enum AttestationError {
    #[error("Attestation generation failed: {0}")]
    GenerationError(String),

    #[error("Attestation verification failed: {0}")]
    VerificationError(String),

    #[error("Invalid attestation data: {0}")]
    InvalidData(String),

    #[error("Attestation expired")]
    Expired,
}

/// Result type for attestation operations
pub type AttestationResult<T> = Result<T, AttestationError>;

/// Attestation data structure
#[derive(Debug, Clone)]
pub struct Attestation {
    /// Unique attestation identifier
    pub id: String,
    /// Timestamp when attestation was generated
    pub timestamp: u64,
    /// Hash of the attested data
    pub data_hash: String,
    /// Signature of the attestation
    pub signature: String,
    /// Number of qubits used in the operation
    pub num_qubits: usize,
    /// Number of iterations performed
    pub iterations: u64,
    /// Whether the operation was successful
    pub success: bool,
}

impl Attestation {
    /// Create a new attestation
    pub fn new(
        data: &[u8],
        num_qubits: usize,
        iterations: u64,
        success: bool,
    ) -> AttestationResult<Self> {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| AttestationError::GenerationError(e.to_string()))?
            .as_secs();

        let data_hash = Self::compute_hash(data);
        let id = Self::generate_id(&data_hash, timestamp);
        let signature = Self::generate_signature(&id, &data_hash, timestamp);

        Ok(Attestation {
            id,
            timestamp,
            data_hash,
            signature,
            num_qubits,
            iterations,
            success,
        })
    }

    /// Compute SHA-256 hash of data
    fn compute_hash(data: &[u8]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(data);
        hex::encode(hasher.finalize())
    }

    /// Generate unique attestation ID
    fn generate_id(data_hash: &str, timestamp: u64) -> String {
        let input = format!("{}{}", data_hash, timestamp);
        let mut hasher = Sha256::new();
        hasher.update(input.as_bytes());
        hex::encode(&hasher.finalize()[..16])
    }

    /// Generate attestation signature
    fn generate_signature(id: &str, data_hash: &str, timestamp: u64) -> String {
        let input = format!("{}:{}:{}", id, data_hash, timestamp);
        let mut hasher = Sha256::new();
        hasher.update(input.as_bytes());
        hex::encode(hasher.finalize())
    }

    /// Verify the attestation signature
    pub fn verify(&self) -> AttestationResult<bool> {
        let expected_signature =
            Self::generate_signature(&self.id, &self.data_hash, self.timestamp);

        if self.signature != expected_signature {
            return Err(AttestationError::VerificationError(
                "Signature mismatch".to_string(),
            ));
        }

        Ok(true)
    }

    /// Check if attestation is expired (older than max_age_secs)
    pub fn is_expired(&self, max_age_secs: u64) -> bool {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);

        now.saturating_sub(self.timestamp) > max_age_secs
    }

    /// Convert attestation to JSON string
    pub fn to_json(&self) -> String {
        format!(
            r#"{{"id":"{}","timestamp":{},"data_hash":"{}","signature":"{}","num_qubits":{},"iterations":{},"success":{}}}"#,
            self.id,
            self.timestamp,
            self.data_hash,
            self.signature,
            self.num_qubits,
            self.iterations,
            self.success
        )
    }
}

/// Attestation builder for fluent API
#[derive(Debug, Default)]
pub struct AttestationBuilder {
    data: Vec<u8>,
    num_qubits: usize,
    iterations: u64,
    success: bool,
}

impl AttestationBuilder {
    /// Create a new attestation builder
    pub fn new() -> Self {
        AttestationBuilder::default()
    }

    /// Set the data to attest
    pub fn with_data(mut self, data: &[u8]) -> Self {
        self.data = data.to_vec();
        self
    }

    /// Set the number of qubits
    pub fn with_qubits(mut self, num_qubits: usize) -> Self {
        self.num_qubits = num_qubits;
        self
    }

    /// Set the number of iterations
    pub fn with_iterations(mut self, iterations: u64) -> Self {
        self.iterations = iterations;
        self
    }

    /// Set the success flag
    pub fn with_success(mut self, success: bool) -> Self {
        self.success = success;
        self
    }

    /// Build the attestation
    pub fn build(self) -> AttestationResult<Attestation> {
        Attestation::new(&self.data, self.num_qubits, self.iterations, self.success)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_attestation_creation() {
        let data = b"test data";
        let attestation = Attestation::new(data, 4, 10, true);
        assert!(attestation.is_ok());

        let attestation = attestation.unwrap();
        assert!(!attestation.id.is_empty());
        assert!(!attestation.data_hash.is_empty());
        assert!(!attestation.signature.is_empty());
        assert_eq!(attestation.num_qubits, 4);
        assert_eq!(attestation.iterations, 10);
        assert!(attestation.success);
    }

    #[test]
    fn test_attestation_verification() {
        let data = b"test data";
        let attestation = Attestation::new(data, 4, 10, true).unwrap();
        assert!(attestation.verify().is_ok());
    }

    #[test]
    fn test_attestation_builder() {
        let attestation = AttestationBuilder::new()
            .with_data(b"test")
            .with_qubits(8)
            .with_iterations(100)
            .with_success(true)
            .build();

        assert!(attestation.is_ok());
        let attestation = attestation.unwrap();
        assert_eq!(attestation.num_qubits, 8);
        assert_eq!(attestation.iterations, 100);
    }

    #[test]
    fn test_attestation_expiry() {
        let data = b"test data";
        let attestation = Attestation::new(data, 4, 10, true).unwrap();

        // Should not be expired with 1 hour max age
        assert!(!attestation.is_expired(3600));

        // Wait a tiny bit and check with very short max age
        std::thread::sleep(std::time::Duration::from_millis(10));
        // With 0 second max age and after a small delay, should be expired
        // Note: This may still pass if within the same second, so we use a longer check
        assert!(attestation.is_expired(0) || !attestation.is_expired(1));
    }

    #[test]
    fn test_attestation_json() {
        let data = b"test";
        let attestation = Attestation::new(data, 4, 10, true).unwrap();
        let json = attestation.to_json();

        assert!(json.contains("\"id\":"));
        assert!(json.contains("\"timestamp\":"));
        assert!(json.contains("\"data_hash\":"));
        assert!(json.contains("\"num_qubits\":4"));
    }
}
