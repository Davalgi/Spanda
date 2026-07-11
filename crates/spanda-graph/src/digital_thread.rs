//! Digital Thread v1 — query capability-to-device trace chains.
//!
use crate::build::{build_dependency_graph, DependencyGraph, GraphEdge, GraphNode, GraphNodeKind};
use serde::{Deserialize, Serialize};
use spanda_ast::nodes::Program;
use spanda_capability::{
    capability_traceability, hardware_traceability, CapabilityTraceRow, HardwareTraceRow,
};
use spanda_config::ResolvedSystemConfig;
use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};

/// Product lifecycle phase for digital thread traceability.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LifecyclePhase {
    Requirement,
    Design,
    Deploy,
    Operate,
    Retire,
}

impl LifecyclePhase {
    /// Ordered lifecycle chain from requirement through retirement.
    pub fn chain() -> &'static [LifecyclePhase] {
        // Return the canonical product lifecycle order.
        &[
            Self::Requirement,
            Self::Design,
            Self::Deploy,
            Self::Operate,
            Self::Retire,
        ]
    }

    /// Snake-case phase name used in JSON and query filters.
    pub fn as_str(self) -> &'static str {
        // Map each phase to its stable wire name.
        match self {
            Self::Requirement => "requirement",
            Self::Design => "design",
            Self::Deploy => "deploy",
            Self::Operate => "operate",
            Self::Retire => "retire",
        }
    }

    /// Zero-based ordinal used to compare phase order.
    pub fn ordinal(self) -> u8 {
        // Rank phases so earlier lifecycle stages sort first.
        match self {
            Self::Requirement => 0,
            Self::Design => 1,
            Self::Deploy => 2,
            Self::Operate => 3,
            Self::Retire => 4,
        }
    }

    /// Next phase in the canonical chain, if any.
    pub fn next(self) -> Option<LifecyclePhase> {
        // Advance one step along requirement → … → retire.
        match self {
            Self::Requirement => Some(Self::Design),
            Self::Design => Some(Self::Deploy),
            Self::Deploy => Some(Self::Operate),
            Self::Operate => Some(Self::Retire),
            Self::Retire => None,
        }
    }
}

/// Lifecycle assignment for a graph node in the digital thread.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LifecycleRow {
    pub node_id: String,
    pub label: String,
    pub kind: String,
    pub phase: LifecyclePhase,
}

/// Explicit lifecycle progression edge between two nodes.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LifecycleEdge {
    pub from: String,
    pub to: String,
    pub phase_from: LifecyclePhase,
    pub phase_to: LifecyclePhase,
    pub relation: String,
}

/// Filters for digital thread graph traversal.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct DigitalThreadQuery {
    #[serde(default)]
    pub capability: Option<String>,
    #[serde(default)]
    pub device_id: Option<String>,
    #[serde(default)]
    pub node_id: Option<String>,
    #[serde(default)]
    pub lifecycle_phase: Option<String>,
    /// Optional phase path such as `requirement->deploy` or `design,operate`.
    #[serde(default)]
    pub phase_path: Option<String>,
}

/// Device link from configuration registry into the trace graph.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DigitalThreadDeviceLink {
    pub device_id: String,
    pub device_type: String,
    pub assigned_robot: Option<String>,
    pub lifecycle_state: Option<String>,
    pub related_capabilities: Vec<String>,
}

/// Digital thread query result for Control Center and SDK consumers.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DigitalThreadReport {
    pub query: DigitalThreadQuery,
    pub source: String,
    pub graph: DependencyGraph,
    pub capability_rows: Vec<CapabilityTraceRow>,
    pub hardware_rows: Vec<HardwareTraceRow>,
    pub device_links: Vec<DigitalThreadDeviceLink>,
    pub lifecycle_rows: Vec<LifecycleRow>,
    pub lifecycle_edges: Vec<LifecycleEdge>,
    pub lifecycle_summary: BTreeMap<String, u32>,
    /// Node ids whose phases lie on the requested `phase_path`, when set.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phase_path_nodes: Option<Vec<String>>,
    pub chain_summary: Vec<String>,
    pub matched_node_count: usize,
    pub matched_edge_count: usize,
}

