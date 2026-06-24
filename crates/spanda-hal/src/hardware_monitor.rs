//! Runtime hardware health monitoring for hardware trigger dispatch.

use spanda_runtime::value::RuntimeValue;
use std::collections::{HashMap, HashSet};

const FAILURE_THRESHOLD: u32 = 2;

/// Tracks sensor/actuator health and maps failures to hardware trigger event names.
#[derive(Debug, Default)]
pub struct HardwareMonitor {
    sensors: Vec<(String, String)>,
    actuators: Vec<(String, String)>,
    injected_faults: HashSet<String>,
    active_events: HashSet<String>,
    dispatched_events: HashSet<String>,
    read_failures: HashMap<String, u32>,
}

impl HardwareMonitor {
    pub fn register_sensor(&mut self, name: impl Into<String>, sensor_type: impl Into<String>) {
        // Description:
        //     Register sensor.
        //
        // Inputs:
        //     &mut self: input value
        //         Caller-supplied &mut self.
        //     name: impl Into<String>
        //         Caller-supplied name.
        //     sensor_type: impl Into<String>
        //         Caller-supplied sensor type.
        //
        // Outputs:
        //     None.
        //
        // Example:
        //     let result = spanda_hal::hardware_monitor::register_sensor(&mut self, name, sensor_type);

        // Append into self.
        self.sensors.push((name.into(), sensor_type.into()));
    }

    pub fn register_actuator(&mut self, name: impl Into<String>, actuator_type: impl Into<String>) {
        // Description:
        //     Register actuator.
        //
        // Inputs:
        //     &mut self: input value
        //         Caller-supplied &mut self.
        //     name: impl Into<String>
        //         Caller-supplied name.
        //     actuator_type: impl Into<String>
        //         Caller-supplied actuator type.
        //
        // Outputs:
        //     None.
        //
        // Example:
        //     let result = spanda_hal::hardware_monitor::register_actuator(&mut self, name, actuator_type);

        // Append into self.
        self.actuators.push((name.into(), actuator_type.into()));
    }

    pub fn has_injected_faults(&self) -> bool {
        // Description:
        //     Has injected faults.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //
        // Outputs:
        //     result: bool
        //         Return value from `has_injected_faults`.
        //
        // Example:
        //     let result = spanda_hal::hardware_monitor::has_injected_faults(&self);

        // Check the injected fault set.
        !self.injected_faults.is_empty()
    }

    pub fn inject_fault(&mut self, fault: impl Into<String>) {
        // Description:
        //     Inject fault.
        //
        // Inputs:
        //     &mut self: input value
        //         Caller-supplied &mut self.
        //     faul: impl Into<String>
        //         Caller-supplied faul.
        //
        // Outputs:
        //     None.
        //
        // Example:

        //     let result = spanda_hal::hardware_monitor::inject_fault(&mut self, faul);

        self.injected_faults.insert(fault.into());
    }

    pub fn injected_faults(&self) -> &HashSet<String> {
        // Description:
        //     Injected faults.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //
        // Outputs:
        //     result: &HashSet<String>
        //         Return value from `injected_faults`.
        //
        // Example:

        //     let result = spanda_hal::hardware_monitor::injected_faults(&self);

        &self.injected_faults
    }

    pub fn sensor_event_for_type(sensor_type: &str) -> Option<&'static str> {
        // Description:
        //     Sensor event for type.
        //
        // Inputs:
        //     sensor_type: &str
        //         Caller-supplied sensor type.
        //
        // Outputs:
        //     result: Option<&'static str>
        //         Return value from `sensor_event_for_type`.
        //
        // Example:
        //     let result = spanda_hal::hardware_monitor::sensor_event_for_type(sensor_type);

