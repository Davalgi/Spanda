//! Platform event emission for interpreter lifecycle hooks.
//!
use spanda_ast::nodes::{Program, RobotDecl};
use spanda_audit::platform_event::names;
use spanda_audit::{AuditRuntime, PlatformEvent};
use spanda_runtime::telemetry_sink::TelemetrySink;
use serde_json::json;

/// Record a mission lifecycle platform event when audit runtime is configured.
pub(crate) fn emit_mission_platform_event(
    audit: Option<&mut AuditRuntime>,
    telemetry: &dyn TelemetrySink,
    event_type: &str,
    program: &Program,
    trace_source: Option<&str>,
    success: bool,
) {
    let mission_key = trace_source
        .map(str::to_string)
        .or_else(|| first_robot_name(program))
        .unwrap_or_else(|| "program".into());
    let event = PlatformEvent::new(
        event_type,
        "spanda-interpreter",
        json!({
            "mission": mission_key,
            "success": success,
            "robot_count": robot_count(program),
        }),
    )
    .with_entity_id(format!("mission/{mission_key}"));
    if let Some(rt) = audit {
        let _ = rt.record_platform_event(&event);
    }
    telemetry.record_platform_event(
        event.event_type.as_str(),
        &event.source,
        event.entity_id.as_deref(),
        event.payload.clone(),
        event.timestamp.timestamp_millis() as f64,
    );
}

pub(crate) fn emit_mission_started(
    audit: Option<&mut AuditRuntime>,
    telemetry: &dyn TelemetrySink,
    program: &Program,
    trace_source: Option<&str>,
) {
    emit_mission_platform_event(
        audit,
        telemetry,
        names::MISSION_STARTED,
        program,
        trace_source,
        true,
    );
}

pub(crate) fn emit_mission_completed(
    audit: Option<&mut AuditRuntime>,
    telemetry: &dyn TelemetrySink,
    program: &Program,
    trace_source: Option<&str>,
    success: bool,
) {
    emit_mission_platform_event(
        audit,
        telemetry,
        names::MISSION_COMPLETED,
        program,
        trace_source,
        success,
    );
}

fn robot_count(program: &Program) -> usize {
    let Program::Program { robots, .. } = program;
    robots.len()
}

fn first_robot_name(program: &Program) -> Option<String> {
    let Program::Program { robots, .. } = program;
    robots.first().map(|robot| match robot {
        RobotDecl::RobotDecl { name, .. } => name.clone(),
    })
}
