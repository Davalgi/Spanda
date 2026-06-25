//! Configuration validation rules and diagnostic reports.
//!
use crate::device_tree::DeviceTree;
use crate::mapping::LogicalPhysicalMap;
use serde::{Deserialize, Serialize};
use spanda_hardware::list_hardware_profiles;
use spanda_package::adapter::framework_packages;
use std::collections::{HashMap, HashSet};

/// Severity of a configuration validation finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ValidationSeverity {
    Error,
    Warning,
    Info,
}

/// Single validation finding.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ValidationFinding {
    pub severity: ValidationSeverity,
    pub code: String,
    pub message: String,
    pub path: Option<String>,
}

/// Aggregated configuration validation report.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct ConfigValidationReport {
    pub findings: Vec<ValidationFinding>,
    pub passed: bool,
}

impl ConfigValidationReport {
    pub fn push(
        &mut self,
        severity: ValidationSeverity,
        code: &str,
        message: String,
        path: Option<String>,
    ) {
        self.findings.push(ValidationFinding {
            severity,
            code: code.into(),
            message,
            path,
        });
        if severity == ValidationSeverity::Error {
            self.passed = false;
        }
    }

    pub fn error_count(&self) -> usize {
        self.findings
            .iter()
            .filter(|f| f.severity == ValidationSeverity::Error)
            .count()
    }

    pub fn warning_count(&self) -> usize {
        self.findings
            .iter()
            .filter(|f| f.severity == ValidationSeverity::Warning)
            .count()
    }
}

pub fn validate_device_tree(tree: &DeviceTree, providers: &[String]) -> ConfigValidationReport {
    // Run device-tree validation rules against a resolved tree.
    //
    // Parameters:
    // - `tree` — fleet/device hierarchy
    // - `providers` — declared provider package names from config
    //
    // Returns:
    // Validation report with errors and warnings.
    //
    // Options:
    // None.
    //
    // Example:
    // let report = validate_device_tree(&resolved.device_tree, &resolved.providers);

    let mut report = ConfigValidationReport {
        passed: true,
        findings: Vec::new(),
    };
    let known_providers: HashSet<&str> = framework_packages()
        .iter()
        .map(|p| p.name)
        .chain(providers.iter().map(String::as_str))
        .collect();
    let profiles: HashSet<String> = list_hardware_profiles().into_iter().collect();

    let Some(ref fleet) = tree.fleet else {
        report.push(
            ValidationSeverity::Warning,
            "fleet.missing",
            "no [fleet] section in device configuration".into(),
            None,
        );
        return report;
    };

    if fleet.robots.is_empty() {
        report.push(
            ValidationSeverity::Warning,
            "fleet.empty",
            format!("fleet '{}' has no robots", fleet.id),
            Some("fleet.robots".into()),
        );
    }

    let mut ports: HashMap<String, String> = HashMap::new();
    let mut buses: HashMap<String, String> = HashMap::new();
    let mut serials: HashMap<String, String> = HashMap::new();

    for robot in &fleet.robots {
        let path = format!("fleet.robots.{}", robot.id);
        if robot.compute.is_none() {
            report.push(
                ValidationSeverity::Error,
                "robot.no_compute",
                format!("robot '{}' has no compute node", robot.id),
                Some(path.clone()),
            );
        }
        if let Some(ref profile) = robot.hardware_profile {
            if !profiles.contains(profile) {
                report.push(
                    ValidationSeverity::Warning,
                    "hardware.profile_unknown",
                    format!(
                        "robot '{}' hardware_profile '{profile}' not in built-in catalog",
                        robot.id
                    ),
                    Some(format!("{path}.hardware_profile")),
                );
            }
        }
        if let Some(ref compute) = robot.compute {
            if let Some(ref serial) = compute.serial {
                if let Some(other) = serials.insert(serial.clone(), compute.id.clone()) {
                    report.push(
                        ValidationSeverity::Error,
                        "compute.duplicate_serial",
                        format!("duplicate serial '{serial}' on {other} and {}", compute.id),
                        Some(format!("{path}.compute.serial")),
                    );
                }
            }
            for device in &compute.devices {
                let dpath = format!("{path}.compute.devices.{}", device.id);
                if let Some(ref provider) = device.provider {
                    if !known_providers.contains(provider.as_str()) {
                        report.push(
                            ValidationSeverity::Error,
                            "provider.unknown",
                            format!(
                                "device '{}' references unknown provider '{provider}'",
                                device.id
                            ),
                            Some(dpath.clone()),
                        );
                    }
                } else {
                    report.push(
                        ValidationSeverity::Warning,
                        "provider.missing",
                        format!("device '{}' has no provider", device.id),
                        Some(dpath.clone()),
                    );
                }
                if let Some(ref port) = device.port {
                    if let Some(other) = ports.insert(port.clone(), device.id.clone()) {
                        report.push(
                            ValidationSeverity::Error,
                            "device.port_conflict",
                            format!("port '{port}' used by both '{other}' and '{}'", device.id),
                            Some(format!("{dpath}.port")),
                        );
                    }
                }
                if let Some(ref bus) = device.bus {
                    if let Some(other) = buses.insert(bus.clone(), device.id.clone()) {
                        report.push(
                            ValidationSeverity::Error,
                            "device.bus_conflict",
                            format!("bus '{bus}' used by both '{other}' and '{}'", device.id),
                            Some(format!("{dpath}.bus")),
                        );
                    }
                }
                if device.firmware.is_none() && device.version.is_none() {
                    report.push(
                        ValidationSeverity::Warning,
                        "device.firmware_missing",
                        format!("device '{}' has no firmware or version metadata", device.id),
                        Some(dpath.clone()),
                    );
                }
                let is_actuator = device.device_type.to_ascii_lowercase().contains("drive")
                    || device.device_type.to_ascii_lowercase().contains("actuator");
                if is_actuator {
                    let has_estop = device.capabilities.iter().any(|c| c == "emergency_stop");
                    if !has_estop {
                        report.push(
                            ValidationSeverity::Error,
                            "safety.no_emergency_stop",
                            format!(
                                "safety-critical actuator '{}' missing emergency_stop capability",
                                device.id
                            ),
                            Some(format!("{dpath}.capabilities")),
                        );
                    }
                    if device.trusted == Some(false) {
                        report.push(
                            ValidationSeverity::Error,
                            "security.untrusted_actuator",
                            format!("untrusted device '{}' cannot control actuator", device.id),
                            Some(dpath.clone()),
                        );
                    }
                }
                let needs_identity = device.port.is_some()
                    || device.bus.is_some()
                    || device
                        .capabilities
                        .iter()
                        .any(|c| c.contains("network") || c.contains("remote"));
                if needs_identity && device.identity.is_none() {
                    report.push(
                        ValidationSeverity::Warning,
                        "security.identity_missing",
                        format!("networked device '{}' has no security identity", device.id),
                        Some(dpath),
                    );
                }
            }
            validate_hardware_profile_match(robot, compute, &mut report);
        }
    }
    report
}

