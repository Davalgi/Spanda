//! Persistent episodic memory store linked to the replay/trace index.
//!
use std::path::{Path, PathBuf};
use std::sync::{LazyLock, Mutex};

use chrono::Utc;
use serde::{Deserialize, Serialize};

const MAX_ENTRIES: usize = 512;

/// One episodic memory entry linked to a replay/trace artifact.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EpisodicEntry {
    pub id: String,
    pub entity_id: String,
    pub category: String,
    pub artifact_kind: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replay_path: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trace_id: Option<String>,
    pub timestamp: String,
}

static EPISODIC_BUFFER: LazyLock<Mutex<Vec<EpisodicEntry>>> =
    LazyLock::new(|| Mutex::new(Vec::new()));

static EPISODIC_LOADED: LazyLock<Mutex<bool>> = LazyLock::new(|| Mutex::new(false));

/// Default on-disk path for episodic memory (`SPANDA_EPISODIC_STORE_FILE` overrides).
pub fn default_episodic_store_path() -> PathBuf {
    std::env::var("SPANDA_EPISODIC_STORE_FILE")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from(".spanda/autonomy-episodic-memory.json"))
}

fn ensure_episodic_loaded() {
    let mut loaded = EPISODIC_LOADED
        .lock()
        .expect("episodic loaded lock poisoned");
    if *loaded {
        return;
    }
    load_episodic_store_from_disk();
    *loaded = true;
}

/// Load episodic entries from the configured store file.
pub fn load_episodic_store_from_disk() {
    // Hydrate the in-process episodic buffer from disk when the file exists.
    //
    // Parameters:
    // None.
    //
    // Returns:
    // Nothing; silently skips missing or invalid files.
    //
    // Options:
    // Path from `SPANDA_EPISODIC_STORE_FILE` or `.spanda/autonomy-episodic-memory.json`.
    //
    // Example:
    // load_episodic_store_from_disk();

    let path = default_episodic_store_path();
    if !path.is_file() {
        return;
    }
    let Ok(content) = std::fs::read_to_string(&path) else {
        return;
    };
    let Ok(entries) = serde_json::from_str::<Vec<EpisodicEntry>>(&content) else {
        return;
    };
    let mut buffer = EPISODIC_BUFFER
        .lock()
        .expect("episodic buffer lock poisoned");
    *buffer = entries;
    if buffer.len() > MAX_ENTRIES {
        let overflow = buffer.len() - MAX_ENTRIES;
        buffer.drain(0..overflow);
    }
}

/// Persist episodic entries to the configured store file.
pub fn persist_episodic_store_to_disk() -> Result<(), String> {
    // Write the current episodic buffer to disk for Control Center restarts.
    //
    // Parameters:
    // None.
    //
    // Returns:
    // `Ok(())` on success, or an I/O/serialize error string.
    //
    // Options:
    // None.
    //
    // Example:
    // persist_episodic_store_to_disk()?;

    let path = default_episodic_store_path();
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| format!("create {}: {e}", parent.display()))?;
    }
    let entries = list_episodic_entries(None);
    let body = serde_json::to_string_pretty(&entries).map_err(|e| e.to_string())?;
    std::fs::write(&path, body).map_err(|e| format!("write {}: {e}", path.display()))
}

/// Append an episodic entry and persist.
pub fn record_episodic_entry(entry: EpisodicEntry) {
    // Record one episodic memory row and flush to disk.
    //
    // Parameters:
    // - `entry` — episodic row to append
    //
    // Returns:
    // Nothing.
    //
    // Options:
    // None.
    //
    // Example:
    // record_episodic_entry(entry);

    ensure_episodic_loaded();
    let mut buffer = EPISODIC_BUFFER
        .lock()
        .expect("episodic buffer lock poisoned");
    buffer.push(entry);
    if buffer.len() > MAX_ENTRIES {
        let overflow = buffer.len() - MAX_ENTRIES;
        buffer.drain(0..overflow);
    }
    drop(buffer);
    let _ = persist_episodic_store_to_disk();
}

/// List episodic entries, optionally filtered by memory category name.
pub fn list_episodic_entries(category: Option<&str>) -> Vec<EpisodicEntry> {
    // Return stored episodic rows, newest last.
    //
    // Parameters:
    // - `category` — optional category filter (`episodic`, `reflex`, …)
    //
    // Returns:
    // Matching entries in insertion order.
    //
    // Options:
    // None.
    //
    // Example:
    // let rows = list_episodic_entries(Some("episodic"));

    ensure_episodic_loaded();
    let buffer = EPISODIC_BUFFER
        .lock()
        .expect("episodic buffer lock poisoned");
    match category {
        Some(cat) => buffer
            .iter()
            .filter(|e| e.category.eq_ignore_ascii_case(cat))
            .cloned()
            .collect(),
        None => buffer.clone(),
    }
}

