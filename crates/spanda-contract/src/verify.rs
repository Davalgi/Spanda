//! Static mission contract verification over existing program declarations.

use spanda_assurance::{extract_continuity_policies, extract_recovery_policies};
use spanda_ast::assurance_decl::MissionPlanDecl;
use spanda_ast::foundations::MissionDecl;
use spanda_ast::nodes::{Program, RobotDecl, SafetyBlock};
use spanda_readiness::verify_mission;

use crate::report::{ContractCheck, ContractVerificationReport, MissionContractReport};

fn robot_has_safety(robot: &RobotDecl) -> bool {
    // Description:
    //     Return whether the robot declares a safety block.
    //
    // Parameters:
    // - `robot` — parsed robot declaration
    //
    // Returns:
    // True when a `safety { }` block is present.
    //
    // Options:
    // None.
    //
    // Example:
    // let ok = robot_has_safety(robot);

    let RobotDecl::RobotDecl { safety, .. } = robot;
    matches!(safety, Some(SafetyBlock::SafetyBlock { .. }))
}

fn program_has_safety_rules(program: &Program) -> bool {
    // Description:
    //     Return whether any robot or program-level safety rule exists.
    //
    // Parameters:
    // - `program` — parsed Spanda program
    //
    // Returns:
    // True when safety rules or zones are declared.
    //
    // Options:
    // None.
    //
    // Example:
    // let ok = program_has_safety_rules(&program);

    let Program::Program {
        robots,
        program_safety_zones,
        ..
    } = program;
    !program_safety_zones.is_empty()
        || robots.iter().any(|robot| {
            let RobotDecl::RobotDecl { safety, .. } = robot;
            matches!(
                safety,
                Some(SafetyBlock::SafetyBlock { rules, zones, .. })
                    if !rules.is_empty() || !zones.is_empty()
            )
        })
}

