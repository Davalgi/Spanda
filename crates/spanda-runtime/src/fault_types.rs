//! Core runtime fault types and health status values.
//!
use serde::{Deserialize, Serialize};

/// Overall runtime health status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[derive(Default)]
pub enum RuntimeHealthStatus {
    Healthy,
    Warning,
    Degraded,
    Critical,
    Crashed,
    Rebooted,
    #[default]
    Unknown,
}

/// Heartbeat monitoring status for a runtime target.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HeartbeatStatus {
    pub target: String,
    pub last_seen_ms: f64,
    pub interval_ms: f64,
    pub timeout_ms: f64,
    pub missed_count: u32,
    pub status: RuntimeHealthStatus,
}

/// Process-level health snapshot.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProcessHealth {
    pub name: String,
    pub pid: Option<u32>,
    pub status: RuntimeHealthStatus,
    pub exit_code: Option<i32>,
    pub restart_count: u32,
    pub uptime_ms: f64,
}

/// Aggregated runtime health for a program.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RuntimeHealth {
    pub overall: RuntimeHealthStatus,
    pub heartbeats: Vec<HeartbeatStatus>,
    pub processes: Vec<ProcessHealth>,
    pub active_faults: Vec<RuntimeFault>,
    pub uptime_ms: f64,
    pub boot_id: Option<String>,
}

/// Kind of runtime fault detected.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RuntimeFaultKind {
    MemoryLeak,
    ProcessCrash,
    RuntimePanic,
    UnexpectedReboot,
    OsReboot,
    WatchdogTimeout,
    OutOfMemory,
    Deadlock,
    TaskStarvation,
    ProviderCrash,
    PackageCrash,
    SensorDriverCrash,
    ActuatorDriverCrash,
    NetworkStackCrash,
    RestartLoop,
    AbnormalShutdown,
    CpuOverload,
    MemoryPressure,
    DiskPressure,
    TelemetryDrop,
    HeartbeatLoss,
}

/// A detected runtime fault with evidence.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RuntimeFault {
    pub kind: RuntimeFaultKind,
    pub target: String,
    pub status: RuntimeHealthStatus,
    pub message: String,
    pub evidence: FaultEvidence,
    pub detected_at_ms: f64,
}

/// Supporting evidence for a fault diagnosis.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct FaultEvidence {
    pub metric: Option<String>,
    pub value: Option<String>,
    pub threshold: Option<String>,
    pub boot_id: Option<String>,
    pub exit_code: Option<i32>,
    pub stack_trace: Option<String>,
    pub related_events: Vec<String>,
}

/// Chronological fault timeline entry.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FaultTimeline {
    pub timestamp_ms: f64,
    pub event: String,
    pub fault_kind: Option<RuntimeFaultKind>,
    pub target: String,
    pub status: RuntimeHealthStatus,
    pub detail: Option<String>,
}

/// Options for fault scanning.
#[derive(Debug, Clone, Default)]
pub struct FaultScanOptions {
    pub inject_crash: bool,
    pub inject_memory_leak: bool,
    pub inject_reboot: bool,
    pub inject_heartbeat_loss: bool,
    pub inject_resource_pressure: bool,
    pub sim_time_ms: f64,
}

/// Full fault scan report for a program.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FaultScanReport {
    pub source: String,
    pub health: RuntimeHealth,
    pub faults: Vec<RuntimeFault>,
    pub timeline: Vec<FaultTimeline>,
    pub heartbeats_configured: u32,
    pub memory_watches_configured: u32,
    pub resource_watches_configured: u32,
    pub restart_policies_configured: u32,
    pub passed: bool,
}

impl RuntimeHealthStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Healthy => "Healthy",
            Self::Warning => "Warning",
            Self::Degraded => "Degraded",
            Self::Critical => "Critical",
            Self::Crashed => "Crashed",
            Self::Rebooted => "Rebooted",
            Self::Unknown => "Unknown",
        }
    }

    pub fn severity_rank(&self) -> u8 {
        match self {
            Self::Healthy => 0,
            Self::Warning => 1,
            Self::Degraded => 2,
            Self::Rebooted => 3,
            Self::Critical => 4,
            Self::Crashed => 5,
            Self::Unknown => 6,
        }
    }
}

impl RuntimeFaultKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::MemoryLeak => "memory_leak",
            Self::ProcessCrash => "process_crash",
            Self::RuntimePanic => "runtime_panic",
            Self::UnexpectedReboot => "unexpected_reboot",
            Self::OsReboot => "os_reboot",
            Self::WatchdogTimeout => "watchdog_timeout",
            Self::OutOfMemory => "out_of_memory",
            Self::Deadlock => "deadlock",
            Self::TaskStarvation => "task_starvation",
            Self::ProviderCrash => "provider_crash",
            Self::PackageCrash => "package_crash",
            Self::SensorDriverCrash => "sensor_driver_crash",
            Self::ActuatorDriverCrash => "actuator_driver_crash",
            Self::NetworkStackCrash => "network_stack_crash",
            Self::RestartLoop => "restart_loop",
            Self::AbnormalShutdown => "abnormal_shutdown",
            Self::CpuOverload => "cpu_overload",
            Self::MemoryPressure => "memory_pressure",
            Self::DiskPressure => "disk_pressure",
            Self::TelemetryDrop => "telemetry_drop",
            Self::HeartbeatLoss => "heartbeat_loss",
        }
    }
}
