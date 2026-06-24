//! Span-aware verification diagnostics for IDE and CLI integration.

use crate::{
    capability_traceability, check_minimum_capabilities, infer_robot_capabilities,
    lookup_capability, minimum::MinimumCapabilityRow,
};
use serde::{Deserialize, Serialize};
use spanda_ast::foundations::{
    HealthCheckDecl, KillSwitchDecl, RequiresCapabilityDecl, RequiresCapabilitySeverity,
};
use spanda_ast::nodes::{Program, RobotDecl, Span, TopicDecl};

/// Single verification diagnostic with source location.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VerificationDiagnostic {
    pub message: String,
    pub line: u32,
    pub column: u32,
    pub severity: String,
    pub category: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_fix: Option<String>,
}

/// Collect capability, traceability, minimum-hardware, health, and kill-switch diagnostics.
pub fn collect_verification_diagnostics(program: &Program) -> Vec<VerificationDiagnostic> {
    // Description:
    //     Collect verification diagnostics.
    //
    // Inputs:
    //     progra: &Program
    //         Caller-supplied progra.
    //
    // Outputs:
    //     result: Vec<VerificationDiagnostic>
    //         Return value from `collect_verification_diagnostics`.
    //
    // Example:

    //     let result = spanda_capability::diagnostics::collect_verification_diagnostics(progra);

    let mut diags = Vec::new();
    let Program::Program {
        robots,
        requires_capabilities,
        kill_switches,
        health_checks,
        health_policies,
        ..
    } = program;

    let trace = capability_traceability(program);
    for err in &trace.errors {
        if let Some(d) = map_traceability_error(err, robots, requires_capabilities) {
            diags.push(d);
        }
    }
    for warn in &trace.warnings {
        if let Some(d) = map_traceability_warning(warn, robots) {
            diags.push(d);
        }
    }

    for row in &trace.capability_rows {
        if row.status == "FAIL" {
            if let Some(req) = requires_capabilities
                .iter()
                .find(|r| r.capability == row.capability)
            {
                diags.push(diag(
                    format!(
                        "Capability '{}' not satisfied by robot '{}'",
                        row.capability, row.provided_by
                    ),
                    req.span.start.line,
                    req.span.start.column,
                    "warning",
                    "capability",
                    capability_fix_for(&row.capability),
                ));
            }
        }
    }

    let minimum = check_minimum_capabilities(program);
    for err in &minimum.errors {
        if let Some(d) = map_minimum_error(err, requires_capabilities, robots, &minimum.rows) {
            diags.push(d);
        }
    }
    for warn in &minimum.warnings {
        diags.push(diag(
            warn.clone(),
            1,
            1,
            "warning",
            "minimum-hardware",
            None,
        ));
    }

    for req in requires_capabilities {
        if lookup_capability(&req.capability).is_none() {
            diags.push(diag(
                format!("Unknown capability '{}'", req.capability),
                req.span.start.line,
                req.span.start.column,
                severity_for(req.severity),
                "capability",
                None,
            ));
        }
    }

    for ks in kill_switches {
        diags.extend(kill_switch_diagnostics(ks, program));
    }
    for robot in robots {
        let RobotDecl::RobotDecl {
            name,
            kill_switches: robot_kill_switches,
            actuators,
            ..
        } = robot;
        for ks in robot_kill_switches {
            diags.extend(kill_switch_diagnostics(ks, program));
        }
        if robot_kill_switches.is_empty() && kill_switches.is_empty() {
            let has_drive = actuators.iter().any(|a| {
                let spanda_ast::nodes::ActuatorDecl::ActuatorDecl { actuator_type, .. } = a;
                actuator_type.contains("Drive")
            });
            if has_drive {
                let span = robot_span(robot);
                diags.push(diag(
                    format!("Robot '{name}' has actuators but no kill_switch handler"),
                    span.start.line,
                    span.start.column,
                    "info",
                    "kill-switch",
                    Some(KILL_SWITCH_STUB.into()),
                ));
            }
        }
    }

    if !health_checks.is_empty() && health_policies.is_empty() {
        for hc in health_checks {
            let HealthCheckDecl::HealthCheckDecl { span, name, .. } = hc;
            diags.push(diag(
                format!("Health check '{name}' has no matching health_policy"),
                span.start.line,
                span.start.column,
                "info",
                "health",
                Some(HEALTH_POLICY_STUB.into()),
            ));
        }
    }

    let robot_reports = infer_robot_capabilities(program);
    for report in robot_reports {
        for row in report.rows {
            if row.status == "PARTIAL" {
                if let Some(robot) = robots.iter().find(|r| robot_name(r) == report.robot) {
                    let span = robot_span(robot);
                    diags.push(diag(
                        format!(
                            "Capability '{}' partially satisfied on robot '{}'",
                            row.capability, report.robot
                        ),
                        span.start.line,
                        span.start.column,
                        "warning",
                        "capability",
                        capability_fix_for(&row.capability),
                    ));
                }
            }
        }
    }

    diags
}

