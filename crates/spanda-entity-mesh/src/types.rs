//! Core types for the Spanda Autonomous Entity Mesh layer.
//!
use serde::{Deserialize, Serialize};
use spanda_config::entity::{EntityHealthStatus, EntityReadinessStatus, EntitySecurityIdentity};
use std::collections::{BTreeMap, HashSet};

/// Discovery transport/provider used by a mesh node.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MeshTransport {
    LocalRuntime,
    Dds,
    Ros2,
    Mqtt,
    Mdns,
    Ble,
    WifiSubnet,
    ManualConfig,
    Lora,
    Satellite,
    Ethernet,
    FiveG,
    Custom(String),
}

impl MeshTransport {
    pub fn as_str(&self) -> &str {
        match self {
            Self::LocalRuntime => "local_runtime",
            Self::Dds => "dds",
            Self::Ros2 => "ros2",
            Self::Mqtt => "mqtt",
            Self::Mdns => "mdns",
            Self::Ble => "ble",
            Self::WifiSubnet => "wifi_subnet",
            Self::ManualConfig => "manual_config",
            Self::Lora => "lora",
            Self::Satellite => "satellite",
            Self::Ethernet => "ethernet",
            Self::FiveG => "5g",
            Self::Custom(name) => name.as_str(),
        }
    }
}

/// Mesh node role in coordination topology.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum MeshNodeRole {
    #[default]
    Participant,
    Relay,
    Coordinator,
    BackupCoordinator,
    Gateway,
    Observer,
}

/// Coordinator election / assignment status for a node.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum MeshCoordinatorStatus {
    #[default]
    None,
    Active,
    Backup,
    Candidate,
    Failed,
    Suspended,
}

/// Routing mode for mesh message delivery.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MeshRoutingMode {
    Direct,
    Relay,
    Coordinator,
    Broadcast,
    Multicast,
    CapabilityBased,
    TrustWeighted,
    ReadinessWeighted,
    Emergency,
}

/// Message priority for mesh routing decisions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum MeshMessagePriority {
    #[default]
    Normal,
    High,
    SafetyCritical,
    Emergency,
}

/// Trust requirement for mesh message delivery.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MeshTrustRequirement {
    pub minimum_trust_score: f64,
    pub require_signed: bool,
    pub require_identity_match: bool,
    pub block_untrusted_relays: bool,
}

impl Default for MeshTrustRequirement {
    fn default() -> Self {
        Self {
            minimum_trust_score: 0.5,
            require_signed: true,
            require_identity_match: true,
            block_untrusted_relays: true,
        }
    }
}

/// Neighbor link between mesh nodes.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MeshNeighbor {
    pub entity_id: String,
    pub node_id: String,
    pub transport: MeshTransport,
    pub reachable: bool,
    pub latency_ms: Option<u32>,
    pub packet_loss: Option<f64>,
    pub last_seen: Option<String>,
}

/// Link quality metrics between two mesh nodes.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MeshLink {
    pub from_entity: String,
    pub to_entity: String,
    pub transport: MeshTransport,
    pub latency_ms: u32,
    pub bandwidth_kbps: Option<u32>,
    pub packet_loss: f64,
    pub trusted: bool,
    pub active: bool,
}

/// Capability advertisement from a mesh node.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MeshCapabilityAdvertisement {
    pub entity_id: String,
    pub capabilities: Vec<String>,
    pub advertised_at: String,
    pub transport: MeshTransport,
    pub verified: bool,
}

/// Trust state projected onto a mesh node.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct MeshTrustState {
    pub entity_id: String,
    pub trust_score: f64,
    pub trust_category: String,
    pub identity_verified: bool,
    pub last_evaluated: Option<String>,
}

/// Readiness state projected onto a mesh node.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct MeshReadinessState {
    pub entity_id: String,
    pub readiness_score: f64,
    pub mission_ready: bool,
    pub blocked_reasons: Vec<String>,
    pub last_evaluated: Option<String>,
}

/// Heartbeat from a mesh node.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MeshHeartbeat {
    pub entity_id: String,
    pub node_id: String,
    pub timestamp: String,
    pub sequence: u64,
    pub health: EntityHealthStatus,
    pub readiness: EntityReadinessStatus,
    pub trust_score: f64,
    pub battery_percent: Option<u8>,
}

/// Route through the mesh graph.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MeshRoute {
    pub route_id: String,
    pub source_entity: String,
    pub target_entity: String,
    pub mode: MeshRoutingMode,
    pub hops: Vec<String>,
    pub trust_score: f64,
    pub readiness_score: f64,
    pub latency_ms: u32,
    pub trusted: bool,
    pub evidence: Vec<String>,
}

/// Partitioned cluster of entities during network split.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MeshCluster {
    pub cluster_id: String,
    pub entity_ids: Vec<String>,
    pub coordinator_entity: Option<String>,
    pub partition_id: String,
}

