//! Injectable telemetry persistence boundary for interpreter mission recording.
//!
use crate::telemetry::RuntimeTelemetry;
use crate::value::RuntimeValue;
use serde_json::Value as JsonValue;
use std::sync::Arc;

/// Extension points for telemetry persistence during interpreter runs.
pub trait TelemetrySink: Send + Sync {
    fn configure_session_persist(&self, enabled: bool);

    fn begin_run_session(&self, source: Option<&str>);

    fn end_run_session(
        &self,
        mission_trace_path: Option<&str>,
        metrics: Option<&RuntimeTelemetry>,
        timestamp_ms: f64,
    );

    fn record_sensor_reading(
        &self,
        sensor_id: &str,
        sensor_type: &str,
        value: &RuntimeValue,
        timestamp_ms: f64,
        robot_id: Option<&str>,
    );

    fn record_health_event(&self, target: &str, status: &str, timestamp_ms: f64);

    fn record_platform_event(
        &self,
        event_type: &str,
        source: &str,
        entity_id: Option<&str>,
        payload: JsonValue,
        timestamp_ms: f64,
    );

    fn record_task_heartbeat(
        &self,
        task_name: &str,
        timestamp_ms: f64,
        robot_id: Option<&str>,
        history_interval_ms: f64,
    );

    fn record_topic_publish(
        &self,
        robot_id: Option<&str>,
        topic_path: &str,
        value: &RuntimeValue,
        timestamp_ms: f64,
    );
}

/// No-op telemetry sink for tests and runs without persistence.
#[derive(Debug, Default, Clone, Copy)]
pub struct NoopTelemetrySink;

impl TelemetrySink for NoopTelemetrySink {
    fn configure_session_persist(&self, _enabled: bool) {}

    fn begin_run_session(&self, _source: Option<&str>) {}

    fn end_run_session(
        &self,
        _mission_trace_path: Option<&str>,
        _metrics: Option<&RuntimeTelemetry>,
        _timestamp_ms: f64,
    ) {
    }

    fn record_sensor_reading(
        &self,
        _sensor_id: &str,
        _sensor_type: &str,
        _value: &RuntimeValue,
        _timestamp_ms: f64,
        _robot_id: Option<&str>,
    ) {
    }

    fn record_health_event(&self, _target: &str, _status: &str, _timestamp_ms: f64) {}

    fn record_platform_event(
        &self,
        _event_type: &str,
        _source: &str,
        _entity_id: Option<&str>,
        _payload: JsonValue,
        _timestamp_ms: f64,
    ) {
    }

    fn record_task_heartbeat(
        &self,
        _task_name: &str,
        _timestamp_ms: f64,
        _robot_id: Option<&str>,
        _history_interval_ms: f64,
    ) {
    }

    fn record_topic_publish(
        &self,
        _robot_id: Option<&str>,
        _topic_path: &str,
        _value: &RuntimeValue,
        _timestamp_ms: f64,
    ) {
    }
}

/// Shared telemetry sink handle passed through run options at the driver boundary.
pub type SharedTelemetrySink = Arc<dyn TelemetrySink>;

/// Default no-op telemetry sink for direct interpreter use without telemetry store.
pub fn default_telemetry_sink() -> SharedTelemetrySink {
    Arc::new(NoopTelemetrySink)
}
