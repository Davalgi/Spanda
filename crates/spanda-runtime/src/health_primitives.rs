//! Runtime health evaluation primitives for interpreter polling and capability analysis.
//!
use crate::health_types::{HealthCheckResult, HealthReport, HealthStatus};
use spanda_ast::foundations::{HealthCheckDecl, HealthPolicyDecl};
use spanda_ast::nodes::Program;

/// Evaluate static health checks from program declarations.
pub fn evaluate_health_checks(program: &Program) -> HealthReport {
    // Evaluate declared health checks and robot-level monitors into a baseline report.
    //
    // Parameters:
    // - `program` — parsed Spanda program AST
    //
    // Returns:
    // Static health report with unknown per-check status until runtime signals arrive.
    //
    // Options:
    // None.
    //
    // Example:
    // let report = evaluate_health_checks(&program);

    let Program::Program {
        health_checks,
        health_policies,
        ..
    } = program;

    let mut checks = Vec::new();
    for hc in health_checks {
        let HealthCheckDecl::HealthCheckDecl {
            name,
            target,
            target_kind,
            requirements,
            conditions,
            ..
        } = hc;
        for req in requirements {
            checks.push(HealthCheckResult {
                name: name.clone(),
                target: target.clone(),
                target_kind: target_kind.clone(),
                metric: format!("require:{req}"),
                operator: "require".into(),
                threshold: req.clone(),
                status: HealthStatus::Unknown,
                message: Some(format!("Fleet requirement: {req}")),
            });
        }
        for cond in conditions {
            checks.push(HealthCheckResult {
                name: name.clone(),
                target: target.clone(),
                target_kind: target_kind.clone(),
                metric: cond.metric.clone(),
                operator: cond.operator.clone(),
                threshold: cond.threshold.clone(),
                status: HealthStatus::Unknown,
                message: Some(format!(
                    "Static check: {} {} {}",
                    cond.metric, cond.operator, cond.threshold
                )),
            });
        }
    }

    // Evaluate robot-level health checks embedded in robots.
    let Program::Program { robots, .. } = program;
    for robot in robots {
        let spanda_ast::nodes::RobotDecl::RobotDecl {
            name,
            health_checks: robot_checks,
            ..
        } = robot;
        for hc in robot_checks {
            let HealthCheckDecl::HealthCheckDecl {
                name: hc_name,
                conditions,
                ..
            } = hc;
            for cond in conditions {
                checks.push(HealthCheckResult {
                    name: hc_name.clone(),
                    target: name.clone(),
                    target_kind: "robot".into(),
                    metric: cond.metric.clone(),
                    operator: cond.operator.clone(),
                    threshold: cond.threshold.clone(),
                    status: HealthStatus::Unknown,
                    message: None,
                });
            }
        }
    }

    let policies: Vec<String> = health_policies
        .iter()
        .map(|p| {
            let HealthPolicyDecl::HealthPolicyDecl { name, .. } = p;
            name.clone()
        })
        .collect();

    let overall = if checks.is_empty() {
        HealthStatus::Unknown
    } else {
        HealthStatus::Healthy
    };

    HealthReport {
        checks,
        overall,
        policies,
    }
}

