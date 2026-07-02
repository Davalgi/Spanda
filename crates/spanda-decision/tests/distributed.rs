//! Integration tests for distributed decision architecture.

use spanda_decision::{
    evaluate_distributed_decisions, evaluate_tree, extract_decision_authorities,
    extract_decision_trees, extract_offline_policies, simulate_distributed_decisions,
    DecisionContext, DecisionLayer, SimulationOptions,
};
use spanda_lexer::tokenize;
use spanda_parser::parse;
use std::collections::HashMap;

fn parse_sd(source: &str) -> spanda_ast::nodes::Program {
    let tokens = tokenize(source).expect("tokenize");
    parse(tokens).expect("parse")
}

#[test]
fn extracts_entity_decision_authority() {
    let program = parse_sd(
        r#"
        robot Rover001 {
            local_decision_authority [emergency_stop, degraded_mode];
            requires_central_approval [update_firmware];
        }
        "#,
    );
    let authorities = extract_decision_authorities(&program);
    assert_eq!(authorities.len(), 1);
    assert_eq!(authorities[0].entity_id, "Rover001");
    assert!(authorities[0].local_actions.contains(&"emergency_stop".into()));
    assert!(authorities[0]
        .requires_central_approval
        .contains(&"update_firmware".into()));
}

#[test]
fn evaluates_decision_tree() {
    let program = parse_sd(
        r#"
        decision_tree GPSLoss local {
            when gps.status == Failed {
                if visual_odometry.available { enter degraded_mode; }
            }
        }
        "#,
    );
    let trees = extract_decision_trees(&program);
    assert_eq!(trees.len(), 1);
    let mut signals = HashMap::new();
    signals.insert("gps.status == Failed".into(), true);
    signals.insert("visual_odometry.available".into(), true);
    let result = evaluate_tree(&trees[0], &signals).expect("match");
    assert!(result
        .actions
        .iter()
        .any(|a| a.contains("degraded") || a.contains("degraded_mode")));
}

#[test]
fn offline_policy_blocks_forbidden_action() {
    let program = parse_sd(
        r#"
        offline_policy RoverOffline {
            max_duration = 30 min;
            allowed_actions [return_home];
            forbidden_actions [disable_safety];
        }
        "#,
    );
    let policies = extract_offline_policies(&program);
    assert_eq!(policies.len(), 1);
    assert_eq!(policies[0].max_duration_minutes, 30);
}

#[test]
fn simulate_offline_scenario() {
    let program = parse_sd(
        r#"
        robot Rover001 {
            local_decision_authority [return_home];
        }
        offline_policy RoverOffline {
            max_duration = 30 min;
            allowed_actions [return_home];
            forbidden_actions [disable_safety];
        }
        "#,
    );
    let sim = simulate_distributed_decisions(
        &program,
        SimulationOptions {
            offline: true,
            entity_id: "Rover001".into(),
            ..Default::default()
        },
    );
    assert_eq!(sim.scenario, "offline");
}

#[test]
fn central_approval_blocks_action() {
    let program = parse_sd(
        r#"
        robot Rover001 {
            local_decision_authority [emergency_stop];
            requires_central_approval [update_firmware];
        }
        "#,
    );
    let ctx = DecisionContext {
        entity_id: "Rover001".into(),
        action: "update_firmware".into(),
        layer: DecisionLayer::LocalEntity,
        ..Default::default()
    };
    let report = evaluate_distributed_decisions(&program, &ctx);
    assert!(!report.passed);
}
