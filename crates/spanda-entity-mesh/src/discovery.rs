//! Entity mesh registry, discovery, and topology management.
//!
use crate::election::{apply_coordinator, elect_coordinator, MeshElectionOptions};
use crate::types::*;
use spanda_config::entity::{
    EntityGraph, EntityHealthStatus, EntityRecord, EntityRegistry,
    EntityRelationship, EntityRelationshipKind,
};
use std::collections::HashSet;

/// Build mesh nodes from the entity registry and optional discovery sources.
pub fn discover_mesh_nodes(
    registry: &EntityRegistry,
    sources: &[MeshDiscoverySource],
) -> MeshDiscoveryResult {
    // Project entity registry records into mesh nodes for each requested source.
    let mut discovered = Vec::new();
    let mut seen = HashSet::new();

    for entity in registry.list() {
        if !seen.insert(entity.id.clone()) {
            continue;
        }
        discovered.push(entity_to_mesh_node(entity, MeshTransport::LocalRuntime));
    }

    if sources
        .iter()
        .any(|s| matches!(s, MeshDiscoverySource::EntityGraph))
    {
        let graph = registry.graph();
        for edge in &graph.edges {
            apply_graph_edge_to_nodes(&mut discovered, edge);
        }
    }

    MeshDiscoveryResult {
        discovered: discovered.clone(),
        sources: sources.to_vec(),
        new_entities: discovered.len() as u32,
        updated_entities: 0,
    }
}

fn entity_to_mesh_node(entity: &EntityRecord, transport: MeshTransport) -> MeshNode {
    MeshNode {
        entity_id: entity.id.clone(),
        node_id: format!("node-{}", entity.id),
        transport,
        reachable: entity.health_status != EntityHealthStatus::Offline,
        neighbors: Vec::new(),
        capabilities: entity.capabilities.clone(),
        health: entity.health_status,
        readiness: entity.readiness_status,
        trust_score: entity_trust_score(entity),
        latency_ms: None,
        bandwidth_kbps: None,
        packet_loss: None,
        hop_count: None,
        last_seen: entity.audit.as_ref().and_then(|a| a.updated_at.clone()),
        battery_percent: None,
        role: MeshNodeRole::Participant,
        coordinator_status: MeshCoordinatorStatus::None,
        supported_protocols: vec!["secure_messaging".into()],
        security_identity: entity.security.clone().unwrap_or_default(),
    }
}

fn entity_trust_score(entity: &EntityRecord) -> f64 {
    match entity.trust_status {
        spanda_config::entity::EntityTrustStatus::Verified => 0.98,
        spanda_config::entity::EntityTrustStatus::Trusted => 0.95,
        spanda_config::entity::EntityTrustStatus::Untrusted => 0.2,
        spanda_config::entity::EntityTrustStatus::Compromised => 0.0,
        spanda_config::entity::EntityTrustStatus::Unknown => 0.5,
    }
}

fn apply_graph_edge_to_nodes(nodes: &mut [MeshNode], edge: &EntityRelationship) {
    let from_idx = nodes.iter().position(|n| n.entity_id == edge.from_id);
    let to_idx = nodes.iter().position(|n| n.entity_id == edge.to_id);
    if let (Some(from), Some(to)) = (from_idx, to_idx) {
        let neighbor = MeshNeighbor {
            entity_id: nodes[to].entity_id.clone(),
            node_id: nodes[to].node_id.clone(),
            transport: nodes[to].transport.clone(),
            reachable: nodes[to].reachable,
            latency_ms: None,
            packet_loss: None,
            last_seen: nodes[to].last_seen.clone(),
        };
        nodes[from].neighbors.push(neighbor);
        if edge.kind == EntityRelationshipKind::CommunicatesWith
            || edge.kind == EntityRelationshipKind::ConnectedTo
        {
            let reverse = MeshNeighbor {
                entity_id: nodes[from].entity_id.clone(),
                node_id: nodes[from].node_id.clone(),
                transport: nodes[from].transport.clone(),
                reachable: nodes[from].reachable,
                latency_ms: None,
                packet_loss: None,
                last_seen: nodes[from].last_seen.clone(),
            };
            nodes[to].neighbors.push(reverse);
        }
    }
}

/// Merge discovery results into an entity mesh instance.
pub fn apply_discovery(mesh: &mut EntityMesh, result: &MeshDiscoveryResult) {
    for node in &result.discovered {
        mesh.nodes.insert(node.entity_id.clone(), node.clone());
    }
    rebuild_topology(mesh);
    mesh.updated_at = chrono::Utc::now().to_rfc3339();
}

/// Rebuild mesh links and topology from current nodes.
pub fn rebuild_topology(mesh: &mut EntityMesh) {
    let mut links = Vec::new();
    for node in mesh.nodes.values() {
        for neighbor in &node.neighbors {
            links.push(MeshLink {
                from_entity: node.entity_id.clone(),
                to_entity: neighbor.entity_id.clone(),
                transport: neighbor.transport.clone(),
                latency_ms: neighbor.latency_ms.unwrap_or(10),
                bandwidth_kbps: node.bandwidth_kbps,
                packet_loss: neighbor.packet_loss.unwrap_or(0.0),
                trusted: node.trust_score >= mesh.partition_policy.min_trust_for_relay,
                active: neighbor.reachable,
            });
        }
    }
    mesh.links = links.clone();
    mesh.topology = MeshTopology {
        nodes: mesh.nodes.values().cloned().collect(),
        links,
        coordinator: mesh.coordinator.clone(),
        partitions: mesh.partitions.clone(),
        updated_at: chrono::Utc::now().to_rfc3339(),
    };
}