        // Match on sensor type and handle each case.
        match sensor_type {
            "Lidar" => Some("LidarFailure"),
            "Camera" | "VisionCamera" | "RGBCamera" => Some("CameraFailure"),
            "IMU" | "BoschBNO055" => Some("ImuFailure"),
            "GPS" => Some("GpsFailure"),
            "GNSS" => Some("GpsFailure"),
            _ => None,
        }
    }

    pub fn actuator_event_for_type(actuator_type: &str) -> Option<&'static str> {
        // Description:
        //     Actuator event for type.
        //
        // Inputs:
        //     actuator_type: &str
        //         Caller-supplied actuator type.
        //
        // Outputs:
        //     result: Option<&'static str>
        //         Return value from `actuator_event_for_type`.
        //
        // Example:
        //     let result = spanda_hal::hardware_monitor::actuator_event_for_type(actuator_type);

        // Match on actuator type and handle each case.
        match actuator_type {
            "DifferentialDrive" | "Wheels" => Some("DriveFailure"),
            "Arm" | "Manipulator" => Some("ActuatorFailure"),
            _ => None,
        }
    }

    fn fault_matches_sensor(fault: &str, sensor_type: &str, sensor_name: &str) -> bool {
        // Description:
        //     Fault matches sensor.
        //
        // Inputs:
        //     faul: &str
        //         Caller-supplied faul.
        //     sensor_type: &str
        //         Caller-supplied sensor type.
        //     sensor_name: &str
        //         Caller-supplied sensor name.
        //
        // Outputs:
        //     result: bool
        //         Return value from `fault_matches_sensor`.
        //
        // Example:
        //     let result = spanda_hal::hardware_monitor::fault_matches_sensor(faul, sensor_type, sensor_name);

        // Compute fault lower for the following logic.
        let fault_lower = fault.to_ascii_lowercase();
        let name_lower = sensor_name.to_ascii_lowercase();

        // Take the branch when fault equals sensor name || fault lower == name lower.
        if fault == sensor_name || fault_lower == name_lower {
            return true;
        }

        // Emit output when sensor event for type provides a event.
        if let Some(event) = Self::sensor_event_for_type(sensor_type) {
            // Take the branch when fault equals to ascii lowercase.
            if fault == event || fault_lower == event.to_ascii_lowercase() {
                return true;
            }
        }

        // Match on sensor type and handle each case.
        match sensor_type {
            "Lidar" => fault_lower.contains("lidar"),
            "Camera" | "VisionCamera" | "RGBCamera" => fault_lower.contains("camera"),
            "IMU" | "BoschBNO055" => fault_lower.contains("imu"),
            "GPS" => fault_lower.contains("gps"),
            _ => false,
        }
    }

    pub fn record_sensor_reading(&mut self, name: &str, sensor_type: &str, reading: &RuntimeValue) {
        // Description:
        //     Record sensor reading.
        //
        // Inputs:
        //     &mut self: input value
        //         Caller-supplied &mut self.
        //     name: &str
        //         Caller-supplied name.
        //     sensor_type: &str
        //         Caller-supplied sensor type.
        //     reading: &RuntimeValue
        //         Caller-supplied reading.
        //
        // Outputs:
        //     None.
        //
        // Example:
        //     let result = spanda_hal::hardware_monitor::record_sensor_reading(&mut self, name, sensor_type, reading);

        // take this path when Self::reading failed(reading).
        if Self::reading_failed(reading) {
            let count = self.read_failures.entry(name.to_string()).or_insert(0);
            *count += 1;

            // Take this path when *count >= FAILURE THRESHOLD.
            if *count >= FAILURE_THRESHOLD {
                // Emit output when sensor event for type provides a event.
                if let Some(event) = Self::sensor_event_for_type(sensor_type) {
                    self.active_events.insert(event.to_string());
                }
            }
        } else {
            self.read_failures.remove(name);
        }
    }

    fn reading_failed(reading: &RuntimeValue) -> bool {
        // Description:
        //     Reading failed.
        //
        // Inputs:
        //     reading: &RuntimeValue
        //         Caller-supplied reading.
        //
        // Outputs:
        //     result: bool
        //         Return value from `reading_failed`.
        //
        // Example:
        //     let result = spanda_hal::hardware_monitor::reading_failed(reading);

        // Produce matches! as the result.
        matches!(
            reading,
            RuntimeValue::Null | RuntimeValue::Void | RuntimeValue::Result { ok: false, .. }
        )
    }

    pub fn evaluate_injected_faults(&mut self) {
        // Description:
        //     Evaluate injected faults.
        //
        // Inputs:
        //     &mut self: input value
        //         Caller-supplied &mut self.
        //
        // Outputs:
        //     None.
        //
        // Example:
        //     let result = spanda_hal::hardware_monitor::evaluate_injected_faults(&mut self);

        // Inject each configured hardware fault.
        for fault in self.injected_faults.clone() {
            // Iterate over sensors with destructured elements.
            for (name, sensor_type) in &self.sensors {
                // Take this path when Self::fault matches sensor(&fault, sensor type, name).
                if Self::fault_matches_sensor(&fault, sensor_type, name) {
                    // Emit output when sensor event for type provides a event.
                    if let Some(event) = Self::sensor_event_for_type(sensor_type) {
                        self.active_events.insert(event.to_string());
                    }
                }
            }

            // Proceed only when is some is available.
            if Self::actuator_event_for_type(&fault).is_some()
                || fault.to_ascii_lowercase().contains("actuator")
                || fault.to_ascii_lowercase().contains("drive")
            {
                self.active_events.insert(fault.clone());
            }
        }
    }

    /// Returns newly detected hardware events to dispatch (edge-triggered).
    pub fn poll_new_events(&mut self) -> Vec<String> {
        // Description:
        //     Poll new events.
        //
        // Inputs:
        //     &mut self: input value
        //         Caller-supplied &mut self.
        //
        // Outputs:
        //     result: Vec<String>
        //         Return value from `poll_new_events`.
        //
        // Example:
        //     let result = spanda_hal::hardware_monitor::poll_new_events(&mut self);

        // Call evaluate injected faults on the current instance.
        self.evaluate_injected_faults();
        let mut new_events = Vec::new();

        // Process each active event.
        for event in &self.active_events {
            // Take this path when self.dispatched events.insert(event.clone()).
            if self.dispatched_events.insert(event.clone()) {
                new_events.push(event.clone());
            }
        }
        new_events
    }

    pub fn clear_event(&mut self, event: &str) {
        // Description:
        //     Clear event.
        //
        // Inputs:
        //     &mut self: input value
        //         Caller-supplied &mut self.
        //     even: &str
        //         Caller-supplied even.
        //
        // Outputs:
        //     None.
        //
        // Example:
        //     let result = spanda_hal::hardware_monitor::clear_event(&mut self, even);

        // Call remove on the current instance.
        self.active_events.remove(event);
        self.dispatched_events.remove(event);
    }

    pub fn overall_health_label(&self) -> &'static str {
        // Description:
        //     Overall health label.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //
        // Outputs:
        //     result: &'static str
        //         Return value from `overall_health_label`.
        //
        // Example:

        //     let result = spanda_hal::hardware_monitor::overall_health_label(&self);

        let faults: Vec<String> = self
            .injected_faults
            .iter()
            .map(|f| f.to_ascii_lowercase())
            .collect();

        if faults
            .iter()
            .any(|f| f.contains("critical") || f.contains("unsafe"))
        {
            return "Critical";
        }
        if !self.active_events.is_empty() || !self.injected_faults.is_empty() {
            return "Degraded";
        }
        "Healthy"
    }

    pub fn runtime_faults(&self) -> Vec<String> {
        // Description:
        //     Runtime faults.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //
        // Outputs:
        //     result: Vec<String>
        //         Return value from `runtime_faults`.
        //
        // Example:

        //     let result = spanda_hal::hardware_monitor::runtime_faults(&self);

        self.injected_faults.iter().cloned().collect()
    }

    pub fn runtime_events(&self) -> Vec<String> {
        // Description:
        //     Runtime events.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //
        // Outputs:
        //     result: Vec<String>
        //         Return value from `runtime_events`.
        //
        // Example:

        //     let result = spanda_hal::hardware_monitor::runtime_events(&self);

        self.active_events.iter().cloned().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn injected_lidar_fault_maps_to_event() {
        // Description:
        //     Injected lidar fault maps to event.
        //
        // Inputs:
        //     None.
        //
        // Outputs:
        //     None.
        //
        // Example:

        //     let result = spanda_hal::hardware_monitor::injected_lidar_fault_maps_to_event();

        let mut monitor = HardwareMonitor::default();
        monitor.register_sensor("lidar", "Lidar");
        monitor.inject_fault("LidarFailure");
        let events = monitor.poll_new_events();
        assert!(events.contains(&"LidarFailure".to_string()));
    }
}
