//! Optional live hardware overlays for interpreter sensor reads.
//!
use crate::value::RuntimeValue;
use std::sync::{LazyLock, Mutex};

/// Overlay live GPS/IMU/camera hardware reads onto simulated sensor values.
pub type LiveSensorOverlay = fn(sensor_type: &str, sensor_name: &str, simulated: &RuntimeValue) -> RuntimeValue;

static LIVE_SENSOR_OVERLAY: LazyLock<Mutex<Option<LiveSensorOverlay>>> = LazyLock::new(|| Mutex::new(None));

/// Register a process-wide live sensor overlay (typically from `spanda-cli` + providers).
pub fn register_live_sensor_overlay(overlay: LiveSensorOverlay) {
    *LIVE_SENSOR_OVERLAY
        .lock()
        .expect("live sensor overlay lock poisoned") = Some(overlay);
}

/// Apply a registered overlay when live sensor pipelines are enabled.
pub fn apply_live_sensor_overlay(
    sensor_type: &str,
    sensor_name: &str,
    simulated: RuntimeValue,
) -> RuntimeValue {
    if !live_sensor_pipeline_enabled() {
        return simulated;
    }
    let overlay = {
        let lock = LIVE_SENSOR_OVERLAY
            .lock()
            .expect("live sensor overlay lock poisoned");
        *lock
    };
    let Some(overlay) = overlay else {
        return simulated;
    };
    overlay(sensor_type, sensor_name, &simulated)
}

fn live_sensor_pipeline_enabled() -> bool {
    std::env::var("SPANDA_LIVE_SENSOR_PIPELINE")
        .or_else(|_| std::env::var("SPANDA_LIVE_FUSION_SENSORS"))
        .ok()
        .as_deref()
        .is_some_and(|v| v == "1" || v.eq_ignore_ascii_case("true"))
}
