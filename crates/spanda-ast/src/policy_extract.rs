//! Extract `@policy` / legacy cognitive-policy names from a parsed program.
//!
use crate::assurance_decl::{AttentionPolicyDecl, HomeostasisPolicyDecl};
use crate::nodes::Program;

/// Collect homeostasis metric names declared in the program.
///
/// Parameters:
/// - `program` — parsed Spanda program AST
///
/// Returns:
/// Flattened metric identifiers from all homeostasis policy decls (attribute or legacy).
///
/// Options:
/// None.
///
/// Example:
/// let names = homeostasis_metric_names(&program);
pub fn homeostasis_metric_names(program: &Program) -> Vec<String> {
    // Walk every homeostasis policy decl and collect metric identifiers.
    let Program::Program {
        homeostasis_policies,
        ..
    } = program;
    let mut names = Vec::new();
    for policy in homeostasis_policies {
        let HomeostasisPolicyDecl::HomeostasisPolicyDecl { metrics, .. } = policy;
        names.extend(metrics.iter().cloned());
    }
    names
}

/// Collect attention rule names declared in the program.
///
/// Parameters:
/// - `program` — parsed Spanda program AST
///
/// Returns:
/// Flattened rule identifiers from all attention policy decls (attribute or legacy).
///
/// Options:
/// None.
///
/// Example:
/// let names = attention_rule_names(&program);
pub fn attention_rule_names(program: &Program) -> Vec<String> {
    // Walk every attention policy decl and collect rule identifiers.
    let Program::Program {
        attention_policies, ..
    } = program;
    let mut names = Vec::new();
    for policy in attention_policies {
        let AttentionPolicyDecl::AttentionPolicyDecl { rules, .. } = policy;
        names.extend(rules.iter().cloned());
    }
    names
}
