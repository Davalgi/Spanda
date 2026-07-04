//! Property-style policy engine tests.

use spanda_lexer::tokenize;
use spanda_parser::parse;
use spanda_policy::{evaluate_policy, list_policies};

fn empty_program() -> spanda_ast::nodes::Program {
    let tokens = tokenize("robot R { }").expect("tokenize");
    parse(tokens).expect("parse")
}

#[test]
fn policy_engine_never_panics_on_unknown_policy_names() {
    // Unknown policy names must fail closed without panicking.
    let program = empty_program();
    let names = ["", "default", "production", "no-such-policy", "!!!", &"a".repeat(64)];
    for name in names {
        let _ = std::panic::catch_unwind(|| {
            let _ = evaluate_policy(&program, name, "property-test");
        })
        .expect("policy engine must not panic");
    }
}

#[test]
fn list_policies_is_empty_for_program_without_policies() {
    // Programs without policy blocks must list no policies.
    let program = empty_program();
    assert!(list_policies(&program).is_empty());
}