fn diag(
    message: String,
    line: u32,
    column: u32,
    severity: &str,
    category: &str,
    suggested_fix: Option<String>,
) -> VerificationDiagnostic {
    // Description:
    //     Diag.
    //
    // Inputs:
    //     essage: String
    //         Caller-supplied essage.
    //     line: u32
    //         Caller-supplied line.
    //     column: u32
    //         Caller-supplied column.
    //     severity: &str
    //         Caller-supplied severity.
    //     category: &str
    //         Caller-supplied category.
    //     suggested_fix: Option<String>
    //         Caller-supplied suggested fix.
    //
    // Outputs:
    //     result: VerificationDiagnostic
    //         Return value from `diag`.
    //
    // Example:

    //     let result = spanda_capability::diagnostics::diag(essage, line, column, severity, category, suggested_fix);

    VerificationDiagnostic {
        message,
        line,
        column,
        severity: severity.into(),
        category: category.into(),
        suggested_fix,
    }
}

const KILL_SWITCH_STUB: &str =
    "kill_switch EmergencyStop {\n    priority: critical;\n    action { emergency_stop; }\n}";

const HEALTH_POLICY_STUB: &str = "health_policy SafetyPolicy {\n    on Critical { enter degraded_mode; }\n    on Unsafe { emergency_stop; }\n}";

fn capability_fix_for(capability: &str) -> Option<String> {
    // Description:
    //     Capability fix for.
    //
    // Inputs:
    //     capability: &str
    //         Caller-supplied capability.
    //
    // Outputs:
    //     result: Option<String>
    //         Return value from `capability_fix_for`.
    //
    // Example:

    //     let result = spanda_capability::diagnostics::capability_fix_for(capability);

    match capability {
        "obstacle_avoidance" => Some("sensor lidar: Lidar;".into()),
        "gps_navigation" => Some("sensor gps: GPS;".into()),
        "emergency_stop" => Some(KILL_SWITCH_STUB.into()),
        _ => None,
    }
}

fn severity_for(severity: RequiresCapabilitySeverity) -> &'static str {
    // Description:
    //     Severity for.
    //
    // Inputs:
    //     severity: RequiresCapabilitySeverity
    //         Caller-supplied severity.
    //
    // Outputs:
    //     result: &'static str
    //         Return value from `severity_for`.
    //
    // Example:

    //     let result = spanda_capability::diagnostics::severity_for(severity);

    match severity {
        RequiresCapabilitySeverity::Error => "error",
        RequiresCapabilitySeverity::Warning => "warning",
        RequiresCapabilitySeverity::Info => "info",
    }
}

fn map_traceability_error(
    err: &str,
    robots: &[RobotDecl],
    requires: &[RequiresCapabilityDecl],
) -> Option<VerificationDiagnostic> {
    // Description:
    //     Map traceability error.
    //
    // Inputs:
    //     err: &str
    //         Caller-supplied err.
    //     robots: &[RobotDecl]
    //         Caller-supplied robots.
    //     requires: &[RequiresCapabilityDecl]
    //         Caller-supplied requires.
    //
    // Outputs:
    //     result: Option<VerificationDiagnostic>
    //         Return value from `map_traceability_error`.
    //
    // Example:

    //     let result = spanda_capability::diagnostics::map_traceability_error(err, robots, requires);

    if let Some(rest) = err.strip_prefix("Robot '") {
        if let Some((name, tail)) = rest.split_once("' uses undeclared hardware '") {
            if let Some(robot) = robots.iter().find(|r| robot_name(r) == name) {
                let span = robot_span(robot);
                return Some(diag(
                    err.to_string(),
                    span.start.line,
                    span.start.column,
                    "error",
                    "traceability",
                    None,
                ));
            }
            let _ = tail;
        }
    }
    if let Some(cap) = err
        .strip_prefix("Unknown capability '")
        .and_then(|s| s.strip_suffix('\''))
    {
        if let Some(req) = requires.iter().find(|r| r.capability == cap) {
            return Some(diag(
                err.to_string(),
                req.span.start.line,
                req.span.start.column,
                "error",
                "capability",
                None,
            ));
        }
    }
    None
}

