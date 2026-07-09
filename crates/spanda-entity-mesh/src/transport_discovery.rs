//! Live transport discovery hooks for Entity Mesh (MQTT, ROS2, DDS).
//!
use crate::types::{MeshDiscoverySource, MeshNode, MeshTransport};
use spanda_config::device_identity::DiscoveryMatch;
use spanda_config::entity::{EntityHealthStatus, EntityRecord, EntityRegistry};

/// Parse REST/CLI discovery source strings into mesh discovery sources.
pub fn parse_mesh_discovery_sources(raw: &[String]) -> Vec<MeshDiscoverySource> {
    raw.iter()
        .filter_map(|value| parse_mesh_discovery_source(value))
        .collect()
}

fn parse_mesh_discovery_source(raw: &str) -> Option<MeshDiscoverySource> {
    match raw.trim().to_lowercase().as_str() {
        "local_runtime" | "local" => Some(MeshDiscoverySource::LocalRuntime),
        "entity_graph" | "graph" => Some(MeshDiscoverySource::EntityGraph),
        "mqtt" => Some(MeshDiscoverySource::Mqtt),
        "ros2" | "ros_2" => Some(MeshDiscoverySource::Ros2),
        "dds" => Some(MeshDiscoverySource::Dds),
        "mdns" => Some(MeshDiscoverySource::Mdns),
        "ble" => Some(MeshDiscoverySource::Ble),
        "wifi_subnet" | "wifi" | "subnet" => Some(MeshDiscoverySource::WifiSubnet),
        "manual_config" | "manual" => Some(MeshDiscoverySource::ManualConfig),
        _ => None,
    }
}

/// Default discovery sources when callers omit an explicit list.
pub fn default_mesh_discovery_sources() -> Vec<MeshDiscoverySource> {
    vec![
        MeshDiscoverySource::LocalRuntime,
        MeshDiscoverySource::EntityGraph,
    ]
}

/// Infer mesh transport from entity metadata tags, labels, or provider name.
pub fn infer_transport_from_entity(entity: &EntityRecord) -> MeshTransport {
    for token in entity
        .tags
        .iter()
        .chain(entity.labels.iter())
        .map(|value| value.to_ascii_lowercase())
    {
        if token.contains("mqtt") {
            return MeshTransport::Mqtt;
        }
        if token.contains("ros2") || token.contains("ros_2") {
            return MeshTransport::Ros2;
        }
        if token.contains("dds") {
            return MeshTransport::Dds;
        }
        if token.contains("mdns") {
            return MeshTransport::Mdns;
        }
        if token.contains("ble") || token.contains("bluetooth") {
            return MeshTransport::Ble;
        }
    }
    if let Some(provider) = entity.provider.as_deref() {
        let provider = provider.to_ascii_lowercase();
        if provider.contains("mqtt") {
            return MeshTransport::Mqtt;
        }
        if provider.contains("ros2") {
            return MeshTransport::Ros2;
        }
        if provider.contains("dds") {
            return MeshTransport::Dds;
        }
    }
    MeshTransport::LocalRuntime
}

fn live_transport_enabled(source: &MeshDiscoverySource) -> bool {
    match source {
        MeshDiscoverySource::Mqtt => {
            std::env::var("SPANDA_LIVE_MQTT").ok().as_deref() == Some("1")
        }
        MeshDiscoverySource::Ros2 | MeshDiscoverySource::Dds => {
            std::env::var("SPANDA_LIVE_ROS2").ok().as_deref() == Some("1")
                || std::env::var("SPANDA_ROS2_LIVE").ok().as_deref() == Some("1")
        }
        _ => false,
    }
}

fn transport_for_source(source: MeshDiscoverySource) -> MeshTransport {
    match source {
        MeshDiscoverySource::Mqtt => MeshTransport::Mqtt,
        MeshDiscoverySource::Ros2 => MeshTransport::Ros2,
        MeshDiscoverySource::Dds => MeshTransport::Dds,
        MeshDiscoverySource::Mdns => MeshTransport::Mdns,
        MeshDiscoverySource::Ble => MeshTransport::Ble,
        MeshDiscoverySource::WifiSubnet => MeshTransport::WifiSubnet,
        MeshDiscoverySource::ManualConfig => MeshTransport::ManualConfig,
        MeshDiscoverySource::LocalRuntime => MeshTransport::LocalRuntime,
        MeshDiscoverySource::EntityGraph => MeshTransport::LocalRuntime,
    }
}

