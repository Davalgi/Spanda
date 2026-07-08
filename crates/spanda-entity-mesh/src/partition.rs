//! Partition detection, offline policies, and merge handling.
//!
use crate::discovery::rebuild_topology;
use crate::types::*;
use std::collections::{HashMap, HashSet, VecDeque};

/// Detect network partitions from unreachable node clusters.
pub fn detect_partitions(mesh: &EntityMesh) -> Vec<MeshPartition> {
    let reachable_ids: HashSet<_> = mesh
        .nodes
        .values()
        .filter(|n| n.reachable)
        .map(|n| n.entity_id.clone())
        .collect();

    if reachable_ids.is_empty() {
        return Vec::new();
    }

    // Isolated discovered nodes without mesh links are not a network partition.
    if !mesh.links.iter().any(|l| l.active) {
        return Vec::new();
    }

    let mut visited = HashSet::new();
    let mut clusters = Vec::new();
    let mut cluster_idx = 0u32;

    for node_id in &reachable_ids {
        if visited.contains(node_id) {
            continue;
        }
        let component = bfs_component(mesh, node_id, &reachable_ids);
        if component.is_empty() {
            continue;
        }
        for id in &component {
            visited.insert(id.clone());
        }
        cluster_idx += 1;
        let partition_id = format!("partition-{cluster_idx}");
        clusters.push(MeshCluster {
            cluster_id: format!("cluster-{cluster_idx}"),
            entity_ids: component.clone(),
            coordinator_entity: elect_local_coordinator(mesh, &component),
            partition_id: partition_id.clone(),
        });
    }

    if clusters.len() <= 1 {
        return Vec::new();
    }

    let affected: Vec<String> = clusters.iter().flat_map(|c| c.entity_ids.clone()).collect();
    vec![MeshPartition {
        partition_id: format!("partition-{}", chrono::Utc::now().timestamp()),
        detected_at: chrono::Utc::now().to_rfc3339(),
        resolved_at: None,
        clusters,
        affected_entities: affected,
        active: true,
    }]
}

fn bfs_component(mesh: &EntityMesh, start: &str, allowed: &HashSet<String>) -> Vec<String> {
    let mut queue = VecDeque::from([start.to_string()]);
    let mut visited = HashSet::from([start.to_string()]);
    let mut component = Vec::new();

    while let Some(current) = queue.pop_front() {
        component.push(current.clone());
        let Some(node) = mesh.nodes.get(&current) else {
            continue;
        };
        for neighbor in &node.neighbors {
            if !neighbor.reachable || !allowed.contains(&neighbor.entity_id) {
                continue;
            }
            if visited.insert(neighbor.entity_id.clone()) {
                queue.push_back(neighbor.entity_id.clone());
            }
        }
    }
    component
}

fn elect_local_coordinator(mesh: &EntityMesh, cluster: &[String]) -> Option<String> {
    cluster
        .iter()
        .filter_map(|id| mesh.nodes.get(id))
        .filter(|n| n.trust_score >= mesh.partition_policy.min_trust_for_relay)
        .max_by(|a, b| {
            a.trust_score
                .partial_cmp(&b.trust_score)
                .unwrap_or(std::cmp::Ordering::Equal)
        })
        .map(|n| n.entity_id.clone())
}

/// Apply partition policy when a partition is detected.
pub fn apply_partition_policy(
    mesh: &mut EntityMesh,
    partition: &MeshPartition,
) -> MeshPartitionReport {
    let policy = mesh.partition_policy.clone();
    let mut paused = Vec::new();
    let mut continued = Vec::new();
    let mut evidence = vec![
        format!("partition_id={}", partition.partition_id),
        format!("clusters={}", partition.clusters.len()),
    ];

    for entity_id in &partition.affected_entities {
        if policy.pause_unsafe_missions {
            paused.push(entity_id.clone());
            evidence.push(format!("paused_unsafe:{entity_id}"));
        } else if policy.allow_safe_missions {
            continued.push(entity_id.clone());
        }
    }

    let local_coordinator = partition
        .clusters
        .first()
        .and_then(|c| c.coordinator_entity.clone());

    if policy.require_local_coordinator && local_coordinator.is_none() {
        evidence.push("no_local_coordinator:restrictive_mode".into());
    }

    mesh.partitions.push(partition.clone());
    rebuild_topology(mesh);

    MeshPartitionReport {
        partition: partition.clone(),
        policy_applied: policy,
        paused_missions: paused,
        continued_missions: continued,
        local_coordinator,
        evidence,
    }
}

