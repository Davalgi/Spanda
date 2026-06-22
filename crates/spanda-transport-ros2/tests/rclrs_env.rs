//! ROS2 rclrs environment flag tests (moved from spanda-core Phase 19).

use spanda_runtime::value::RuntimeValue;
use spanda_transport_ros2::rclrs::try_rclrs_publish;
use spanda_transport_ros2::{native_sdk_available, rclrs_enabled};
use std::sync::Mutex;

static ENV_LOCK: Mutex<()> = Mutex::new(());

#[test]
fn native_sdk_reports_unavailable_without_ros_by_default() {
    assert!(!native_sdk_available());
}

#[test]
fn rclrs_transport_chain_respects_env_flag() {
    let _lock = ENV_LOCK.lock().unwrap();
    std::env::remove_var("SPANDA_ROS2_RCLRS");
    assert!(!rclrs_enabled());

    std::env::set_var("SPANDA_ROS2_RCLRS", "1");
    assert!(rclrs_enabled());
    assert!(!native_sdk_available());

    let value = RuntimeValue::String {
        value: "hello".into(),
    };
    let _ = try_rclrs_publish("/spanda/test", &value);

    std::env::remove_var("SPANDA_ROS2_RCLRS");
}
