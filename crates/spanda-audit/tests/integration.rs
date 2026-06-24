//! Integration tests for spanda-audit.

use spanda_audit::{
    sha256, sign, verify_signature, AuditBackend, AuditRuntime, DeviceIdentity, JsonAuditBackend,
    LedgerBackend, LocalAuditBackend, MockLedgerBackend, RecordId,
};

#[test]
fn audit_record_creation_and_hashing() {
    // Description:
    //     Audit record creation and hashing.
    //
    // Inputs:
    //     None.
    //
    // Outputs:
    //     None.
    //
    // Example:

    //     let result = spanda_audit::integration::audit_record_creation_and_hashing();

    let mut rt = AuditRuntime::new("MissionAudit", vec!["robot.pose".into()]);
    let id = rt
        .record_event("pose_update", r#"{"x":1.0,"y":0.0}"#)
        .unwrap();
    assert_eq!(rt.record_count(), 1);
    assert!(rt.verify_record(&id).unwrap());
}

#[test]
fn signature_verification_with_identity() {
    // Description:
    //     Signature verification with identity.
    //
    // Inputs:
    //     None.
    //
    // Outputs:
    //     None.
    //
    // Example:

    //     let result = spanda_audit::integration::signature_verification_with_identity();

    let identity = DeviceIdentity::new("rover-001", "pub-key-abc");
    let mut rt = AuditRuntime::new("MissionAudit", vec![]).with_identity(identity);
    let id = rt.record_event("test_event", "payload").unwrap();
    assert!(rt.verify_record(&id).unwrap());
}

#[test]
fn provenance_record_creation() {
    // Description:
    //     Provenance record creation.
    //
    // Inputs:
    //     None.
    //
    // Outputs:
    //     None.
    //
    // Example:

    //     let result = spanda_audit::integration::provenance_record_creation();

    let identity = DeviceIdentity::new("rover-001", "device-key");
    let mut rt = AuditRuntime::new("MissionAudit", vec![])
        .with_identity(identity)
        .with_provenance("sha256", "rover-001");
    let id = rt.record_event("mission_start", "{}").unwrap();
    let prov = rt.create_provenance("MissionRecord", &id).unwrap();
    assert_eq!(prov.name, "MissionRecord");
    assert!(rt.verify_provenance_signature(&prov));
}

#[test]
fn mock_ledger_anchoring() {
    // Description:
    //     Mock ledger anchoring.
    //
    // Inputs:
    //     None.
    //
    // Outputs:
    //     None.
    //
    // Example:

    //     let result = spanda_audit::integration::mock_ledger_anchoring();

    let mut ledger = MockLedgerBackend::new();
    let hash = sha256("mission-root");
    let tx = ledger.anchor_hash(&hash).unwrap();
    assert!(ledger.verify_anchor(&hash).unwrap());
    assert_eq!(tx.0, "mock-tx-1");
}

#[test]
fn json_audit_export() {
    // Description:
    //     Json audit export.
    //
    // Inputs:
    //     None.
    //
    // Outputs:
    //     None.
    //
    // Example:

    //     let result = spanda_audit::integration::json_audit_export();

    let mut backend = JsonAuditBackend::new();
    let record = spanda_audit::AuditRecord {
        id: RecordId("r1".into()),
        timestamp: chrono::Utc::now(),
        event_type: "test".into(),
        payload: "{}".into(),
        hash: sha256("test"),
        signature: None,
        signer_id: None,
        signing_key: None,
        previous_hash: None,
    };
    backend.append(record).unwrap();
    let json = backend.export_json().unwrap();
    assert!(json.contains("\"records\""));
}

#[test]
fn local_backend_chain_integrity() {
    // Description:
    //     Local backend chain integrity.
    //
    // Inputs:
    //     None.
    //
    // Outputs:
    //     None.
    //
    // Example:

    //     let result = spanda_audit::integration::local_backend_chain_integrity();

    let mut rt = AuditRuntime::new("Test", vec![]);
    let id1 = rt.record_event("e1", "a").unwrap();
    let id2 = rt.record_event("e2", "b").unwrap();
    assert!(rt.verify_record(&id1).unwrap());
    assert!(rt.verify_record(&id2).unwrap());
    let _ = LocalAuditBackend::new();
}

#[test]
fn crypto_sign_verify() {
    // Description:
    //     Crypto sign verify.
    //
    // Inputs:
    //     None.
    //
    // Outputs:
    //     None.
    //
    // Example:

    //     let result = spanda_audit::integration::crypto_sign_verify();

    let sig = sign("data", "key");
    assert!(verify_signature("data", &sig, "key"));
}
