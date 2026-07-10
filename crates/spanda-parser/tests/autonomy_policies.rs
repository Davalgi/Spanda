//! Parser coverage for bio-inspired autonomy policy declarations.

use spanda_ast::assurance_decl::{AttentionPolicyDecl, HomeostasisPolicyDecl};
use spanda_lexer::tokenize;
use spanda_parser::parse;

#[test]
fn parse_at_policy_homeostasis_and_attention() {
    // `@policy` forms parse into homeostasis / attention decls.
    //
    // Parameters:
    // None.
    //
    // Returns:
    // None.
    //
    // Options:
    // None.
    //
    // Example:
    // parse_at_policy_homeostasis_and_attention();

    let source = r#"
@policy(kind: "homeostasis")
platform {
    metric cpu_pct;
    metric memory_pct;
    metric battery_pct;
}

@policy(kind: "attention")
mission_focus {
    rule suppress_low_priority;
    rule boost_critical_health;
}
"#;
    let tokens = tokenize(source).expect("tokenize autonomy policies");
    let program = parse(tokens).expect("parse autonomy policies");
    let spanda_ast::nodes::Program::Program {
        homeostasis_policies,
        attention_policies,
        ..
    } = program;
    assert_eq!(homeostasis_policies.len(), 1);
    assert_eq!(attention_policies.len(), 1);
    match &homeostasis_policies[0] {
        HomeostasisPolicyDecl::HomeostasisPolicyDecl {
            name,
            metrics,
            legacy_syntax,
            ..
        } => {
            assert_eq!(name, "platform");
            assert_eq!(metrics, &["cpu_pct", "memory_pct", "battery_pct"]);
            assert!(!legacy_syntax);
        }
    }
    match &attention_policies[0] {
        AttentionPolicyDecl::AttentionPolicyDecl {
            name,
            rules,
            legacy_syntax,
            ..
        } => {
            assert_eq!(name, "mission_focus");
            assert_eq!(rules, &["suppress_low_priority", "boost_critical_health"]);
            assert!(!legacy_syntax);
        }
    }
}

#[test]
fn legacy_homeostasis_policy_keyword_rejected() {
    // Legacy `homeostasis_policy` is a hard parse error after the keyword removal.
    //
    // Parameters:
    // None.
    //
    // Returns:
    // None.
    //
    // Options:
    // None.
    //
    // Example:
    // legacy_homeostasis_policy_keyword_rejected();

    let source = r#"
homeostasis_policy platform {
    metric cpu_pct;
}
"#;
    let tokens = tokenize(source).expect("tokenize");
    let err = parse(tokens).expect_err("legacy keyword must fail");
    let msg = err.to_string();
    assert!(
        msg.contains("Expected") || msg.contains("homeostasis") || msg.contains("Unexpected"),
        "unexpected error: {msg}"
    );
}

#[test]
fn legacy_attention_policy_keyword_rejected() {
    // Legacy `attention_policy` is a hard parse error after the keyword removal.
    //
    // Parameters:
    // None.
    //
    // Returns:
    // None.
    //
    // Options:
    // None.
    //
    // Example:
    // legacy_attention_policy_keyword_rejected();

    let source = r#"
attention_policy mission_focus {
    rule suppress_low_priority;
}
"#;
    let tokens = tokenize(source).expect("tokenize");
    let err = parse(tokens).expect_err("legacy keyword must fail");
    let msg = err.to_string();
    assert!(
        msg.contains("Expected") || msg.contains("attention") || msg.contains("Unexpected"),
        "unexpected error: {msg}"
    );
}
