//! Span-aware recovery policy diagnostics for IDE and `spanda check --readiness-json`.

use spanda_ast::assurance_decl::RecoveryPolicyDecl;
use spanda_ast::nodes::{Program, RobotDecl, TopicDecl};
use spanda_capability::VerificationDiagnostic;

fn normalize_action(action: &str) -> String {
    action
        .to_ascii_lowercase()
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect()
}

fn action_is_high_risk(action: &str) -> bool {
    let lower = normalize_action(action);
    lower.contains("resumemission")
        || lower.contains("restartfleet")
        || lower.contains("unsafe")
        || lower.contains("opengate")
}

fn robot_has_approval_topic(robot: &RobotDecl) -> bool {
    let RobotDecl::RobotDecl { topics, .. } = robot;
    topics.iter().any(|topic| {
        let TopicDecl::TopicDecl { message_type, .. } = topic;
        message_type == "Approval"
    })
}

fn program_has_approval_path(program: &Program) -> bool {
    let Program::Program { robots, .. } = program;
    robots.iter().any(robot_has_approval_topic)
}

/// Collect recovery-policy diagnostics for static analysis and IDE hints.
pub fn collect_recovery_diagnostics(program: &Program) -> Vec<VerificationDiagnostic> {
    let Program::Program {
        recovery_policies,
        mitigations,
        health_checks,
        health_policies,
        anomaly_handlers,
        fleets,
        robots,
        ..
    } = program;

    let mut diags = Vec::new();
    let has_health =
        !health_checks.is_empty() || !health_policies.is_empty() || !anomaly_handlers.is_empty();
    let has_recovery = !recovery_policies.is_empty() || !mitigations.is_empty();

    if has_health && !has_recovery {
        let line = health_checks
            .first()
            .map(|h| h.span().start.line)
            .or_else(|| health_policies.first().map(|h| h.span().start.line))
            .or_else(|| anomaly_handlers.first().map(|h| h.span().start.line))
            .unwrap_or(1);
        let column = health_checks
            .first()
            .map(|h| h.span().start.column)
            .unwrap_or(1);
        diags.push(VerificationDiagnostic {
            message: "Health or anomaly handling declared without recovery_policy or mitigation"
                .into(),
            line,
            column,
            severity: "warning".into(),
            category: "recovery:policy".into(),
            suggested_fix: Some(
                "Add recovery_policy or mitigation branches for detected failure modes".into(),
            ),
        });
    }

    let approval_path = program_has_approval_path(program);
    for policy in recovery_policies {
        let RecoveryPolicyDecl::RecoveryPolicyDecl {
            name,
            branches,
            span,
        } = policy;
        if branches.is_empty() {
            diags.push(VerificationDiagnostic {
                message: format!("recovery_policy '{name}' has no on branches"),
                line: span.start.line,
                column: span.start.column,
                severity: "warning".into(),
                category: "recovery:policy".into(),
                suggested_fix: Some("Add on <condition> { actions; } branches".into()),
            });
            continue;
        }
        for branch in branches {
            let trigger_lower = branch.condition.to_ascii_lowercase();
            if (trigger_lower.contains("fleet") || trigger_lower.contains("swarm"))
                && fleets.is_empty()
            {
                diags.push(VerificationDiagnostic {
                    message: format!(
                        "recovery_policy '{name}' references fleet failures but no fleet is declared"
                    ),
                    line: branch.span.start.line,
                    column: branch.span.start.column,
                    severity: "error".into(),
                    category: "recovery:fleet".into(),
                    suggested_fix: Some("Declare fleet <Name> { members; } or adjust trigger".into()),
                });
            }
            for action in &branch.actions {
                if action_is_high_risk(action) && !approval_path {
                    diags.push(VerificationDiagnostic {
                        message: format!(
                            "High-risk recovery action '{action}' should have an Approval topic or operator path"
                        ),
                        line: branch.span.start.line,
                        column: branch.span.start.column,
                        severity: "warning".into(),
                        category: "recovery:approval".into(),
                        suggested_fix: Some(
                            "Add topic approval: Approval subscribe on \"/ops/approval\"; or mission requires approval Operator"
                                .into(),
                        ),
                    });
                }
            }
        }
        let _ = robots;
    }

    diags
}

trait HasSpan {
    fn span(&self) -> spanda_ast::nodes::Span;
}

impl HasSpan for spanda_ast::foundations::HealthCheckDecl {
    fn span(&self) -> spanda_ast::nodes::Span {
        let spanda_ast::foundations::HealthCheckDecl::HealthCheckDecl { span, .. } = self;
        *span
    }
}

impl HasSpan for spanda_ast::foundations::HealthPolicyDecl {
    fn span(&self) -> spanda_ast::nodes::Span {
        let spanda_ast::foundations::HealthPolicyDecl::HealthPolicyDecl { span, .. } = self;
        *span
    }
}

impl HasSpan for spanda_ast::assurance_decl::AnomalyHandlerDecl {
    fn span(&self) -> spanda_ast::nodes::Span {
        let spanda_ast::assurance_decl::AnomalyHandlerDecl::AnomalyHandlerDecl { span, .. } = self;
        *span
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use spanda_lexer::tokenize;
    use spanda_parser::parse;

    #[test]
    fn warns_when_high_risk_action_lacks_approval_topic() {
        let program = parse(tokenize(
            r#"
recovery_policy Risky {
    on gps.failed { resume mission; }
}
robot R { sensor gps: GPS; actuator w: DifferentialDrive; safety { max_speed = 1 m/s; } behavior b() {} }
"#,
        ).unwrap()).unwrap();
        let diags = collect_recovery_diagnostics(&program);
        assert!(diags.iter().any(|d| d.category == "recovery:approval"));
    }
}