/// Build and filter a digital thread from program AST, traceability, and device registry.
pub fn query_digital_thread(
    program: &Program,
    source: &str,
    config: Option<&ResolvedSystemConfig>,
    query: &DigitalThreadQuery,
) -> DigitalThreadReport {
    // Build the full dependency graph and capability/hardware matrices.
    let full_graph = build_dependency_graph(program, source, config);
    let trace = capability_traceability(program);
    let hardware = hardware_traceability(program);

    // Link configured devices as overlays onto the program graph.
    let device_links = link_devices(config, &trace.capability_rows);
    let mut lifecycle_rows = build_lifecycle_rows(&full_graph.nodes, &device_links);
    append_device_lifecycle_rows(&mut lifecycle_rows, &device_links);

    // Derive explicit lifecycle chain edges from dependencies and device overlays.
    let mut lifecycle_edges =
        build_lifecycle_edges(&full_graph, &lifecycle_rows, &device_links);
    let lifecycle_summary = summarize_lifecycle(&lifecycle_rows);

    // Apply seed filters to the dependency subgraph.
    let (mut nodes, mut edges) = filter_graph(
        &full_graph,
        query,
        &trace.capability_rows,
        &device_links,
        &lifecycle_rows,
    );

    // Inject device overlay nodes into the matched graph when present.
    inject_device_nodes(&mut nodes, &mut edges, &device_links, query, &lifecycle_rows);

    // Restrict to an optional phase path (requirement → … → retire segment).
    let phase_path = parse_phase_path(query.phase_path.as_deref());
    let phase_path_nodes = if let Some(path) = &phase_path {
        let allowed: HashSet<LifecyclePhase> = path.iter().copied().collect();
        let path_node_ids: Vec<String> = lifecycle_rows
            .iter()
            .filter(|row| allowed.contains(&row.phase))
            .map(|row| row.node_id.clone())
            .collect();
        let path_set: HashSet<String> = path_node_ids.iter().cloned().collect();
        nodes.retain(|node| path_set.contains(&node.id));
        edges.retain(|edge| path_set.contains(&edge.from) && path_set.contains(&edge.to));
        lifecycle_edges.retain(|edge| {
            path_set.contains(&edge.from)
                && path_set.contains(&edge.to)
                && phase_in_path_order(path, edge.phase_from, edge.phase_to)
        });
        Some(path_node_ids)
    } else {
        None
    };

    // Keep lifecycle rows/edges aligned with the matched node set when filtered.
    let matched_ids: HashSet<String> = nodes.iter().map(|node| node.id.clone()).collect();
    if query.capability.is_some()
        || query.device_id.is_some()
        || query.node_id.is_some()
        || query.lifecycle_phase.is_some()
        || phase_path.is_some()
    {
        lifecycle_rows.retain(|row| matched_ids.contains(&row.node_id));
        lifecycle_edges
            .retain(|edge| matched_ids.contains(&edge.from) && matched_ids.contains(&edge.to));
    }

    let chain_summary = summarize_chain(
        query,
        &nodes,
        &edges,
        &device_links,
        &lifecycle_edges,
        phase_path.as_deref(),
    );
    let matched_node_count = nodes.len();
    let matched_edge_count = edges.len();
    let capability_rows = filter_capability_rows(&trace.capability_rows, query);
    let hardware_rows = filter_hardware_rows(&hardware.hardware_rows, query, &capability_rows);

    DigitalThreadReport {
        query: query.clone(),
        source: source.to_string(),
        graph: DependencyGraph {
            source: full_graph.source,
            nodes,
            edges,
        },
        capability_rows,
        hardware_rows,
        device_links,
        lifecycle_rows,
        lifecycle_edges,
        lifecycle_summary,
        phase_path_nodes,
        chain_summary,
        matched_node_count,
        matched_edge_count,
    }
}

