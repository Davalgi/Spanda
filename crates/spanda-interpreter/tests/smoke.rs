//! Smoke tests for `spanda-interpreter` re-exports.
//!

use spanda_interpreter::{run, RunOptions, SimRobotBackend};

#[test]
fn reexported_run_executes_minimal_program() {
    // Re-exported run executes minimal program.
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
    // let result = reexported_run_executes_minimal_program();

    let source = r#"
module hello;

robot HelloBot {
  actuator speaker: DifferentialDrive;

  behavior hello() {
    speaker.stop();
  }
}
"#;

    // Run the hello-world fixture through the staging crate API.
    let result = run(source, RunOptions::default());
    assert!(result.is_ok(), "run failed: {:?}", result.err());
}

#[test]
fn sim_robot_backend_type_alias_matches_simulator() {
    // Sim robot backend type alias matches simulator.
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
    // let result = sim_robot_backend_type_alias_matches_simulator();

    // Confirm the staging alias names the same simulator type as core.
    fn assert_sim_type<T: spanda_interpreter::RobotBackend>() {}
    assert_sim_type::<SimRobotBackend>();
}