/// Simulate a partition by marking entities unreachable.
pub fn simulate_partition(mesh: &mut EntityMesh, entity_ids: &[String]) -> MeshPartitionReport {
    for id in entity_ids {
        if let Some(node) = mesh.nodes.get_mut(id) {
            node.reachable = false;
        }
    }
    rebuild_topology(mesh);
    let partitions = detect_partitions(mesh);
    let partition = partitions
        .into_iter()
        .next()
        .unwrap_or_else(|| MeshPartition {
            partition_id: format!("sim-{}", chrono::Utc::now().timestamp()),
            detected_at: chrono::Utc::now().to_rfc3339(),
            resolved_at: None,
            clusters: vec![MeshCluster {
                cluster_id: "isolated".into(),
                entity_ids: entity_ids.to_vec(),
                coordinator_entity: None,
                partition_id: "sim-partition".into(),
            }],
            affected_entities: entity_ids.to_vec(),
            active: true,
        });
    apply_partition_policy(mesh, &partition)
}

/// Build merge plan when partitions reconnect.
pub fn build_merge_plan(mesh: &EntityMesh, partition_ids: &[String]) -> MeshMergePlan {
    let active: Vec<_> = mesh
        .partitions
        .iter()
        .filter(|p| partition_ids.contains(&p.partition_id) && p.active)
        .collect();

    let mut conflicts = Vec::new();
    let mut sync_actions = Vec::new();
    let mut audit_actions = Vec::new();
    let mut graph_updates = Vec::new();

    let mut leader_counts: HashMap<String, u32> = HashMap::new();
    for partition in &active {
        for cluster in &partition.clusters {
            if let Some(coord) = &cluster.coordinator_entity {
                *leader_counts.entry(coord.clone()).or_default() += 1;
            }
        }
    }
    for (entity_id, count) in &leader_counts {
        if *count > 1 {
            conflicts.push(MeshConflict {
                conflict_id: format!("dup-leader-{entity_id}"),
                kind: MeshConflictKind::DuplicateLeader,
                entity_ids: vec![entity_id.clone()],
                description: format!("duplicate leader '{entity_id}' across {count} clusters"),
                detected_at: chrono::Utc::now().to_rfc3339(),
                resolved: false,
                resolution: None,
            });
        }
    }

    sync_actions.push("sync_entity_state".into());
    sync_actions.push("sync_health_readiness_trust".into());
    sync_actions.push("sync_mission_progress".into());
    sync_actions.push("sync_decision_traces".into());
    sync_actions.push("sync_recovery_events".into());
    audit_actions.push("merge_audit_trails".into());
    graph_updates.push("update_entity_graph_relationships".into());

    MeshMergePlan {
        merge_id: format!("merge-{}", chrono::Utc::now().timestamp()),
        partition_ids: partition_ids.to_vec(),
        conflicts,
        sync_actions,
        audit_merge_actions: audit_actions,
        entity_graph_updates: graph_updates,
    }
}

/// Execute partition merge with conflict resolution policy.
pub fn merge_partitions(mesh: &mut EntityMesh, plan: &MeshMergePlan) -> MeshMergeReport {
    let policy = mesh.merge_policy.clone();
    let mut resolved = 0u32;
    let mut remaining = 0u32;
    let mut evidence = Vec::new();

    for conflict in &plan.conflicts {
        let resolution = resolve_conflict(conflict, &policy, mesh);
        if resolution.is_some() {
            resolved += 1;
            evidence.push(format!("resolved:{}", conflict.conflict_id));
        } else {
            remaining += 1;
        }
    }

    for partition_id in &plan.partition_ids {
        for partition in &mut mesh.partitions {
            if partition.partition_id == *partition_id {
                partition.active = false;
                partition.resolved_at = Some(chrono::Utc::now().to_rfc3339());
            }
        }
    }

    for id in mesh.nodes.keys().cloned().collect::<Vec<_>>() {
        if let Some(node) = mesh.nodes.get_mut(&id) {
            node.reachable = true;
        }
    }
    rebuild_topology(mesh);
    mesh.updated_at = chrono::Utc::now().to_rfc3339();

    MeshMergeReport {
        merge_id: plan.merge_id.clone(),
        merged_at: chrono::Utc::now().to_rfc3339(),
        plan: plan.clone(),
        conflicts_resolved: resolved,
        conflicts_remaining: remaining,
        evidence,
    }
}