fn link_devices(
    config: Option<&ResolvedSystemConfig>,
    capability_rows: &[CapabilityTraceRow],
) -> Vec<DigitalThreadDeviceLink> {
    // Return no overlays when Control Center has no resolved system config.
    let Some(resolved) = config else {
        return Vec::new();
    };
    let registry = &resolved.device_registry;
    registry
        .devices
        .iter()
        .map(|device| {
            // Relate devices to capabilities by hardware type or assigned robot.
            let related_capabilities = capability_rows
                .iter()
                .filter(|row| {
                    row.hardware.eq_ignore_ascii_case(&device.device_type)
                        || device
                            .assigned_robot
                            .as_deref()
                            .map(|r| row.required_by.eq_ignore_ascii_case(r))
                            .unwrap_or(false)
                })
                .map(|row| row.capability.clone())
                .collect();
            DigitalThreadDeviceLink {
                device_id: device.id.clone(),
                device_type: device.device_type.clone(),
                assigned_robot: device.assigned_robot.clone(),
                lifecycle_state: device.lifecycle_state.clone(),
                related_capabilities,
            }
        })
        .collect()
}

fn parse_lifecycle_phase(raw: &str) -> Option<LifecyclePhase> {
    // Accept common aliases for each lifecycle phase.
    match raw.to_ascii_lowercase().as_str() {
        "requirement" | "requirements" => Some(LifecyclePhase::Requirement),
        "design" => Some(LifecyclePhase::Design),
        "deploy" | "deployment" => Some(LifecyclePhase::Deploy),
        "operate" | "operation" | "operations" => Some(LifecyclePhase::Operate),
        "retire" | "retirement" => Some(LifecyclePhase::Retire),
        _ => None,
    }
}

/// Parse a phase path query such as `requirement->deploy` or `design,operate`.
fn parse_phase_path(raw: Option<&str>) -> Option<Vec<LifecyclePhase>> {
    // Treat missing or blank input as no path filter.
    let Some(raw) = raw.map(str::trim).filter(|value| !value.is_empty()) else {
        return None;
    };

    // Normalize arrow separators into commas before splitting.
    let normalized = raw.replace("->", ",").replace('→', ",");
    let mut phases = Vec::new();
    for part in normalized.split(',') {
        let trimmed = part.trim();
        if trimmed.is_empty() {
            continue;
        }

        // Reject the whole path when any segment is unknown.
        let Some(phase) = parse_lifecycle_phase(trimmed) else {
            return None;
        };
        phases.push(phase);
    }

    // Expand a two-endpoint path into the inclusive chain between them.
    if phases.len() == 2 {
        let start = phases[0].ordinal();
        let end = phases[1].ordinal();
        if start <= end {
            phases = LifecyclePhase::chain()
                .iter()
                .copied()
                .filter(|phase| phase.ordinal() >= start && phase.ordinal() <= end)
                .collect();
        }
    }

    if phases.is_empty() {
        None
    } else {
        Some(phases)
    }
}

fn phase_in_path_order(
    path: &[LifecyclePhase],
    phase_from: LifecyclePhase,
    phase_to: LifecyclePhase,
) -> bool {
    // Locate both endpoints in the requested path order.
    let from_idx = path.iter().position(|phase| *phase == phase_from);
    let to_idx = path.iter().position(|phase| *phase == phase_to);
    match (from_idx, to_idx) {
        (Some(from), Some(to)) => from < to,
        _ => false,
    }
}

fn infer_lifecycle_phase(kind: GraphNodeKind, device_lifecycle: Option<&str>) -> LifecyclePhase {
    // Prefer an explicit retired device overlay over kind inference.
    if device_lifecycle
        .map(|state| state.eq_ignore_ascii_case("retired") || state.eq_ignore_ascii_case("retire"))
        .unwrap_or(false)
    {
        return LifecyclePhase::Retire;
    }

    // Fall back to kind-based phase assignment when no overlay applies.
    match kind {
        GraphNodeKind::Mission => LifecyclePhase::Requirement,
        GraphNodeKind::Capability | GraphNodeKind::Safety => LifecyclePhase::Design,
        GraphNodeKind::Robot
        | GraphNodeKind::Hardware
        | GraphNodeKind::Sensor
        | GraphNodeKind::Actuator => LifecyclePhase::Deploy,
        GraphNodeKind::Provider | GraphNodeKind::Package => LifecyclePhase::Operate,
    }
}

