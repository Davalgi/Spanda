//! Release-hardening security regressions for distributed decisions.

use spanda_decision::{
    clear_persisted_nonce_registry, offline_decision_expired, register_persisted_nonce,
    sign_decision_tree, sign_offline_policy, untrusted_entity_may_not_takeover,
    validate_offline_policy_trust, verify_decision_tree_signature, verify_offline_policy_signature,
    DecisionAuthority, DecisionLayer, DecisionTreeSpec, OfflinePolicySpec,
};
use spanda_runtime::decision_trace::{
    decision_envelope_signing_payload, sign_decision_envelope, v3_decision_payload_with_extras,
    verify_v3_decision_signature, DecisionTraceExtras,
};
use std::sync::Mutex;

static ENV_LOCK: Mutex<()> = Mutex::new(());

fn with_temp_nonce_store<F: FnOnce()>(f: F) {
    let _guard = ENV_LOCK.lock().unwrap();
    let temp = tempfile::tempdir().expect("tempdir");
    std::env::set_var(
        "SPANDA_DECISION_NONCE_CACHE",
        temp.path().join("nonces.json").display().to_string(),
    );
    clear_persisted_nonce_registry();
    f();
    std::env::remove_var("SPANDA_DECISION_NONCE_CACHE");
}

fn sample_offline_policy() -> OfflinePolicySpec {
    OfflinePolicySpec {
        name: "RoverOffline".into(),
        max_duration_minutes: 30,
        allowed_actions: vec!["return_home".into()],
        forbidden_actions: vec!["disable_kill_switch".into()],
        policy_version: "1.0.0".into(),
        signature: None,
        expires_at_ms: None,
    }
}

#[test]
fn distributed_decision_replay_attack_is_rejected() {
    // Replayed decision nonces must be rejected.
    with_temp_nonce_store(|| {
        register_persisted_nonce("release-hardening-nonce").expect("first accept");
        let err = register_persisted_nonce("release-hardening-nonce").expect_err("replay");
        assert!(
            err.to_lowercase().contains("replay"),
            "unexpected error: {err}"
        );
    });
}

#[test]
fn decision_policy_tampering_is_detected() {
    // Tampered decision trees must fail signature verification.
    let spec = DecisionTreeSpec {
        name: "GPSLoss".into(),
        scope: "local".into(),
        layer: spanda_decision::DecisionLayer::LocalEntity,
        version: "1.0.0".into(),
        branches: vec![],
        signature: None,
    };
    let key = "policy-tamper-key";
    let sig = sign_decision_tree(&spec, key);
    let mut signed = spec.clone();
    signed.signature = Some(sig);
    assert!(verify_decision_tree_signature(&signed, key));
    signed.version = "9.9.9".into();
    assert!(!verify_decision_tree_signature(&signed, key));
}

#[test]
fn offline_decision_abuse_expires_past_policy_max() {
    // Offline decisions past max duration must expire.
    let policy = sample_offline_policy();
    assert!(offline_decision_expired(&policy, 30).is_ok());
    let err = offline_decision_expired(&policy, 31).expect_err("expired");
    assert!(err.to_lowercase().contains("expire") || err.contains("30"));
}

#[test]
fn offline_policy_signature_rejects_fake_coordinator() {
    // Policies signed by an attacker key must not verify under the trust key.
    let mut policy = sample_offline_policy();
    let attacker = "attacker-coordinator-key";
    let official = "official-coordinator-key";
    policy.signature = Some(sign_offline_policy(&policy, attacker));
    assert!(verify_offline_policy_signature(&policy, attacker));
    assert!(!verify_offline_policy_signature(&policy, official));

    std::env::set_var("SPANDA_DECISION_REQUIRE_SIGNED_OFFLINE_POLICY", "1");
    std::env::set_var("SPANDA_DECISION_POLICY_TRUST_KEY", official);
    let trust = validate_offline_policy_trust(&policy);
    std::env::remove_var("SPANDA_DECISION_REQUIRE_SIGNED_OFFLINE_POLICY");
    std::env::remove_var("SPANDA_DECISION_POLICY_TRUST_KEY");
    assert!(trust.is_err(), "fake coordinator policy must fail trust");
}

#[test]
fn decision_trace_signature_rejects_tampered_envelope() {
    // Tampered v3 decision envelopes must fail verification.
    let key = "trace-sign-key";
    std::env::set_var("SPANDA_DECISION_POLICY_SIGNING_KEY", key);
    std::env::set_var("SPANDA_DECISION_POLICY_TRUST_KEY", key);
    let payload = v3_decision_payload_with_extras(
        "d-release-1",
        Some("mission"),
        "emergency_stop",
        "obstacle",
        "reflex",
        "Rover001",
        serde_json::json!({}),
        DecisionTraceExtras {
            sim_time_ms: Some(100.0),
            ..Default::default()
        },
    );
    verify_v3_decision_signature(&payload).expect("valid signature");
    // Tamper the embedded signing payload.
    let mut tampered_payload = payload.clone();
    if let Some(env) = tampered_payload
        .get_mut("security_envelope")
        .and_then(|value| value.as_object_mut())
    {
        env.insert(
            "signing_payload".into(),
            serde_json::json!("tampered-payload"),
        );
    }
    assert!(verify_v3_decision_signature(&tampered_payload).is_err());
    // Tamper an outer field while leaving the embedded payload intact.
    let mut tampered_outer = payload.clone();
    if let Some(obj) = tampered_outer.as_object_mut() {
        obj.insert("decision".into(), serde_json::json!("disable_kill_switch"));
    }
    let err = verify_v3_decision_signature(&tampered_outer).expect_err("outer tamper");
    assert!(
        err.to_lowercase().contains("tamper") || err.to_lowercase().contains("match"),
        "unexpected error: {err}"
    );
    std::env::remove_var("SPANDA_DECISION_POLICY_SIGNING_KEY");
    std::env::remove_var("SPANDA_DECISION_POLICY_TRUST_KEY");
}

#[test]
fn entity_permission_bypass_for_takeover_is_rejected() {
    // Untrusted entities must not perform fleet takeover.
    let authority = DecisionAuthority {
        entity_id: "RogueBot".into(),
        local_actions: vec!["pause_mission".into()],
        requires_central_approval: vec!["fleet_takeover".into()],
        layer: DecisionLayer::LocalEntity,
    };
    let err = untrusted_entity_may_not_takeover(&authority, "fleet_takeover", &["TrustedLeader"])
        .expect_err("untrusted takeover blocked");
    assert!(
        err.to_lowercase().contains("untrusted") || err.to_lowercase().contains("takeover"),
        "unexpected error: {err}"
    );
}

#[test]
fn envelope_signing_payload_does_not_embed_secrets() {
    // Signing payloads must not include key material.
    let payload = decision_envelope_signing_payload(
        "d-1",
        "Rover",
        "local_entity",
        "pause",
        "gps",
        "1.0.0",
        &Some("abc".into()),
        "n-1",
        100.0,
    );
    let sig = sign_decision_envelope(&payload, "super-secret-key-material");
    assert!(!sig.is_empty());
    assert!(!payload.contains("super-secret-key-material"));
}
