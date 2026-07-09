//! REST API operations for Spanda Autonomous Entity Mesh (CLI/SDK parity).
//!
use serde::Deserialize;
use spanda_deploy_http::HttpResponse;
use spanda_entity_mesh::{
    apply_discovery, build_entity_mesh, build_merge_plan, compute_route, discover_mesh_nodes,
    evaluate_mesh_health, find_capability, merge_partitions, mesh_graph_json, parse_mesh_discovery_sources,
    simulate_partition, default_mesh_discovery_sources, parse_mesh_discovery_sources,
    MeshRouteOptions, MeshRoutingMode,
};

use crate::handlers::{bad_request, json_ok};
use crate::state::ControlCenterState;

const API_VERSION: &str = "v1";

fn mesh_from_state(state: &ControlCenterState) -> spanda_entity_mesh::EntityMesh {
    let registry = state.entity_registry();
    build_entity_mesh(&registry, "control-center")
}

/// GET /v1/mesh/topology
pub fn mesh_topology(state: &ControlCenterState) -> HttpResponse {
    let mesh = mesh_from_state(state);
    json_ok(&serde_json::json!({
        "version": API_VERSION,
        "topology": mesh.topology,
        "mesh_id": mesh.mesh_id,
    }))
}

/// GET /v1/mesh/nodes
pub fn mesh_nodes(state: &ControlCenterState) -> HttpResponse {
    let mesh = mesh_from_state(state);
    json_ok(&serde_json::json!({
        "version": API_VERSION,
        "nodes": mesh.nodes.values().collect::<Vec<_>>(),
        "count": mesh.nodes.len(),
    }))
}

/// GET /v1/mesh/routes
pub fn mesh_routes(state: &ControlCenterState, query: &str) -> HttpResponse {
    let mesh = mesh_from_state(state);
    let source = parse_query_param(query, "source");
    let target = parse_query_param(query, "target");
    if let (Some(src), Some(tgt)) = (source, target) {
        match compute_route(
            &mesh,
            &src,
            &tgt,
            &MeshRouteOptions {
                mode: Some(MeshRoutingMode::TrustWeighted),
                min_trust: 0.5,
                ..Default::default()
            },
        ) {
            Ok(route) => {
                return json_ok(&serde_json::json!({
                    "version": API_VERSION,
                    "route": route,
                }));
            }
            Err(err) => return bad_request(&err),
        }
    }
    json_ok(&serde_json::json!({
        "version": API_VERSION,
        "history": mesh.route_history,
        "count": mesh.route_history.len(),
    }))
}

/// GET /v1/mesh/partitions
pub fn mesh_partitions(state: &ControlCenterState) -> HttpResponse {
    let mesh = mesh_from_state(state);
    json_ok(&serde_json::json!({
        "version": API_VERSION,
        "partitions": mesh.partitions,
        "active": mesh.partitions.iter().filter(|p| p.active).count(),
    }))
}

/// GET /v1/mesh/health
pub fn mesh_health(state: &ControlCenterState) -> HttpResponse {
    let mesh = mesh_from_state(state);
    let health = evaluate_mesh_health(&mesh, &Default::default());
    json_ok(&serde_json::json!({
        "version": API_VERSION,
        "health": health,
    }))
}

/// GET /v1/mesh/graph
pub fn mesh_graph(state: &ControlCenterState) -> HttpResponse {
    let mesh = mesh_from_state(state);
    json_ok(&serde_json::json!({
        "version": API_VERSION,
        "graph": mesh_graph_json(&mesh),
    }))
}

#[derive(Debug, Default, Deserialize)]
pub struct MeshDiscoverRequest {
    #[serde(default)]
    pub sources: Vec<String>,
}

/// POST /v1/mesh/discover
pub fn mesh_discover(state: &ControlCenterState, body: &str) -> HttpResponse {
    let req: MeshDiscoverRequest = serde_json::from_str(body).unwrap_or_default();
    let registry = state.entity_registry();
    let sources = if req.sources.is_empty() {
        default_mesh_discovery_sources()
    } else {
        let parsed = parse_mesh_discovery_sources(&req.sources);
        if parsed.is_empty() {
            default_mesh_discovery_sources()
        } else {
            parsed
        }
    };
    let result = discover_mesh_nodes(&registry, &sources);
    let mut mesh = build_entity_mesh(&registry, "control-center");
    apply_discovery(&mut mesh, &result);
    json_ok(&serde_json::json!({
        "version": API_VERSION,
        "discovery": result,
    }))
}

