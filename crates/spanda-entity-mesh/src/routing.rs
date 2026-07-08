//! Trust-aware and readiness-aware mesh routing.
//!
use crate::types::*;
use std::collections::{HashSet, VecDeque};

/// Route planning options.
#[derive(Debug, Clone, Default)]
pub struct MeshRouteOptions {
    pub mode: Option<MeshRoutingMode>,
    pub target_capability: Option<String>,
    pub min_trust: f64,
    pub mission_priority: MeshMessagePriority,
    pub allow_untrusted_relay: bool,
}

/// Compute a route between source and target entities.
pub fn compute_route(
    mesh: &EntityMesh,
    source: &str,
    target: &str,
    options: &MeshRouteOptions,
) -> Result<MeshRoute, String> {
    let mode = options
        .mode
        .clone()
        .unwrap_or(MeshRoutingMode::TrustWeighted);

    match mode {
        MeshRoutingMode::Direct => compute_direct_route(mesh, source, target, options),
        MeshRoutingMode::Broadcast => compute_broadcast_route(mesh, source, options),
        MeshRoutingMode::CapabilityBased => {
            let cap = options
                .target_capability
                .as_deref()
                .ok_or("capability-based routing requires target_capability")?;
            compute_capability_route(mesh, source, cap, options)
        }
        _ => compute_weighted_route(mesh, source, target, &mode, options),
    }
}

fn compute_direct_route(
    mesh: &EntityMesh,
    source: &str,
    target: &str,
    options: &MeshRouteOptions,
) -> Result<MeshRoute, String> {
    let source_node = mesh.nodes.get(source).ok_or("source entity not in mesh")?;
    let target_node = mesh.nodes.get(target).ok_or("target entity not in mesh")?;

    if !source_node.reachable || !target_node.reachable {
        return Err("source or target unreachable".into());
    }

    let direct = source_node
        .neighbors
        .iter()
        .any(|n| n.entity_id == target && n.reachable);
    let hops = if direct {
        vec![source.into(), target.into()]
    } else {
        bfs_path(mesh, source, target)?
    };

    validate_route_trust(mesh, &hops, options)?;
    build_route(mesh, source, target, MeshRoutingMode::Direct, hops, options)
}

fn compute_weighted_route(
    mesh: &EntityMesh,
    source: &str,
    target: &str,
    mode: &MeshRoutingMode,
    options: &MeshRouteOptions,
) -> Result<MeshRoute, String> {
    let hops = bfs_path(mesh, source, target)?;
    validate_route_trust(mesh, &hops, options)?;
    build_route(mesh, source, target, mode.clone(), hops, options)
}

fn compute_broadcast_route(
    mesh: &EntityMesh,
    source: &str,
    options: &MeshRouteOptions,
) -> Result<MeshRoute, String> {
    let source_node = mesh.nodes.get(source).ok_or("source entity not in mesh")?;
    if source_node.trust_score < options.min_trust {
        return Err("source trust too low for broadcast".into());
    }
    let hops: Vec<String> = mesh
        .nodes
        .values()
        .filter(|n| n.reachable)
        .map(|n| n.entity_id.clone())
        .collect();
    build_route(
        mesh,
        source,
        "broadcast",
        MeshRoutingMode::Broadcast,
        hops,
        options,
    )
}

fn compute_capability_route(
    mesh: &EntityMesh,
    source: &str,
    capability: &str,
    options: &MeshRouteOptions,
) -> Result<MeshRoute, String> {
    let target = mesh
        .nodes
        .values()
        .filter(|n| n.reachable && n.capabilities.iter().any(|c| c == capability))
        .max_by(|a, b| {
            route_weight(a, options)
                .partial_cmp(&route_weight(b, options))
                .unwrap_or(std::cmp::Ordering::Equal)
        })
        .ok_or_else(|| format!("no reachable entity with capability '{capability}'"))?;

    compute_weighted_route(
        mesh,
        source,
        &target.entity_id,
        &MeshRoutingMode::CapabilityBased,
        options,
    )
}

