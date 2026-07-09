//! Automotive sensor hub and live-read integration tests.
//!
use spanda_providers::radar_env_lock::RadarEnvLock;
use spanda_providers::{
    bootstrap_providers_for_packages, dispatch_official_package_call, read_radar_distance,
    seed_automotive_demos,
};
use spanda_runtime::value::RuntimeValue;

fn clear_live_radar_env() {
    std::env::remove_var("SPANDA_LIVE_RADAR");
    std::env::remove_var("SPANDA_RADAR_CMD");
}

#[test]
fn automotive_hub_seeds_radar_distance() {
    let _lock = RadarEnvLock::acquire().expect("radar env lock");
    clear_live_radar_env();
    seed_automotive_demos();
    let value = read_radar_distance("front-radar");
    assert!((value - 25.0).abs() < f64::EPSILON);
}

#[test]
fn package_dispatch_reads_radar_when_capability_granted() {
    let _lock = RadarEnvLock::acquire().expect("radar env lock");
    clear_live_radar_env();
    let mut registry = bootstrap_providers_for_packages(&["spanda-radar"]);
    let value = dispatch_official_package_call(
        &mut registry,
        "sensors.radar",
        "read",
        &[],
        None,
        None,
        0.0,
    )
    .expect("radar read dispatch");
    match value {
        RuntimeValue::Number { value, .. } => assert!(value > 0.0),
        other => panic!("expected number, got {other:?}"),
    }
}

#[test]
fn live_radar_cmd_overrides_hub_stub() {
    let _lock = RadarEnvLock::acquire().expect("radar env lock");
    clear_live_radar_env();
    std::env::set_var("SPANDA_LIVE_RADAR", "1");
    std::env::set_var("SPANDA_RADAR_CMD", "echo 99.0");
    seed_automotive_demos();
    let value = read_radar_distance("front-radar");
    assert!((value - 99.0).abs() < f64::EPSILON);
    clear_live_radar_env();
}

#[test]
fn live_lin_cmd_overrides_hub_stub() {
    let _lock = RadarEnvLock::acquire().expect("radar env lock");
    std::env::remove_var("SPANDA_LIVE_LIN");
    std::env::remove_var("SPANDA_LIN_CMD");
    seed_automotive_demos();
    std::env::set_var("SPANDA_LIVE_LIN", "1");
    std::env::set_var("SPANDA_LIN_CMD", "echo 17.5");
    let value = spanda_providers::read_lin_signal("steering-angle");
    assert!((value - 17.5).abs() < f64::EPSILON);
    std::env::remove_var("SPANDA_LIVE_LIN");
    std::env::remove_var("SPANDA_LIN_CMD");
}

#[test]
fn live_uds_cmd_overrides_hub_stub() {
    let _lock = RadarEnvLock::acquire().expect("radar env lock");
    std::env::remove_var("SPANDA_LIVE_UDS");
    std::env::remove_var("SPANDA_UDS_CMD");
    seed_automotive_demos();
    std::env::set_var("SPANDA_LIVE_UDS", "1");
    std::env::set_var("SPANDA_UDS_CMD", "echo C1234");
    let value = spanda_providers::read_uds_dtc("powertrain-ecu");
    assert_eq!(value, "C1234");
    std::env::remove_var("SPANDA_LIVE_UDS");
    std::env::remove_var("SPANDA_UDS_CMD");
}

#[test]
fn live_v2x_cmd_overrides_hub_stub() {
    let _lock = RadarEnvLock::acquire().expect("radar env lock");
    std::env::remove_var("SPANDA_LIVE_V2X");
    std::env::remove_var("SPANDA_V2X_CMD");
    seed_automotive_demos();
    std::env::set_var("SPANDA_LIVE_V2X", "1");
    std::env::set_var("SPANDA_V2X_CMD", "echo hazard_ahead");
    let value = spanda_providers::read_v2x_message("bsm");
    assert_eq!(value, "hazard_ahead");
    std::env::remove_var("SPANDA_LIVE_V2X");
    std::env::remove_var("SPANDA_V2X_CMD");
}