fn discovery_match_to_mesh_node(
    discovery: &DiscoveryMatch,
    transport: MeshTransport,
) -> MeshNode {
    let latency_ms = discovery.probe.latency_ms.map(|value| value as u32);
    MeshNode {
        entity_id: discovery.device_id.clone(),
        node_id: format!("node-{}", discovery.device_id),
        transport,
        reachable: discovery.probe.reachable,
        neighbors: Vec::new(),
        capabilities: Vec::new(),
        health: if discovery.probe.reachable {
            EntityHealthStatus::Healthy
        } else {
            EntityHealthStatus::Offline
        },
        readiness: Default::default(),
        trust_score: if discovery.probe.reachable { 0.75 } else { 0.2 },
        latency_ms,
        bandwidth_kbps: None,
        packet_loss: if discovery.probe.reachable {
            Some(0.0)
        } else {
            Some(1.0)
        },
        hop_count: Some(1),
        last_seen: None,
        battery_percent: None,
        role: Default::default(),
        coordinator_status: Default::default(),
        supported_protocols: vec!["secure_messaging".into()],
        security_identity: Default::default(),
    }
}

/// Probe live transports when env-gated and merge nodes into discovery results.
pub fn discover_live_transport_nodes(source: MeshDiscoverySource) -> Vec<MeshNode> {
    if !live_transport_enabled(&source) {
        return Vec::new();
    }
    let timeout_ms = std::env::var("SPANDA_MESH_DISCOVERY_TIMEOUT_MS")
        .ok()
        .and_then(|value| value.parse().ok())
        .unwrap_or(2000);
    let matches = match source {
        MeshDiscoverySource::Mqtt => spanda_config::discovery_live::probe_mqtt(timeout_ms),
        MeshDiscoverySource::Ros2 => spanda_config::discovery_live::probe_ros2(timeout_ms),
        MeshDiscoverySource::Dds => spanda_config::discovery_live::probe_ros2(timeout_ms),
        _ => Vec::new(),
    };
    let transport = transport_for_source(source);
    matches
        .iter()
        .map(|discovery| discovery_match_to_mesh_node(discovery, transport.clone()))
        .collect()
}

/// Merge a live transport node with an existing registry-backed node when ids match.
pub fn enrich_node_with_live_transport(node: &mut MeshNode, live: &MeshNode) {
    node.transport = live.transport.clone();
    if live.latency_ms.is_some() {
        node.latency_ms = live.latency_ms;
    }
    if live.packet_loss.is_some() {
        node.packet_loss = live.packet_loss;
    }
    node.reachable = node.reachable && live.reachable;
}

/// Attach live transport metrics to registry nodes when probes match entity ids.
pub fn apply_live_transport_probes(
    _registry: &EntityRegistry,
    nodes: &mut [MeshNode],
    sources: &[MeshDiscoverySource],
) {
    for source in sources {
        if !live_transport_enabled(source) {
            continue;
        }
        for live in discover_live_transport_nodes(source.clone()) {
            if let Some(node) = nodes.iter_mut().find(|n| n.entity_id == live.entity_id) {
                enrich_node_with_live_transport(node, &live);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use spanda_config::entity::{EntityKind, EntityRecord};

    #[test]
    fn parse_mesh_sources_from_strings() {
        let parsed = parse_mesh_discovery_sources(&[
            "local_runtime".into(),
            "mqtt".into(),
            "ros2".into(),
        ]);
        assert_eq!(
            parsed,
            vec![
                MeshDiscoverySource::LocalRuntime,
                MeshDiscoverySource::Mqtt,
                MeshDiscoverySource::Ros2,
            ]
        );
    }

    #[test]
    fn infer_transport_from_entity_tags() {
        let entity = EntityRecord {
            id: "gw-1".into(),
            entity_type: EntityKind::Gateway,
            tags: vec!["mqtt-bridge".into()],
            ..EntityRecord::default()
        };
        assert_eq!(
            infer_transport_from_entity(&entity),
            MeshTransport::Mqtt
        );
    }
}
