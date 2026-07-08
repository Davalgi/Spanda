//! Architecture compatibility regression tests for Entity Mesh.

use spanda_config::entity::{
    EntityKind, EntityReadinessStatus, EntityRecord, EntityRegistry, EntityTrustStatus,
};
use spanda_decision::{clear_nonce_registry, reflex_may_act_without_central, DecisionLayer};
use spanda_entity_mesh::{
    build_entity_mesh, build_mesh_message, compute_route, coordinator_is_communication_role_only,
    delegation_requires_recovery_orchestrator, elect_coordinator, partition_blocks_high_risk,
    plan_delegation, sign_mesh_message, validate_mesh_message, MeshDelegationRequest,
    MeshElectionOptions, MeshMessagePriority, MeshRouteOptions, MeshRoutingMode,
    MeshSecurityVerdict,
};
use spanda_security::RobotIdentity;

#[test]
fn secure_messaging_still_wraps_mesh_messages() {
    let identity = RobotIdentity::new("robot-a", "regression-key");
    let mut msg = build_mesh_message("robot-a", Some("robot-b"), MeshMessagePriority::Normal);
    sign_mesh_message(&mut msg, &identity);
    assert!(msg.signed_envelope.is_some());
    assert!(msg.signature.is_some());
}

#[test]
fn untrusted_mesh_relay_rejected_for_safety_critical() {
    let mut registry = EntityRegistry::default();
    for (id, trust) in [
        ("src", EntityTrustStatus::Verified),
        ("relay", EntityTrustStatus::Untrusted),
        ("dst", EntityTrustStatus::Trusted),
    ] {
        registry.entities.insert(
            id.into(),
            EntityRecord {
                id: id.into(),
                entity_type: EntityKind::Robot,
                trust_status: trust,
                readiness_status: EntityReadinessStatus::Ready,
                ..EntityRecord::default()
            },
        );
    }
    let mut mesh = build_entity_mesh(&registry, "regression");
    mesh.nodes.get_mut("src").unwrap().neighbors = vec![spanda_entity_mesh::MeshNeighbor {
        entity_id: "relay".into(),
        node_id: "node-relay".into(),
        transport: spanda_entity_mesh::MeshTransport::LocalRuntime,
        reachable: true,
        latency_ms: Some(1),
        packet_loss: Some(0.0),
        last_seen: None,
    }];
    mesh.nodes.get_mut("relay").unwrap().neighbors = vec![spanda_entity_mesh::MeshNeighbor {
        entity_id: "dst".into(),
        node_id: "node-dst".into(),
        transport: spanda_entity_mesh::MeshTransport::LocalRuntime,
        reachable: true,
        latency_ms: Some(1),
        packet_loss: Some(0.0),
        last_seen: None,
    }];
    spanda_entity_mesh::rebuild_topology(&mut mesh);
    let err = compute_route(
        &mesh,
        "src",
        "dst",
        &MeshRouteOptions {
            mission_priority: MeshMessagePriority::SafetyCritical,
            min_trust: 0.6,
            allow_untrusted_relay: false,
            mode: Some(MeshRoutingMode::TrustWeighted),
            ..Default::default()
        },
    )
    .unwrap_err();
    assert!(err.contains("trust") || err.contains("untrusted"));
}

#[test]
fn takeover_still_goes_through_recovery_continuity() {
    let registry = EntityRegistry::default();
    let mesh = build_entity_mesh(&registry, "regression");
    let result = plan_delegation(
        &mesh,
        &registry,
        &MeshDelegationRequest {
            offline_entity_id: "offline".into(),
            required_capabilities: vec!["thermal_camera".into()],
            min_trust: 0.5,
            min_readiness: 0.5,
            source_entity: None,
        },
    );
    assert!(delegation_requires_recovery_orchestrator(&result));
    assert!(result.recovery_orchestrator_required);
}

#[test]
fn elected_coordinator_without_authority_cannot_take_control() {
    let registry = EntityRegistry::default();
    let mesh = build_entity_mesh(&registry, "regression");
    let coord = elect_coordinator(&mesh, &MeshElectionOptions::default()).ok();
    if let Some(c) = coord {
        assert!(coordinator_is_communication_role_only(&c));
    }
}

#[test]
fn partition_mode_cannot_start_high_risk_missions() {
    let mut mesh = build_entity_mesh(&EntityRegistry::default(), "regression");
    mesh.partitions.push(spanda_entity_mesh::MeshPartition {
        partition_id: "p1".into(),
        detected_at: chrono::Utc::now().to_rfc3339(),
        resolved_at: None,
        clusters: vec![],
        affected_entities: vec![],
        active: true,
    });
    assert!(partition_blocks_high_risk(&mesh));
}

#[test]
fn reflex_safety_still_works_without_mesh() {
    clear_nonce_registry();
    assert!(reflex_may_act_without_central(
        "emergency_stop",
        DecisionLayer::Reflex
    ));
}

#[test]
fn replayed_mesh_message_rejected_by_security_layer() {
    let mut mesh = build_entity_mesh(&EntityRegistry::default(), "regression");
    let msg = build_mesh_message("a", None, MeshMessagePriority::Normal);
    let nonce = msg.nonce.clone();
    assert!(matches!(
        validate_mesh_message(&mut mesh, &msg, chrono::Utc::now().timestamp()),
        MeshSecurityVerdict::Accepted
    ));
    let replay = spanda_entity_mesh::MeshMessage {
        nonce,
        ..build_mesh_message("a", None, MeshMessagePriority::Normal)
    };
    assert!(matches!(
        validate_mesh_message(&mut mesh, &replay, chrono::Utc::now().timestamp()),
        MeshSecurityVerdict::Rejected(_)
    ));
}
