//! Static and trace explainability builders.

use spanda_ast::nodes::Program;
use spanda_contract::verify_contract;
use spanda_decision::audit_decisions_from_trace;
use spanda_hardware::{verify_program_compatibility, VerifyOptions};
use spanda_readiness::{
    evaluate_readiness, evaluate_safety_coverage, generate_safety_report, verify_mission,
    ReadinessOptions,
};

use crate::report::{ExplainReport, ExplainSection};

fn program_structure(program: &Program) -> ExplainSection {
    // Description:
    //     Summarize top-level program structure.
    //
    // Parameters:
    // - `program` — parsed program
    //
    // Returns:
    // Structure explain section.
    //
    // Options:
    // None.
    //
    // Example:
    // let section = program_structure(&program);

    let Program::Program {
        robots,
        mission_plans,
        recovery_policies,
        continuity_policies,
        fleets,
        ..
    } = program;
    ExplainSection {
        topic: "structure".into(),
        summary: format!(
            "{} robot(s), {} mission plan(s), {} recovery policy(ies), {} continuity policy(ies), {} fleet(s)",
            robots.len(),
            mission_plans.len(),
            recovery_policies.len(),
            continuity_policies.len(),
            fleets.len()
        ),
        details: robots
            .iter()
            .map(|robot| {
                let spanda_ast::nodes::RobotDecl::RobotDecl { name, .. } = robot;
                format!("robot {name}")
            })
            .collect(),
    }
}

/// Explain program structure and linked operational surfaces.
pub fn explain_program(program: &Program, source_label: &str) -> ExplainReport {
    // Description:
    //     Build a multi-section explainability report for a program.
    //
    // Parameters:
    // - `program` — parsed program
    // - `source_label` — file label
    //
    // Returns:
    // Explainability report with structure, contract, readiness, verify, and safety sections.
    //
    // Options:
    // None.
    //
    // Example:
    // let report = explain_program(&program, "rover.sd");

    let mut sections = vec![program_structure(program)];
    sections.push(explain_readiness(program, source_label).sections[0].clone());
    sections.push(explain_verify(program, source_label).sections[0].clone());
    sections.push(explain_safety(program, source_label).sections[0].clone());
    let contract = verify_contract(program, source_label);
    sections.push(ExplainSection {
        topic: "contract".into(),
        summary: if contract.passed {
            "Mission contract verification passed".into()
        } else {
            format!(
                "Mission contract verification failed ({} issue(s))",
                contract.issues.len()
            )
        },
        details: contract
            .checks
            .iter()
            .map(|check| format!("{}: {}", check.name, check.detail))
            .collect(),
    });
    ExplainReport {
        program: source_label.into(),
        sections,
    }
}

/// Explain readiness scoring failures and blockers.
pub fn explain_readiness(program: &Program, source_label: &str) -> ExplainReport {
    // Description:
    //     Explain readiness go/no-go results in plain language.
    //
    // Parameters:
    // - `program` — parsed program
    // - `source_label` — file label
    //
    // Returns:
    // Explainability report with readiness section.
    //
    // Options:
    // None.
    //
    // Example:
    // let report = explain_readiness(&program, "rover.sd");

    let report = evaluate_readiness(program, &ReadinessOptions::default());
    let summary = if report.mission_ready {
        format!(
            "Mission ready with score {}/{}",
            report.score.total, report.score.maximum
        )
    } else {
        format!(
            "Mission not ready — score {}/{}",
            report.score.total, report.score.maximum
        )
    };
    let details = report
        .issues
        .iter()
        .map(|issue| format!("[{:?}] {}", issue.severity, issue.message))
        .collect();
    ExplainReport {
        program: source_label.into(),
        sections: vec![ExplainSection {
            topic: "readiness".into(),
            summary,
            details,
        }],
    }
}

/// Explain hardware and mission verification results.
pub fn explain_verify(program: &Program, source_label: &str) -> ExplainReport {
    // Description:
    //     Explain verify compatibility and mission achievability.
    //
    // Parameters:
    // - `program` — parsed program
    // - `source_label` — file label
    //
    // Returns:
    // Explainability report with verify section.
    //
    // Options:
    // None.
    //
    // Example:
    // let report = explain_verify(&program, "rover.sd");

    let hw = verify_program_compatibility(program, &VerifyOptions::default());
    let missions = verify_mission(program, None);
    let mut details = Vec::new();
    for item in &hw.items {
        if item.severity != spanda_hardware::CompatSeverity::Pass {
            details.push(format!("[{}] {}", item.category, item.message));
        }
    }
    for mission in &missions {
        if !mission.achievable {
            details.extend(mission.issues.iter().cloned());
        }
    }
    let summary = if hw.compatible && missions.iter().all(|m| m.achievable) {
        "Hardware and mission verification passed".into()
    } else {
        format!(
            "Verification reported {} hardware item(s) and {} mission issue(s)",
            details.len(),
            missions.iter().map(|m| m.issues.len()).sum::<usize>()
        )
    };
    ExplainReport {
        program: source_label.into(),
        sections: vec![ExplainSection {
            topic: "verify".into(),
            summary,
            details,
        }],
    }
}

/// Explain safety rules and coverage gaps.
pub fn explain_safety(program: &Program, source_label: &str) -> ExplainReport {
    // Description:
    //     Explain safety case and scenario coverage.
    //
    // Parameters:
    // - `program` — parsed program
    // - `source_label` — file label
    //
    // Returns:
    // Explainability report with safety section.
    //
    // Options:
    // None.
    //
    // Example:
    // let report = explain_safety(&program, "rover.sd");

    let safety = generate_safety_report(program, source_label);
    let coverage = evaluate_safety_coverage(program, source_label);
    let mut details = safety.safety_rules.clone();
    details.extend(
        coverage
            .scenarios
            .iter()
            .filter(|scenario| scenario.status != spanda_readiness::SafetyCoverageStatus::Covered)
            .map(|scenario| format!("{}: {:?}", scenario.name, scenario.gaps)),
    );
    let summary = format!(
        "Safety deployable={} coverage={}%",
        safety.deployable, coverage.overall_coverage_pct
    );
    ExplainReport {
        program: source_label.into(),
        sections: vec![ExplainSection {
            topic: "safety".into(),
            summary,
            details,
        }],
    }
}

/// Explain decisions recorded in a mission trace.
pub fn explain_trace(trace_path: &str) -> Result<ExplainReport, String> {
    // Description:
    //     Explain autonomous decisions from a mission trace file.
    //
    // Parameters:
    // - `trace_path` — path to trace JSON
    //
    // Returns:
    // Explainability report or load error.
    //
    // Options:
    // None.
    //
    // Example:
    // let report = explain_trace("mission.trace")?;

    let audit = audit_decisions_from_trace(trace_path)?;
    let details = audit
        .timeline
        .decisions
        .iter()
        .map(|record| {
            format!(
                "{} @ {:.0}ms — {} ({})",
                record.decision_id, record.timestamp_ms, record.decision, record.reason
            )
        })
        .collect();
    Ok(ExplainReport {
        program: trace_path.into(),
        sections: vec![ExplainSection {
            topic: "decisions".into(),
            summary: format!("{} decision(s) in trace", audit.decision_count),
            details,
        }],
    })
}