/// Active or historical network partition.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MeshPartition {
    pub partition_id: String,
    pub detected_at: String,
    pub resolved_at: Option<String>,
    pub clusters: Vec<MeshCluster>,
    pub affected_entities: Vec<String>,
    pub active: bool,
}

/// Elected mesh leader/coordinator (communication role only).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MeshLeader {
    pub entity_id: String,
    pub elected_at: String,
    pub election_method: String,
    pub trust_score: f64,
    pub readiness_score: f64,
    pub term: u64,
}

/// Active mesh coordinator state.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MeshCoordinator {
    pub entity_id: String,
    pub status: MeshCoordinatorStatus,
    pub leader: Option<MeshLeader>,
    pub backup_entity_ids: Vec<String>,
    pub quorum_size: u32,
}

/// Full mesh topology snapshot.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct MeshTopology {
    pub nodes: Vec<MeshNode>,
    pub links: Vec<MeshLink>,
    pub coordinator: Option<MeshCoordinator>,
    pub partitions: Vec<MeshPartition>,
    pub updated_at: String,
}

/// Mesh node state tracked by the entity mesh layer.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MeshNode {
    pub entity_id: String,
    pub node_id: String,
    pub transport: MeshTransport,
    pub reachable: bool,
    pub neighbors: Vec<MeshNeighbor>,
    pub capabilities: Vec<String>,
    pub health: EntityHealthStatus,
    pub readiness: EntityReadinessStatus,
    pub trust_score: f64,
    pub latency_ms: Option<u32>,
    pub bandwidth_kbps: Option<u32>,
    pub packet_loss: Option<f64>,
    pub hop_count: Option<u32>,
    pub last_seen: Option<String>,
    pub battery_percent: Option<u8>,
    pub role: MeshNodeRole,
    pub coordinator_status: MeshCoordinatorStatus,
    pub supported_protocols: Vec<String>,
    pub security_identity: EntitySecurityIdentity,
}

/// Auditable mesh message envelope (wraps secure messaging).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MeshMessage {
    pub message_id: String,
    pub source_entity: String,
    pub target_entity: Option<String>,
    pub target_capability: Option<String>,
    pub route: Option<MeshRoute>,
    pub priority: MeshMessagePriority,
    pub ttl_secs: u32,
    pub timestamp: String,
    pub nonce: String,
    pub signature: Option<String>,
    pub encryption_required: bool,
    pub trust_requirement: MeshTrustRequirement,
    pub payload_hash: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signed_envelope: Option<spanda_security::SignedMessage>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payload: Option<String>,
}

/// Sync state for partition merge and reconnection.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct MeshSyncState {
    pub entity_id: String,
    pub health: EntityHealthStatus,
    pub readiness: EntityReadinessStatus,
    pub trust_score: f64,
    pub mission_progress: Option<f64>,
    pub decision_trace_count: u32,
    pub recovery_event_count: u32,
    pub audit_event_count: u32,
    pub config_version: Option<String>,
    pub synced_at: String,
}

/// Conflict detected during partition merge.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MeshConflictKind {
    MissionState,
    Decision,
    DuplicateLeader,
    StalePolicy,
    DivergedRecovery,
    DuplicateIdentity,
    CapabilityAdvertisement,
}

/// Conflict record for merge resolution.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MeshConflict {
    pub conflict_id: String,
    pub kind: MeshConflictKind,
    pub entity_ids: Vec<String>,
    pub description: String,
    pub detected_at: String,
    pub resolved: bool,
    pub resolution: Option<String>,
}

/// Policy for partition behavior.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MeshPartitionPolicy {
    pub allow_safe_missions: bool,
    pub pause_unsafe_missions: bool,
    pub require_local_coordinator: bool,
    pub min_trust_for_relay: f64,
    pub max_partition_duration_secs: Option<u32>,
}

impl Default for MeshPartitionPolicy {
    fn default() -> Self {
        Self {
            allow_safe_missions: true,
            pause_unsafe_missions: true,
            require_local_coordinator: true,
            min_trust_for_relay: 0.6,
            max_partition_duration_secs: Some(3600),
        }
    }
}

/// Evidence report when a partition is detected.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MeshPartitionReport {
    pub partition: MeshPartition,
    pub policy_applied: MeshPartitionPolicy,
    pub paused_missions: Vec<String>,
    pub continued_missions: Vec<String>,
    pub local_coordinator: Option<String>,
    pub evidence: Vec<String>,
}

/// Policy for partition merge conflict resolution.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MeshMergePolicy {
    pub prefer_higher_trust: bool,
    pub prefer_newer_timestamp: bool,
    pub merge_audit_trails: bool,
    pub block_high_risk_on_conflict: bool,
}

