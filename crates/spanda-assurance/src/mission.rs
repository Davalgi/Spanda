//! Mission planning and execution assurance.
//!
use crate::types::{AnomalySeverity, MissionAbortReason, MissionExecutionState, MissionPlan};
use spanda_ast::assurance_decl::MissionPlanDecl;
use spanda_ast::nodes::Program;
use spanda_capability::infer_robot_capabilities;
use spanda_config::{mission_policy, ResolvedSystemConfig};
use spanda_readiness::{verify_mission, MissionVerificationReport};

/// Mission assurance report.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MissionAssuranceReport {
    pub plans: Vec<MissionPlan>,
    pub execution: MissionExecutionState,
    pub verification: spanda_readiness::MissionVerificationReport,
    pub abort_reasons: Vec<MissionAbortReason>,
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
    // Description:
    //     Verify mission assurance.
    //
    // Inputs:
    //     progra: &Program
    //         Caller-supplied progra.
    //
    // Outputs:
    //     result: MissionAssuranceReport
    //         Return value from `verify_mission_assurance`.
    //
    // Example:

    //     let result = spanda_assurance::mission::verify_mission_assurance(progra);

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

    let execution = MissionExecutionState {
        plan: plans.first().map(|p| p.name.clone()).unwrap_or_default(),
        current_step: plans.first().and_then(|p| p.steps.first().cloned()),
        status: if passed {
            "ready".into()
        } else {
            "blocked".into()
        },
    };

    MissionAssuranceReport {
        plans,
        execution,
        verification,
        abort_reasons,
        passed,
    }
}
