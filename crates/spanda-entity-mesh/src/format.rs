//! Text and JSON formatting for mesh CLI output.
//!
use crate::types::*;
use serde_json::json;

/// Output format for mesh CLI commands.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MeshFormat {
    #[default]
    Text,
    Json,
}

/// Format mesh topology for display.
pub fn format_topology(topology: &MeshTopology, format: MeshFormat) -> String {
    match format {
        MeshFormat::Json => serde_json::to_string_pretty(topology).unwrap_or_else(|_| "{}".into()),
        MeshFormat::Text => {
            let mut out = String::from("Spanda Entity Mesh Topology\n\n");
            out.push_str(&format!("Nodes: {}\n", topology.nodes.len()));
            for node in &topology.nodes {
                out.push_str(&format!(
                    "  {} [{}] reachable={} trust={:.2}\n",
                    node.entity_id,
                    node.transport.as_str(),
                    node.reachable,
                    node.trust_score
                ));
            }
            out.push_str(&format!("\nLinks: {}\n", topology.links.len()));
            for link in &topology.links {
                out.push_str(&format!(
                    "  {} -> {} ({}ms, loss {:.1}%)\n",
                    link.from_entity,
                    link.to_entity,
                    link.latency_ms,
                    link.packet_loss * 100.0
                ));
            }
            if let Some(coord) = &topology.coordinator {
                out.push_str(&format!(
                    "\nCoordinator: {} ({:?})\n",
                    coord.entity_id, coord.status
                ));
            }
            out
        }
    }
}

/// Format mesh health report.
pub fn format_health(health: &MeshHealthReport, format: MeshFormat) -> String {
    match format {
        MeshFormat::Json => serde_json::to_string_pretty(health).unwrap_or_else(|_| "{}".into()),
        MeshFormat::Text => {
            let mut out = String::from("Mesh Health\n\n");
            out.push_str(&format!(
                "Nodes: {}/{} reachable\n",
                health.reachable_nodes, health.total_nodes
            ));
            out.push_str(&format!("Avg trust: {:.2}\n", health.average_trust_score));
            out.push_str(&format!("Avg latency: {}ms\n", health.average_latency_ms));
            out.push_str(&format!(
                "Active partitions: {}\n",
                health.active_partitions
            ));
            out.push_str(&format!(
                "Topology components: {}\n",
                health.topology_components
            ));
            if !health.offline_nodes.is_empty() {
                out.push_str(&format!("Offline: {}\n", health.offline_nodes.join(", ")));
            }
            if !health.issues.is_empty() {
                out.push_str("\nIssues:\n");
                for issue in &health.issues {
                    out.push_str(&format!("  - {issue}\n"));
                }
            }
            out
        }
    }
}

/// Format mesh node list.
pub fn format_node_list(nodes: &[MeshNode], format: MeshFormat) -> String {
    match format {
        MeshFormat::Json => serde_json::to_string_pretty(nodes).unwrap_or_else(|_| "[]".into()),
        MeshFormat::Text => {
            let mut out = String::from("Mesh Nodes\n\n");
            for node in nodes {
                out.push_str(&format!(
                    "{:<24} {:<12} trust={:.2} caps={}\n",
                    node.entity_id,
                    node.transport.as_str(),
                    node.trust_score,
                    node.capabilities.join(",")
                ));
            }
            out
        }
    }
}

/// Format merge report.
pub fn format_merge_report(report: &MeshMergeReport, format: MeshFormat) -> String {
    match format {
        MeshFormat::Json => serde_json::to_string_pretty(report).unwrap_or_else(|_| "{}".into()),
        MeshFormat::Text => format!(
            "Merge {} at {}\n  Resolved: {}\n  Remaining: {}\n",
            report.merge_id,
            report.merged_at,
            report.conflicts_resolved,
            report.conflicts_remaining
        ),
    }
}

/// Format route for display.
pub fn format_route(route: &MeshRoute, format: MeshFormat) -> String {
    match format {
        MeshFormat::Json => serde_json::to_string_pretty(route).unwrap_or_else(|_| "{}".into()),
        MeshFormat::Text => format!(
            "Route {} -> {} via {:?}\n  Hops: {}\n  Trust: {:.2} Latency: {}ms\n",
            route.source_entity,
            route.target_entity,
            route.mode,
            route.hops.join(" -> "),
            route.trust_score,
            route.latency_ms
        ),
    }
}

/// Format capability search results.
pub fn format_capability_results(
    ads: &[MeshCapabilityAdvertisement],
    format: MeshFormat,
) -> String {
    match format {
        MeshFormat::Json => serde_json::to_string_pretty(ads).unwrap_or_else(|_| "[]".into()),
        MeshFormat::Text => {
            if ads.is_empty() {
                return "No entities found with requested capability.\n".into();
            }
            let mut out = String::from("Capability matches:\n\n");
            for ad in ads {
                out.push_str(&format!(
                    "  {} — {}\n",
                    ad.entity_id,
                    ad.capabilities.join(", ")
                ));
            }
            out
        }
    }
}

/// Format mesh graph as JSON nodes/edges for Control Center.
pub fn mesh_graph_json(mesh: &EntityMesh) -> serde_json::Value {
    json!({
        "nodes": mesh.nodes.values().map(|n| json!({
            "id": n.entity_id,
            "transport": n.transport.as_str(),
            "reachable": n.reachable,
            "trust_score": n.trust_score,
            "role": format!("{:?}", n.role),
        })).collect::<Vec<_>>(),
        "edges": mesh.links.iter().map(|l| json!({
            "from": l.from_entity,
            "to": l.to_entity,
            "trusted": l.trusted,
            "active": l.active,
        })).collect::<Vec<_>>(),
    })
}
