//! Logical-to-physical device mapping derived from configuration and programs.
//!
use crate::device_tree::{DeviceNode, RobotNode};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Mapping between logical program entities and physical configuration devices.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct LogicalPhysicalMap {
    pub robots: HashMap<String, RobotMapping>,
    pub sensors: HashMap<String, SensorMapping>,
    pub actuators: HashMap<String, ActuatorMapping>,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RobotMapping {
    pub logical_id: String,
    pub physical_robot_id: String,
    pub hardware_profile: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SensorMapping {
    pub logical_name: String,
    pub physical_device_id: String,
    pub robot_id: String,
    pub device_type: String,
    pub capabilities: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ActuatorMapping {
    pub logical_name: String,
    pub physical_device_id: String,
    pub robot_id: String,
    pub device_type: String,
    pub capabilities: Vec<String>,
    pub has_emergency_stop: bool,
}

impl LogicalPhysicalMap {
    pub fn from_device_tree(tree: &crate::device_tree::DeviceTree) -> Self {
        // Build logical-to-physical mappings from the resolved device tree.
        //
        // Parameters:
        // - `tree` — parsed fleet/device hierarchy
        //
        // Returns:
        // Mapping tables keyed by logical and physical identifiers.
        //
        // Options:
        // None.
        //
        // Example:
        // let map = LogicalPhysicalMap::from_device_tree(&resolved.device_tree);

        let mut map = Self::default();
        let Some(ref fleet) = tree.fleet else {
            return map;
        };
        for robot in &fleet.robots {
            map.robots.insert(
                robot.id.clone(),
                RobotMapping {
                    logical_id: robot.id.clone(),
                    physical_robot_id: robot.id.clone(),
                    hardware_profile: robot.hardware_profile.clone(),
                },
            );
            if let Some(ref compute) = robot.compute {
                for device in &compute.devices {
                    classify_device(robot, device, &mut map);
                }
            }
        }
        let _ = fleet;
        map
    }

    pub fn verify(&self) -> Vec<String> {
        // Check that every actuator mapping includes emergency stop when required.
        //
        // Parameters:
        // None.
        //
        // Returns:
        // Diagnostic messages for mapping gaps.
        //
        // Options:
        // None.
        //
        // Example:
        // let issues = map.verify();

        let mut issues = Vec::new();
        issues.extend(self.warnings.clone());
        for (name, actuator) in &self.actuators {
            let is_drive =
                actuator.device_type.contains("Drive") || actuator.device_type.contains("Actuator");
            if is_drive && !actuator.has_emergency_stop {
                issues.push(format!(
                    "actuator '{name}' ({}) missing emergency_stop capability",
                    actuator.physical_device_id
                ));
            }
        }
        issues
    }
}

fn classify_device(robot: &RobotNode, device: &DeviceNode, map: &mut LogicalPhysicalMap) {
    let dtype = device.device_type.to_ascii_lowercase();
    if dtype.contains("gps")
        || dtype.contains("lidar")
        || dtype.contains("camera")
        || dtype.contains("imu")
        || dtype.contains("sensor")
    {
        map.sensors.insert(
            device.id.clone(),
            SensorMapping {
                logical_name: device.id.clone(),
                physical_device_id: device.id.clone(),
                robot_id: robot.id.clone(),
                device_type: device.device_type.clone(),
                capabilities: device.capabilities.clone(),
            },
        );
    } else if dtype.contains("drive")
        || dtype.contains("actuator")
        || dtype.contains("arm")
        || dtype.contains("motor")
    {
        let has_estop = device.capabilities.iter().any(|c| c == "emergency_stop");
        map.actuators.insert(
            device.id.clone(),
            ActuatorMapping {
                logical_name: device.id.clone(),
                physical_device_id: device.id.clone(),
                robot_id: robot.id.clone(),
                device_type: device.device_type.clone(),
                capabilities: device.capabilities.clone(),
                has_emergency_stop: has_estop,
            },
        );
    }
}
