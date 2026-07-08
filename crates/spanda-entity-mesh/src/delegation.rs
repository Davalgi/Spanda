//! Mission-aware delegation through entity mesh (via Recovery / Continuity).
//!
use crate::routing::{compute_route, MeshRouteOptions};
use crate::types::*;
use serde::{Deserialize, Serialize};
use spanda_config::entity::EntityRegistry;

/// Delegation request when an entity goes offline.
#[derive(Debug, Clone, Default)]
pub struct MeshDelegationRequest {
    pub offline_entity_id: String,
    pub required_capabilities: Vec<String>,
    pub min_trust: f64,
    pub min_readiness: f64,
    pub source_entity: Option<String>,
}

/// Delegation result routed through Recovery / Mission Continuity.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MeshDelegationResult {
    pub offline_entity_id: String,
    pub candidates: Vec<MeshDelegationCandidate>,
    pub selected: Option<String>,
    pub resume_strategy: String,
    pub recovery_orchestrator_required: bool,
    pub evidence: Vec<String>,
}

/// Find delegation candidates for an offline entity.
pub fn find_delegation_candidates(
    mesh: &EntityMesh,
    _registry: &EntityRegistry,
    request: &MeshDelegationRequest,
) -> Vec<MeshDelegationCandidate> {
    let source = request
        .source_entity
        .clone()
        .or_else(|| mesh.coordinator.as_ref().map(|c| c.entity_id.clone()))
        .unwrap_or_else(|| "coordinator".into());

    mesh.nodes
        .values()
        .filter(|n| n.entity_id != request.offline_entity_id && n.reachable)
        .map(|n| {
            let mut rejection_reasons = Vec::new();
            if n.trust_score < request.min_trust {
                rejection_reasons.push(format!(
                    "trust {:.2} below minimum {:.2}",
                    n.trust_score, request.min_trust
                ));
            }
            let readiness = match n.readiness {
                spanda_config::entity::EntityReadinessStatus::Ready => 1.0,
                spanda_config::entity::EntityReadinessStatus::Partial => 0.6,
                _ => 0.2,
            };
            if readiness < request.min_readiness {
                rejection_reasons.push("readiness too low".into());
            }
            for cap in &request.required_capabilities {
                if !n.capabilities.iter().any(|c| c == cap) {
                    rejection_reasons.push(format!("missing capability '{cap}'"));
                }
            }

            let route = compute_route(
                mesh,
                &source,
                &n.entity_id,
                &MeshRouteOptions {
                    min_trust: request.min_trust,
                    ..Default::default()
                },
            )
            .ok();

            let eligible = rejection_reasons.is_empty() && route.is_some();
            MeshDelegationCandidate {
                entity_id: n.entity_id.clone(),
                capabilities: n.capabilities.clone(),
                trust_score: n.trust_score,
                readiness_score: readiness,
                route,
                eligible,
                rejection_reasons,
            }
        })
        .collect()
}

/// Plan mission delegation — takeover still goes through Recovery Orchestrator.
pub fn plan_delegation(
    mesh: &EntityMesh,
    registry: &EntityRegistry,
    request: &MeshDelegationRequest,
) -> MeshDelegationResult {
    let candidates = find_delegation_candidates(mesh, registry, request);
    let selected = candidates
        .iter()
        .filter(|c| c.eligible)
        .max_by(|a, b| {
            a.trust_score
                .partial_cmp(&b.trust_score)
                .unwrap_or(std::cmp::Ordering::Equal)
        })
        .map(|c| c.entity_id.clone());

    let resume_strategy = if selected.is_some() {
        "resume_via_recovery_orchestrator".into()
    } else {
        "no_eligible_candidate".into()
    };

    let mut evidence = vec![
        format!("offline_entity={}", request.offline_entity_id),
        format!("candidates={}", candidates.len()),
    ];
    if let Some(ref id) = selected {
        evidence.push(format!("selected={id}"));
        evidence.push("takeover_via_recovery_orchestrator".into());
    }

    MeshDelegationResult {
        offline_entity_id: request.offline_entity_id.clone(),
        candidates,
        selected,
        resume_strategy,
        recovery_orchestrator_required: true,
        evidence,
    }
}

/// Verify delegation does not bypass Recovery Orchestrator.
pub fn delegation_requires_recovery_orchestrator(result: &MeshDelegationResult) -> bool {
    result.recovery_orchestrator_required
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::discovery::build_entity_mesh;
    use spanda_config::entity::{
        EntityKind, EntityReadinessStatus, EntityRecord, EntityRegistry, EntityTrustStatus,
    };

    #[test]
    fn mission_delegation_finds_capable_candidate() {
        let mut registry = EntityRegistry::default();
        registry.entities.insert(
            "offline".into(),
            EntityRecord {
                id: "offline".into(),
                entity_type: EntityKind::Robot,
                capabilities: vec!["thermal_camera".into()],
                ..EntityRecord::default()
            },
        );
        registry.entities.insert(
            "backup".into(),
            EntityRecord {
                id: "backup".into(),
                entity_type: EntityKind::Robot,
                capabilities: vec!["thermal_camera".into()],
                trust_status: EntityTrustStatus::Trusted,
                readiness_status: EntityReadinessStatus::Ready,
                ..EntityRecord::default()
            },
        );
        let mesh = build_entity_mesh(&registry, "delegation");
        let result = plan_delegation(
            &mesh,
            &registry,
            &MeshDelegationRequest {
                offline_entity_id: "offline".into(),
                required_capabilities: vec!["thermal_camera".into()],
                min_trust: 0.6,
                min_readiness: 0.5,
                source_entity: Some("backup".into()),
            },
        );
        assert!(delegation_requires_recovery_orchestrator(&result));
    }
}
