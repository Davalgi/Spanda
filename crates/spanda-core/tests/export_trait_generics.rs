//! Cross-module export trait and Experimental generics hardening tests.
//!
use spanda_ast::nodes::Program;
use spanda_core::{check, check_with_registry, compile, run, ModuleRegistry, RunOptions};
use spanda_lexer::tokenize;
use spanda_parser::parse;

#[test]
fn export_trait_cross_module_impl() {
    // Description:
    //     Export trait cross module impl.
    //
    // Inputs:
    //     None.
    //
    // Outputs:
    //     None.
    //
    // Example:

    //     let result = spanda_core::export_trait_generics::export_trait_cross_module_impl();

    let traits_mod = r#"
module navigation.traits;

export trait PathPlanner {
  fn plan(goal: Pose) -> Path;
}
"#;
    let main = r#"
module navigation;

import navigation.traits;

struct Pose {
  x: Distance;
  y: Distance;
  heading: Angle;
}

robot R {
  actuator wheels: DifferentialDrive;
  agent Nav {
    tools [wheels];
    goal "Navigate";
    plan { wheels.stop(); }
  }
  impl PathPlanner for Nav {
    fn plan(goal: Pose) -> Path {
      wheels.stop();
    }
  }
  behavior run() {
    Nav.plan(Pose { x: 0.0 m, y: 0.0 m, heading: 0.0 rad });
  }
}
"#;
    let traits_program = compile(traits_mod).expect("traits module").program;
    let mut registry = ModuleRegistry::new();
    registry.register("navigation.traits", &traits_program);
    check_with_registry(main, &registry).expect("exported trait should import");
    let opts = RunOptions {
        module_registry: Some(registry),
        ..Default::default()
    };
    run(main, opts).expect("cross-module trait impl should run");
}

#[test]
fn private_trait_not_imported() {
    // Description:
    //     Private trait not imported.
    //
    // Inputs:
    //     None.
    //
    // Outputs:
    //     None.
    //
    // Example:

    //     let result = spanda_core::export_trait_generics::private_trait_not_imported();

    let traits_mod = r#"
module navigation.traits;

trait HiddenPlanner {
  fn plan(goal: Pose) -> Path;
}
"#;
    let main = r#"
module navigation;
import navigation.traits;

robot R {
  actuator wheels: DifferentialDrive;
  agent Nav { tools [wheels]; goal "x"; plan { wheels.stop(); } }
  impl HiddenPlanner for Nav {
    fn plan(goal: Pose) -> Path { wheels.stop(); }
  }
}
"#;
    let traits_program = compile(traits_mod).expect("traits module").program;
    let mut registry = ModuleRegistry::new();
    registry.register("navigation.traits", &traits_program);
    let err = check_with_registry(main, &registry).expect_err("private trait must not import");
    assert!(
        err.diagnostics()
            .iter()
            .any(|d| d.message.contains("Unknown trait")),
        "got {:?}",
        err.diagnostics()
    );
}

#[test]
fn generic_empty_type_params_rejected() {
    // Description:
    //     Generic empty type params rejected.
    //
    // Inputs:
    //     None.
    //
    // Outputs:
    //     None.
    //
    // Example:

    //     let result = spanda_core::export_trait_generics::generic_empty_type_params_rejected();

    let source = r#"
module m;
export fn bad<>() -> Int { return 1; }
"#;
    let tokens = tokenize(source).expect("tokenize");
    let err = parse(tokens).expect_err("empty <> should fail");
    assert!(err.to_string().contains("type parameter"), "got {err}");
}

#[test]
fn generic_type_bound_rejected() {
    // Description:
    //     Generic type bound rejected.
    //
    // Inputs:
    //     None.
    //
    // Outputs:
    //     None.
    //
    // Example:

    //     let result = spanda_core::export_trait_generics::generic_type_bound_rejected();

    let source = r#"
module m;
export fn id<T: Clone>(value: T) -> T { return value; }
"#;
    let tokens = tokenize(source).expect("tokenize");
    let err = parse(tokens).expect_err("type bounds should fail");
    assert!(
        err.to_string().contains("Type bounds are not supported"),
        "got {err}"
    );
}