fn device_node_id(device_id: &str) -> String {
    // Build a stable graph id for a configured device overlay.
    format!("device:{device_id}").to_ascii_lowercase()
}

fn device_lifecycle_phase(link: &DigitalThreadDeviceLink) -> LifecyclePhase {
    // Map device registry lifecycle_state onto digital-thread phases.
    if link
        .lifecycle_state
        .as_deref()
        .is_some_and(|state| state.eq_ignore_ascii_case("retired"))
    {
        LifecyclePhase::Retire
    } else {
        LifecyclePhase::Operate
    }
}

fn build_lifecycle_rows(
    nodes: &[GraphNode],
    device_links: &[DigitalThreadDeviceLink],
) -> Vec<LifecycleRow> {
    // Collect retired device ids for overlay-aware phase inference.
    let retired_devices: HashSet<String> = device_links
        .iter()
        .filter(|link| {
            link.lifecycle_state
                .as_deref()
                .is_some_and(|state| state.eq_ignore_ascii_case("retired"))
        })
        .map(|link| link.device_id.clone())
        .collect();
    nodes
        .iter()
        .map(|node| {
            // Prefer device overlay lifecycle when the node is tied to a device.
            let device_lifecycle = device_links
                .iter()
                .find(|link| {
                    link.assigned_robot
                        .as_deref()
                        .is_some_and(|robot| node.id.contains(robot))
                        || retired_devices.contains(&link.device_id)
                })
                .and_then(|link| link.lifecycle_state.as_deref());
            let phase = infer_lifecycle_phase(node.kind, device_lifecycle);
            LifecycleRow {
                node_id: node.id.clone(),
                label: node.label.clone(),
                kind: format!("{:?}", node.kind).to_ascii_lowercase(),
                phase,
            }
        })
        .collect()
}

fn append_device_lifecycle_rows(
    rows: &mut Vec<LifecycleRow>,
    device_links: &[DigitalThreadDeviceLink],
) {
    // Skip devices already represented as lifecycle rows.
    let existing: HashSet<String> = rows.iter().map(|row| row.node_id.clone()).collect();
    for link in device_links {
        let id = device_node_id(&link.device_id);
        if existing.contains(&id) {
            continue;
        }
        rows.push(LifecycleRow {
            node_id: id,
            label: link.device_id.clone(),
            kind: "device".into(),
            phase: device_lifecycle_phase(link),
        });
    }
}

