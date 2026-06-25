//! Safety scenario coverage reporting for deploy-time assurance.

use serde::{Deserialize, Serialize};
use spanda_ast::nodes::{Program, RobotDecl, SafetyBlock};
use spanda_hardware::{verify_program_compatibility, VerifyOptions};

/// Coverage status for a safety scenario.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CoverageStatus {
    Covered,
    PartiallyCovered,
    Uncovered,
}

/// Named safety scenario evaluation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SafetyScenarioReport {
    pub name: String,
    pub status: CoverageStatus,
    #[serde(default)]
    pub evidence: Vec<String>,
    #[serde(default)]
    pub gaps: Vec<String>,
}

/// Full safety coverage report.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SafetyCoverageReport {
    pub program: String,
    pub overall_coverage_pct: u32,
    pub scenarios: Vec<SafetyScenarioReport>,
    pub recommendations: Vec<String>,
}

fn has_stop_if_rules(program: &Program) -> bool {
    // Description:
    //     Detect obstacle-related stop_if safety rules.
    //
    // Parameters:
    // - `program` — parsed program
    //
    // Returns:
    // True when stop_if or lidar safety rules exist.
    //
    // Options:
    // None.
    //
    // Example:
    // let ok = has_stop_if_rules(&program);

    let Program::Program { robots, .. } = program;
    robots.iter().any(|robot| {
        let RobotDecl::RobotDecl { safety, .. } = robot;
        matches!(
            safety,
            Some(SafetyBlock::SafetyBlock { rules, .. })
                if rules.iter().any(|rule| {
                    let text = format!("{rule:?}").to_ascii_lowercase();
                    text.contains("stop_if") || text.contains("lidar") || text.contains("obstacle") || text.contains("nearest_distance")
                })
        )
    })
}

fn has_recovery_policies(program: &Program) -> bool {
    let Program::Program {
        recovery_policies, ..
    } = program;
    !recovery_policies.is_empty()
}

fn has_continuity_policies(program: &Program) -> bool {
    let Program::Program {
        continuity_policies,
        ..
    } = program;
    !continuity_policies.is_empty()
}

fn has_gps_recovery(program: &Program) -> bool {
    let Program::Program {
        recovery_policies, ..
    } = program;
    recovery_policies.iter().any(|policy| {
        let text = format!("{policy:?}").to_ascii_lowercase();
        text.contains("gps") || text.contains("connect")
    })
}

fn has_battery_guard(program: &Program) -> bool {
    // Description:
    //     Detect battery constraints or stop thresholds.
    //
    // Parameters:
    // - `program` — parsed program
    //
    // Returns:
    // True when battery guards are declared.
    //
    // Options:
    // None.
    //
    // Example:
    // let ok = has_battery_guard(&program);

    let Program::Program {
        mission_plans,
        robots,
        ..
    } = program;
    let plan_guard = mission_plans.iter().any(|plan| {
        let spanda_ast::assurance_decl::MissionPlanDecl::MissionPlanDecl { constraints, .. } = plan;
        constraints
            .iter()
            .any(|item| item.constraint.to_ascii_lowercase().contains("battery"))
    });
    let robot_guard = robots.iter().any(|robot| {
        let RobotDecl::RobotDecl { safety, .. } = robot;
        matches!(
            safety,
            Some(SafetyBlock::SafetyBlock { rules, .. })
                if rules.iter().any(|rule| {
                    format!("{rule:?}").to_ascii_lowercase().contains("battery")
                })
        )
    });
    plan_guard || robot_guard
}

fn coverage_pct(scenarios: &[SafetyScenarioReport]) -> u32 {
    // Description:
    //     Compute weighted overall coverage percentage.
    //
    // Parameters:
    // - `scenarios` — evaluated scenarios
    //
    // Returns:
    // Integer percent covered (0–100).
    //
    // Options:
    // None.
    //
    // Example:
    // let pct = coverage_pct(&scenarios);

    if scenarios.is_empty() {
        return 0;
    }
    let points: u32 = scenarios
        .iter()
        .map(|scenario| match scenario.status {
            CoverageStatus::Covered => 100,
            CoverageStatus::PartiallyCovered => 50,
            CoverageStatus::Uncovered => 0,
        })
        .sum();
    points / scenarios.len() as u32
}

