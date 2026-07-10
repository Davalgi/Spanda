//! GPS, IMU, and camera sensor hub integration tests.
//!
use spanda_providers::{
    bootstrap_providers_for_packages, dispatch_official_package_call, read_camera_sample,
    read_gps_fix, read_imu_sample, seed_sensor_demos,
};
use spanda_runtime::value::RuntimeValue;
use std::sync::{Mutex, OnceLock};

static ENV_LOCK: OnceLock<Mutex<()>> = OnceLock::new();

fn env_lock() -> &'static Mutex<()> {
    // Serialize tests that mutate process-wide live-sensor env vars.
    //
    // Parameters:
    // None.
    //
    // Returns:
    // Shared mutex guarding env mutation across this test binary.
    //
    // Options:
    // None.
    //
    // Example:
    // let _guard = env_lock().lock().unwrap();

    ENV_LOCK.get_or_init(|| Mutex::new(()))
}

fn clear_live_sensor_env() {
    // Clear live GPS/IMU/camera env so hub stubs are deterministic.
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
    // clear_live_sensor_env();

    for key in [
        "SPANDA_LIVE_GPS",
        "SPANDA_GPS_CMD",
        "SPANDA_LIVE_IMU",
        "SPANDA_IMU_CMD",
        "SPANDA_LIVE_CAMERA",
        "SPANDA_CAMERA_CMD",
        "SPANDA_LIVE_SENSOR_PIPELINE",
        "SPANDA_LIVE_FUSION_SENSORS",
    ] {
        std::env::remove_var(key);
    }
}

#[test]
fn sensor_hub_seeds_gps_imu_camera() {
    // Description:
    //     Sensor hub seeds gps imu camera.
    //
    // Inputs:
    //     None.
    //
    // Outputs:
    //     None.
    //
    // Example:

    let _guard = env_lock().lock().unwrap();
    clear_live_sensor_env();
    seed_sensor_demos();
    let gps = read_gps_fix("gps");
    assert!((gps.lat - 37.7749).abs() < f64::EPSILON);
    let imu = read_imu_sample("imu");
    assert!((imu.yaw - 1.57).abs() < 0.01);
    let camera = read_camera_sample("camera");
    assert_eq!(camera.width, 1280);
}

#[test]
fn package_dispatch_reads_imu_when_capability_granted() {
    // Description:
    //     Package dispatch reads imu when capability granted.
    //
    // Inputs:
    //     None.
    //
    // Outputs:
    //     None.
    //
    // Example:

    let _guard = env_lock().lock().unwrap();
    clear_live_sensor_env();
    seed_sensor_demos();
    let mut registry = bootstrap_providers_for_packages(&["spanda-imu"]);
    let value =
        dispatch_official_package_call(&mut registry, "sensors.imu", "read", &[], None, None, 0.0)
            .expect("imu read dispatch");
    match value {
        RuntimeValue::Object { type_name, .. } => assert_eq!(type_name, "IMUReading"),
        other => panic!("expected IMU object, got {other:?}"),
    }
}

#[test]
fn package_dispatch_reads_camera_when_capability_granted() {
    // Description:
    //     Package dispatch reads camera when capability granted.
    //
    // Inputs:
    //     None.
    //
    // Outputs:
    //     None.
    //
    // Example:

    let _guard = env_lock().lock().unwrap();
    clear_live_sensor_env();
    seed_sensor_demos();
    let mut registry = bootstrap_providers_for_packages(&["spanda-camera"]);
    let value = dispatch_official_package_call(
        &mut registry,
        "sensors.camera",
        "read",
        &[],
        None,
        None,
        0.0,
    )
    .expect("camera read dispatch");
    match value {
        RuntimeValue::Object { type_name, .. } => assert_eq!(type_name, "CameraFrame"),
        other => panic!("expected camera object, got {other:?}"),
    }
}

#[test]
fn live_gps_cmd_overrides_hub_stub() {
    // Description:
    //     Live gps cmd overrides hub stub.
    //
    // Inputs:
    //     None.
    //
    // Outputs:
    //     None.
    //
    // Example:

    let _guard = env_lock().lock().unwrap();
    clear_live_sensor_env();
    seed_sensor_demos();
    std::env::set_var("SPANDA_LIVE_GPS", "1");
    std::env::set_var("SPANDA_GPS_CMD", "echo 12.34,56.78,100.0,45.0");
    let gps = read_gps_fix("gps");
    assert!((gps.lat - 12.34).abs() < f64::EPSILON);
    assert!((gps.lon - 56.78).abs() < f64::EPSILON);
    clear_live_sensor_env();
}

#[test]
fn live_imu_cmd_overrides_hub_stub() {
    // Description:
    //     Live imu cmd overrides hub stub.
    //
    // Inputs:
    //     None.
    //
    // Outputs:
    //     None.
    //
    // Example:

    let _guard = env_lock().lock().unwrap();
    clear_live_sensor_env();
    seed_sensor_demos();
    std::env::set_var("SPANDA_LIVE_IMU", "1");
    std::env::set_var("SPANDA_IMU_CMD", "echo 0.1,0.2,3.14,0,0,9.8");
    let imu = read_imu_sample("imu");
    assert!((imu.yaw - 3.14).abs() < 0.01);
    clear_live_sensor_env();
}

#[test]
fn live_camera_cmd_overrides_hub_stub() {
    // Description:
    //     Live camera cmd overrides hub stub.
    //
    // Inputs:
    //     None.
    //
    // Outputs:
    //     None.
    //
    // Example:

    let _guard = env_lock().lock().unwrap();
    clear_live_sensor_env();
    seed_sensor_demos();
    std::env::set_var("SPANDA_LIVE_CAMERA", "1");
    std::env::set_var("SPANDA_CAMERA_CMD", "echo 1920,1080,0.42");
    let camera = read_camera_sample("camera");
    assert_eq!(camera.width, 1920);
    assert_eq!(camera.height, 1080);
    assert!((camera.motion_score - 0.42).abs() < f64::EPSILON);
    clear_live_sensor_env();
}

#[test]
fn live_fusion_sensor_readings_use_gps_imu_camera() {
    // Description:
    //     Live fusion sensor readings use gps imu camera.
    //
    // Inputs:
    //     None.
    //
    // Outputs:
    //     None.
    //
    // Example:

    let _guard = env_lock().lock().unwrap();
    clear_live_sensor_env();
    seed_sensor_demos();
    std::env::set_var("SPANDA_LIVE_FUSION_SENSORS", "1");
    let readings = spanda_providers::live_fusion_sensor_readings("rover");
    assert_eq!(readings.len(), 4);
    assert!(readings.iter().any(|(name, _, _)| name == "gps_lat"));
    assert!(readings.iter().any(|(name, _, _)| name == "camera_motion"));
    clear_live_sensor_env();
}