/// Build explicit lifecycle chain edges from program dependencies and device overlays.
fn build_lifecycle_edges(
    graph: &DependencyGraph,
    lifecycle_rows: &[LifecycleRow],
    device_links: &[DigitalThreadDeviceLink],
) -> Vec<LifecycleEdge> {
    // Index node phases for O(1) edge classification.
    let phase_by_id: HashMap<&str, LifecyclePhase> = lifecycle_rows
        .iter()
        .map(|row| (row.node_id.as_str(), row.phase))
        .collect();
    let mut edges = Vec::new();
    let mut seen: HashSet<(String, String, LifecyclePhase, LifecyclePhase)> = HashSet::new();

    let mut push_edge = |edge: LifecycleEdge| {
        // Deduplicate identical lifecycle edges.
        let key = (
            edge.from.clone(),
            edge.to.clone(),
            edge.phase_from,
            edge.phase_to,
        );
        if seen.insert(key) {
            edges.push(edge);
        }
    };

    // Promote dependency edges that advance along the lifecycle order.
    for edge in &graph.edges {
        let Some(&phase_from) = phase_by_id.get(edge.from.as_str()) else {
            continue;
        };
        let Some(&phase_to) = phase_by_id.get(edge.to.as_str()) else {
            continue;
        };

        // Keep edges that move forward in the product lifecycle.
        if phase_from.ordinal() < phase_to.ordinal() {
            push_edge(LifecycleEdge {
                from: edge.from.clone(),
                to: edge.to.clone(),
                phase_from,
                phase_to,
                relation: format!("lifecycle:{}", edge.relation),
            });
        } else if phase_to.ordinal() < phase_from.ordinal() {
            // Orient reverse dependency edges into lifecycle order.
            push_edge(LifecycleEdge {
                from: edge.to.clone(),
                to: edge.from.clone(),
                phase_from: phase_to,
                phase_to: phase_from,
                relation: format!("lifecycle:{}", edge.relation),
            });
        }
    }

    // Attach device overlay nodes into the operate/retire stages.
    for link in device_links {
        let device_id = device_node_id(&link.device_id);
        let device_phase = device_lifecycle_phase(link);

        // Connect assigned robots (deploy) into the device operate/retire node.
        if let Some(robot) = &link.assigned_robot {
            let robot_id = format!("robot:{robot}").to_ascii_lowercase();
            if let Some(&robot_phase) = phase_by_id.get(robot_id.as_str()) {
                if robot_phase.ordinal() < device_phase.ordinal()
                    || (robot_phase == LifecyclePhase::Deploy
                        && device_phase == LifecyclePhase::Operate)
                {
                    push_edge(LifecycleEdge {
                        from: robot_id,
                        to: device_id.clone(),
                        phase_from: robot_phase,
                        phase_to: device_phase,
                        relation: "device_overlay".into(),
                    });
                }
            }
        }

        // Connect related capabilities (design) toward the device node.
        for capability in &link.related_capabilities {
            let capability_id = format!("capability:{capability}").to_ascii_lowercase();
            if let Some(&cap_phase) = phase_by_id.get(capability_id.as_str()) {
                if cap_phase.ordinal() < device_phase.ordinal() {
                    push_edge(LifecycleEdge {
                        from: capability_id,
                        to: device_id.clone(),
                        phase_from: cap_phase,
                        phase_to: device_phase,
                        relation: "device_overlay".into(),
                    });
                }
            }
        }
    }

    // Ensure consecutive phase pairs with nodes share at least one chain edge.
    bridge_adjacent_phases(&lifecycle_rows, &mut push_edge);

    // Stable order for deterministic JSON and tests.
    edges.sort_by(|left, right| {
        left.phase_from
            .ordinal()
            .cmp(&right.phase_from.ordinal())
            .then(left.from.cmp(&right.from))
            .then(left.to.cmp(&right.to))
    });
    edges
}

fn bridge_adjacent_phases(
    lifecycle_rows: &[LifecycleRow],
    push_edge: &mut impl FnMut(LifecycleEdge),
) {
    // Bucket node ids by lifecycle phase.
    let mut by_phase: BTreeMap<u8, Vec<&LifecycleRow>> = BTreeMap::new();
    for row in lifecycle_rows {
        by_phase.entry(row.phase.ordinal()).or_default().push(row);
    }

    // Walk consecutive phases in the canonical chain.
    for window in LifecyclePhase::chain().windows(2) {
        let from_phase = window[0];
        let to_phase = window[1];
        let Some(from_rows) = by_phase.get(&from_phase.ordinal()) else {
            continue;
        };
        let Some(to_rows) = by_phase.get(&to_phase.ordinal()) else {
            continue;
        };
        if from_rows.is_empty() || to_rows.is_empty() {
            continue;
        }

        // Link the first node of each adjacent phase as an explicit chain step.
        let from_row = from_rows[0];
        let to_row = to_rows[0];
        push_edge(LifecycleEdge {
            from: from_row.node_id.clone(),
            to: to_row.node_id.clone(),
            phase_from: from_phase,
            phase_to: to_phase,
            relation: "lifecycle_chain".into(),
        });
    }
}

fn summarize_lifecycle(rows: &[LifecycleRow]) -> BTreeMap<String, u32> {
    let mut summary = BTreeMap::new();
    for row in rows {
        let key = row.phase.as_str().to_string();
        *summary.entry(key).or_insert(0) += 1;
    }
    summary
}

