//! In-process and file-backed reflex trace buffer for runtime and API replay.
//!
use std::path::{Path, PathBuf};
use std::sync::{LazyLock, Mutex};

use chrono::Utc;

use crate::reflex::{evaluate_reflex_priority, list_reflex_actions, ReflexTrace};

const MAX_TRACES: usize = 256;

static REFLEX_TRACE_BUFFER: LazyLock<Mutex<Vec<ReflexTrace>>> =
    LazyLock::new(|| Mutex::new(Vec::new()));

static TRACE_STORE_LOADED: LazyLock<Mutex<bool>> = LazyLock::new(|| Mutex::new(false));

/// Default on-disk path for reflex traces (`SPANDA_AUTONOMY_TRACE_FILE` overrides).
pub fn default_trace_store_path() -> PathBuf {
    std::env::var("SPANDA_AUTONOMY_TRACE_FILE")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from(".spanda/autonomy-reflex-traces.json"))
}

fn ensure_trace_store_loaded() {
    let mut loaded = TRACE_STORE_LOADED
        .lock()
        .expect("trace store loaded lock poisoned");
    if *loaded {
        return;
    }
    load_reflex_traces_from_disk();
    *loaded = true;
}

/// Load reflex traces from the configured trace store file.
pub fn load_reflex_traces_from_disk() {
    let path = default_trace_store_path();
    if !path.is_file() {
        return;
    }
    let Ok(content) = std::fs::read_to_string(&path) else {
        return;
    };
    let Ok(traces) = serde_json::from_str::<Vec<ReflexTrace>>(&content) else {
        return;
    };
    let mut buffer = REFLEX_TRACE_BUFFER
        .lock()
        .expect("reflex trace buffer lock poisoned");
    *buffer = traces;
    if buffer.len() > MAX_TRACES {
        let overflow = buffer.len() - MAX_TRACES;
        buffer.drain(0..overflow);
    }
}

/// Persist reflex traces to the configured trace store file.
pub fn persist_reflex_traces_to_disk() -> Result<(), String> {
    let path = default_trace_store_path();
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| format!("create {}: {e}", parent.display()))?;
    }
    let traces = list_recorded_reflex_traces();
    let body = serde_json::to_string_pretty(&traces).map_err(|e| e.to_string())?;
    std::fs::write(&path, body).map_err(|e| format!("write {}: {e}", path.display()))
}

/// Append a reflex trace to the rolling buffer and persist when configured.
pub fn record_reflex_trace(trace: ReflexTrace) {
    ensure_trace_store_loaded();
    let mut buffer = REFLEX_TRACE_BUFFER
        .lock()
        .expect("reflex trace buffer lock poisoned");
    buffer.push(trace);
    if buffer.len() > MAX_TRACES {
        let overflow = buffer.len() - MAX_TRACES;
        buffer.drain(0..overflow);
    }
    drop(buffer);
    let _ = persist_reflex_traces_to_disk();
}

/// Return recorded runtime traces (newest last).
pub fn list_recorded_reflex_traces() -> Vec<ReflexTrace> {
    ensure_trace_store_loaded();
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    static TEST_LOCK: Mutex<()> = Mutex::new(());

    #[test]
    fn trace_store_roundtrip_tmpdir() {
        let _guard = TEST_LOCK.lock().expect("test lock");
        let dir = tempfile::tempdir().expect("tempdir");
        let path = dir.path().join("traces.json");
        std::env::set_var(
            "SPANDA_AUTONOMY_TRACE_FILE",
            path.to_string_lossy().as_ref(),
        );
        record_reflex_trace(ReflexTrace {
            reflex_id: "reflex.test".into(),
            entity_id: "robot-test".into(),
            trigger: "test".into(),
            action_taken: "noop".into(),
            timestamp: Utc::now().to_rfc3339(),
            priority: 1,
        });
        assert!(path.is_file());
        let mut buffer = REFLEX_TRACE_BUFFER.lock().expect("lock");
        buffer.clear();
        drop(buffer);
        *TRACE_STORE_LOADED.lock().expect("lock") = false;
        load_reflex_traces_from_disk();
        let traces = list_recorded_reflex_traces();
        assert!(traces.iter().any(|t| t.entity_id == "robot-test"));
        std::env::remove_var("SPANDA_AUTONOMY_TRACE_FILE");
    }
}
