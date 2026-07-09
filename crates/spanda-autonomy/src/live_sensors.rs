//! Optional live multi-sensor readings merged into sensory fusion at the CLI boundary.
//!
use std::sync::{LazyLock, Mutex};

use crate::fusion::SensorConfidence;

/// Supplier registered by the CLI when live automotive/IoT bridges are enabled.
pub type LiveSensorSupplier = fn(&str) -> Vec<(String, f64, f64)>;

static LIVE_SENSOR_SUPPLIER: LazyLock<Mutex<Option<LiveSensorSupplier>>> =
    LazyLock::new(|| Mutex::new(None));

/// Register a process-wide live sensor supplier (typically from `spanda-cli` + providers).
pub fn register_live_sensor_supplier(supplier: LiveSensorSupplier) {
    *LIVE_SENSOR_SUPPLIER
        .lock()
        .expect("live sensor supplier lock poisoned") = Some(supplier);
}

fn live_fusion_sensors_enabled() -> bool {
    std::env::var("SPANDA_LIVE_FUSION_SENSORS")
        .ok()
        .as_deref()
        .is_some_and(|v| v == "1" || v.eq_ignore_ascii_case("true"))
}

/// Merge entity-derived readings with optional live sensor samples when enabled.
pub fn merge_live_sensor_readings(
    entity_id: &str,
    base: Vec<SensorConfidence>,
) -> Vec<SensorConfidence> {
    if !live_fusion_sensors_enabled() {
        return base;
    }

    let supplier = *LIVE_SENSOR_SUPPLIER
        .lock()
        .expect("live sensor supplier lock poisoned");
    let Some(supplier) = supplier else {
        return base;
    };

    let mut merged = base;
    for (source, value, confidence) in supplier(entity_id) {
        merged.push(SensorConfidence {
            source,
            value,
            confidence,
            timestamp: None,
        });
    }
    merged
}
