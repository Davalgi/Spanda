//! Native compile-driver smoke tests.
//!
use spanda_driver::{check, compile, run, RunOptions};

#[test]
fn driver_compiles_minimal_robot() {
    // Description:
    //     Driver compiles minimal robot.
    //
    // Inputs:
    //     None.
    //
    // Outputs:
    //     None.
    //
    // Example:

    //     let result = spanda_driver::smoke::driver_compiles_minimal_robot();

    let source = r#"
robot HelloBot {
  actuator speaker: DifferentialDrive;
  behavior hello() {
    speaker.stop();
  }
}
"#;
    let result = compile(source);
    assert!(result.is_ok(), "compile failed: {:?}", result.err());
}

#[test]
fn driver_run_executes_minimal_program() {
    // Description:
    //     Driver run executes minimal program.
    //
    // Inputs:
    //     None.
    //
    // Outputs:
    //     None.
    //
    // Example:

    //     let result = spanda_driver::smoke::driver_run_executes_minimal_program();

    let source = r#"
robot HelloBot {
  actuator speaker: DifferentialDrive;
  behavior hello() { speaker.stop(); }
}
"#;
    let result = run(source, RunOptions::default());
    assert!(result.is_ok(), "run failed: {:?}", result.err());
}

#[test]
fn driver_check_rejects_invalid_syntax() {
    // Description:
    //     Driver check rejects invalid syntax.
    //
    // Inputs:
    //     None.
    //
    // Outputs:
    //     None.
    //
    // Example:

    //     let result = spanda_driver::smoke::driver_check_rejects_invalid_syntax();

    let source = "robot {";
    let result = check(source);
    assert!(result.is_err());
}