fn bfs_path(mesh: &EntityMesh, source: &str, target: &str) -> Result<Vec<String>, String> {
    let mut queue = VecDeque::from([(source.to_string(), vec![source.to_string()])]);
    let mut visited = HashSet::from([source.to_string()]);

    while let Some((current, path)) = queue.pop_front() {
        if current == target {
            return Ok(path);
        }
        let Some(node) = mesh.nodes.get(&current) else {
            continue;
        };
        for neighbor in &node.neighbors {
            if !neighbor.reachable || visited.contains(&neighbor.entity_id) {
                continue;
            }
            visited.insert(neighbor.entity_id.clone());
            let mut next_path = path.clone();
            next_path.push(neighbor.entity_id.clone());
            queue.push_back((neighbor.entity_id.clone(), next_path));
        }
    }
    Err(format!("no route from {source} to {target}"))
}

fn route_weight(node: &MeshNode, options: &MeshRouteOptions) -> f64 {
    let trust = node.trust_score;
    let readiness = match node.readiness {
        spanda_config::entity::EntityReadinessStatus::Ready => 1.0,
        spanda_config::entity::EntityReadinessStatus::Partial => 0.6,
        spanda_config::entity::EntityReadinessStatus::NotReady => 0.2,
        _ => 0.5,
    };
    let priority_boost = match options.mission_priority {
        MeshMessagePriority::Emergency => 0.2,
        MeshMessagePriority::SafetyCritical => 0.15,
        _ => 0.0,
    };
    trust * 0.6 + readiness * 0.4 + priority_boost
}

fn validate_route_trust(
    mesh: &EntityMesh,
    hops: &[String],
    options: &MeshRouteOptions,
) -> Result<(), String> {
    for hop in hops {
        let Some(node) = mesh.nodes.get(hop) else {
            return Err(format!("route hop '{hop}' not in mesh"));
        };
        if node.trust_score < options.min_trust {
            return Err(format!(
                "route through '{hop}' blocked: trust {:.2} below minimum {:.2}",
                node.trust_score, options.min_trust
            ));
        }
        if options.mission_priority == MeshMessagePriority::SafetyCritical
            && !options.allow_untrusted_relay
            && node.trust_score < 0.7
            && hop != hops.first().unwrap_or(&String::new())
            && hop != hops.last().unwrap_or(&String::new())
        {
            return Err(format!(
                "safety-critical route rejected: untrusted relay '{hop}'"
            ));
        }
    }
    Ok(())
}

fn build_route(
    mesh: &EntityMesh,
    source: &str,
    target: &str,
    mode: MeshRoutingMode,
    hops: Vec<String>,
    options: &MeshRouteOptions,
) -> Result<MeshRoute, String> {
    let trust_score = hops
        .iter()
        .filter_map(|h| mesh.nodes.get(h))
        .map(|n| n.trust_score)
        .fold(f64::INFINITY, f64::min);
    let readiness_score = hops
        .iter()
        .filter_map(|h| mesh.nodes.get(h))
        .map(|n| match n.readiness {
            spanda_config::entity::EntityReadinessStatus::Ready => 1.0,
            spanda_config::entity::EntityReadinessStatus::Partial => 0.6,
            _ => 0.3,
        })
        .fold(f64::INFINITY, f64::min);
    let latency_ms = hops
        .windows(2)
        .filter_map(|pair| {
            mesh.links
                .iter()
                .find(|l| l.from_entity == pair[0] && l.to_entity == pair[1] && l.active)
        })
        .map(|l| l.latency_ms)
        .sum();

    let trusted = trust_score >= options.min_trust;
    let mut evidence = vec![format!("mode={mode:?}"), format!("hops={}", hops.len())];
    if options.mission_priority == MeshMessagePriority::SafetyCritical {
        evidence.push("safety_critical_route_validated".into());
    }

    Ok(MeshRoute {
        route_id: format!(
            "route-{}-{}-{}",
            source,
            target,
            chrono::Utc::now().timestamp()
        ),
        source_entity: source.into(),
        target_entity: target.into(),
        mode,
        hops,
        trust_score,
        readiness_score,
        latency_ms,
        trusted,
        evidence,
    })
}