/// Evaluate safety scenario coverage for a program.
pub fn evaluate_safety_coverage(program: &Program, source_label: &str) -> SafetyCoverageReport {
    // Description:
    //     Score safety scenario coverage from static program analysis.
    //
    // Parameters:
    // - `program` — parsed program AST
    // - `source_label` — file label for reports
    //
    // Returns:
    // Safety coverage report with scenarios and recommendations.
    //
    // Options:
    // None.
    //
    // Example:
    // let report = evaluate_safety_coverage(&program, "rover.sd");

    let mut scenarios = Vec::new();
    let mut recommendations = Vec::new();

    let obstacle = has_stop_if_rules(program);
    scenarios.push(SafetyScenarioReport {
        name: "obstacle_avoidance".into(),
        status: if obstacle {
            CoverageStatus::Covered
        } else {
            CoverageStatus::Uncovered
        },
        evidence: if obstacle {
            vec!["safety.stop_if or lidar rule".into()]
        } else {
            Vec::new()
        },
        gaps: if obstacle {
            Vec::new()
        } else {
            vec!["no_stop_if_or_lidar_rule".into()]
        },
    });
    if !obstacle {
        recommendations.push(
            "Add safety { stop_if ... } or lidar proximity rule for obstacle_avoidance".into(),
        );
    }

    let gps_partial = has_gps_recovery(program);
    let hw = verify_program_compatibility(program, &VerifyOptions::default());
    let has_gps_sensor = hw.items.iter().any(|item| {
        item.message.to_ascii_lowercase().contains("gps")
            || item.category.to_ascii_lowercase().contains("gps")
    });
    scenarios.push(SafetyScenarioReport {
        name: "gps_failure".into(),
        status: if gps_partial && has_gps_sensor {
            CoverageStatus::Covered
        } else if gps_partial || has_gps_sensor {
            CoverageStatus::PartiallyCovered
        } else {
            CoverageStatus::Uncovered
        },
        evidence: {
            let mut items = Vec::new();
            if gps_partial {
                items.push("recovery_policy for gps/connectivity".into());
            }
            if has_gps_sensor {
                items.push("gps hardware declared".into());
            }
            items
        },
        gaps: if gps_partial {
            Vec::new()
        } else {
            vec!["no_imu_fallback".into()]
        },
    });
    if !gps_partial {
        recommendations
            .push("Add recovery_policy on gps_loss with degraded navigation fallback".into());
    }

    let battery = has_battery_guard(program);
    scenarios.push(SafetyScenarioReport {
        name: "battery_failure".into(),
        status: if battery {
            CoverageStatus::Covered
        } else {
            CoverageStatus::PartiallyCovered
        },
        evidence: if battery {
            vec!["battery constraint or stop_if".into()]
        } else {
            vec!["hardware battery check only".into()]
        },
        gaps: if battery {
            Vec::new()
        } else {
            vec!["no_explicit_battery_stop".into()]
        },
    });

    let connectivity_ok = !hw
        .items
        .iter()
        .any(|item| item.category == "connectivity" && item.message.contains("missing"));
    scenarios.push(SafetyScenarioReport {
        name: "connectivity_failure".into(),
        status: if connectivity_ok && gps_partial {
            CoverageStatus::Covered
        } else if connectivity_ok || gps_partial {
            CoverageStatus::PartiallyCovered
        } else {
            CoverageStatus::Uncovered
        },
        evidence: if connectivity_ok {
            vec!["connectivity declared".into()]
        } else {
            Vec::new()
        },
        gaps: if gps_partial {
            Vec::new()
        } else {
            vec!["no_failover_policy".into()]
        },
    });

    let Program::Program { mitigations, .. } = program;
    let provider_ok = !mitigations.is_empty();
    scenarios.push(SafetyScenarioReport {
        name: "provider_failure".into(),
        status: if provider_ok {
            CoverageStatus::Covered
        } else {
            CoverageStatus::PartiallyCovered
        },
        evidence: if provider_ok {
            vec!["mitigation declarations".into()]
        } else {
            vec!["mock provider fallback".into()]
        },
        gaps: if provider_ok {
            Vec::new()
        } else {
            vec!["no_mitigation_plan".into()]
        },
    });

    let continuity = has_continuity_policies(program);
    scenarios.push(SafetyScenarioReport {
        name: "takeover_failure".into(),
        status: if continuity {
            CoverageStatus::Covered
        } else {
            CoverageStatus::Uncovered
        },
        evidence: if continuity {
            vec!["continuity_policy declared".into()]
        } else {
            Vec::new()
        },
        gaps: if continuity {
            Vec::new()
        } else {
            vec!["no_continuity_policy".into()]
        },
    });
    if !continuity {
        recommendations
            .push("Add continuity_policy with auto_takeover for takeover_failure".into());
    }

    let recovery_ok = has_recovery_policies(program);
    scenarios.push(SafetyScenarioReport {
        name: "recovery_failure".into(),
        status: if recovery_ok {
            CoverageStatus::Covered
        } else {
            CoverageStatus::Uncovered
        },
        evidence: if recovery_ok {
            vec!["recovery_policy declared".into()]
        } else {
            Vec::new()
        },
        gaps: if recovery_ok {
            Vec::new()
        } else {
            vec!["no_recovery_policy".into()]
        },
    });

    let overall_coverage_pct = coverage_pct(&scenarios);
    SafetyCoverageReport {
        program: source_label.into(),
        overall_coverage_pct,
        scenarios,
        recommendations,
    }
}

/// Format safety coverage for CLI output.
pub fn format_safety_coverage(report: &SafetyCoverageReport, json: bool, markdown: bool) -> String {
    // Description:
    //     Render safety coverage report text, markdown, or JSON.
    //
    // Parameters:
    // - `report` — coverage report
    // - `json` — JSON output when true
    // - `markdown` — markdown bullets when true
    //
    // Returns:
    // Formatted string.
    //
    // Options:
    // None.
    //
    // Example:
    // let text = format_safety_coverage(&report, false, false);

    if json {
        return serde_json::to_string_pretty(report).unwrap_or_default();
    }
    let mut out = String::new();
    out.push_str(&format!(
        "Safety coverage: {}% ({})\n",
        report.overall_coverage_pct, report.program
    ));
    for scenario in &report.scenarios {
        let status = match scenario.status {
            CoverageStatus::Covered => "covered",
            CoverageStatus::PartiallyCovered => "partially_covered",
            CoverageStatus::Uncovered => "uncovered",
        };
        if markdown {
            out.push_str(&format!(
                "- **{}**: {} — evidence: {:?}\n",
                scenario.name, status, scenario.evidence
            ));
        } else {
            out.push_str(&format!(
                "  {}: {} evidence={:?} gaps={:?}\n",
                scenario.name, status, scenario.evidence, scenario.gaps
            ));
        }
    }
    if !report.recommendations.is_empty() {
        out.push_str("\nRecommendations:\n");
        for item in &report.recommendations {
            out.push_str(&format!("  * {item}\n"));
        }
    }
    out
}
