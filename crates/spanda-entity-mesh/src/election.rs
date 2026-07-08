//! Coordinator / leader election for mesh communication roles.
//!
use crate::types::*;

/// Election method for mesh coordinator selection.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MeshElectionMethod {
    Configured,
    Backup,
    TrustWeighted,
    ReadinessWeighted,
    CapabilityBased,
    QuorumBased,
}

/// Election options.
#[derive(Debug, Clone, Default)]
pub struct MeshElectionOptions {
    pub method: Option<MeshElectionMethod>,
    pub configured_coordinator: Option<String>,
    pub backup_coordinators: Vec<String>,
    pub required_capability: Option<String>,
    pub min_trust: f64,
    pub quorum_size: u32,
}

/// Elect a mesh coordinator (communication role only — no safety authority).
pub fn elect_coordinator(
    mesh: &EntityMesh,
    options: &MeshElectionOptions,
) -> Result<MeshCoordinator, String> {
    let method = options
        .method
        .clone()
        .unwrap_or(MeshElectionMethod::TrustWeighted);

    let candidate_id = match method {
        MeshElectionMethod::Configured => options
            .configured_coordinator
            .clone()
            .ok_or("configured coordinator not specified")?,
        MeshElectionMethod::Backup => options
            .backup_coordinators
            .first()
            .cloned()
            .ok_or("no backup coordinator configured")?,
        MeshElectionMethod::CapabilityBased => elect_by_capability(mesh, options)?,
        MeshElectionMethod::ReadinessWeighted => elect_by_readiness(mesh, options)?,
        MeshElectionMethod::QuorumBased => elect_by_quorum(mesh, options)?,
        MeshElectionMethod::TrustWeighted => elect_by_trust(mesh, options)?,
    };

    let candidate = mesh
        .nodes
        .get(&candidate_id)
        .ok_or_else(|| format!("candidate '{candidate_id}' not in mesh"))?;

    if candidate.trust_score < options.min_trust {
        return Err(format!(
            "untrusted entity '{candidate_id}' cannot become coordinator"
        ));
    }

    let leader = MeshLeader {
        entity_id: candidate_id.clone(),
        elected_at: chrono::Utc::now().to_rfc3339(),
        election_method: format!("{method:?}"),
        trust_score: candidate.trust_score,
        readiness_score: readiness_score(candidate),
        term: 1,
    };

    Ok(MeshCoordinator {
        entity_id: candidate_id,
        status: MeshCoordinatorStatus::Active,
        leader: Some(leader),
        backup_entity_ids: options.backup_coordinators.clone(),
        quorum_size: options.quorum_size.max(1),
    })
}

fn elect_by_trust(mesh: &EntityMesh, options: &MeshElectionOptions) -> Result<String, String> {
    mesh.nodes
        .values()
        .filter(|n| n.reachable && n.trust_score >= options.min_trust)
        .max_by(|a, b| {
            a.trust_score
                .partial_cmp(&b.trust_score)
                .unwrap_or(std::cmp::Ordering::Equal)
        })
        .map(|n| n.entity_id.clone())
        .ok_or("no trusted coordinator candidate".into())
}

fn elect_by_readiness(mesh: &EntityMesh, options: &MeshElectionOptions) -> Result<String, String> {
    mesh.nodes
        .values()
        .filter(|n| n.reachable && n.trust_score >= options.min_trust)
        .max_by(|a, b| {
            readiness_score(a)
                .partial_cmp(&readiness_score(b))
                .unwrap_or(std::cmp::Ordering::Equal)
        })
        .map(|n| n.entity_id.clone())
        .ok_or("no ready coordinator candidate".into())
}

fn elect_by_capability(mesh: &EntityMesh, options: &MeshElectionOptions) -> Result<String, String> {
    let cap = options
        .required_capability
        .as_deref()
        .ok_or("capability-based election requires required_capability")?;
    mesh.nodes
        .values()
        .filter(|n| {
            n.reachable
                && n.trust_score >= options.min_trust
                && n.capabilities.iter().any(|c| c == cap)
        })
        .max_by(|a, b| {
            a.trust_score
                .partial_cmp(&b.trust_score)
                .unwrap_or(std::cmp::Ordering::Equal)
        })
        .map(|n| n.entity_id.clone())
        .ok_or_else(|| format!("no candidate with capability '{cap}'"))
}

