//! Native compile-driver smoke tests.
//!
use spanda_ast::nodes::RobotDecl;
use spanda_driver::{check, compile, run, RunOptions};

#[test]
fn killer_demo_program_has_ai_for_backend_notices() {
    let source =
        std::fs::read_to_string("../../examples/showcase/killer_demo.sd").expect("killer_demo");
    let program = compile(&source).expect("compile killer_demo").program;
    let has_ai = program.robots().iter().any(|robot| {
        let RobotDecl::RobotDecl {
            ai_models, agents, ..
        } = robot;
        !ai_models.is_empty() || !agents.is_empty()
    });
    assert!(has_ai, "expected killer_demo robots to declare AI");
    assert!(!program.robots().is_empty());
}

#[test]
fn emit_backend_notices_for_killer_demo() {
    let source =
        std::fs::read_to_string("../../examples/showcase/killer_demo.sd").expect("killer_demo");
    let program = compile(&source).expect("compile killer_demo").program;
    std::env::remove_var("SPANDA_QUIET");
    spanda_runtime::backend_notice::emit_program_backend_notices(&program);
}

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
