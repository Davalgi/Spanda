//! CLI-injected telemetry sink wiring telemetry store into the interpreter.
//!
use spanda_runtime::telemetry_sink::SharedTelemetrySink;
use spanda_telemetry_store::TelemetryStoreSink;

/// Shared telemetry sink for default `spanda` CLI runs.
pub fn default_telemetry_sink() -> SharedTelemetrySink {
    std::sync::Arc::new(TelemetryStoreSink)
}
