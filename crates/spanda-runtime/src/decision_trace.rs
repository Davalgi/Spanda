//! v3 decision trace payload helpers for mission trace emission.

use serde_json::{json, Value};

/// Return true when distributed decision trace emission is enabled.
pub fn decision_trace_enabled() -> bool {
    std::env::var("SPANDA_DECISION_TRACE")
        .map(|v| matches!(v.as_str(), "1" | "true" | "yes" | "on"))
        .unwrap_or(false)
}

/// Build a v3 decision record payload for mission trace frames.
pub fn v3_decision_payload(
    decision_id: &str,
    mission: Option<&str>,
    decision: &str,
    reason: &str,
    layer: &str,
    entity_id: &str,
    evidence: Value,
) -> Value {
    json!({
        "version": 3,
        "decision_id": decision_id,
        "mission": mission,
        "decision": decision,
        "reason": reason,
        "layer": layer,
        "entity_id": entity_id,
        "evidence": evidence,
        "alternatives_considered": [],
        "safety_checks": [{"rule": "distributed_decision", "passed": true}],
    })
}
