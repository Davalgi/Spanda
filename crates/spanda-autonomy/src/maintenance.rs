//! Maintenance and sleep mode — low-risk operational windows.
//!
use std::path::PathBuf;
use std::sync::{LazyLock, Mutex};

use serde::{Deserialize, Serialize};

/// Scheduled maintenance window.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MaintenanceWindow {
    pub id: String,
    pub start: String,
    pub end: String,
    pub activities: Vec<String>,
}

/// Sleep mode — minimal activity state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SleepMode {
    Active,
    LowActivity,
    Sleep,
    Maintenance,
}

/// Low-activity mode descriptor.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LowActivityMode {
    pub entity_id: String,
    pub mode: SleepMode,
    pub allowed_tasks: Vec<String>,
}

/// Scheduled recovery during maintenance window.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScheduledRecovery {
    pub entity_id: String,
    pub window_id: String,
    pub playbook_id: String,
}

/// Calibration window for sensors and actuators.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CalibrationWindow {
    pub entity_id: String,
    pub sensors: Vec<String>,
    pub scheduled_at: String,
}

/// Firmware or model update window.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UpdateWindow {
    pub entity_id: String,
    pub update_kind: String,
    pub scheduled_at: String,
}

static WINDOW_STORE: LazyLock<Mutex<Vec<MaintenanceWindow>>> =
    LazyLock::new(|| Mutex::new(Vec::new()));

static WINDOW_LOADED: LazyLock<Mutex<bool>> = LazyLock::new(|| Mutex::new(false));

/// Default path for maintenance windows (`SPANDA_MAINTENANCE_WINDOW_FILE` overrides).
pub fn default_maintenance_window_path() -> PathBuf {
    std::env::var("SPANDA_MAINTENANCE_WINDOW_FILE")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from(".spanda/maintenance-windows.json"))
}

fn ensure_windows_loaded() {
    let mut loaded = WINDOW_LOADED
        .lock()
        .expect("maintenance window loaded lock poisoned");
    if *loaded {
        return;
    }
    load_maintenance_windows_from_disk();
    *loaded = true;
}

/// Load maintenance windows from disk.
pub fn load_maintenance_windows_from_disk() {
    // Hydrate scheduled maintenance windows from the store file.
    //
    // Parameters:
    // None.
    //
    // Returns:
    // Nothing; skips missing or invalid files.
    //
    // Options:
    // Path from `SPANDA_MAINTENANCE_WINDOW_FILE` or `.spanda/maintenance-windows.json`.
    //
    // Example:
    // load_maintenance_windows_from_disk();

    let path = default_maintenance_window_path();
    if !path.is_file() {
        return;
    }
    let Ok(content) = std::fs::read_to_string(&path) else {
        return;
    };
    let Ok(windows) = serde_json::from_str::<Vec<MaintenanceWindow>>(&content) else {
        return;
    };
    let mut store = WINDOW_STORE
        .lock()
        .expect("maintenance window store lock poisoned");
    *store = windows;
}

/// Persist maintenance windows to disk.
pub fn persist_maintenance_windows_to_disk() -> Result<(), String> {
    // Write the current maintenance window schedule to disk.
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
    // persist_maintenance_windows_to_disk()?;

    let path = default_maintenance_window_path();
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| format!("create {}: {e}", parent.display()))?;
    }
    let windows = list_maintenance_windows();
    let body = serde_json::to_string_pretty(&windows).map_err(|e| e.to_string())?;
    std::fs::write(&path, body).map_err(|e| format!("write {}: {e}", path.display()))
}

/// List all scheduled maintenance windows.
pub fn list_maintenance_windows() -> Vec<MaintenanceWindow> {
    // Return the current maintenance schedule.
    //
    // Parameters:
    // None.
    //
    // Returns:
    // Window descriptors in store order.
    //
    // Options:
    // None.
    //
    // Example:
    // let windows = list_maintenance_windows();

    ensure_windows_loaded();
    WINDOW_STORE
        .lock()
        .expect("maintenance window store lock poisoned")
        .clone()
}

/// Insert or replace a maintenance window by id and persist.
pub fn set_maintenance_window(window: MaintenanceWindow) -> MaintenanceWindow {
    // Upsert a maintenance window into the schedule store.
    //
    // Parameters:
    // - `window` — window to insert or replace by `id`
    //
    // Returns:
    // The stored window.
    //
    // Options:
    // None.
    //
    // Example:
    // let saved = set_maintenance_window(window);

    ensure_windows_loaded();
    let mut store = WINDOW_STORE
        .lock()
        .expect("maintenance window store lock poisoned");
    if let Some(slot) = store.iter_mut().find(|w| w.id == window.id) {
        *slot = window.clone();
    } else {
        store.push(window.clone());
    }
    drop(store);
    let _ = persist_maintenance_windows_to_disk();
    window
}