/// Find entities advertising a capability.
pub fn find_capability(mesh: &EntityMesh, capability: &str) -> Vec<MeshCapabilityAdvertisement> {
    mesh.capability_ads
        .iter()
        .filter(|ad| ad.capabilities.iter().any(|c| c == capability))
        .cloned()
        .chain(
            mesh.nodes
                .values()
                .filter(|n| n.capabilities.iter().any(|c| c == capability))
                .map(|n| MeshCapabilityAdvertisement {
                    entity_id: n.entity_id.clone(),
                    capabilities: n.capabilities.clone(),
                    advertised_at: mesh.updated_at.clone(),
                    transport: n.transport.clone(),
                    verified: n.trust_score >= 0.6,
                }),
        )
        .collect()
}

/// Register capability advertisements from mesh nodes.
pub fn refresh_capability_ads(mesh: &mut EntityMesh) {
    mesh.capability_ads = mesh
        .nodes
        .values()
        .filter(|n| !n.capabilities.is_empty())
        .map(|n| MeshCapabilityAdvertisement {
            entity_id: n.entity_id.clone(),
            capabilities: n.capabilities.clone(),
            advertised_at: chrono::Utc::now().to_rfc3339(),
            transport: n.transport.clone(),
            verified: n.trust_score >= 0.6,
        })
        .collect();
}

/// Sync entity graph relationships from mesh topology (additive only).
pub fn mesh_to_entity_graph_edges(mesh: &EntityMesh) -> Vec<EntityRelationship> {
    let mut edges = Vec::new();
    let mut seen = HashSet::new();
    for link in &mesh.links {
        if !link.active {
            continue;
        }
        let key = format!("{}->{}", link.from_entity, link.to_entity);
        if !seen.insert(key) {
            continue;
        }
        edges.push(EntityRelationship {
            from_id: link.from_entity.clone(),
            to_id: link.to_entity.clone(),
            kind: EntityRelationshipKind::CommunicatesWith,
            label: Some("mesh_link".into()),
        });
    }
    edges
}

/// Initialize entity mesh from registry with default discovery sources.
pub fn build_entity_mesh(registry: &EntityRegistry, mesh_id: &str) -> EntityMesh {
    let sources = vec![
        MeshDiscoverySource::LocalRuntime,
        MeshDiscoverySource::EntityGraph,
    ];
    let discovery = discover_mesh_nodes(registry, &sources);
    let mut mesh = EntityMesh {
        mesh_id: mesh_id.into(),
        ..EntityMesh::default()
    };
    apply_discovery(&mut mesh, &discovery);
    refresh_capability_ads(&mut mesh);
    if let Ok(coordinator) = elect_coordinator(&mesh, &MeshElectionOptions::default()) {
        apply_coordinator(&mut mesh, coordinator);
    }
    mesh
}

/// Inspect a single mesh node by entity id.
pub fn inspect_node<'a>(mesh: &'a EntityMesh, entity_id: &str) -> Option<&'a MeshNode> {
    mesh.nodes.get(entity_id)
}

/// List all mesh nodes.
pub fn list_nodes(mesh: &EntityMesh) -> Vec<MeshNode> {
    mesh.nodes.values().cloned().collect()
}

/// Merge entity graph into mesh for topology awareness.
pub fn enrich_from_entity_graph(mesh: &mut EntityMesh, graph: &EntityGraph) {
    for edge in &graph.edges {
        let from = mesh.nodes.get(&edge.from_id).cloned();
        let to = mesh.nodes.get(&edge.to_id).cloned();
        if let (Some(from_node), Some(to_node)) = (from, to) {
            if let Some(node) = mesh.nodes.get_mut(&from_node.entity_id) {
                if !node
                    .neighbors
                    .iter()
                    .any(|n| n.entity_id == to_node.entity_id)
                {
                    node.neighbors.push(MeshNeighbor {
                        entity_id: to_node.entity_id.clone(),
                        node_id: to_node.node_id.clone(),
                        transport: to_node.transport.clone(),
                        reachable: to_node.reachable,
                        latency_ms: None,
                        packet_loss: None,
                        last_seen: to_node.last_seen.clone(),
                    });
                }
            }
        }
    }
    rebuild_topology(mesh);
}

#[cfg(test)]
mod tests {
    use super::*;
    use spanda_config::entity::{EntityKind, EntityRecord};

    fn sample_registry() -> EntityRegistry {
        let mut registry = EntityRegistry::default();
        registry.entities.insert(
            "robot-a".into(),
            EntityRecord {
                id: "robot-a".into(),
                entity_type: EntityKind::Robot,
                capabilities: vec!["thermal_camera".into(), "relay_node".into()],
                ..EntityRecord::default()
            },
        );
        registry.entities.insert(
            "robot-b".into(),
            EntityRecord {
                id: "robot-b".into(),
                entity_type: EntityKind::Robot,
                capabilities: vec!["heavy_payload".into()],
                ..EntityRecord::default()
            },
        );
        registry
    }

    #[test]
    fn entity_discovery_projects_registry_nodes() {
        let registry = sample_registry();
        let result = discover_mesh_nodes(
            &registry,
            &[
                MeshDiscoverySource::LocalRuntime,
                MeshDiscoverySource::EntityGraph,
            ],
        );
        assert_eq!(result.discovered.len(), 2);
        assert!(result
            .discovered
            .iter()
            .any(|n| n.capabilities.contains(&"thermal_camera".into())));
    }

    #[test]
    fn capability_find_matches_advertised_capabilities() {
        let mesh = build_entity_mesh(&sample_registry(), "test");
        let found = find_capability(&mesh, "thermal_camera");
        assert!(!found.is_empty());
    }
}