/// Evaluate health checks against runtime fault and event signals from hardware monitoring.
pub fn evaluate_runtime_health(
    faults: &[String],
    active_events: &[String],
    program: &Program,
) -> HealthReport {
    // Refine static health checks using live hardware monitor fault and event labels.
    //
    // Parameters:
    // - `faults` — active hardware fault labels
    // - `active_events` — active hardware event labels
    // - `program` — parsed program AST
    //
    // Returns:
    // Health report with per-check runtime status and rolled-up overall health.
    //
    // Options:
    // None.
    //
    // Example:
    // let report = evaluate_runtime_health(&faults, &events, &program);

    let mut report = evaluate_health_checks(program);
    let fault_lower: Vec<String> = faults.iter().map(|f| f.to_ascii_lowercase()).collect();
    let event_lower: Vec<String> = active_events
        .iter()
        .map(|f| f.to_ascii_lowercase())
        .collect();

    for check in &mut report.checks {
        check.status = runtime_status_for_metric(&check.metric, &fault_lower, &event_lower);
        check.message = Some(format!("Runtime status: {:?}", check.status));
    }

    report.overall = if report.checks.iter().any(|c| {
        matches!(
            c.status,
            HealthStatus::Critical | HealthStatus::Failed | HealthStatus::Unsafe
        )
    }) {
        HealthStatus::Critical
    } else if report.checks.iter().any(|c| {
        matches!(
            c.status,
            HealthStatus::Degraded | HealthStatus::Warning | HealthStatus::Offline
        )
    }) || !faults.is_empty()
    {
        HealthStatus::Degraded
    } else if report.checks.is_empty() {
        HealthStatus::Unknown
    } else {
        HealthStatus::Healthy
    };

    report
}

/// Refine fleet-target health checks using fleet membership, requirements, and runtime faults.
pub fn apply_fleet_health_checks(
    report: &mut HealthReport,
    _program: &Program,
    fleets: &crate::robotics::FleetRegistry,
    faults: &[String],
) {
    // Apply fleet membership and requirement checks to fleet-target health rows.
    //
    // Parameters:
    // - `report` — health report to refine in place
    // - `_program` — parsed program AST (reserved for future policy hooks)
    // - `fleets` — active fleet registry
    // - `faults` — active runtime fault labels
    //
    // Returns:
    // None (mutates `report`).
    //
    // Options:
    // None.
    //
    // Example:
    // apply_fleet_health_checks(&mut report, &program, &fleets, &faults);

    let fault_lower: Vec<String> = faults.iter().map(|f| f.to_ascii_lowercase()).collect();
    for check in &mut report.checks {
        if check.target_kind != "fleet" {
            continue;
        }
        let members = fleets.members(&check.target).unwrap_or(&[]);
        if members.is_empty() {
            check.status = HealthStatus::Unknown;
            continue;
        }

        if check.metric.starts_with("require:") {
            let req = check.threshold.clone();
            check.status = evaluate_fleet_requirement(&req, members, &fault_lower);
            check.message = Some(format!(
                "Fleet '{}' requirement '{req}' => {:?} (members={})",
                check.target,
                check.status,
                members.len()
            ));
            continue;
        }

        let member_hit = members.iter().any(|member| {
            let member_lower = member.to_ascii_lowercase();
            fault_lower.iter().any(|f| {
                f.contains(&member_lower) || f.contains("critical") || f.contains("unsafe")
            })
        });
        check.status = if member_hit {
            HealthStatus::Critical
        } else if fault_lower.is_empty() {
            HealthStatus::Healthy
        } else {
            HealthStatus::Degraded
        };
        check.message = Some(format!(
            "Fleet '{}' members={} status={:?}",
            check.target,
            members.len(),
            check.status
        ));
    }

    report.overall = if report.checks.iter().any(|c| {
        matches!(
            c.status,
            HealthStatus::Critical | HealthStatus::Failed | HealthStatus::Unsafe
        )
    }) {
        HealthStatus::Critical
    } else if report.checks.iter().any(|c| {
        matches!(
            c.status,
            HealthStatus::Degraded | HealthStatus::Warning | HealthStatus::Offline
        )
    }) || !faults.is_empty()
    {
        HealthStatus::Degraded
    } else if report.checks.is_empty() {
        HealthStatus::Unknown
    } else {
        HealthStatus::Healthy
    };
}

