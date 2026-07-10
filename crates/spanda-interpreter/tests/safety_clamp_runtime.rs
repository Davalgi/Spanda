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

#[test]
fn safety_max_angular_clamps_drive_at_runtime() {
    // Drive above safety.max_angular must be clamped at runtime.
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
    // safety_max_angular_clamps_drive_at_runtime();

    let source = r#"
robot Rover {
  actuator wheels: DifferentialDrive;
  safety {
    max_speed = 1.0 m/s;
    max_angular = 0.3 rad/s;
  }

  behavior turn() {
    wheels.drive(linear: 0.1 m/s, angular: 2.0 rad/s);
  }
}
"#;
    let program = parse_source(source);
    let result = run_program(
        &program,
        RunOptions {
            entry_behavior: Some("turn".into()),
            max_loop_iterations: 1,
            ..Default::default()
        },
    )
    .expect("run");
    assert!(
        (result.state.velocity.angular - 0.3).abs() < 1e-9,
        "expected angular velocity clamped to 0.3 rad/s, got {}",
        result.state.velocity.angular
    );
}

#[test]
fn safety_max_speed_clamps_follow_cruise_at_runtime() {
    // follow(path:) cruise speed must respect safety.max_speed (default cruise is 0.5 m/s).
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
    // safety_max_speed_clamps_follow_cruise_at_runtime();

    let source = r#"
robot Rover {
  actuator wheels: DifferentialDrive;
  safety { max_speed = 0.2 m/s; }

  behavior go() {
    let start = pose(x: 0.0 m, y: 0.0 m, theta: 0.0 rad);
    let goal = pose(x: 5.0 m, y: 0.0 m, theta: 0.0 rad);
    let path = trajectory(from: start, to: goal, steps: 4);
    wheels.follow(path: path);
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
        (result.state.velocity.linear - 0.2).abs() < 1e-9,
        "expected follow cruise clamped to 0.2 m/s, got {}",
        result.state.velocity.linear
    );
}

#[test]
fn safety_zone_reclamps_follow_cruise_per_tick() {
    // follow() must re-clamp cruise when the robot enters a slower safety zone mid-path.
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
    // safety_zone_reclamps_follow_cruise_per_tick();

    let source = r#"
safety_zone SlowLane {
  max_speed 0.1 m/s;
}

robot Rover {
  actuator wheels: DifferentialDrive;
  safety {
    max_speed = 1.0 m/s;
    zone SlowLane circle at (3.0 m, 0.0 m) radius 1.5 m;
  }

  behavior go() {
    let start = pose(x: 0.0 m, y: 0.0 m, theta: 0.0 rad);
    let goal = pose(x: 5.0 m, y: 0.0 m, theta: 0.0 rad);
    let path = trajectory(from: start, to: goal, steps: 10);
    wheels.follow(path: path);
    loop every 100ms { }
  }
}
"#;
    let program = parse_source(source);
    let result = run_program(
        &program,
        RunOptions {
            entry_behavior: Some("go".into()),
            max_loop_iterations: 80,
            ..Default::default()
        },
    )
    .expect("run");
    assert!(
        result.state.pose.x > 1.5,
        "expected robot to advance into SlowLane, pose.x={}",
        result.state.pose.x
    );
    assert!(
        (result.state.velocity.linear - 0.1).abs() < 1e-9,
        "expected follow cruise re-clamped to zone 0.1 m/s, got {}",
        result.state.velocity.linear
    );
}

