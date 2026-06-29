//! Telemetry-store-backed device telemetry sink for provider IoT hub calls.
//!
use spanda_runtime::device_telemetry_sink::DeviceTelemetrySink;

/// Device telemetry persistence delegating to `spanda-telemetry-store`.
#[derive(Debug, Default, Clone, Copy)]
pub struct TelemetryStoreDeviceSink;

impl DeviceTelemetrySink for TelemetryStoreDeviceSink {
    fn persist_enabled(&self) -> bool {
        crate::persist_enabled()
    }

    fn wall_timestamp_ms(&self) -> f64 {
        crate::wall_timestamp_ms()
    }

    fn record_device_telemetry(
        &self,
        device_id: &str,
        metric: &str,
        value: &spanda_runtime::value::RuntimeValue,
        timestamp_ms: f64,
        robot_id: Option<&str>,
    ) {
        let _ = crate::record_device_telemetry(device_id, metric, value, timestamp_ms, robot_id);
    }

    fn record_device_heartbeat(
        &self,
        device_id: &str,
        timestamp_ms: f64,
        robot_id: Option<&str>,
        protocol: Option<&str>,
        history_interval_ms: f64,
    ) {
        let _ = crate::record_device_heartbeat(
            device_id,
            timestamp_ms,
            robot_id,
            protocol,
            history_interval_ms,
        );
    }

    fn is_heartbeat_metric(&self, metric: &str) -> bool {
        crate::is_heartbeat_metric(metric)
    }
}
