//! Compile-time validation for AI provider and serialize format string literals.
//!
use spanda_core::check;

#[test]
fn unknown_ai_provider_rejected() {
    // Description:
    //     Unknown ai provider rejected.
    //
    // Inputs:
    //     None.
    //
    // Outputs:
    //     None.
    //
    // Example:

    //     let result = spanda_core::stringly_seams::unknown_ai_provider_rejected();

    let source = r#"
robot R {
  actuator wheels: DifferentialDrive;
  ai_model planner: LLM { provider: "not-a-real-provider"; model: "x"; }
  behavior run() { wheels.stop(); }
}
"#;
    let err = check(source).expect_err("unknown AI provider should fail");
    assert!(
        err.diagnostics()
            .iter()
            .any(|d| d.message.contains("Unknown AI provider")),
        "got {:?}",
        err.diagnostics()
    );
}

#[test]
fn known_ai_provider_accepted() {
    // Description:
    //     Known ai provider accepted.
    //
    // Inputs:
    //     None.
    //
    // Outputs:
    //     None.
    //
    // Example:

    //     let result = spanda_core::stringly_seams::known_ai_provider_accepted();

    let source = r#"
robot R {
  actuator wheels: DifferentialDrive;
  ai_model planner: LLM { provider: "mock"; model: "x"; }
  behavior run() { wheels.stop(); }
}
"#;
    check(source).expect("known AI provider should type-check");
}

#[test]
fn unknown_serialize_format_rejected() {
    // Description:
    //     Unknown serialize format rejected.
    //
    // Inputs:
    //     None.
    //
    // Outputs:
    //     None.
    //
    // Example:

    //     let result = spanda_core::stringly_seams::unknown_serialize_format_rejected();

    let source = r#"
robot R {
  actuator wheels: DifferentialDrive;
  behavior run() {
    let _ = serialize(1, "xml");
    wheels.stop();
  }
}
"#;
    let err = check(source).expect_err("unknown serialize format should fail");
    assert!(
        err.diagnostics()
            .iter()
            .any(|d| d.message.contains("Unknown serialize format")),
        "got {:?}",
        err.diagnostics()
    );
}

#[test]
fn known_serialize_format_accepted() {
    // Description:
    //     Known serialize format accepted.
    //
    // Inputs:
    //     None.
    //
    // Outputs:
    //     None.
    //
    // Example:

    //     let result = spanda_core::stringly_seams::known_serialize_format_accepted();

    let source = r#"
robot R {
  actuator wheels: DifferentialDrive;
  behavior run() {
    let _ = serialize(1, "json");
    wheels.stop();
  }
}
"#;
    check(source).expect("known serialize format should type-check");
}
