use spanda_contract::verify_contract;
use spanda_lexer::tokenize;
use spanda_parser::parse as parse_tokens;

const WAREHOUSE: &str = include_str!("../../../examples/showcase/differentiation/warehouse.sd");

fn parse_program(source: &str) -> spanda_ast::nodes::Program {
    let tokens = tokenize(source).expect("tokenize");
    parse_tokens(tokens).expect("parse")
}

#[test]
fn contract_verify_warehouse_passes() {
    let program = parse_program(WAREHOUSE);
    let report = verify_contract(&program, "warehouse.sd");
    assert!(!report.contracts.is_empty(), "expected mission_plan contract");
    assert!(
        report.contracts.iter().any(|c| c.name == "WarehouseInventoryScan"),
        "mission plan name"
    );
    assert!(report.checks.iter().any(|c| c.name == "safety_clause" && c.passed));
    assert!(report.checks.iter().any(|c| c.name == "continuity_clause" && c.passed));
    assert!(report.checks.iter().any(|c| c.name == "recovery_clause" && c.passed));
    assert!(report.passed, "expected contract verification to pass: {:?}", report.issues);
}
