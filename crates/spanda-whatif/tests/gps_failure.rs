//! Integration tests for what-if scenario analysis.

use spanda_lexer::tokenize;
use spanda_parser::parse;
use spanda_whatif::{run_what_if_analysis, WhatIfOptions};

const GPS_FAILURE: &str = include_str!("../../../examples/showcase/what_if/gps_failure.sd");

fn parse_program(source: &str) -> spanda_ast::nodes::Program {
    parse(tokenize(source).unwrap()).unwrap()
}

#[test]
fn what_if_gps_failure_scenario_reports_recovery_plan() {
    let program = parse_program(GPS_FAILURE);
    let report = run_what_if_analysis(
        &program,
        "gps_failure.sd",
        &WhatIfOptions {
            scenarios: vec!["gps_failure".into()],
            ..Default::default()
        },
    );
    assert_eq!(report.scenarios.len(), 1);
    let scenario = &report.scenarios[0];
    assert_eq!(scenario.scenario, "gps_failure");
    assert_eq!(scenario.impact, "navigation_degraded");
    assert!(!scenario.recovery_plan.is_empty());
    assert!(scenario.probability > 0.0 && scenario.probability < 1.0);
}

#[test]
fn what_if_defaults_infer_gps_for_rover_program() {
    let program = parse_program(GPS_FAILURE);
    let report = run_what_if_analysis(&program, "gps_failure.sd", &WhatIfOptions::default());
    assert!(!report.scenarios.is_empty());
    assert!(report.scenarios.iter().any(|s| s.scenario.contains("gps")));
}
