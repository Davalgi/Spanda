//! Platform event emission for readiness evaluation.
//!
use spanda_audit::platform_event::names;
use spanda_audit::{AuditRuntime, PlatformEvent};
use spanda_runtime::publish_platform_event;
use serde_json::json;

use crate::entity_health::EntityHealthReport;
use crate::entity_readiness::EntityReadinessReport;

/// Record readiness platform events for an entity readiness report.
pub fn record_readiness_platform_event(
    audit: &mut AuditRuntime,
    report: &EntityReadinessReport,
) {
    let event = PlatformEvent::new(
        names::READINESS_CHANGED,
        "spanda-readiness",
        json!({
            "entity_type": report.entity_type,
            "readiness_status": report.readiness_status,
            "mission_ready": report.mission_ready,
            "score": report.score,
            "issue_count": report.issues.len(),
            "sources": report.sources,
        }),
    )
    .with_entity_id(report.entity_id.clone());
    publish_platform_event(Some(audit), &event);

    if !report.mission_ready {
        let gate_event = PlatformEvent::new(
            names::READINESS_GATE_FAILED,
            "spanda-readiness",
            json!({
                "entity_type": report.entity_type,
                "score": report.score,
                "issues": report.issues,
            }),
        )
        .with_entity_id(report.entity_id.clone());
        publish_platform_event(Some(audit), &gate_event);
    }
}

/// Record health platform events for an entity health report.
pub fn record_entity_health_platform_events(report: &EntityHealthReport) {
    let event = PlatformEvent::new(
        names::HEALTH_CHANGED,
        "spanda-readiness",
        json!({
            "entity_type": report.entity_type,
            "health_status": report.health_status,
            "diagnostic_count": report.diagnostics.len(),
            "health_checks_failed": report.metrics.health_checks_failed,
            "sources": report.sources,
        }),
    )
    .with_entity_id(report.entity_id.clone());
    publish_platform_event(None, &event);

    if report.metrics.health_checks_failed > 0 {
        let failed_event = PlatformEvent::new(
            names::HEALTH_CHECK_FAILED,
            "spanda-readiness",
            json!({
                "entity_type": report.entity_type,
                "failed_checks": report.metrics.health_checks_failed,
                "passed_checks": report.metrics.health_checks_passed,
            }),
        )
        .with_entity_id(report.entity_id.clone());
        publish_platform_event(None, &failed_event);
    }

    if report
        .diagnostics
        .iter()
        .any(|d| d.severity == "critical" || d.severity == "error")
    {
        let degraded_event = PlatformEvent::new(
            names::DEGRADED_MODE_ENTERED,
            "spanda-readiness",
            json!({
                "entity_type": report.entity_type,
                "trigger": "entity_health_evaluation",
                "diagnostics": report.diagnostics,
            }),
        )
        .with_entity_id(report.entity_id.clone());
        publish_platform_event(None, &degraded_event);
    }
}
