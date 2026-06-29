//! Runtime health check types shared across capability analysis and interpreter polling.
//!
use serde::{Deserialize, Serialize};

/// Component health status values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum HealthStatus {
    Healthy,
    Degraded,
    Warning,
    Critical,
    Failed,
    Unknown,
    Offline,
    Unsafe,
}

/// Result of evaluating a single health check.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HealthCheckResult {
    pub name: String,
    pub target: String,
    pub target_kind: String,
    pub metric: String,
    pub operator: String,
    pub threshold: String,
    pub status: HealthStatus,
    pub message: Option<String>,
}

/// Aggregated health report for a program.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HealthReport {
    pub checks: Vec<HealthCheckResult>,
    pub overall: HealthStatus,
    pub policies: Vec<String>,
}

/// Health traceability matrix row.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HealthTraceRow {
    pub component: String,
    pub health_check: String,
    pub metric: String,
    pub threshold: String,
    pub status: String,
    pub action: Option<String>,
}
