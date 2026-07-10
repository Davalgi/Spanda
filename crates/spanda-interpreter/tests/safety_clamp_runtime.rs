//! Runtime enforcement of safety { max_speed } on the interpreter motion path.

use spanda_interpreter::{run_program, RunOptions};
use spanda_lexer::tokenize;
use spanda_parser::parse;

fn parse_source(source: &str) -> spanda_ast::nodes::Program {
    let tokens = tokenize(source).expect("tokenize");
    parse(tokens).expect("parse")
}

#[test]
fn safety_max_speed_clamps_drive_at_runtime() {
    // Drive above safety.max_speed must be clamped, not applied verbatim.
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
    // safety_max_speed_clamps_drive_at_runtime();

    let source = r#"
robot Rover {
  actuator wheels: DifferentialDrive;
  safety { max_speed = 0.5 m/s; }

  behavior go() {
    wheels.drive(linear: 2.0 m/s, angular: 0.0 rad/s);
  }
}
"#;
    let program = parse_source(source);
    let result = run_program(
        &program,
        RunOptions {
            entry_behavior: Some("go".into()),
            max_loop_iterations: 1,
            ..Default::default()
        },
    )
    .expect("run");
    assert!(
        (result.state.velocity.linear - 0.5).abs() < 1e-9,
        "expected linear velocity clamped to 0.5 m/s, got {}",
        result.state.velocity.linear
    );
}

#[test]
fn safety_max_speed_clamps_execute_safe_action_at_runtime() {
    // execute(SafeAction) re-applies max_speed clamp as defense in depth.
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
    // safety_max_speed_clamps_execute_safe_action_at_runtime();

    let source = r#"
robot Rover {
  sensor lidar: Lidar on "/scan";
  actuator wheels: DifferentialDrive;
  ai_model planner: LLM { provider: "mock"; model: "p"; temperature: 0.0; }
  safety { max_speed = 0.4 m/s; }

  behavior go() {
    let proposal = planner.reason(prompt: "fast");
    let action = safety.validate(proposal);
    wheels.execute(action);
  }
}
"#;
    let program = parse_source(source);
    let result = run_program(
        &program,
        RunOptions {
            entry_behavior: Some("go".into()),
            max_loop_iterations: 1,
            ..Default::default()
        },
    )
    .expect("run");
    assert!(
        result.state.velocity.linear <= 0.4 + 1e-9,
        "expected execute path linear <= 0.4 m/s, got {}",
        result.state.velocity.linear
    );
}
