//! hardware compat support for Spanda.
//!
use spanda_core::{
    check, verify_compatibility, verify_compatibility_target, CompatSeverity, VerifyOptions,
};

#[test]
fn custom_hardware_profile_parsed_and_verified() {
    // Description:
    //     Custom hardware profile parsed and verified.
    //
    // Inputs:
    //     None.
    //
    // Outputs:
    //     None.
    //
    // Example:

    //     let result = spanda_core::hardware_compat::custom_hardware_profile_parsed_and_verified();

    let source = r#"
hardware Tiny {
  sensors [ IMU ];
  actuators [ DifferentialDrive ];
}

robot Rover {
  sensor imu: IMU;
  actuator wheels: DifferentialDrive;
  behavior run() { wheels.stop(); }
}

deploy Rover to Tiny;
"#;
    check(source).expect("should type-check");
    let report = verify_compatibility_target(source, None).expect("verify should run");
    assert!(report.compatible);
}

#[test]
fn missing_sensor_on_target_fails_verify() {
    // Description:
    //     Missing sensor on target fails verify.
    //
    // Inputs:
    //     None.
    //
    // Outputs:
    //     None.
    //
    // Example:

    //     let result = spanda_core::hardware_compat::missing_sensor_on_target_fails_verify();

    let source = r#"
robot Rover {
  sensor camera: Camera on "/camera";
  sensor lidar: Lidar on "/scan";
  actuator wheels: DifferentialDrive;
  behavior run() { wheels.stop(); }
}

deploy Rover to ESP32;
"#;
    check(source).expect("should type-check");
    let report = verify_compatibility_target(source, None).expect("verify should run");
    assert!(!report.compatible);
    assert!(report
        .items
        .iter()
        .any(|i| { i.severity == CompatSeverity::Error && i.message.contains("Camera") }));
}

#[test]
fn missing_actuator_on_target_fails_verify() {
    // Description:
    //     Missing actuator on target fails verify.
    //
    // Inputs:
    //     None.
    //
    // Outputs:
    //     None.
    //
    // Example:

    //     let result = spanda_core::hardware_compat::missing_actuator_on_target_fails_verify();

    let source = r#"
robot Rover {
  sensor imu: IMU;
  actuator arm: RoboticArm;
  behavior run() { arm.grip(); }
}

deploy Rover to RoverV1;
"#;
    check(source).expect("should type-check");
    let report = verify_compatibility_target(source, None).expect("verify should run");
    assert!(!report.compatible);
    assert!(report
        .items
        .iter()
        .any(|i| i.message.contains("RoboticArm")));
}

#[test]
fn cli_target_overrides_deploy() {
    // Description:
    //     Cli target overrides deploy.
    //
    // Inputs:
    //     None.
    //
    // Outputs:
    //     None.
    //
    // Example:

    //     let result = spanda_core::hardware_compat::cli_target_overrides_deploy();

    let source = r#"
robot Rover {
  sensor camera: Camera on "/camera";
  actuator wheels: DifferentialDrive;
  behavior run() { wheels.stop(); }
}
"#;
    let ok = verify_compatibility_target(source, Some("RoverV1")).expect("verify");
    assert!(ok.compatible);
    let bad = verify_compatibility_target(source, Some("ESP32")).expect("verify");
    assert!(!bad.compatible);
}

#[test]
fn rover_deploy_example_compatible() {
    // Description:
    //     Rover deploy example compatible.
    //
    // Inputs:
    //     None.
    //
    // Outputs:
    //     None.
    //
    // Example:

    //     let result = spanda_core::hardware_compat::rover_deploy_example_compatible();

    let source = std::fs::read_to_string(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../examples/hardware/rover_deploy.sd"
    ))
    .expect("read example");
    check(&source).expect("example should type-check");
    let report = verify_compatibility_target(&source, None).expect("verify");
    assert!(report.compatible, "{:?}", report.items);
}

#[test]
fn requires_hardware_memory_check() {
    // Description:
    //     Requires hardware memory check.
    //
    // Inputs:
    //     None.
    //
    // Outputs:
    //     None.
    //
    // Example:

    //     let result = spanda_core::hardware_compat::requires_hardware_memory_check();

    let source = r#"
requires_hardware {
  memory >= 8 GB;
}

robot Rover {
  sensor imu: IMU;
  actuator wheels: DifferentialDrive;
  behavior run() { wheels.stop(); }
}

deploy Rover to ESP32;
"#;
    let report = verify_compatibility_target(source, None).expect("verify");
    assert!(!report.compatible);
    assert!(report.items.iter().any(|i| i.category == "memory"));
}

#[test]
fn ai_model_gpu_requirement_fails_on_esp32() {
    // Description:
    //     Ai model gpu requirement fails on esp32.
    //
    // Inputs:
    //     None.
    //
    // Outputs:
    //     None.
    //
    // Example:

    //     let result = spanda_core::hardware_compat::ai_model_gpu_requirement_fails_on_esp32();

    let source = r#"
robot Rover {
  sensor camera: Camera on "/camera";
  actuator wheels: DifferentialDrive;

  ai_model Vision: VisionModel {
    memory_required: 2048;
    gpu_required: true;
  }

  behavior run() { wheels.stop(); }
}

deploy Rover to ESP32;
"#;
    let report = verify_compatibility_target(source, None).expect("verify");
    assert!(!report.compatible);
    assert!(report.items.iter().any(|i| i.category == "ai"));
}

#[test]
fn compatibility_matrix_all_targets() {
    // Description:
    //     Compatibility matrix all targets.
    //
    // Inputs:
    //     None.
    //
    // Outputs:
    //     None.
    //
    // Example:

    //     let result = spanda_core::hardware_compat::compatibility_matrix_all_targets();

    let source = r#"
robot Rover {
  sensor imu: IMU;
  actuator wheels: DifferentialDrive;
  behavior run() { wheels.stop(); }
}
"#;
    let report = verify_compatibility(
        source,
        &VerifyOptions {
            all_targets: true,
            ..Default::default()
        },
    )
    .expect("verify");
    assert!(report.matrix.is_some());
    let matrix = report.matrix.unwrap();
    assert!(matrix.cells.len() >= 5);
}

#[test]
fn task_budget_memory_exceeds_esp32() {
    // Description:
    //     Task budget memory exceeds esp32.
    //
    // Inputs:
    //     None.
    //
    // Outputs:
    //     None.
    //
    // Example:

    //     let result = spanda_core::hardware_compat::task_budget_memory_exceeds_esp32();

    let source = r#"
robot Rover {
  sensor imu: IMU;
  actuator wheels: DifferentialDrive;

  task control every 100ms {
    budget {
      memory <= 512 MB;
    }
    wheels.stop();
  }
}
"#;
    let report = verify_compatibility_target(source, Some("ESP32")).expect("verify");
    assert!(!report.compatible);
}

#[test]
fn requires_network_bandwidth_check() {
    // Description:
    //     Requires network bandwidth check.
    //
    // Inputs:
    //     None.
    //
    // Outputs:
    //     None.
    //
    // Example:

    //     let result = spanda_core::hardware_compat::requires_network_bandwidth_check();

    let source = r#"
requires_network {
  bandwidth >= 50 Mbps;
}

robot Rover {
  sensor imu: IMU;
  actuator wheels: DifferentialDrive;
  behavior run() { wheels.stop(); }
}

deploy Rover to ESP32;
"#;
    let report = verify_compatibility_target(source, None).expect("verify");
    assert!(!report.compatible);
    assert!(report.items.iter().any(|i| i.category == "network"));
}
