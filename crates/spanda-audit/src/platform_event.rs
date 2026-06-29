//! Platform event envelope types shared across Spanda subsystems.
//!
//! Canonical event names and categories are defined in `docs/event-model.md` and
//! `scripts/architecture-manifest.yaml` (`event_types`).

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Namespaced platform event type (e.g. `ReadinessChanged`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlatformEventType(pub String);

impl PlatformEventType {
    pub fn new(name: impl Into<String>) -> Self {
        Self(name.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Common JSON envelope for platform events (telemetry, audit, Control Center).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlatformEvent {
    #[serde(rename = "type")]
    pub event_type: PlatformEventType,
    pub timestamp: DateTime<Utc>,
    pub source: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_id: Option<String>,
    pub payload: Value,
}

impl PlatformEvent {
    pub fn new(
        event_type: impl Into<String>,
        source: impl Into<String>,
        payload: Value,
    ) -> Self {
        Self {
            event_type: PlatformEventType::new(event_type),
            timestamp: Utc::now(),
            source: source.into(),
            entity_id: None,
            payload,
        }
    }

    pub fn with_entity_id(mut self, entity_id: impl Into<String>) -> Self {
        self.entity_id = Some(entity_id.into());
        self
    }

    pub fn with_timestamp(mut self, timestamp: DateTime<Utc>) -> Self {
        self.timestamp = timestamp;
        self
    }

    pub fn namespaced_type(&self) -> String {
        format!("spanda.events.{}", self.event_type.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn platform_event_serializes_envelope_fields() {
        let event = PlatformEvent::new(
            "ReadinessChanged",
            "spanda-readiness",
            json!({"score": 0.92}),
        )
        .with_entity_id("robot/warehouse-alpha");

        let json = serde_json::to_value(&event).unwrap();
        assert_eq!(json["type"], "ReadinessChanged");
        assert_eq!(json["source"], "spanda-readiness");
        assert_eq!(json["entity_id"], "robot/warehouse-alpha");
        assert_eq!(json["payload"]["score"], 0.92);
    }

    #[test]
    fn namespaced_type_prefixes_event_name() {
        let event = PlatformEvent::new("MissionStarted", "spanda-interpreter", json!({}));
        assert_eq!(event.namespaced_type(), "spanda.events.MissionStarted");
    }
}