#[test]
fn generic_duplicate_type_param_rejected() {
    // Description:
    //     Generic duplicate type param rejected.
    //
    // Inputs:
    //     None.
    //
    // Outputs:
    //     None.
    //
    // Example:

    //     let result = spanda_core::export_trait_generics::generic_duplicate_type_param_rejected();

    let source = r#"
module m;
export fn pair<T, T>(a: T, b: T) -> T { return a; }
"#;
    let tokens = tokenize(source).expect("tokenize");
    let err = parse(tokens).expect_err("duplicate type params should fail");
    assert!(
        err.to_string().contains("Duplicate type parameter"),
        "got {err}"
    );
}

#[test]
fn provider_bare_ident_accepted() {
    // Description:
    //     Provider bare ident accepted.
    //
    // Inputs:
    //     None.
    //
    // Outputs:
    //     None.
    //
    // Example:

    //     let result = spanda_core::export_trait_generics::provider_bare_ident_accepted();

    let source = r#"
robot R {
  actuator wheels: DifferentialDrive;
  ai_model planner: LLM { provider: mock; model: "x"; }
  behavior run() { wheels.stop(); }
}
"#;
    check(source).expect("bare provider ident should type-check");
}

#[test]
fn serialize_bare_format_ident_accepted() {
    // Description:
    //     Serialize bare format ident accepted.
    //
    // Inputs:
    //     None.
    //
    // Outputs:
    //     None.
    //
    // Example:

    //     let result = spanda_core::export_trait_generics::serialize_bare_format_ident_accepted();

    let source = r#"
robot R {
  actuator wheels: DifferentialDrive;
  behavior run() {
    let _ = serialize(1, json);
    wheels.stop();
  }
}
"#;
    check(source).expect("bare serialize format ident should type-check");
    run(source, RunOptions::default()).expect("bare format ident should run");
}

#[test]
fn at_policy_homeostasis_parses() {
    // Description:
    //     At policy homeostasis parses.
    //
    // Inputs:
    //     None.
    //
    // Outputs:
    //     None.
    //
    // Example:

    //     let result = spanda_core::export_trait_generics::at_policy_homeostasis_parses();

    let source = r#"
@policy(kind: "homeostasis")
PatrolHomeostasis {
  metric battery_pct;
}

robot R {
  actuator wheels: DifferentialDrive;
  behavior run() { wheels.stop(); }
}
"#;
    let tokens = tokenize(source).expect("tokenize @policy");
    let program = parse(tokens).expect("parse @policy homeostasis");
    let Program::Program {
        homeostasis_policies,
        ..
    } = program;
    assert_eq!(homeostasis_policies.len(), 1);
    match &homeostasis_policies[0] {
        spanda_ast::assurance_decl::HomeostasisPolicyDecl::HomeostasisPolicyDecl {
            name,
            metrics,
            legacy_syntax,
            ..
        } => {
            assert_eq!(name, "PatrolHomeostasis");
            assert_eq!(metrics, &["battery_pct".to_string()]);
            assert!(!legacy_syntax);
        }
    }
}

#[test]
fn at_policy_attention_parses() {
    // Description:
    //     At policy attention parses.
    //
    // Inputs:
    //     None.
    //
    // Outputs:
    //     None.
    //
    // Example:

    let source = r#"
@policy(kind: "attention")
MissionFocus {
  rule suppress_low_priority;
}

robot R {
  actuator wheels: DifferentialDrive;
  behavior run() { wheels.stop(); }
}
"#;
    let tokens = tokenize(source).expect("tokenize @policy attention");
    let program = parse(tokens).expect("parse @policy attention");
    let Program::Program {
        attention_policies, ..
    } = program;
    assert_eq!(attention_policies.len(), 1);
    match &attention_policies[0] {
        spanda_ast::assurance_decl::AttentionPolicyDecl::AttentionPolicyDecl {
            name,
            rules,
            legacy_syntax,
            ..
        } => {
            assert_eq!(name, "MissionFocus");
            assert_eq!(rules, &["suppress_low_priority".to_string()]);
            assert!(!legacy_syntax);
        }
    }
}