fn map_traceability_warning(warn: &str, robots: &[RobotDecl]) -> Option<VerificationDiagnostic> {
    // Description:
    //     Map traceability warning.
    //
    // Inputs:
    //     warn: &str
    //         Caller-supplied warn.
    //     robots: &[RobotDecl]
    //         Caller-supplied robots.
    //
    // Outputs:
    //     result: Option<VerificationDiagnostic>
    //         Return value from `map_traceability_warning`.
    //
    // Example:

    //     let result = spanda_capability::diagnostics::map_traceability_warning(warn, robots);

    if let Some(rest) = warn.strip_prefix("Actuator '") {
        if let Some((actuator, tail)) = rest.split_once("' on robot '") {
            if let Some(rname) = tail.strip_suffix("' has no safety gate") {
                if let Some(robot) = robots.iter().find(|r| robot_name(r) == rname) {
                    let span = robot_span(robot);
                    return Some(diag(
                        warn.to_string(),
                        span.start.line,
                        span.start.column,
                        "warning",
                        "traceability",
                        Some("safety { max_speed = 1.0 m/s; }".into()),
                    ));
                }
            }
            let _ = actuator;
        }
    }
    None
}

fn map_minimum_error(
    err: &str,
    requires: &[RequiresCapabilityDecl],
    robots: &[RobotDecl],
    rows: &[MinimumCapabilityRow],
) -> Option<VerificationDiagnostic> {
    // Description:
    //     Map minimum error.
    //
    // Inputs:
    //     err: &str
    //         Caller-supplied err.
    //     requires: &[RequiresCapabilityDecl]
    //         Caller-supplied requires.
    //     robots: &[RobotDecl]
    //         Caller-supplied robots.
    //     rows: &[MinimumCapabilityRow]
    //         Caller-supplied rows.
    //
    // Outputs:
    //     result: Option<VerificationDiagnostic>
    //         Return value from `map_minimum_error`.
    //
    // Example:

    //     let result = spanda_capability::diagnostics::map_minimum_error(err, requires, robots, rows);

    let fix_from_rows = rows
        .iter()
        .find(|row| err.contains(&row.capability))
        .and_then(|row| row.suggested_fixes.first().cloned())
        .or_else(|| {
            if err.contains("obstacle_avoidance") {
                Some("Add Lidar sensor to hardware profile".into())
            } else if err.contains("gps_navigation") {
                Some("Add GPS sensor to hardware profile".into())
            } else {
                None
            }
        });
    for req in requires {
        if err.contains(&req.capability) {
            return Some(diag(
                err.to_string(),
                req.span.start.line,
                req.span.start.column,
                severity_for(req.severity),
                "minimum-hardware",
                fix_from_rows
                    .clone()
                    .or_else(|| capability_fix_for(&req.capability)),
            ));
        }
    }
    for robot in robots {
        if err.contains(robot_name(robot)) {
            let span = robot_span(robot);
            return Some(diag(
                err.to_string(),
                span.start.line,
                span.start.column,
                "error",
                "minimum-hardware",
                fix_from_rows,
            ));
        }
    }
    None
}

fn kill_switch_diagnostics(ks: &KillSwitchDecl, program: &Program) -> Vec<VerificationDiagnostic> {
    // Description:
    //     Kill switch diagnostics.
    //
    // Inputs:
    //     ks: &KillSwitchDecl
    //         Caller-supplied ks.
    //     progra: &Program
    //         Caller-supplied progra.
    //
    // Outputs:
    //     result: Vec<VerificationDiagnostic>
    //         Return value from `kill_switch_diagnostics`.
    //
    // Example:

    //     let result = spanda_capability::diagnostics::kill_switch_diagnostics(ks, progra);

    let KillSwitchDecl::KillSwitchDecl {
        name,
        remote_signed,
        span,
        ..
    } = ks;
    let mut diags = Vec::new();
    if *remote_signed && !program_has_signed_comm(program) {
        diags.push(diag(
            format!(
                "Kill switch '{name}' requires remote_signed but no signed secure comm or topic policy is declared"
            ),
            span.start.line,
            span.start.column,
            "error",
            "kill-switch",
            Some("identity RobotIdentity { id: \"device\"; public_key: \"key\"; } secure { signed required; }".into()),
        ));
    }
    diags
}