/// Record a computed route in mesh history.
pub fn record_route(mesh: &mut EntityMesh, route: MeshRoute) {
    mesh.route_history.push(route);
    if mesh.route_history.len() > 500 {
        mesh.route_history.drain(0..100);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::discovery::build_entity_mesh;
    use spanda_config::entity::{
        EntityHealthStatus, EntityKind, EntityReadinessStatus, EntityRecord, EntityRegistry,
        EntityTrustStatus,
    };

    fn mesh_with_relay() -> EntityMesh {
        let mut registry = EntityRegistry::default();
        for (id, trust) in [
            ("coordinator", EntityTrustStatus::Verified),
            ("trusted-relay", EntityTrustStatus::Trusted),
            ("untrusted-relay", EntityTrustStatus::Untrusted),
            ("target", EntityTrustStatus::Trusted),
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
        let mut mesh = build_entity_mesh(&registry, "route-test");
        mesh.nodes.get_mut("coordinator").unwrap().neighbors = vec![MeshNeighbor {
            entity_id: "trusted-relay".into(),
            node_id: "node-trusted-relay".into(),
            transport: MeshTransport::LocalRuntime,
            reachable: true,
            latency_ms: Some(5),
            packet_loss: Some(0.0),
            last_seen: None,
        }];
        mesh.nodes.get_mut("trusted-relay").unwrap().neighbors = vec![
            MeshNeighbor {
                entity_id: "coordinator".into(),
                node_id: "node-coordinator".into(),
                transport: MeshTransport::LocalRuntime,
                reachable: true,
                latency_ms: Some(5),
                packet_loss: Some(0.0),
                last_seen: None,
            },
            MeshNeighbor {
                entity_id: "target".into(),
                node_id: "node-target".into(),
                transport: MeshTransport::LocalRuntime,
                reachable: true,
                latency_ms: Some(5),
                packet_loss: Some(0.0),
                last_seen: None,
            },
        ];
        mesh.nodes.get_mut("untrusted-relay").unwrap().neighbors = vec![MeshNeighbor {
            entity_id: "target".into(),
            node_id: "node-target".into(),
            transport: MeshTransport::LocalRuntime,
            reachable: true,
            latency_ms: Some(5),
            packet_loss: Some(0.0),
            last_seen: None,
        }];
        mesh.nodes.get_mut("target").unwrap().neighbors = vec![MeshNeighbor {
            entity_id: "trusted-relay".into(),
            node_id: "node-trusted-relay".into(),
            transport: MeshTransport::LocalRuntime,
            reachable: true,
            latency_ms: Some(5),
            packet_loss: Some(0.0),
            last_seen: None,
        }];
        crate::discovery::rebuild_topology(&mut mesh);
        mesh
    }

    #[test]
    fn trusted_route_succeeds() {
        let mesh = mesh_with_relay();
        let route = compute_route(
            &mesh,
            "coordinator",
            "target",
            &MeshRouteOptions {
                min_trust: 0.6,
                mission_priority: MeshMessagePriority::Normal,
                ..Default::default()
            },
        )
        .expect("route");
        assert!(route.trusted);
        assert!(route.hops.contains(&"trusted-relay".into()));
    }

    #[test]
    fn untrusted_relay_rejected_for_safety_critical() {
        let mut mesh = mesh_with_relay();
        mesh.nodes.get_mut("trusted-relay").unwrap().reachable = false;
        mesh.nodes.get_mut("coordinator").unwrap().neighbors = vec![MeshNeighbor {
            entity_id: "untrusted-relay".into(),
            node_id: "node-untrusted-relay".into(),
            transport: MeshTransport::LocalRuntime,
            reachable: true,
            latency_ms: Some(5),
            packet_loss: Some(0.0),
            last_seen: None,
        }];
        crate::discovery::rebuild_topology(&mut mesh);
        let err = compute_route(
            &mesh,
            "coordinator",
            "target",
            &MeshRouteOptions {
                min_trust: 0.6,
                mission_priority: MeshMessagePriority::SafetyCritical,
                allow_untrusted_relay: false,
                ..Default::default()
            },
        )
        .unwrap_err();
        assert!(err.contains("untrusted relay") || err.contains("trust"));
    }
}
