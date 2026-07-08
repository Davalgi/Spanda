//! Integration with readiness, assurance, diagnosis, and recovery layers.
//!
use crate::heartbeat::evaluate_mesh_health;
use crate::partition::partition_blocks_high_risk;
use crate::types::*;
use serde::{Deserialize, Serialize};

/// Assess readiness impact from mesh status.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MeshReadinessImpact {
    pub entity_id: String,
    pub mission_ready: bool,
    pub blocked_reasons: Vec<String>,
    pub mesh_health_score: f64,
}

/// Compute readiness impact for an entity based on mesh state.
pub fn mesh_readiness_impact(mesh: &EntityMesh, entity_id: &str) -> MeshReadinessImpact {
    let health = evaluate_mesh_health(mesh, &Default::default());
    let mut blocked = Vec::new();
    let mut mission_ready = true;

    if health.active_partitions > 0 {
        blocked.push("partition active may block mission".into());
        mission_ready = false;
    }

    if let Some(node) = mesh.nodes.get(entity_id) {
        if !node.reachable {
            blocked.push("entity unreachable in mesh".into());
            mission_ready = false;
        }
        if node.trust_score < mesh.partition_policy.min_trust_for_relay {
            blocked.push("relay trust too low for safety-critical operation".into());
        }
    }

    if mesh.coordinator.is_none() {
        blocked.push("no trusted route to coordinator".into());
        mission_ready = false;
    } else if health.reachable_nodes > 1 {
        // Backup routes available improves resilience.
    }

    let mesh_health_score = if health.total_nodes == 0 {
        0.0
    } else {
        health.reachable_nodes as f64 / health.total_nodes as f64
    };

    MeshReadinessImpact {
        entity_id: entity_id.into(),
        mission_ready,
        blocked_reasons: blocked,
        mesh_health_score,
    }
}

/// Build assurance evidence bundle for mesh operations.
pub fn build_assurance_evidence(mesh: &EntityMesh) -> MeshAssuranceEvidence {
    MeshAssuranceEvidence {
        topology_snapshot: Some(mesh.topology.clone()),
        route_evidence: mesh
            .route_history
            .iter()
            .flat_map(|r| r.evidence.clone())
            .collect(),
        partition_history: mesh.partitions.clone(),
        election_evidence: mesh
            .coordinator
            .as_ref()
            .and_then(|c| c.leader.as_ref())
            .map(|l| vec![format!("elected:{}:{}", l.entity_id, l.election_method)])
            .unwrap_or_default(),
        message_trust_evidence: vec!["secure_messaging_required".into()],
        delegation_evidence: vec!["takeover_via_recovery_orchestrator".into()],
        sync_evidence: mesh
            .sync_states
            .keys()
            .map(|id| format!("synced:{id}"))
            .collect(),
    }
}

/// Answer mesh-related diagnosis questions.
pub fn diagnose_mesh(mesh: &EntityMesh, question: &str) -> MeshDiagnosisReport {
    let q = question.to_ascii_lowercase();
    let health = evaluate_mesh_health(mesh, &Default::default());

    if q.contains("communication fail") || q.contains("why did communication") {
        return MeshDiagnosisReport {
            question: question.into(),
            answer: if health.offline_nodes.is_empty() {
                "No offline nodes; check route trust or partition state.".into()
            } else {
                format!(
                    "Communication failed: {} node(s) offline.",
                    health.offline_nodes.len()
                )
            },
            affected_entities: health.offline_nodes.clone(),
            route_used: mesh.route_history.last().cloned(),
            partition_active: health.active_partitions > 0,
            evidence: health.issues.clone(),
        };
    }

    if q.contains("coordinator") {
        let answer = match &mesh.coordinator {
            Some(c) => format!("Coordinator is '{}' (status {:?})", c.entity_id, c.status),
            None => "No coordinator elected.".into(),
        };
        return MeshDiagnosisReport {
            question: question.into(),
            answer,
            affected_entities: mesh
                .coordinator
                .as_ref()
                .map(|c| vec![c.entity_id.clone()])
                .unwrap_or_default(),
            route_used: None,
            partition_active: health.active_partitions > 0,
            evidence: health.issues,
        };
    }

    if q.contains("partition") {
        return MeshDiagnosisReport {
            question: question.into(),
            answer: format!(
                "{} active partition(s); {} entities affected.",
                health.active_partitions,
                mesh.partitions
                    .iter()
                    .filter(|p| p.active)
                    .flat_map(|p| p.affected_entities.clone())
                    .count()
            ),
            affected_entities: mesh
                .partitions
                .iter()
                .flat_map(|p| p.affected_entities.clone())
                .collect(),
            route_used: None,
            partition_active: health.active_partitions > 0,
            evidence: health.issues,
        };
    }

    MeshDiagnosisReport {
        question: question.into(),
        answer: "See mesh topology and health for details.".into(),
        affected_entities: Vec::new(),
        route_used: mesh.route_history.last().cloned(),
        partition_active: health.active_partitions > 0,
        evidence: health.issues,
    }
}

