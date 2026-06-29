//! Injectable IoT device telemetry persistence for provider bootstrap paths.
//!
use crate::value::RuntimeValue;
use std::sync::{Arc, OnceLock};

/// Extension points for IoT hub telemetry persistence outside interpreter runs.
pub trait DeviceTelemetrySink: Send + Sync {
    fn persist_enabled(&self) -> bool;

    fn wall_timestamp_ms(&self) -> f64;

    fn record_device_telemetry(
        &self,
        device_id: &str,
        metric: &str,
        value: &RuntimeValue,
        timestamp_ms: f64,
        robot_id: Option<&str>,
    );

    fn record_device_heartbeat(
        &self,
        device_id: &str,
        timestamp_ms: f64,
        robot_id: Option<&str>,
        protocol: Option<&str>,
        history_interval_ms: f64,
    );

    fn is_heartbeat_metric(&self, metric: &str) -> bool;
}

/// No-op device telemetry sink for tests and runs without persistence.
#[derive(Debug, Default, Clone, Copy)]
pub struct NoopDeviceTelemetrySink;

impl DeviceTelemetrySink for NoopDeviceTelemetrySink {
    fn persist_enabled(&self) -> bool {
        false
    }

    fn wall_timestamp_ms(&self) -> f64 {
        0.0
    }

    fn record_device_telemetry(
        &self,
        _device_id: &str,
        _metric: &str,
        _value: &RuntimeValue,
        _timestamp_ms: f64,
        _robot_id: Option<&str>,
    ) {
    }

    fn record_device_heartbeat(
        &self,
        _device_id: &str,
        _timestamp_ms: f64,
        _robot_id: Option<&str>,
        _protocol: Option<&str>,
        _history_interval_ms: f64,
    ) {
    }

    fn is_heartbeat_metric(&self, _metric: &str) -> bool {
        false
    }
}

static DEVICE_TELEMETRY_SINK: OnceLock<Arc<dyn DeviceTelemetrySink>> = OnceLock::new();

/// Install the process-wide device telemetry sink from the CLI or service boundary.
pub fn set_device_telemetry_sink(sink: Arc<dyn DeviceTelemetrySink>) {
    let _ = DEVICE_TELEMETRY_SINK.set(sink);
}

/// Shared device telemetry sink for provider IoT hub calls.
pub fn device_telemetry_sink() -> Arc<dyn DeviceTelemetrySink> {
    DEVICE_TELEMETRY_SINK
        .get()
        .cloned()
        .unwrap_or_else(|| Arc::new(NoopDeviceTelemetrySink))
}