fn inject_device_nodes(
    nodes: &mut Vec<GraphNode>,
    edges: &mut Vec<GraphEdge>,
    device_links: &[DigitalThreadDeviceLink],
    query: &DigitalThreadQuery,
    lifecycle_rows: &[LifecycleRow],
) {
    // Skip when there are no device overlays to surface.
    if device_links.is_empty() {
        return;
    }

    let existing: HashSet<String> = nodes.iter().map(|node| node.id.clone()).collect();
    for link in device_links {
        // Honor device_id filter when present.
        if let Some(wanted) = &query.device_id {
            if &link.device_id != wanted {
                continue;
            }
        }

        // Honor lifecycle_phase filter for device overlay nodes.
        let phase = device_lifecycle_phase(link);
        if let Some(raw) = query
            .lifecycle_phase
            .as_deref()
            .filter(|value| !value.is_empty())
        {
            if let Some(wanted) = parse_lifecycle_phase(raw) {
                if phase != wanted {
                    continue;
                }
            }
        }

        let id = device_node_id(&link.device_id);
        if existing.contains(&id) {
            continue;
        }

        // Only inject devices that appear in lifecycle rows (always true after append).
        if !lifecycle_rows.iter().any(|row| row.node_id == id) {
            continue;
        }

        let mut metadata = HashMap::new();
        metadata.insert("device_type".into(), link.device_type.clone());
        if let Some(state) = &link.lifecycle_state {
            metadata.insert("lifecycle_state".into(), state.clone());
        }
        nodes.push(GraphNode {
            id: id.clone(),
            label: link.device_id.clone(),
            kind: GraphNodeKind::Hardware,
            metadata,
        });

        // Wire device edges into the matched dependency graph for neighbor highlight.
        if let Some(robot) = &link.assigned_robot {
            let robot_id = format!("robot:{robot}").to_ascii_lowercase();
            if nodes.iter().any(|node| node.id == robot_id) {
                edges.push(GraphEdge {
                    from: robot_id,
                    to: id.clone(),
                    relation: "device_overlay".into(),
                });
            }
        }
        for capability in &link.related_capabilities {
            let capability_id = format!("capability:{capability}").to_ascii_lowercase();
            if nodes.iter().any(|node| node.id == capability_id) {
                edges.push(GraphEdge {
                    from: capability_id,
                    to: id.clone(),
                    relation: "device_overlay".into(),
                });
            }
        }
    }
}

fn filter_graph(
    graph: &DependencyGraph,
    query: &DigitalThreadQuery,
    capability_rows: &[CapabilityTraceRow],
    device_links: &[DigitalThreadDeviceLink],
    lifecycle_rows: &[LifecycleRow],
) -> (Vec<GraphNode>, Vec<GraphEdge>) {
    // Return the full graph when no seed filters are set (phase_path applied later).
    if query.capability.is_none()
        && query.device_id.is_none()
        && query.node_id.is_none()
        && query.lifecycle_phase.is_none()
    {
        return (graph.nodes.clone(), graph.edges.clone());
    }

    let mut seed_ids: HashSet<String> = HashSet::new();
    if let Some(node_id) = &query.node_id {
        seed_ids.insert(node_id.clone());
    }
    if let Some(capability) = &query.capability {
        seed_ids.insert(format!("capability:{capability}").to_ascii_lowercase());
        for row in capability_rows {
            if row.capability.eq_ignore_ascii_case(capability) {
                seed_ids.insert(format!("hardware:{}", row.hardware).to_ascii_lowercase());
                seed_ids.insert(format!("robot:{}", row.required_by).to_ascii_lowercase());
            }
        }
    }
    if let Some(device_id) = &query.device_id {
        if let Some(link) = device_links.iter().find(|d| d.device_id == *device_id) {
            seed_ids.insert(device_node_id(device_id));
            if let Some(robot) = &link.assigned_robot {
                seed_ids.insert(format!("robot:{robot}").to_ascii_lowercase());
            }
            for capability in &link.related_capabilities {
                seed_ids.insert(format!("capability:{capability}").to_ascii_lowercase());
            }
        }
    }
    if let Some(phase_raw) = query
        .lifecycle_phase
        .as_deref()
        .filter(|value| !value.is_empty())
    {
        if let Some(wanted) = parse_lifecycle_phase(phase_raw) {
            for row in lifecycle_rows {
                if row.phase == wanted {
                    seed_ids.insert(row.node_id.clone());
                }
            }
        }
    }

    let node_map: HashMap<String, GraphNode> = graph
        .nodes
        .iter()
        .map(|node| (node.id.clone(), node.clone()))
        .collect();
    let mut visited: HashSet<String> = HashSet::new();
    let mut queue: VecDeque<String> = seed_ids.into_iter().collect();
    while let Some(id) = queue.pop_front() {
        if !visited.insert(id.clone()) {
            continue;
        }
        for edge in &graph.edges {
            if edge.from == id && !visited.contains(&edge.to) {
                queue.push_back(edge.to.clone());
            }
            if edge.to == id && !visited.contains(&edge.from) {
                queue.push_back(edge.from.clone());
            }
        }
    }

    let nodes: Vec<GraphNode> = visited
        .iter()
        .filter_map(|id| node_map.get(id).cloned())
        .filter(|node| lifecycle_phase_matches(query, lifecycle_rows, &node.id))
        .collect();
    let node_set: HashSet<String> = visited;
    let edges: Vec<GraphEdge> = graph
        .edges
        .iter()
        .filter(|edge| node_set.contains(&edge.from) && node_set.contains(&edge.to))
        .cloned()
        .collect();
    (nodes, edges)
}

