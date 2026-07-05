//! Bio-inspired resilient autonomy architecture for Spanda.
//!
//! Extends distributed decision architecture with engineering patterns inspired
//! by biological nervous systems: reflex arcs, peripheral autonomy, sensory fusion,
//! attention, homeostasis, platform immunity, operational memory, habituation,
//! damage-risk modeling, adaptive recovery, and maintenance windows.
//!
pub mod adaptive_recovery;
pub mod attention;
pub mod damage_risk;
pub mod entity;
pub mod format;
pub mod fusion;
pub mod habituation;
pub mod homeostasis;
pub mod immunity;
pub mod maintenance;
pub mod memory;
pub mod peripheral;
pub mod reflex;
pub mod registry;
pub mod runtime;
pub mod trace_buffer;
pub mod types;

pub use adaptive_recovery::{
    compute_recovery_confidence, compute_strategy_preference, AdaptiveRecoveryPolicy,
    RecoveryConfidence, RecoveryHistory, StrategyPreference, StrategySuccessRate,
};
pub use attention::{
    compute_attention_score, rank_events, AttentionPolicy, AttentionScore, AttentionWindow,
    EventPriority, SignalPriority, SuppressionRule,
};
pub use damage_risk::{
    evaluate_damage_risk, DamageRisk, HarmPotential, ProtectiveAction, RiskSignal, SafetyPainIndex,
};
pub use entity::{attach_default_autonomy_profile, enrich_entity_autonomy, EntityAutonomyContext};
pub use registry::apply_registry_autonomy_profiles;
pub use runtime::{
    entity_damage_risk_index, platform_telemetry_snapshot, recovery_confidence_from_history,
    sensor_readings_from_entity, update_platform_telemetry, PlatformTelemetrySnapshot,
};
pub use fusion::{
    detect_signal_conflict, fuse_observations, ConfidencePolicy, ConfidenceScore, FusedObservation,
    SensorConfidence, SignalAgreement, SignalConflict,
};
pub use habituation::{
    analyze_alert_fatigue, apply_habituation, apply_sensitization, AlertEscalation,
    AlertFatigueMetric, AlertSuppression, HabituationPolicy, RepetitionPattern,
    SensitizationPolicy,
};
pub use homeostasis::{
    evaluate_homeostasis, CorrectionAction, DriftSignal, HomeostasisPolicy, StabilityMetric,
    StabilityRange, StabilityReport,
};
pub use immunity::{
    evaluate_immunity, evaluate_quarantine_decision, ImmuneEvent, ImmunePolicy, IsolationDecision,
    QuarantineAction, ThreatResponse, TrustBoundaryViolation,
};
pub use maintenance::{
    CalibrationWindow, LowActivityMode, MaintenanceWindow, ScheduledRecovery, SleepMode,
    UpdateWindow,
};
pub use memory::{
    build_operational_memory_model, categorize_memory, enrich_entity_memory_refs,
    map_trace_to_memory, EpisodicMemory, MemoryCategory, OperationalMemoryModel,
    ProceduralMemory, ReflexMemory, SemanticMemory, WorkingMemory,
};
pub use peripheral::{
    EdgeCoordinator, LocalAutonomyNode, PeripheralAutonomyLayer, PeripheralNode,
    RegionalCoordinator,
};
pub use reflex::{
    evaluate_reflex_priority, list_reflex_actions, ReflexAction, ReflexArc, ReflexController,
    ReflexTrace,
};
pub use trace_buffer::{
    default_trace_store_path, list_recorded_reflex_traces, load_reflex_traces_from_disk,
    persist_reflex_traces_to_disk, record_reflex_trace, record_runtime_reflex,
};
pub use types::*;