fn validate_hardware_profile_match(
    robot: &crate::device_tree::RobotNode,
    compute: &crate::device_tree::ComputeNode,
    report: &mut ConfigValidationReport,
) {
    let Some(ref profile_name) = robot.hardware_profile else {
        return;
    };
    let profiles = spanda_hardware::builtin_profiles();
    let Some(profile) = profiles.get(profile_name) else {
        return;
    };
    let device_types: HashSet<String> = compute
        .devices
        .iter()
        .map(|d| d.device_type.clone())
        .collect();
    for sensor in &profile.sensors {
        let found = device_types
            .iter()
            .any(|dt| dt.contains(sensor) || sensor.contains(dt));
        if !found {
            report.push(
                ValidationSeverity::Warning,
                "hardware.profile_sensor_gap",
                format!(
                    "robot '{}' profile '{profile_name}' expects sensor '{sensor}' but none matched",
                    robot.id
                ),
                Some(format!("fleet.robots.{}.hardware_profile", robot.id)),
            );
        }
    }
    for actuator in &profile.actuators {
        let found = device_types
            .iter()
            .any(|dt| dt.contains(actuator) || actuator.contains(dt));
        if !found {
            report.push(
                ValidationSeverity::Warning,
                "hardware.profile_actuator_gap",
                format!(
                    "robot '{}' profile '{profile_name}' expects actuator '{actuator}' but none matched",
                    robot.id
                ),
                Some(format!("fleet.robots.{}.hardware_profile", robot.id)),
            );
        }
    }
}

pub fn validate_logical_map(map: &LogicalPhysicalMap) -> ConfigValidationReport {
    // Validate logical-to-physical mapping completeness.
    //
    // Parameters:
    // - `map` — derived logical/physical mapping
    //
    // Returns:
    // Validation report for mapping gaps.
    //
    // Options:
    // None.
    //
    // Example:
    // let report = validate_logical_map(&resolved.logical_map);

    let mut report = ConfigValidationReport {
        passed: true,
        findings: Vec::new(),
    };
    for issue in map.verify() {
        let severity = if issue.contains("emergency_stop") {
            ValidationSeverity::Error
        } else {
            ValidationSeverity::Warning
        };
        report.push(severity, "mapping.gap", issue, None);
    }
    report
}
