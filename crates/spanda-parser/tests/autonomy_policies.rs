//! Parser coverage for bio-inspired autonomy policy declarations.

use spanda_ast::assurance_decl::{AttentionPolicyDecl, HomeostasisPolicyDecl};
use spanda_lexer::tokenize;
use spanda_parser::parse;

#[test]
fn parse_homeostasis_and_attention_policies() {
    let source = r#"
homeostasis_policy platform {
    metric cpu_pct;
    metric memory_pct;
    metric battery_pct;
}

attention_policy mission_focus {
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
        HomeostasisPolicyDecl::HomeostasisPolicyDecl { name, metrics, .. } => {
            assert_eq!(name, "platform");
            assert_eq!(metrics, &["cpu_pct", "memory_pct", "battery_pct"]);
        }
    }
    match &attention_policies[0] {
        AttentionPolicyDecl::AttentionPolicyDecl { name, rules, .. } => {
            assert_eq!(name, "mission_focus");
            assert_eq!(rules, &["suppress_low_priority", "boost_critical_health"]);
        }
    }
}