fn lifecycle_phase_matches(
    query: &DigitalThreadQuery,
    lifecycle_rows: &[LifecycleRow],
    node_id: &str,
) -> bool {
    let Some(raw) = query
        .lifecycle_phase
        .as_deref()
        .filter(|value| !value.is_empty())
    else {
        return true;
    };
    let Some(wanted) = parse_lifecycle_phase(raw) else {
        return true;
    };
    lifecycle_rows
        .iter()
        .find(|row| row.node_id == node_id)
        .is_some_and(|row| row.phase == wanted)
}

fn summarize_chain(
    query: &DigitalThreadQuery,
    nodes: &[GraphNode],
    edges: &[GraphEdge],
    device_links: &[DigitalThreadDeviceLink],
    lifecycle_edges: &[LifecycleEdge],
    phase_path: Option<&[LifecyclePhase]>,
) -> Vec<String> {
    let mut lines = vec![format!(
        "Digital thread query: {}",
        serde_json::to_string(query).unwrap_or_else(|_| "{}".into())
    )];
    lines.push(format!(
        "Matched {} nodes, {} edges, {} lifecycle edges",
        nodes.len(),
        edges.len(),
        lifecycle_edges.len()
    ));
    if let Some(phase) = query
        .lifecycle_phase
        .as_deref()
        .filter(|value| !value.is_empty())
    {
        lines.push(format!("Lifecycle phase filter: {phase}"));
    }
    if let Some(path) = phase_path {
        let labels: Vec<&str> = path.iter().map(|phase| phase.as_str()).collect();
        lines.push(format!("Phase path: {}", labels.join(" → ")));
    }
    for edge in lifecycle_edges.iter().take(12) {
        lines.push(format!(
            "{} [{}] --{}--> {} [{}]",
            edge.from,
            edge.phase_from.as_str(),
            edge.relation,
            edge.to,
            edge.phase_to.as_str()
        ));
    }
    for edge in edges.iter().take(8) {
        lines.push(format!("{} --{}--> {}", edge.from, edge.relation, edge.to));
    }
    if let Some(device_id) = &query.device_id {
        if let Some(link) = device_links.iter().find(|d| d.device_id == *device_id) {
            lines.push(format!(
                "Device {} ({}) → capabilities: {}",
                link.device_id,
                link.device_type,
                link.related_capabilities.join(", ")
            ));
        }
    }
    lines
}

fn filter_capability_rows(
    rows: &[CapabilityTraceRow],
    query: &DigitalThreadQuery,
) -> Vec<CapabilityTraceRow> {
    if let Some(capability) = &query.capability {
        return rows
            .iter()
            .filter(|row| row.capability.eq_ignore_ascii_case(capability))
            .cloned()
            .collect();
    }
    if query.device_id.is_some() || query.node_id.is_some() {
        return rows.to_vec();
    }
    rows.to_vec()
}