#[derive(Debug, Default, Deserialize)]
pub struct MeshFindCapabilityRequest {
    pub capability: String,
}

/// POST /v1/mesh/find-capability
pub fn mesh_find_capability(state: &ControlCenterState, body: &str) -> HttpResponse {
    let req: MeshFindCapabilityRequest = serde_json::from_str(body).unwrap_or_default();
    if req.capability.is_empty() {
        return bad_request("capability required");
    }
    let mesh = mesh_from_state(state);
    let matches = find_capability(&mesh, &req.capability);
    json_ok(&serde_json::json!({
        "version": API_VERSION,
        "capability": req.capability,
        "matches": matches,
    }))
}

#[derive(Debug, Default, Deserialize)]
pub struct MeshSimulatePartitionRequest {
    #[serde(default)]
    pub entity_ids: Vec<String>,
}

/// POST /v1/mesh/simulate-partition
pub fn mesh_simulate_partition(state: &ControlCenterState, body: &str) -> HttpResponse {
    let req: MeshSimulatePartitionRequest = serde_json::from_str(body).unwrap_or_default();
    if req.entity_ids.is_empty() {
        return bad_request("entity_ids required");
    }
    let mut mesh = mesh_from_state(state);
    let report = simulate_partition(&mut mesh, &req.entity_ids);
    json_ok(&serde_json::json!({
        "version": API_VERSION,
        "report": report,
    }))
}

/// GET /v1/mesh/merge-report
pub fn mesh_merge_report(state: &ControlCenterState) -> HttpResponse {
    let mesh = mesh_from_state(state);
    let plan = build_merge_plan(
        &mesh,
        &mesh
            .partitions
            .iter()
            .filter(|p| p.active)
            .map(|p| p.partition_id.clone())
            .collect::<Vec<_>>(),
    );
    let report = merge_partitions(&mut { mesh.clone() }, &plan);
    json_ok(&serde_json::json!({
        "version": API_VERSION,
        "report": report,
    }))
}

fn parse_query_param(query: &str, key: &str) -> Option<String> {
    query.split('&').find_map(|pair| {
        let (k, v) = pair.split_once('=')?;
        if k == key {
            Some(v.to_string())
        } else {
            None
        }
    })
}

/// JSON string helper for gRPC parity.
pub fn mesh_topology_json(state: &ControlCenterState) -> String {
    mesh_topology(state).body
}

/// JSON string helper for gRPC parity.
pub fn mesh_nodes_json(state: &ControlCenterState) -> String {
    mesh_nodes(state).body
}

/// JSON string helper for gRPC parity.
pub fn mesh_routes_json(state: &ControlCenterState, query: &str) -> String {
    mesh_routes(state, query).body
}

/// JSON string helper for gRPC parity.
pub fn mesh_partitions_json(state: &ControlCenterState) -> String {
    mesh_partitions(state).body
}

/// JSON string helper for gRPC parity.
pub fn mesh_health_json(state: &ControlCenterState) -> String {
    mesh_health(state).body
}

/// JSON string helper for gRPC parity.
pub fn mesh_graph_json_api(state: &ControlCenterState) -> String {
    mesh_graph(state).body
}

/// JSON string helper for gRPC parity.
pub fn mesh_merge_report_json(state: &ControlCenterState) -> String {
    mesh_merge_report(state).body
}

/// JSON string helper for gRPC parity.
pub fn mesh_discover_json(state: &ControlCenterState, body: &str) -> String {
    mesh_discover(state, body).body
}

/// JSON string helper for gRPC parity.
pub fn mesh_find_capability_json(state: &ControlCenterState, body: &str) -> String {
    mesh_find_capability(state, body).body
}

/// JSON string helper for gRPC parity.
pub fn mesh_simulate_partition_json(state: &ControlCenterState, body: &str) -> String {
    mesh_simulate_partition(state, body).body
}