impl Default for MeshMergePolicy {
    fn default() -> Self {
        Self {
            prefer_higher_trust: true,
            prefer_newer_timestamp: true,
            merge_audit_trails: true,
            block_high_risk_on_conflict: true,
        }
    }
}

/// Plan for merging partitioned mesh state.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MeshMergePlan {
    pub merge_id: String,
    pub partition_ids: Vec<String>,
    pub conflicts: Vec<MeshConflict>,
    pub sync_actions: Vec<String>,
    pub audit_merge_actions: Vec<String>,
    pub entity_graph_updates: Vec<String>,
}

/// Report after partition merge completes.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MeshMergeReport {
    pub merge_id: String,
    pub merged_at: String,
    pub plan: MeshMergePlan,
    pub conflicts_resolved: u32,
    pub conflicts_remaining: u32,
    pub evidence: Vec<String>,
}

/// Discovery source for entity mesh nodes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MeshDiscoverySource {
    LocalRuntime,
    Dds,
    Ros2,
    Mqtt,
    Mdns,
    Ble,
    WifiSubnet,
    ManualConfig,
    EntityGraph,
}

/// Result of mesh discovery scan.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct MeshDiscoveryResult {
    pub discovered: Vec<MeshNode>,
    pub sources: Vec<MeshDiscoverySource>,
    pub new_entities: u32,
    pub updated_entities: u32,
}

/// Mesh health summary.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct MeshHealthReport {
    pub total_nodes: u32,
    pub reachable_nodes: u32,
    pub offline_nodes: Vec<String>,
    pub degraded_links: Vec<String>,
    pub active_partitions: u32,
    pub coordinator_status: Option<MeshCoordinatorStatus>,
    pub average_trust_score: f64,
    pub average_latency_ms: u32,
    pub issues: Vec<String>,
}

/// Assurance evidence bundle for mesh operations.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct MeshAssuranceEvidence {
    pub topology_snapshot: Option<MeshTopology>,
    pub route_evidence: Vec<String>,
    pub partition_history: Vec<MeshPartition>,
    pub election_evidence: Vec<String>,
    pub message_trust_evidence: Vec<String>,
    pub delegation_evidence: Vec<String>,
    pub sync_evidence: Vec<String>,
}

/// Diagnosis explanation for mesh communication failures.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct MeshDiagnosisReport {
    pub question: String,
    pub answer: String,
    pub affected_entities: Vec<String>,
    pub route_used: Option<MeshRoute>,
    pub partition_active: bool,
    pub evidence: Vec<String>,
}

/// Delegation candidate for mission continuity through mesh.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MeshDelegationCandidate {
    pub entity_id: String,
    pub capabilities: Vec<String>,
    pub trust_score: f64,
    pub readiness_score: f64,
    pub route: Option<MeshRoute>,
    pub eligible: bool,
    pub rejection_reasons: Vec<String>,
}

/// Nonce registry for mesh message replay protection.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct MeshNonceRegistry {
    pub seen: HashSet<String>,
}

impl MeshNonceRegistry {
    pub fn register(&mut self, nonce: &str) -> Result<(), String> {
        if nonce.is_empty() {
            return Err("empty mesh nonce rejected".into());
        }
        if !self.seen.insert(nonce.to_string()) {
            return Err(format!("replayed mesh nonce '{nonce}' rejected"));
        }
        Ok(())
    }

    pub fn clear(&mut self) {
        self.seen.clear();
    }
}

/// Main entity mesh state container.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EntityMesh {
    pub mesh_id: String,
    pub nodes: BTreeMap<String, MeshNode>,
    pub links: Vec<MeshLink>,
    pub topology: MeshTopology,
    pub coordinator: Option<MeshCoordinator>,
    pub partitions: Vec<MeshPartition>,
    pub capability_ads: Vec<MeshCapabilityAdvertisement>,
    pub partition_policy: MeshPartitionPolicy,
    pub merge_policy: MeshMergePolicy,
    pub nonce_registry: MeshNonceRegistry,
    pub route_history: Vec<MeshRoute>,
    pub sync_states: BTreeMap<String, MeshSyncState>,
    pub conflicts: Vec<MeshConflict>,
    pub updated_at: String,
}

impl Default for EntityMesh {
    fn default() -> Self {
        Self {
            mesh_id: "default".into(),
            nodes: BTreeMap::new(),
            links: Vec::new(),
            topology: MeshTopology::default(),
            coordinator: None,
            partitions: Vec::new(),
            capability_ads: Vec::new(),
            partition_policy: MeshPartitionPolicy::default(),
            merge_policy: MeshMergePolicy::default(),
            nonce_registry: MeshNonceRegistry::default(),
            route_history: Vec::new(),
            sync_states: BTreeMap::new(),
            conflicts: Vec::new(),
            updated_at: chrono::Utc::now().to_rfc3339(),
        }
    }
}
