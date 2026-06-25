//! Integration tests for verify-time integrity verification.

use spanda_lexer::tokenize;
use spanda_parser::parse;
use spanda_tamper::{
    generate_integrity_report, ArtifactIntegrityStatus,
};

fn parse_file(relative: &str) -> spanda_ast::nodes::Program {
    let path = format!("{}/{}", env!("CARGO_MANIFEST_DIR"), relative);
    let source = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {path}: {e}"));
    let tokens = tokenize(&source).expect("tokenize");
    parse(tokens).expect("parse")
}

#[test]
fn warehouse_integrity_without_baseline_lists_hashes() {
    let program = parse_file("../../examples/showcase/policy/warehouse.sd");
    let report = generate_integrity_report(&program, "warehouse.sd", None, None);
    assert!(report.passed);
    assert!(report.artifacts.iter().all(|artifact| {
        artifact.status == ArtifactIntegrityStatus::Unknown
    }));
    assert!(report.artifacts.iter().any(|artifact| artifact.artifact_type == "mission"));
}

#[test]
fn warehouse_matches_itself_as_baseline() {
    let program = parse_file("../../examples/showcase/policy/warehouse.sd");
    let report = generate_integrity_report(
        &program,
        "warehouse.sd",
        Some(&program),
        Some("warehouse.sd"),
    );
    assert!(report.passed);
    assert!(report.artifacts.iter().all(|artifact| {
        artifact.status == ArtifactIntegrityStatus::Trusted
    }));
}

#[test]
fn readiness_rover_differs_from_warehouse_baseline() {
    let warehouse = parse_file("../../examples/showcase/policy/warehouse.sd");
    let rover = parse_file("../../examples/showcase/readiness/rover.sd");
    let report = generate_integrity_report(
        &rover,
        "rover.sd",
        Some(&warehouse),
        Some("warehouse.sd"),
    );
    assert!(!report.passed);
    assert!(report.artifacts.iter().any(|artifact| {
        artifact.status == ArtifactIntegrityStatus::Modified
    }));
}