/// Recovery actions informed by mesh awareness.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MeshRecoveryAction {
    pub action: String,
    pub entity_id: Option<String>,
    pub evidence: Vec<String>,
}

/// Recommend recovery actions based on mesh state.
pub fn mesh_recovery_actions(mesh: &EntityMesh) -> Vec<MeshRecoveryAction> {
    let mut actions = Vec::new();
    let health = evaluate_mesh_health(mesh, &Default::default());

    for id in &health.offline_nodes {
        actions.push(MeshRecoveryAction {
            action: "reassign_mission".into(),
            entity_id: Some(id.clone()),
            evidence: vec!["entity_offline".into()],
        });
    }

    if health.active_partitions > 0 {
        actions.push(MeshRecoveryAction {
            action: "merge_after_partition".into(),
            entity_id: None,
            evidence: vec![format!("partitions={}", health.active_partitions)],
        });
    }

    if crate::heartbeat::coordinator_failed(mesh) {
        actions.push(MeshRecoveryAction {
            action: "elect_coordinator".into(),
            entity_id: None,
            evidence: vec!["coordinator_failure".into()],
        });
    }

    for link in &health.degraded_links {
        actions.push(MeshRecoveryAction {
            action: "switch_relay".into(),
            entity_id: None,
            evidence: vec![format!("degraded:{link}")],
        });
    }

    actions
}

/// High-risk missions blocked during partition.
pub fn readiness_blocked_by_partition(mesh: &EntityMesh) -> bool {
    partition_blocks_high_risk(mesh)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::discovery::build_entity_mesh;
    use spanda_config::entity::{EntityKind, EntityRecord, EntityRegistry};

    #[test]
    fn readiness_impact_when_no_coordinator() {
        let mesh = build_entity_mesh(&EntityRegistry::default(), "impact");
        let impact = mesh_readiness_impact(&mesh, "any");
        assert!(!impact.mission_ready);
    }

    #[test]
    fn assurance_evidence_includes_topology() {
        let mesh = build_entity_mesh(&EntityRegistry::default(), "assurance");
        let evidence = build_assurance_evidence(&mesh);
        assert!(evidence.topology_snapshot.is_some());
    }

    #[test]
    fn diagnosis_explains_communication_failure() {
        let mut registry = EntityRegistry::default();
        registry.entities.insert(
            "offline-node".into(),
            EntityRecord {
                id: "offline-node".into(),
                entity_type: EntityKind::Robot,
                ..EntityRecord::default()
            },
        );
        let mut mesh = build_entity_mesh(&registry, "diag");
        mesh.nodes.get_mut("offline-node").unwrap().reachable = false;
        let report = diagnose_mesh(&mesh, "why did communication fail?");
        assert!(report.answer.contains("offline") || !report.affected_entities.is_empty());
    }
}
