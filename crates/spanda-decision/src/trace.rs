//! Parse mission traces into decision audit records.

use spanda_runtime::replay::MissionTrace;

use crate::report::{
    DecisionAuditReport, DecisionChain, DecisionEvidence, DecisionRecord, DecisionTimeline,
};

fn payload_is_decision_v3(payload: &serde_json::Value) -> bool {
    // Description:
    //     Detect embedded v3 decision records in trace payloads.
    //
    // Parameters:
    // - `payload` — trace frame JSON payload
    //
    // Returns:
    // True when payload matches decision schema.
    //
    // Options:
    // None.
    //
    // Example:
    // let ok = payload_is_decision_v3(&payload);

    payload.get("decision_id").is_some() && payload.get("decision").is_some()
}

fn decision_from_payload(
    payload: &serde_json::Value,
    sim_time_ms: f64,
    event: &str,
) -> Option<DecisionRecord> {
    // Description:
    //     Convert a trace payload into a decision record when possible.
    //
    // Parameters:
    // - `payload` — frame payload
    // - `sim_time_ms` — simulation timestamp
    // - `event` — trace event name
    //
    // Returns:
    // Decision record or None when payload is not decision-shaped.
    //
    // Options:
    // None.
    //
    // Example:
    // let record = decision_from_payload(&payload, 0.0, "safety_validate");

    if payload_is_decision_v3(payload) {
        let decision_id = payload
            .get("decision_id")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown")
            .to_string();
        let decision = payload
            .get("decision")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown")
            .to_string();
        let reason = payload
            .get("reason")
            .and_then(|v| v.as_str())
            .unwrap_or("trace_payload")
            .to_string();
        let mission = payload
            .get("mission")
            .and_then(|v| v.as_str())
            .map(str::to_string);
        let evidence = payload
            .get("evidence")
            .and_then(|v| v.as_object())
            .cloned()
            .unwrap_or_default();
        return Some(DecisionRecord {
            version: payload.get("version").and_then(|v| v.as_u64()).unwrap_or(3) as u32,
            decision_id,
            mission,
            timestamp_ms: sim_time_ms,
            decision,
            reason,
            evidence: DecisionEvidence { fields: evidence },
            alternatives_considered: payload
                .get("alternatives_considered")
                .and_then(|v| v.as_array())
                .cloned()
                .unwrap_or_default(),
            safety_checks: payload
                .get("safety_checks")
                .and_then(|v| v.as_array())
                .cloned()
                .unwrap_or_default(),
            action: payload.get("action").cloned(),
            source_event: event.to_string(),
        });
    }

    let lower = event.to_ascii_lowercase();
    let synthesized = lower.contains("safety")
        || lower.contains("recovery")
        || lower.contains("continuity")
        || lower.contains("takeover")
        || lower.contains("provider_call")
        || lower.contains("kill_switch")
        || lower.contains("approval");
    if !synthesized {
        return None;
    }

    let decision = if lower.contains("safety") {
        "safety_gate"
    } else if lower.contains("recovery") {
        "recovery_action"
    } else if lower.contains("continuity") || lower.contains("takeover") {
        "continuity_handoff"
    } else if lower.contains("provider_call") {
        "provider_dispatch"
    } else if lower.contains("kill_switch") {
        "emergency_stop"
    } else {
        "operational_decision"
    };

    Some(DecisionRecord {
        version: 1,
        decision_id: format!("d-{sim_time_ms:.0}-{event}"),
        mission: payload
            .get("mission")
            .and_then(|v| v.as_str())
            .map(str::to_string),
        timestamp_ms: sim_time_ms,
        decision: decision.into(),
        reason: format!("synthesized_from_event:{event}"),
        evidence: DecisionEvidence {
            fields: payload.as_object().cloned().unwrap_or_default(),
        },
        alternatives_considered: Vec::new(),
        safety_checks: Vec::new(),
        action: payload.get("action").cloned(),
        source_event: event.to_string(),
    })
}

/// Build a decision audit report from a mission trace file path.
pub fn audit_decisions_from_trace(trace_path: &str) -> Result<DecisionAuditReport, String> {
    // Description:
    //     Load a mission trace and extract decision records.
    //
    // Parameters:
    // - `trace_path` — path to `.trace` JSON file
    //
    // Returns:
    // Decision audit report or load error message.
    //
    // Options:
    // None.
    //
    // Example:
    // let report = audit_decisions_from_trace("mission.trace")?;

    let trace = MissionTrace::load(trace_path).map_err(|e| e.to_string())?;
    let mut decisions = Vec::new();
    for frame in &trace.frames {
        if let Some(record) = decision_from_payload(&frame.payload, frame.sim_time_ms, &frame.event)
        {
            decisions.push(record);
        }
    }
    let mission = decisions
        .iter()
        .find_map(|record| record.mission.clone());
    let timeline = DecisionTimeline {
        source: trace.source.clone(),
        decisions: decisions.clone(),
    };
    let chains = if decisions.is_empty() {
        Vec::new()
    } else {
        vec![DecisionChain {
            mission,
            records: decisions.clone(),
        }]
    };
    Ok(DecisionAuditReport {
        source: trace_path.to_string(),
        decision_count: decisions.len(),
        timeline,
        chains,
    })
}

/// Build plain-language explanations from a mission trace.
pub fn explain_decisions_from_trace(trace_path: &str) -> Result<String, String> {
    // Description:
    //     Load a trace and render decision explanations.
    //
    // Parameters:
    // - `trace_path` — mission trace path
    //
    // Returns:
    // Explanation text or error message.
    //
    // Options:
    // None.
    //
    // Example:
    // let text = explain_decisions_from_trace("mission.trace")?;

    let report = audit_decisions_from_trace(trace_path)?;
    Ok(crate::report::format_decision_explanations(&report))
}
