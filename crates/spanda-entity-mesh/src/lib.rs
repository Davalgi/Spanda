//! Spanda Autonomous Entity Mesh — trust-aware inter-entity communication and resilience.
//!
//! Entity Mesh sits **above** transport providers (MQTT, DDS, ROS2, BLE, etc.) and adds:
//! entity discovery, capability discovery, trust-aware routing, partition handling,
//! coordinator election, mission-aware delegation, and state synchronization.
//!
//! It does **not** implement packet routing or replace existing transports.
//! All mesh messages use the existing secure messaging model. Takeover decisions still
//! flow through Recovery Orchestrator / Mission Continuity.
//!
pub mod delegation;
pub mod discovery;
pub mod election;
pub mod format;
pub mod heartbeat;
pub mod integration;
pub mod partition;
pub mod routing;
pub mod security;
pub mod sync;
pub mod types;

pub use delegation::{
    delegation_requires_recovery_orchestrator, find_delegation_candidates, plan_delegation,
    MeshDelegationRequest, MeshDelegationResult,
};
pub use discovery::{
    apply_discovery, build_entity_mesh, discover_mesh_nodes, enrich_from_entity_graph,
    find_capability, inspect_node, list_nodes, mesh_to_entity_graph_edges, rebuild_topology,
    refresh_capability_ads,
};
pub use election::{
    apply_coordinator, coordinator_is_communication_role_only, elect_coordinator,
    MeshElectionMethod, MeshElectionOptions,
};
pub use format::{
    format_capability_results, format_health, format_merge_report, format_node_list, format_route,
    format_topology, mesh_graph_json, MeshFormat,
};
pub use heartbeat::{
    coordinator_failed, evaluate_mesh_health, ingest_heartbeat, MeshHeartbeatPolicy,
};
pub use integration::{
    build_assurance_evidence, diagnose_mesh, mesh_readiness_impact, mesh_recovery_actions,
    readiness_blocked_by_partition, MeshReadinessImpact, MeshRecoveryAction,
};
pub use partition::{
    apply_partition_policy, build_merge_plan, detect_partitions, merge_partitions,
    partition_blocks_high_risk, simulate_partition,
};
pub use routing::{compute_route, record_route, MeshRouteOptions};
pub use security::{
    build_mesh_message, sign_mesh_message, validate_mesh_message, MeshSecurityVerdict,
};
pub use sync::{
    apply_sync_states, collect_sync_payload, entity_to_sync_state, sync_payload_is_secret_free,
    MeshSyncOptions,
};
pub use types::*;
