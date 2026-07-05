//! In-process reflex trace buffer for runtime and API replay.
//!
use std::sync::{LazyLock, Mutex};

use chrono::Utc;

use crate::reflex::{evaluate_reflex_priority, list_reflex_actions, ReflexTrace};

const MAX_TRACES: usize = 256;

static REFLEX_TRACE_BUFFER: LazyLock<Mutex<Vec<ReflexTrace>>> =
    LazyLock::new(|| Mutex::new(Vec::new()));

/// Append a reflex trace to the rolling in-process buffer.
pub fn record_reflex_trace(trace: ReflexTrace) {
    let mut buffer = REFLEX_TRACE_BUFFER
        .lock()
        .expect("reflex trace buffer lock poisoned");
    buffer.push(trace);
    if buffer.len() > MAX_TRACES {
        let overflow = buffer.len() - MAX_TRACES;
        buffer.drain(0..overflow);
    }
}

/// Return recorded runtime traces (newest last).
pub fn list_recorded_reflex_traces() -> Vec<ReflexTrace> {
    REFLEX_TRACE_BUFFER
        .lock()
        .expect("reflex trace buffer lock poisoned")
        .clone()
}

/// Record a reflex trace from a live runtime event (kill switch, decision tree, etc.).
pub fn record_runtime_reflex(
    entity_id: impl Into<String>,
    hint: &str,
    trigger: impl Into<String>,
    action_taken: impl Into<String>,
) {
    let entity_id = entity_id.into();
    let trigger = trigger.into();
    let action_taken = action_taken.into();
    let actions = list_reflex_actions();

    let trace = if let Some(matched) = evaluate_reflex_priority(&actions, hint) {
        ReflexTrace {
            reflex_id: matched.id.clone(),
            entity_id,
            trigger: if trigger.is_empty() {
                matched.trigger.clone()
            } else {
                trigger
            },
            action_taken: if action_taken.is_empty() {
                matched.action.clone()
            } else {
                action_taken
            },
            timestamp: Utc::now().to_rfc3339(),
            priority: matched.priority,
        }
    } else {
        ReflexTrace {
            reflex_id: format!("reflex.{hint}"),
            entity_id,
            trigger,
            action_taken,
            timestamp: Utc::now().to_rfc3339(),
            priority: 128,
        }
    };
    record_reflex_trace(trace);
}
