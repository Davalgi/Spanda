//! Mesh state synchronization (no secrets).
//!
use crate::types::*;
use spanda_config::entity::EntityRecord;

/// Fields synchronized across mesh partitions.
#[derive(Debug, Clone)]
pub struct MeshSyncOptions {
    pub sync_health: bool,
    pub sync_readiness: bool,
    pub sync_trust: bool,
    pub sync_mission_progress: bool,
    pub sync_decision_traces: bool,
    pub sync_recovery_events: bool,
    pub sync_audit_events: bool,
    pub sync_telemetry_summary: bool,
    pub sync_config_version: bool,
}

impl Default for MeshSyncOptions {
    fn default() -> Self {
        Self {
            sync_health: true,
            sync_readiness: true,
            sync_trust: true,
            sync_mission_progress: true,
            sync_decision_traces: true,
            sync_recovery_events: true,
            sync_audit_events: true,
            sync_telemetry_summary: true,
            sync_config_version: true,
        }
    }
}

/// Build sync state from entity record (excludes secrets).
pub fn entity_to_sync_state(entity: &EntityRecord) -> MeshSyncState {
    MeshSyncState {
        entity_id: entity.id.clone(),
        health: entity.health_status,
        readiness: entity.readiness_status,
        trust_score: match entity.trust_status {
            spanda_config::entity::EntityTrustStatus::Verified => 0.98,
            spanda_config::entity::EntityTrustStatus::Trusted => 0.95,
            spanda_config::entity::EntityTrustStatus::Untrusted => 0.2,
            spanda_config::entity::EntityTrustStatus::Compromised => 0.0,
            spanda_config::entity::EntityTrustStatus::Unknown => 0.5,
        },
        mission_progress: None,
        decision_trace_count: 0,
        recovery_event_count: 0,
        audit_event_count: entity.audit.as_ref().map(|_| 1).unwrap_or(0),
        config_version: entity.version.clone(),
        synced_at: chrono::Utc::now().to_rfc3339(),
    }
}

/// Apply sync states to mesh (does not sync secrets).
pub fn apply_sync_states(mesh: &mut EntityMesh, states: &[MeshSyncState]) {
    for state in states {
        mesh.sync_states
            .insert(state.entity_id.clone(), state.clone());
        if let Some(node) = mesh.nodes.get_mut(&state.entity_id) {
            node.health = state.health;
            node.readiness = state.readiness;
            node.trust_score = state.trust_score;
        }
    }
    mesh.updated_at = chrono::Utc::now().to_rfc3339();
}

/// Collect sync payload for reconnection merge.
pub fn collect_sync_payload(mesh: &EntityMesh, options: &MeshSyncOptions) -> Vec<MeshSyncState> {
    mesh.sync_states
        .values()
        .filter(|_| {
            options.sync_health
                || options.sync_readiness
                || options.sync_trust
                || options.sync_mission_progress
        })
        .cloned()
        .collect()
}

/// Verify sync payload contains no secret fields.
pub fn sync_payload_is_secret_free(states: &[MeshSyncState]) -> bool {
    let blob = serde_json::to_string(states)
        .unwrap_or_default()
        .to_lowercase();
    !blob.contains("password")
        && !blob.contains("private_key")
        && !blob.contains("secret")
        && !blob.contains("api_key")
}
