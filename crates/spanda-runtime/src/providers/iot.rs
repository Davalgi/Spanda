//! Generic IoT provider trait contracts for package-first integrations.

use crate::value::RuntimeValue;
use serde::{Deserialize, Serialize};

/// IoT device identity and lifecycle.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IoTDevice {
    pub id: String,
    pub protocol: String,
    pub topic: Option<String>,
}

/// Device shadow desired/reported state.
#[derive(Debug, Clone, PartialEq)]
pub struct DeviceShadow {
    pub device_id: String,
    pub desired: RuntimeValue,
    pub reported: RuntimeValue,
}

/// Telemetry reading from an IoT device.
#[derive(Debug, Clone, PartialEq)]
pub struct Telemetry {
    pub device_id: String,
    pub metric: String,
    pub value: RuntimeValue,
    pub timestamp_ms: f64,
}

/// Remote command to an IoT device.
#[derive(Debug, Clone, PartialEq)]
pub struct Command {
    pub device_id: String,
    pub action: String,
    pub payload: RuntimeValue,
}

/// Sensor reading from IoT telemetry.
#[derive(Debug, Clone, PartialEq)]
pub struct SensorReading {
    pub sensor_id: String,
    pub reading: RuntimeValue,
}

/// Actuator command for IoT devices.
#[derive(Debug, Clone, PartialEq)]
pub struct ActuatorCommand {
    pub actuator_id: String,
    pub command: String,
    pub value: RuntimeValue,
}

/// Device lifecycle and registration.
pub trait IoTDeviceProvider: Send + Sync {
    fn register_device(&mut self, device: &IoTDevice) -> Result<(), String>;
    fn device_state(&self, device_id: &str) -> Option<IoTDevice>;
}

/// Telemetry ingestion and query.
pub trait TelemetryProvider: Send + Sync {
    fn publish_telemetry(&mut self, telemetry: Telemetry);
    fn latest(&self, device_id: &str, metric: &str) -> Option<Telemetry>;
}

/// Remote command dispatch.
pub trait CommandProvider: Send + Sync {
    fn send_command(&mut self, command: Command) -> Result<(), String>;
}

/// Device shadow sync.
pub trait DeviceShadowProvider: Send + Sync {
    fn update_desired(&mut self, shadow: DeviceShadow);
    fn reported_state(&self, device_id: &str) -> Option<DeviceShadow>;
}