fn filter_hardware_rows(
    rows: &[HardwareTraceRow],
    _query: &DigitalThreadQuery,
    capability_rows: &[CapabilityTraceRow],
) -> Vec<HardwareTraceRow> {
    if capability_rows.is_empty() {
        return rows.to_vec();
    }
    let hardware: HashSet<String> = capability_rows
        .iter()
        .map(|row| row.hardware.clone())
        .collect();
    rows.iter()
        .filter(|row| {
            hardware.is_empty()
                || hardware
                    .iter()
                    .any(|h| h.eq_ignore_ascii_case(&row.hardware_component))
        })
        .cloned()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use spanda_lexer::tokenize;
    use spanda_parser::parse;
    use std::path::PathBuf;

    fn load_defense_rover() -> (Program, String) {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../examples/showcase/compliance/defense_rover.sd");
        let source = std::fs::read_to_string(&path).expect("defense_rover.sd");
        let tokens = tokenize(&source).expect("tokenize");
        let program = parse(tokens).expect("parse");
        (program, source)
    }

    #[test]
    fn query_capability_filters_graph() {
        let (program, _) = load_defense_rover();
        let report = query_digital_thread(
            &program,
            "defense_rover.sd",
            None,
            &DigitalThreadQuery::default(),
        );
        assert!(report.matched_node_count > 0);
        assert!(!report.chain_summary.is_empty());
        assert!(!report.lifecycle_rows.is_empty());
    }

    #[test]
    fn lifecycle_phase_filter_limits_nodes() {
        let (program, _) = load_defense_rover();
        let report = query_digital_thread(
            &program,
            "defense_rover.sd",
            None,
            &DigitalThreadQuery {
                lifecycle_phase: Some("design".into()),
                ..DigitalThreadQuery::default()
            },
        );
        assert!(report.matched_node_count <= report.lifecycle_rows.len());
    }

    #[test]
    fn lifecycle_edges_advance_along_chain() {
        let (program, _) = load_defense_rover();
        let report = query_digital_thread(
            &program,
            "defense_rover.sd",
            None,
            &DigitalThreadQuery::default(),
        );
        assert!(
            !report.lifecycle_edges.is_empty(),
            "expected explicit lifecycle edges"
        );
        for edge in &report.lifecycle_edges {
            assert!(
                edge.phase_from.ordinal() < edge.phase_to.ordinal(),
                "lifecycle edge must advance: {:?} -> {:?}",
                edge.phase_from,
                edge.phase_to
            );
        }

        // Adjacent chain bridges should cover consecutive phases present in the report.
        let phases: HashSet<LifecyclePhase> =
            report.lifecycle_rows.iter().map(|row| row.phase).collect();
        for window in LifecyclePhase::chain().windows(2) {
            let from = window[0];
            let to = window[1];
            if phases.contains(&from) && phases.contains(&to) {
                assert!(
                    report
                        .lifecycle_edges
                        .iter()
                        .any(|edge| edge.phase_from == from && edge.phase_to == to),
                    "missing adjacent lifecycle edge {from:?} -> {to:?}"
                );
            }
        }
    }

    #[test]
    fn phase_path_query_filters_lifecycle_edges() {
        let (program, _) = load_defense_rover();
        let report = query_digital_thread(
            &program,
            "defense_rover.sd",
            None,
            &DigitalThreadQuery {
                phase_path: Some("requirement->deploy".into()),
                ..DigitalThreadQuery::default()
            },
        );
        let path_nodes = report.phase_path_nodes.expect("phase_path_nodes");
        assert!(!path_nodes.is_empty());
        for edge in &report.lifecycle_edges {
            assert!(edge.phase_from.ordinal() <= LifecyclePhase::Deploy.ordinal());
            assert!(edge.phase_to.ordinal() <= LifecyclePhase::Deploy.ordinal());
            assert!(edge.phase_from.ordinal() < edge.phase_to.ordinal());
        }
        for row in &report.lifecycle_rows {
            assert!(row.phase.ordinal() <= LifecyclePhase::Deploy.ordinal());
        }
    }

    #[test]
    fn parse_phase_path_expands_endpoints() {
        let path = parse_phase_path(Some("design->operate")).expect("path");
        assert_eq!(
            path,
            vec![
                LifecyclePhase::Design,
                LifecyclePhase::Deploy,
                LifecyclePhase::Operate
            ]
        );
    }
}
