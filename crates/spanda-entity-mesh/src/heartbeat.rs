//! Mesh heartbeat monitoring and health reporting.
//!
use crate::partition::detect_partitions;
use crate::types::*;

/// Heartbeat evaluation thresholds.
#[derive(Debug, Clone)]
pub struct MeshHeartbeatPolicy {
    pub offline_after_secs: u64,
    pub degraded_latency_ms: u32,
    pub degraded_packet_loss: f64,
    pub trust_degradation_threshold: f64,
}

impl Default for MeshHeartbeatPolicy {
    fn default() -> Self {
        Self {
            offline_after_secs: 30,
            degraded_latency_ms: 200,
            degraded_packet_loss: 0.05,
            trust_degradation_threshold: 0.4,
        }
    }
}

/// Process incoming heartbeats and update node state.
pub fn ingest_heartbeat(mesh: &mut EntityMesh, heartbeat: MeshHeartbeat) {
    if let Some(node) = mesh.nodes.get_mut(&heartbeat.entity_id) {
        node.health = heartbeat.health;
        node.readiness = heartbeat.readiness;
        node.trust_score = heartbeat.trust_score;
        node.last_seen = Some(heartbeat.timestamp.clone());
        node.battery_percent = heartbeat.battery_percent;
        node.reachable = true;
    }
    mesh.updated_at = chrono::Utc::now().to_rfc3339();
}

/// Evaluate mesh health from current node and link state.
pub fn evaluate_mesh_health(mesh: &EntityMesh, policy: &MeshHeartbeatPolicy) -> MeshHealthReport {
    let total = mesh.nodes.len() as u32;
    let reachable = mesh.nodes.values().filter(|n| n.reachable).count() as u32;
    let offline: Vec<String> = mesh
        .nodes
        .values()
        .filter(|n| !n.reachable)
        .map(|n| n.entity_id.clone())
        .collect();

    let degraded_links: Vec<String> = mesh
        .links
        .iter()
        .filter(|l| {
            l.active
                && (l.latency_ms > policy.degraded_latency_ms
                    || l.packet_loss > policy.degraded_packet_loss)
        })
        .map(|l| format!("{}->{}", l.from_entity, l.to_entity))
        .collect();

    let topology_splits = detect_partitions(mesh);
    let active_partitions = mesh.partitions.iter().filter(|p| p.active).count() as u32;
    let topology_components = topology_splits.len() as u32;

    let avg_trust = if mesh.nodes.is_empty() {
        0.0
    } else {
        mesh.nodes.values().map(|n| n.trust_score).sum::<f64>() / mesh.nodes.len() as f64
    };

    let avg_latency = if mesh.links.is_empty() {
        0
    } else {
        mesh.links.iter().map(|l| l.latency_ms).sum::<u32>() / mesh.links.len() as u32
    };

    let mut issues = Vec::new();
    if !offline.is_empty() {
        issues.push(format!("{} node(s) offline", offline.len()));
    }
    if !degraded_links.is_empty() {
        issues.push(format!("{} degraded link(s)", degraded_links.len()));
    }
    if active_partitions > 0 {
        issues.push(format!("{active_partitions} active partition(s)"));
    }
    if topology_components > 0 && active_partitions == 0 {
        issues.push(format!("{topology_components} topology component(s)"));
    }
    if avg_trust < policy.trust_degradation_threshold {
        issues.push("mesh trust degraded".into());
    }
    if mesh.coordinator.is_none() {
        issues.push("no active coordinator".into());
    }

    MeshHealthReport {
        total_nodes: total,
        reachable_nodes: reachable,
        offline_nodes: offline,
        degraded_links,
        active_partitions,
        topology_components,
        coordinator_status: mesh.coordinator.as_ref().map(|c| c.status.clone()),
        average_trust_score: avg_trust,
        average_latency_ms: avg_latency,
        issues,
    }
}

/// Detect coordinator failure from heartbeats.
pub fn coordinator_failed(mesh: &EntityMesh) -> bool {
    mesh.coordinator.as_ref().is_some_and(|c| {
        mesh.nodes
            .get(&c.entity_id)
            .is_none_or(|n| !n.reachable || n.coordinator_status == MeshCoordinatorStatus::Failed)
    })
}