fn evaluate_fleet_requirement(
    requirement: &str,
    members: &[String],
    faults: &[String],
) -> HealthStatus {
    let req = requirement.to_ascii_lowercase();
    if req.contains("no robot unsafe") || req.contains("no robot critical") {
        let hit = members.iter().any(|member| {
            let member_lower = member.to_ascii_lowercase();
            faults.iter().any(|f| {
                f.contains(&member_lower) && (f.contains("unsafe") || f.contains("critical"))
            })
        });
        return if hit {
            HealthStatus::Unsafe
        } else {
            HealthStatus::Healthy
        };
    }
    if req.contains("at_least") && req.contains('%') {
        let percent = req
            .split_whitespace()
            .find_map(|token| token.trim_end_matches('%').parse::<f64>().ok())
            .unwrap_or(80.0);
        let healthy_members = members
            .iter()
            .filter(|member| {
                let member_lower = member.to_ascii_lowercase();
                !faults.iter().any(|f| {
                    f.contains(&member_lower)
                        && (f.contains("critical")
                            || f.contains("unsafe")
                            || f.contains("degraded")
                            || f.contains("offline"))
                })
            })
            .count();
        let ratio = (healthy_members as f64 / members.len() as f64) * 100.0;
        return if ratio + f64::EPSILON >= percent {
            HealthStatus::Healthy
        } else {
            HealthStatus::Degraded
        };
    }
    if req.contains("healthy") {
        let all_ok = members.iter().all(|member| {
            let member_lower = member.to_ascii_lowercase();
            !faults.iter().any(|f| f.contains(&member_lower))
        });
        return if all_ok {
            HealthStatus::Healthy
        } else {
            HealthStatus::Degraded
        };
    }
    HealthStatus::Unknown
}

fn runtime_status_for_metric(metric: &str, faults: &[String], events: &[String]) -> HealthStatus {
    let metric_lower = metric.to_ascii_lowercase();
    let signals: Vec<&str> = faults
        .iter()
        .chain(events.iter())
        .map(String::as_str)
        .collect();

    if metric_lower.contains("emergency_stop") {
        return if signals
            .iter()
            .any(|s| s.contains("emergency") || s.contains("kill"))
        {
            HealthStatus::Unsafe
        } else {
            HealthStatus::Healthy
        };
    }

    if metric_lower.contains("gps") {
        if signals
            .iter()
            .any(|s| s.contains("gps") && s.contains("critical"))
        {
            return HealthStatus::Critical;
        }
        if signals.iter().any(|s| s.contains("gps")) {
            return HealthStatus::Degraded;
        }
        return HealthStatus::Healthy;
    }

    if metric_lower.contains("camera") {
        if signals.iter().any(|s| s.contains("camera")) {
            return HealthStatus::Offline;
        }
        return HealthStatus::Healthy;
    }

    if metric_lower.contains("battery") {
        if signals.iter().any(|s| s.contains("critical")) {
            return HealthStatus::Critical;
        }
        return HealthStatus::Healthy;
    }

    if signals
        .iter()
        .any(|s| s.contains("critical") || s.contains("unsafe"))
    {
        HealthStatus::Critical
    } else if signals
        .iter()
        .any(|s| s.contains("degraded") || s.contains("offline"))
    {
        HealthStatus::Degraded
    } else if signals.is_empty() {
        HealthStatus::Healthy
    } else {
        HealthStatus::Warning
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use spanda_lexer::tokenize;
    use spanda_parser::parse;

    fn parse_source(source: &str) -> spanda_ast::nodes::Program {
        parse(tokenize(source).expect("tokenize")).expect("parse")
    }

    #[test]
    fn runtime_health_marks_gps_fault_degraded() {
        let source = r#"
health_check RoverHealth for robot Rover {
    check gps.status == Healthy;
}
"#;
        let program = parse_source(source);
        let report = evaluate_runtime_health(&["GPSDegraded".into()], &[], &program);
        assert!(report
            .checks
            .iter()
            .any(|c| c.status == HealthStatus::Degraded));
    }

    #[test]
    fn fleet_requirement_percent_marks_degraded_when_below_threshold() {
        let status = evaluate_fleet_requirement(
            "at_least 80% robots Healthy",
            &["A".into(), "B".into(), "C".into(), "D".into(), "E".into()],
            &["a_degraded".into(), "b_degraded".into()],
        );
        assert_eq!(status, HealthStatus::Degraded);
    }
}
