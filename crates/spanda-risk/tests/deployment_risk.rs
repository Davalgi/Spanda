//! Integration tests for mission risk scoring.

use spanda_lexer::tokenize;
use spanda_parser::parse;
use spanda_risk::evaluate_mission_risk;

const WAREHOUSE: &str = include_str!("../../../examples/showcase/differentiation/warehouse.sd");

fn parse_program(source: &str) -> spanda_ast::nodes::Program {
    parse(tokenize(source).unwrap()).unwrap()
}

#[test]
fn mission_risk_warehouse_produces_score_and_factors() {
    let program = parse_program(WAREHOUSE);
    let report = evaluate_mission_risk(&program, WAREHOUSE, "warehouse.sd");
    assert!(report.score.total <= 100);
    assert!(!report.score.tier.is_empty());
    assert!(report.factors.iter().any(|f| f.name == "readiness"));
    assert!(report.factors.iter().any(|f| f.name == "safety_coverage"));
}

#[test]
fn mission_risk_includes_fleet_factor_for_warehouse_fleet() {
    let program = parse_program(WAREHOUSE);
    let report = evaluate_mission_risk(&program, WAREHOUSE, "warehouse.sd");
    assert!(report.factors.iter().any(|f| f.name == "fleet_dependency"));
}