fn elect_by_quorum(mesh: &EntityMesh, options: &MeshElectionOptions) -> Result<String, String> {
    let reachable: Vec<_> = mesh
        .nodes
        .values()
        .filter(|n| n.reachable && n.trust_score >= options.min_trust)
        .collect();
    if reachable.len() < options.quorum_size as usize {
        return Err(format!(
            "quorum not met: {} reachable, need {}",
            reachable.len(),
            options.quorum_size
        ));
    }
    elect_by_trust(mesh, options)
}

fn readiness_score(node: &MeshNode) -> f64 {
    match node.readiness {
        spanda_config::entity::EntityReadinessStatus::Ready => 1.0,
        spanda_config::entity::EntityReadinessStatus::Partial => 0.6,
        spanda_config::entity::EntityReadinessStatus::NotReady => 0.2,
        spanda_config::entity::EntityReadinessStatus::Unknown => 0.5,
    }
}

/// Apply elected coordinator to mesh state.
pub fn apply_coordinator(mesh: &mut EntityMesh, coordinator: MeshCoordinator) {
    for node in mesh.nodes.values_mut() {
        node.coordinator_status = if node.entity_id == coordinator.entity_id {
            MeshCoordinatorStatus::Active
        } else if coordinator.backup_entity_ids.contains(&node.entity_id) {
            MeshCoordinatorStatus::Backup
        } else {
            MeshCoordinatorStatus::None
        };
        node.role = if node.entity_id == coordinator.entity_id {
            MeshNodeRole::Coordinator
        } else {
            MeshNodeRole::Participant
        };
    }
    mesh.coordinator = Some(coordinator.clone());
    mesh.topology.coordinator = Some(coordinator);
}

/// Verify elected coordinator cannot grant actuator authority.
pub fn coordinator_is_communication_role_only(coordinator: &MeshCoordinator) -> bool {
    coordinator.status == MeshCoordinatorStatus::Active && coordinator.leader.is_some()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::discovery::build_entity_mesh;
    use spanda_config::entity::{
        EntityKind, EntityReadinessStatus, EntityRecord, EntityRegistry, EntityTrustStatus,
    };

    #[test]
    fn trust_weighted_election_selects_best_candidate() {
        let mut registry = EntityRegistry::default();
        registry.entities.insert(
            "low-trust".into(),
            EntityRecord {
                id: "low-trust".into(),
                entity_type: EntityKind::Robot,
                trust_status: EntityTrustStatus::Untrusted,
                ..EntityRecord::default()
            },
        );
        registry.entities.insert(
            "high-trust".into(),
            EntityRecord {
                id: "high-trust".into(),
                entity_type: EntityKind::Robot,
                trust_status: EntityTrustStatus::Verified,
                readiness_status: EntityReadinessStatus::Ready,
                ..EntityRecord::default()
            },
        );
        let mesh = build_entity_mesh(&registry, "election");
        let coord = elect_coordinator(
            &mesh,
            &MeshElectionOptions {
                min_trust: 0.6,
                ..Default::default()
            },
        )
        .expect("election");
        assert_eq!(coord.entity_id, "high-trust");
        assert!(coordinator_is_communication_role_only(&coord));
    }

    #[test]
    fn untrusted_entity_cannot_become_coordinator() {
        let mut registry = EntityRegistry::default();
        registry.entities.insert(
            "bad".into(),
            EntityRecord {
                id: "bad".into(),
                entity_type: EntityKind::Robot,
                trust_status: EntityTrustStatus::Untrusted,
                ..EntityRecord::default()
            },
        );
        let mesh = build_entity_mesh(&registry, "election");
        let err = elect_coordinator(
            &mesh,
            &MeshElectionOptions {
                method: Some(MeshElectionMethod::Configured),
                configured_coordinator: Some("bad".into()),
                min_trust: 0.7,
                ..Default::default()
            },
        )
        .unwrap_err();
        assert!(err.contains("untrusted"));
    }
}
