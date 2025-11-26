//! Attestation Tests
//!
//! Integration tests for the QRAP attestation module.

use qrap_core::attestation::{Attestation, AttestationBuilder};

#[test]
fn test_attestation_basic_creation() {
    let data = b"test attestation data";
    let attestation = Attestation::new(data, 4, 10, true);

    assert!(attestation.is_ok());
    let att = attestation.unwrap();
    assert!(!att.id.is_empty());
    assert!(!att.data_hash.is_empty());
    assert!(!att.signature.is_empty());
    assert_eq!(att.num_qubits, 4);
    assert_eq!(att.iterations, 10);
    assert!(att.success);
}

#[test]
fn test_attestation_different_data() {
    let data1 = b"data one";
    let data2 = b"data two";

    let att1 = Attestation::new(data1, 4, 10, true).unwrap();
    let att2 = Attestation::new(data2, 4, 10, true).unwrap();

    // Different data should produce different hashes
    assert_ne!(att1.data_hash, att2.data_hash);
    // IDs should be different
    assert_ne!(att1.id, att2.id);
}

#[test]
fn test_attestation_verification_valid() {
    let data = b"valid attestation";
    let attestation = Attestation::new(data, 8, 100, true).unwrap();

    let result = attestation.verify();
    assert!(result.is_ok());
    assert!(result.unwrap());
}

#[test]
fn test_attestation_verification_tampered() {
    let data = b"original data";
    let mut attestation = Attestation::new(data, 4, 10, true).unwrap();

    // Tamper with the attestation
    attestation.signature = "tampered_signature".to_string();

    let result = attestation.verify();
    assert!(result.is_err());
}

#[test]
fn test_attestation_expiry_not_expired() {
    let data = b"fresh attestation";
    let attestation = Attestation::new(data, 4, 10, true).unwrap();

    // Should not be expired with 1 hour max age
    assert!(!attestation.is_expired(3600));
}

#[test]
fn test_attestation_expiry_immediately_expired() {
    let data = b"old attestation";
    let attestation = Attestation::new(data, 4, 10, true).unwrap();

    // Wait a tiny bit to ensure timestamp difference
    std::thread::sleep(std::time::Duration::from_secs(1));

    // Should be expired with 0 seconds max age after waiting
    assert!(attestation.is_expired(0));
}

#[test]
fn test_attestation_json_format() {
    let data = b"json test";
    let attestation = Attestation::new(data, 4, 10, true).unwrap();
    let json = attestation.to_json();

    // Verify JSON structure
    assert!(json.starts_with('{'));
    assert!(json.ends_with('}'));
    assert!(json.contains("\"id\":"));
    assert!(json.contains("\"timestamp\":"));
    assert!(json.contains("\"data_hash\":"));
    assert!(json.contains("\"signature\":"));
    assert!(json.contains("\"num_qubits\":4"));
    assert!(json.contains("\"iterations\":10"));
    assert!(json.contains("\"success\":true"));
}

#[test]
fn test_attestation_builder_basic() {
    let attestation = AttestationBuilder::new()
        .with_data(b"builder test")
        .with_qubits(8)
        .with_iterations(50)
        .with_success(true)
        .build();

    assert!(attestation.is_ok());
}

#[test]
fn test_attestation_builder_default_values() {
    let attestation = AttestationBuilder::new().build();

    assert!(attestation.is_ok());
    let att = attestation.unwrap();
    assert_eq!(att.num_qubits, 0);
    assert_eq!(att.iterations, 0);
    assert!(!att.success);
}

#[test]
fn test_attestation_with_empty_data() {
    let attestation = Attestation::new(&[], 4, 10, true);

    assert!(attestation.is_ok());
    let att = attestation.unwrap();
    assert!(!att.data_hash.is_empty());
}

#[test]
fn test_attestation_with_large_data() {
    let large_data: Vec<u8> = (0..10000).map(|i| (i % 256) as u8).collect();
    let attestation = Attestation::new(&large_data, 4, 10, true);

    assert!(attestation.is_ok());
}

#[test]
fn test_attestation_with_failure_status() {
    let data = b"failed operation";
    let attestation = Attestation::new(data, 4, 10, false).unwrap();

    assert!(!attestation.success);
    assert!(attestation.verify().is_ok());
}

#[test]
fn test_multiple_attestations_consistency() {
    let data = b"consistency test";

    // Create multiple attestations for the same data
    let att1 = Attestation::new(data, 4, 10, true).unwrap();
    let att2 = Attestation::new(data, 4, 10, true).unwrap();

    // Same data should produce same hash
    assert_eq!(att1.data_hash, att2.data_hash);

    // But different timestamps should produce different IDs
    // (In practice, these might be the same if created quickly enough)
}
