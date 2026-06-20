//! struct literals support for Spanda.
//!
use spanda_core::{check, run, RunOptions};

#[test]
fn struct_literal_constructs_pose() {
    // Struct literal constructs pose.
    //
    // Parameters:
    // None.
    //
    // Returns:
    // Nothing.
    //
    // Options:
    // None.
    //
    // Example:
    // let result = spanda_core::struct_literals::struct_literal_constructs_pose();

    let source = r#"
struct Pose {
  x: Distance;
  y: Distance;
  heading: Angle;
}

robot R {
  actuator wheels: DifferentialDrive;
  behavior run() {
    let goal = Pose { x: 1.0 m, y: 2.0 m, heading: 0.5 rad };
    let _x = goal.x;
    wheels.stop();
  }
}
"#;
    check(source).expect("struct literal should type-check");
    run(source, RunOptions::default()).expect("struct literal should run");
}

#[test]
fn struct_literal_requires_all_fields() {
    // Struct literal requires all fields.
    //
    // Parameters:
    // None.
    //
    // Returns:
    // Nothing.
    //
    // Options:
    // None.
    //
    // Example:
    // let result = spanda_core::struct_literals::struct_literal_requires_all_fields();

    let source = r#"
struct Pose {
  x: Distance;
  y: Distance;
  heading: Angle;
}
robot R { actuator wheels: DifferentialDrive; behavior run() { let p = Pose { x: 1.0 m }; } }
"#;
    let err = check(source).expect_err("missing struct fields should fail");
    assert!(
        err.diagnostics()
            .iter()
            .any(|d| d.message.contains("Missing struct field")),
        "got {:?}",
        err.diagnostics()
    );
}