/// Index `.trace` files under `root` into the episodic store for an entity.
pub fn index_replay_traces_for_entity(entity_id: &str, root: &Path) -> usize {
    // Walk a project tree for `.trace` files and link them as episodic memory.
    //
    // Parameters:
    // - `entity_id` — entity that owns the indexed traces
    // - `root` — directory to scan recursively
    //
    // Returns:
    // Number of newly recorded entries.
    //
    // Options:
    // Skips paths already present as `replay_path`.
    //
    // Example:
    // let n = index_replay_traces_for_entity("rover-001", Path::new("."));

    ensure_episodic_loaded();
    let existing: std::collections::HashSet<String> = list_episodic_entries(None)
        .into_iter()
        .filter_map(|e| e.replay_path)
        .collect();
    let mut paths = Vec::new();
    collect_trace_paths(root, root, &mut paths, 200);
    let mut added = 0usize;
    for rel in paths {
        if existing.contains(&rel) {
            continue;
        }
        let stem = Path::new(&rel)
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("trace");
        record_episodic_entry(EpisodicEntry {
            id: format!("ep-{}-{added}", Utc::now().timestamp_millis()),
            entity_id: entity_id.into(),
            category: "episodic".into(),
            artifact_kind: "replay".into(),
            replay_path: Some(rel.clone()),
            trace_id: Some(format!("trace:{stem}")),
            timestamp: Utc::now().to_rfc3339(),
        });
        added += 1;
    }
    added
}

fn collect_trace_paths(root: &Path, dir: &Path, out: &mut Vec<String>, limit: usize) {
    if out.len() >= limit {
        return;
    }
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        if out.len() >= limit {
            break;
        }
        let path = entry.path();
        if path.is_dir() {
            collect_trace_paths(root, &path, out, limit);
            continue;
        }
        if path.extension().and_then(|ext| ext.to_str()) != Some("trace") {
            continue;
        }
        let rel = path
            .strip_prefix(root)
            .map(|p| p.display().to_string())
            .unwrap_or_else(|_| path.display().to_string());
        out.push(rel);
    }
}

/// Episodic refs for an entity drawn from the persistent store (replay-linked).
pub fn episodic_refs_for_entity(entity_id: &str) -> Vec<String> {
    // Build episodic memory reference strings from the persistent store.
    //
    // Parameters:
    // - `entity_id` — entity to filter
    //
    // Returns:
    // Replay paths and trace ids for that entity.
    //
    // Options:
    // None.
    //
    // Example:
    // let refs = episodic_refs_for_entity("rover-001");

    list_episodic_entries(Some("episodic"))
        .into_iter()
        .filter(|e| e.entity_id == entity_id)
        .flat_map(|e| {
            let mut refs = Vec::new();
            if let Some(path) = e.replay_path {
                refs.push(format!("replay:{path}"));
            }
            if let Some(tid) = e.trace_id {
                refs.push(tid);
            }
            refs
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    static TEST_LOCK: Mutex<()> = Mutex::new(());

    #[test]
    fn episodic_store_roundtrip_and_index() {
        let _guard = TEST_LOCK.lock().expect("test lock");
        let dir = tempfile::tempdir().expect("tempdir");
        let store = dir.path().join("episodic.json");
        std::env::set_var(
            "SPANDA_EPISODIC_STORE_FILE",
            store.to_string_lossy().as_ref(),
        );
        {
            let mut buffer = EPISODIC_BUFFER.lock().expect("lock");
            buffer.clear();
        }
        *EPISODIC_LOADED.lock().expect("lock") = false;

        let trace_dir = dir.path().join("traces");
        std::fs::create_dir_all(&trace_dir).expect("mkdir");
        std::fs::write(trace_dir.join("mission.trace"), b"trace").expect("write");

        let added = index_replay_traces_for_entity("robot-ep", dir.path());
        assert!(added >= 1);
        let refs = episodic_refs_for_entity("robot-ep");
        assert!(refs.iter().any(|r| r.contains("mission.trace")));
        assert!(store.is_file());

        std::env::remove_var("SPANDA_EPISODIC_STORE_FILE");
    }
}