fn program_has_signed_comm(program: &Program) -> bool {
    // Description:
    //     Program has signed comm.
    //
    // Inputs:
    //     progra: &Program
    //         Caller-supplied progra.
    //
    // Outputs:
    //     result: bool
    //         Return value from `program_has_signed_comm`.
    //
    // Example:

    //     let result = spanda_capability::diagnostics::program_has_signed_comm(progra);

    let Program::Program { robots, .. } = program;
    robots.iter().any(|robot| {
        let RobotDecl::RobotDecl {
            secure_comm,
            topics,
            trust,
            ..
        } = robot;
        secure_comm.is_some()
            || trust.is_some()
            || topics.iter().any(|topic| {
                let TopicDecl::TopicDecl { secure, .. } = topic;
                secure
                    .as_ref()
                    .is_some_and(|s| s.signed || s.signed_required)
            })
    })
}

fn robot_name(robot: &RobotDecl) -> &str {
    // Description:
    //     Robot name.
    //
    // Inputs:
    //     robo: &RobotDecl
    //         Caller-supplied robo.
    //
    // Outputs:
    //     result: &str
    //         Return value from `robot_name`.
    //
    // Example:

    //     let result = spanda_capability::diagnostics::robot_name(robo);

    let RobotDecl::RobotDecl { name, .. } = robot;
    name
}

fn robot_span(robot: &RobotDecl) -> Span {
    // Description:
    //     Robot span.
    //
    // Inputs:
    //     robo: &RobotDecl
    //         Caller-supplied robo.
    //
    // Outputs:
    //     result: Span
    //         Return value from `robot_span`.
    //
    // Example:

    //     let result = spanda_capability::diagnostics::robot_span(robo);

    let RobotDecl::RobotDecl { span, .. } = robot;
    *span
}

#[cfg(test)]
mod tests {
    use super::*;
    use spanda_lexer::tokenize;
    use spanda_parser::parse;

    fn parse_source(source: &str) -> Program {
        // Description:
        //     Parse source.
        //
        // Inputs:
        //     source: &str
        //         Caller-supplied source.
        //
        // Outputs:
        //     result: Program
        //         Return value from `parse_source`.
        //
        // Example:

        //     let result = spanda_capability::diagnostics::parse_source(source);

        parse(tokenize(source).expect("tokenize")).expect("parse")
    }

    #[test]
    fn remote_kill_switch_without_signed_policy_warns() {
        // Description:
        //     Remote kill switch without signed policy warns.
        //
        // Inputs:
        //     None.
        //
        // Outputs:
        //     None.
        //
        // Example:

        //     let result = spanda_capability::diagnostics::remote_kill_switch_without_signed_policy_warns();

        let source = r#"
kill_switch EmergencyStop {
    priority: critical;
    remote_signed;
    action { emergency_stop; }
}
"#;
        let program = parse_source(source);
        let diags = collect_verification_diagnostics(&program);
        assert!(diags
            .iter()
            .any(|d| d.category == "kill-switch" && d.severity == "error"));
        assert!(
            diags.iter().any(|d| {
                d.suggested_fix
                    .as_deref()
                    .is_some_and(|fix| fix.contains("identity RobotIdentity"))
            }),
            "expected identity + signed comm quick-fix"
        );
    }

    #[test]
    fn undeclared_hardware_produces_spanned_error() {
        // Description:
        //     Undeclared hardware produces spanned error.
        //
        // Inputs:
        //     None.
        //
        // Outputs:
        //     None.
        //
        // Example:

        //     let result = spanda_capability::diagnostics::undeclared_hardware_produces_spanned_error();

        let source = r#"
robot Rover {
    uses hardware MissingBoard;
    actuator wheels: DifferentialDrive;
}
"#;
        let program = parse_source(source);
        let diags = collect_verification_diagnostics(&program);
        assert!(diags
            .iter()
            .any(|d| d.category == "traceability" && d.line > 1));
    }
}