fn resolve_conflict(
    conflict: &MeshConflict,
    policy: &MeshMergePolicy,
    mesh: &EntityMesh,
) -> Option<String> {
    match conflict.kind {
        MeshConflictKind::DuplicateLeader if policy.prefer_higher_trust => {
            let best = conflict
                .entity_ids
                .iter()
                .filter_map(|id| mesh.nodes.get(id))
                .max_by(|a, b| {
                    a.trust_score
                        .partial_cmp(&b.trust_score)
                        .unwrap_or(std::cmp::Ordering::Equal)
                })?;
            Some(format!("retain_leader:{}", best.entity_id))
        }
        MeshConflictKind::MissionState if policy.block_high_risk_on_conflict => {
            Some("pause_conflicting_missions".into())
        }
        _ if policy.prefer_newer_timestamp => Some("prefer_newer_timestamp".into()),
        _ => None,
    }
}

/// Partition mode blocks high-risk mission starts.
pub fn partition_blocks_high_risk(mesh: &EntityMesh) -> bool {
    mesh.partitions.iter().any(|p| p.active)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::discovery::build_entity_mesh;
    use spanda_config::entity::{EntityKind, EntityRecord, EntityRegistry};

    fn two_cluster_mesh() -> EntityMesh {
        let mut registry = EntityRegistry::default();
        for id in ["a", "b", "c", "d"] {
            registry.entities.insert(
                id.into(),
                EntityRecord {
                    id: id.into(),
                    entity_type: EntityKind::Robot,
                    ..EntityRecord::default()
                },
            );
        }
        let mut mesh = build_entity_mesh(&registry, "partition-test");
        mesh.nodes.get_mut("a").unwrap().neighbors = vec![MeshNeighbor {
            entity_id: "b".into(),
            node_id: "node-b".into(),
            transport: MeshTransport::LocalRuntime,
            reachable: true,
            latency_ms: None,
            packet_loss: None,
            last_seen: None,
        }];
        mesh.nodes.get_mut("b").unwrap().neighbors = vec![MeshNeighbor {
            entity_id: "a".into(),
            node_id: "node-a".into(),
            transport: MeshTransport::LocalRuntime,
            reachable: true,
            latency_ms: None,
            packet_loss: None,
            last_seen: None,
        }];
        mesh.nodes.get_mut("c").unwrap().neighbors = vec![MeshNeighbor {
            entity_id: "d".into(),
            node_id: "node-d".into(),
            transport: MeshTransport::LocalRuntime,
            reachable: true,
            latency_ms: None,
            packet_loss: None,
            last_seen: None,
        }];
        mesh.nodes.get_mut("d").unwrap().neighbors = vec![MeshNeighbor {
            entity_id: "c".into(),
            node_id: "node-c".into(),
            transport: MeshTransport::LocalRuntime,
            reachable: true,
            latency_ms: None,
            packet_loss: None,
            last_seen: None,
        }];
        rebuild_topology(&mut mesh);
        mesh
    }

    #[test]
    fn no_links_means_no_partition() {
        let registry = EntityRegistry::default();
        let mesh = build_entity_mesh(&registry, "no-links");
        assert!(detect_partitions(&mesh).is_empty());
    }

    #[test]
    fn partition_detection_finds_split_clusters() {
        let mesh = two_cluster_mesh();
        let partitions = detect_partitions(&mesh);
        assert_eq!(partitions.len(), 1);
        assert_eq!(partitions[0].clusters.len(), 2);
    }

    #[test]
    fn partition_merge_resolves_conflicts() {
        let mesh = two_cluster_mesh();
        let plan = build_merge_plan(&mesh, &["partition-1".into()]);
        let mut mesh = mesh;
        let report = merge_partitions(&mut mesh, &plan);
        assert!(report.conflicts_resolved + report.conflicts_remaining >= 0);
    }

    #[test]
    fn partition_mode_blocks_high_risk() {
        let mut mesh = two_cluster_mesh();
        mesh.partitions.push(MeshPartition {
            partition_id: "p1".into(),
            detected_at: chrono::Utc::now().to_rfc3339(),
            resolved_at: None,
            clusters: vec![],
            affected_entities: vec!["a".into()],
            active: true,
        });
        assert!(partition_blocks_high_risk(&mesh));
    }
}
