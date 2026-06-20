//! In-process ROS2 via rclpy (Python bridge) when `SPANDA_ROS2_RCLRS=1`.
//!
//! Uses the same rclpy node as `SPANDA_ROS2_LIVE` but is tried before the
//! `ros2` CLI path. Full native `rclrs` linking remains behind `ros2-rclrs`.

use crate::runtime::RuntimeValue;
use crate::transport_live::{
    try_ros2_bridge_publish, try_ros2_bridge_service_call, try_ros2_bridge_subscribe,
};

pub fn rclrs_enabled() -> bool {
    std::env::var("SPANDA_ROS2_RCLRS").is_ok()
}

pub fn rclrs_available() -> bool {
    rclrs_enabled()
}

pub fn try_rclrs_publish(topic: &str, value: &RuntimeValue) -> bool {
    if !rclrs_enabled() {
        return false;
    }
    try_ros2_bridge_publish(topic, value)
}

pub fn try_rclrs_subscribe(topic: &str) -> bool {
    if !rclrs_enabled() {
        return false;
    }
    try_ros2_bridge_subscribe(topic)
}

pub fn try_rclrs_service_call(service: &str, service_type: &str, request: &str) -> bool {
    if !rclrs_enabled() {
        return false;
    }
    try_ros2_bridge_service_call(service, service_type, request)
}

#[cfg(feature = "ros2-rclrs")]
pub fn init_node(_name: &str) -> Result<(), String> {
    Err("native rclrs not linked — using rclpy bridge when SPANDA_ROS2_RCLRS=1".into())
}

#[cfg(not(feature = "ros2-rclrs"))]
pub fn init_node(_name: &str) -> Result<(), String> {
    Err("enable ros2-rclrs feature for native node init".into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rclrs_off_by_default() {
        std::env::remove_var("SPANDA_ROS2_RCLRS");
        assert!(!rclrs_enabled());
    }
}
