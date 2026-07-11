//! Mission planning and execution assurance.
//!
use crate::types::{AnomalySeverity, MissionAbortReason, MissionExecutionState, MissionPlan};
use spanda_ast::assurance_decl::MissionPlanDecl;
use spanda_ast::nodes::Program;
use spanda_capability::infer_robot_capabilities;
use spanda_config::{mission_policy, EntityRecoveryConfidence, ResolvedSystemConfig};
use spanda_readiness::{verify_mission, MissionVerificationReport};

/// Mission assurance report.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MissionAssuranceReport {
    pub plans: Vec<MissionPlan>,
    pub execution: MissionExecutionState,
    pub verification: spanda_readiness::MissionVerificationReport,
    pub abort_reasons: Vec<MissionAbortReason>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replan_recommendation: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preferred_recovery_strategy: Option<String>,
    pub passed: bool,
}

/// Verify mission plans against readiness mission verification.
pub fn verify_mission_assurance(program: &Program) -> MissionAssuranceReport {
    verify_mission_assurance_with_config(program, None)
}

/// Verify mission plans using optional `[mission]` configuration thresholds.
pub fn verify_mission_assurance_with_config(
    program: &Program,
    config: Option<&ResolvedSystemConfig>,
) -> MissionAssuranceReport {
    verify_mission_assurance_with_recovery(program, config, None)
}

/// Verify mission plans and surface adaptive recovery strategy preference on abort/replan.
pub fn verify_mission_assurance_with_recovery(
    program: &Program,
    config: Option<&ResolvedSystemConfig>,
    recovery: Option<&EntityRecoveryConfidence>,
) -> MissionAssuranceReport {
    // Verify mission assurance and fold recovery-confidence into abort/replan.
    //
    // Parameters:
    // - `program` — Spanda program with optional mission plans
    // - `config` — optional resolved system configuration
    // - `recovery` — optional entity recovery confidence (preferred strategy + score)
    //
    // Returns:
    // Mission assurance report with abort reasons and optional replan recommendation.
    //
    // Options:
    // When `recovery` is low or prefers a strategy, status may become `replan` / `blocked`.
    //
    // Example:
    // let report = verify_mission_assurance_with_recovery(&program, Some(&cfg), Some(&rc));

    let Program::Program { mission_plans, .. } = program;
    let plans: Vec<MissionPlan> = mission_plans
        .iter()
        .map(|decl| {
            let MissionPlanDecl::MissionPlanDecl {
                name,
                steps,
                constraints,
                ..
            } = decl;
            MissionPlan {
                name: name.clone(),
                steps: steps.iter().map(|s| s.name.clone()).collect(),
                constraints: constraints.iter().map(|c| c.constraint.clone()).collect(),
            }
        })
        .collect();

    let verifications = verify_mission(program, None);
    let all_achievable = verifications.iter().all(|v| v.achievable);
    let verification = verifications
        .into_iter()
        .next()
        .unwrap_or(MissionVerificationReport {
            achievable: true,
            mission_name: None,
            robot: None,
            required_capabilities: Vec::new(),
            hardware_satisfied: true,
            capabilities_satisfied: true,
            connectivity_satisfied: true,
            battery_sufficient: true,
            compute_sufficient: true,
            safety_satisfied: true,
            issues: Vec::new(),
        });

    let mut passed = all_achievable && !plans.is_empty() || mission_plans.is_empty();

    let mut abort_reasons: Vec<MissionAbortReason> = verification
        .issues
        .iter()
        .map(|issue| MissionAbortReason {
            reason: issue.clone(),
            severity: AnomalySeverity::High,
        })
        .collect();

    if let Some(cfg) = config {
        let policy = mission_policy(cfg);
        if policy.require_plans && plans.is_empty() {
            passed = false;
            abort_reasons.push(MissionAbortReason {
                reason: "Configuration requires at least one mission plan".into(),
                severity: AnomalySeverity::High,
            });
        }
        for cap in &policy.required_capabilities {
            let robot_caps = infer_robot_capabilities(program);
            let satisfied = robot_caps.iter().any(|r| {
                r.declared.iter().any(|c| c == cap)
                    || r.inferred.iter().any(|c| c == cap)
                    || r.rows
                        .iter()
                        .any(|row| row.capability == *cap && row.status == "OK")
            });
            if !satisfied {
                passed = false;
                abort_reasons.push(MissionAbortReason {
                    reason: format!("Configured required capability missing: {cap}"),
                    severity: AnomalySeverity::High,
                });
            }
        }
    }

    // Surface adaptive learning strategy preference on the abort/replan path.
    let mut replan_recommendation = None;
    let mut preferred_recovery_strategy = None;
    if let Some(rc) = recovery {
        preferred_recovery_strategy = rc.preferred_strategy.clone();
        let escalate_below = 0.30_f64;
        let min_attempts = 3_u32;
        if rc.attempts >= min_attempts && rc.score < escalate_below {
            passed = false;
            abort_reasons.push(MissionAbortReason {
                reason: format!(
                    "Recovery confidence {:.0}% below escalate threshold with {} attempts — abort",
                    rc.score * 100.0,
                    rc.attempts
                ),
                severity: AnomalySeverity::High,
            });
        } else if let Some(ref strategy) = rc.preferred_strategy {
            replan_recommendation = Some(format!(
                "Prefer recovery strategy '{strategy}' (confidence {:.0}%) on replan",
                rc.score * 100.0
            ));
            if rc.score < escalate_below {
                abort_reasons.push(MissionAbortReason {
                    reason: format!(
                        "Preferred strategy '{strategy}' is below escalate threshold — replan before continue"
                    ),
                    severity: AnomalySeverity::Medium,
                });
            }
        }
    }

    let status = if !passed {
        "blocked".into()
    } else if replan_recommendation.is_some() {
        "replan".into()
    } else {
        "ready".into()
    };

    let execution = MissionExecutionState {
        plan: plans.first().map(|p| p.name.clone()).unwrap_or_default(),
        current_step: plans.first().and_then(|p| p.steps.first().cloned()),
        status,
    };

    MissionAssuranceReport {
        plans,
        execution,
        verification,
        abort_reasons,
        replan_recommendation,
        preferred_recovery_strategy,
        passed,
    }
}