/// Verify mission contracts implied by mission plans, robot missions, and policy declarations.
pub fn verify_contract(program: &Program, source_label: &str) -> ContractVerificationReport {
    // Description:
    //     Build a contract verification report from static program analysis.
    //
    // Parameters:
    // - `program` — type-checked AST root
    // - `source_label` — file path or label for reports
    //
    // Returns:
    // Contract verification report with per-mission summaries and checks.
    //
    // Options:
    // None.
    //
    // Example:
    // let report = verify_contract(&program, "warehouse.sd");

    let Program::Program {
        mission_plans,
        robots,
        health_checks,
        health_policies,
        ..
    } = program;

    let continuity = extract_continuity_policies(program);
    let recovery = extract_recovery_policies(program);
    let has_continuity = !continuity.is_empty();
    let has_recovery = !recovery.is_empty();
    let has_safety = program_has_safety_rules(program);

    let mut contracts = Vec::new();
    let mut checks = Vec::new();
    let mut issues = Vec::new();

    for plan in mission_plans {
        let MissionPlanDecl::MissionPlanDecl {
            name,
            steps,
            constraints,
            ..
        } = plan;
        let objectives: Vec<String> = steps.iter().map(|step| step.name.clone()).collect();
        let constraint_lines: Vec<String> = constraints
            .iter()
            .map(|item| item.constraint.clone())
            .collect();
        let invariants: Vec<String> = constraint_lines
            .iter()
            .filter(|line| line.contains("invariant") || line.contains(">=") || line.contains("<="))
            .cloned()
            .collect();
        contracts.push(MissionContractReport {
            name: name.clone(),
            kind: "mission_plan".into(),
            objectives,
            constraints: constraint_lines,
            assumptions: health_checks
                .iter()
                .map(
                    |spanda_ast::foundations::HealthCheckDecl::HealthCheckDecl { name, .. }| {
                        format!("health_check:{name}")
                    },
                )
                .chain(health_policies.iter().map(
                    |spanda_ast::foundations::HealthPolicyDecl::HealthPolicyDecl {
                         name, ..
                     }| { format!("health_policy:{name}") },
                ))
                .collect(),
            invariants,
            guarantees: vec!["mission_steps_declared".into()],
            continuity_aligned: has_continuity,
            recovery_aligned: has_recovery,
            safety_aligned: has_safety,
        });
    }

    for robot in robots {
        let RobotDecl::RobotDecl {
            name,
            mission,
            safety,
            ..
        } = robot;
        let Some(mission) = mission.as_ref() else {
            continue;
        };
        let MissionDecl::MissionDecl {
            name: mission_name,
            steps,
            required_capabilities,
            required_approvals,
            duration_hours,
            ..
        } = mission;
        contracts.push(MissionContractReport {
            name: mission_name
                .clone()
                .unwrap_or_else(|| format!("{name}_mission")),
            kind: format!("robot_mission:{name}"),
            objectives: steps.clone(),
            constraints: vec![format!("duration_hours={duration_hours:?}")],
            assumptions: required_capabilities.clone(),
            invariants: required_approvals
                .iter()
                .map(|approval| format!("requires_approval:{}", approval.actor))
                .collect(),
            guarantees: vec!["capabilities_declared".into()],
            continuity_aligned: has_continuity,
            recovery_aligned: has_recovery,
            safety_aligned: robot_has_safety(robot) || has_safety,
        });
        let _ = safety;
    }

    if contracts.is_empty() {
        issues.push("No mission_plan or robot mission declarations found".into());
        checks.push(ContractCheck {
            name: "mission_declared".into(),
            category: "structure".into(),
            passed: false,
            detail: "Add mission_plan or robot mission block".into(),
        });
    } else {
        checks.push(ContractCheck {
            name: "mission_declared".into(),
            category: "structure".into(),
            passed: true,
            detail: format!("{} contract(s) derived", contracts.len()),
        });
    }

    if !has_safety {
        issues.push("No safety rules or zones declared for contract safety clause".into());
        checks.push(ContractCheck {
            name: "safety_clause".into(),
            category: "safety".into(),
            passed: false,
            detail: "Add robot safety { } or program safety zones".into(),
        });
    } else {
        checks.push(ContractCheck {
            name: "safety_clause".into(),
            category: "safety".into(),
            passed: true,
            detail: "Safety rules present".into(),
        });
    }

    if !has_continuity && !robots.is_empty() {
        checks.push(ContractCheck {
            name: "continuity_clause".into(),
            category: "continuity".into(),
            passed: false,
            detail: "Fleet/robot missions without continuity_policy".into(),
        });
    } else if has_continuity {
        checks.push(ContractCheck {
            name: "continuity_clause".into(),
            category: "continuity".into(),
            passed: true,
            detail: format!("{} continuity policy(ies)", continuity.len()),
        });
    }

    if !has_recovery {
        checks.push(ContractCheck {
            name: "recovery_clause".into(),
            category: "recovery".into(),
            passed: false,
            detail: "No recovery_policy declared".into(),
        });
    } else {
        checks.push(ContractCheck {
            name: "recovery_clause".into(),
            category: "recovery".into(),
            passed: true,
            detail: format!("{} recovery policy(ies)", recovery.len()),
        });
    }

    let mission_reports = verify_mission(program, None);
    let has_robot_missions = robots.iter().any(|robot| {
        let RobotDecl::RobotDecl { mission, .. } = robot;
        mission.is_some()
    });
    let mission_ok = if has_robot_missions {
        mission_reports.iter().all(|report| report.achievable)
    } else {
        true
    };
    checks.push(ContractCheck {
        name: "mission_achievable".into(),
        category: "verify".into(),
        passed: mission_ok,
        detail: if mission_ok {
            "Readiness mission verification passed".into()
        } else {
            "Readiness mission verification reported blockers".into()
        },
    });
    if !mission_ok {
        for report in mission_reports {
            if has_robot_missions || report.mission_name.is_some() {
                issues.extend(report.issues.clone());
            }
        }
    }

    let passed = issues.is_empty() && checks.iter().all(|check| check.passed);
    ContractVerificationReport {
        program: source_label.into(),
        passed,
        contracts,
        checks,
        issues,
    }
}
