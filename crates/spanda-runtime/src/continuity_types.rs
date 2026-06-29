//! Mission continuity enums and structs shared between runtime and assurance layers.
//!
use crate::recovery_types::ValidationGateResult;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Trigger that initiates a continuity evaluation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContinuityTrigger {
    RobotFailed,
    RobotDegraded,
    DeviceDisconnected,
    FleetMemberOffline,
    SwarmMemberLost,
    CommunicationInterrupted,
    BatteryCritical,
    HardwareCapabilityLost,
}

/// Scope for succession and delegation targets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SuccessionScope {
    Robot,
    Device,
    Fleet,
    Swarm,
    Group,
    Crowd,
    MissionCluster,
}

/// Takeover execution mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TakeoverMode {
    /// Continue from last checkpoint.
    Resume,
    /// Start mission again from the beginning.
    Restart,
    /// Restart only the failed stage.
    PartialRestart,
    /// Backup agent already synchronized.
    ShadowTakeover,
    /// Immediate replacement.
    HotTakeover,
    /// Replacement initialized after failure.
    ColdTakeover,
    /// Transfer control to operator.
    HumanTakeover,
}

/// Continuation decision from the decision engine.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContinuationDecision {
    Continue,
    Restart,
    PartialRestart,
    Abort,
    HumanApprovalRequired,
}

/// Mission execution state snapshot for continuity checkpoints.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MissionExecutionState {
    pub plan: String,
    pub current_step: Option<String>,
    pub status: String,
}

/// Snapshot of mission, robot, health, safety, and capability state at a checkpoint.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MissionCheckpoint {
    pub name: String,
    pub progress_percent: f64,
    pub mission_state: MissionExecutionState,
    pub robot_state: String,
    pub health_state: String,
    pub safety_state: String,
    pub capability_state: String,
}

/// Full mission state snapshot for transfer.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MissionStateSnapshot {
    pub mission: String,
    pub completed_steps: Vec<String>,
    pub current_goal: Option<String>,
    pub progress_percent: f64,
    pub checkpoints: Vec<MissionCheckpoint>,
}

/// Payload transferred to a successor.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MissionStateTransfer {
    pub from_entity: String,
    pub to_entity: String,
    pub snapshot: MissionStateSnapshot,
    pub transferable: bool,
    pub transfer_notes: Vec<String>,
}

/// Input context for continuity evaluation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContinuityContext {
    pub mission: String,
    pub failed_entity: String,
    pub trigger: ContinuityTrigger,
    pub progress_percent: f64,
    pub scope: SuccessionScope,
    pub current_step: Option<String>,
    pub checkpoints: Vec<String>,
}

impl Default for ContinuityContext {
    fn default() -> Self {
        Self {
            mission: "default_mission".into(),
            failed_entity: "Rover".into(),
            trigger: ContinuityTrigger::RobotFailed,
            progress_percent: 0.0,
            scope: SuccessionScope::Robot,
            current_step: None,
            checkpoints: Vec::new(),
        }
    }
}

/// Continuity policy extracted from declarations.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContinuityPolicySpec {
    pub name: String,
    pub triggers: Vec<(String, Vec<String>)>,
}

/// Takeover coordination report.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TakeoverReport {
    pub mission: String,
    pub failed_entity: String,
    pub successor: String,
    pub mode: TakeoverMode,
    pub decision: ContinuationDecision,
    pub state_transfer: MissionStateTransfer,
    pub safety_gates: Vec<ValidationGateResult>,
    pub evidence: ContinuityEvidence,
    pub succeeded: bool,
    pub diagnosis: String,
}

/// Assurance evidence for continuity decisions.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContinuityEvidence {
    pub takeover_evidence: Vec<String>,
    pub delegation_evidence: Vec<String>,
    pub continuity_evidence: Vec<String>,
    pub safety_gates: Vec<ValidationGateResult>,
    pub diagnosis: Option<String>,
    pub recovery_outcome: Option<String>,
}

/// On-disk checkpoint index keyed by `mission::robot`.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ContinuityCheckpointStore {
    pub entries: HashMap<String, MissionStateSnapshot>,
}
