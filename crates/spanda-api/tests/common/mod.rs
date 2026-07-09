//! Shared helpers for spanda-api integration tests.

use std::sync::{Mutex, MutexGuard};
use tempfile::TempDir;

static STATE_DIR_ENV_LOCK: Mutex<()> = Mutex::new(());

/// Holds a temp Control Center state directory and serializes env mutations across tests.
pub struct TempStateDirGuard {
    _dir: TempDir,
    _lock: MutexGuard<'static, ()>,
}

impl TempStateDirGuard {
    pub fn new() -> Self {
        let lock = STATE_DIR_ENV_LOCK
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        let dir = TempDir::new().expect("tempdir");
        std::env::set_var(
            "SPANDA_CONTROL_CENTER_STATE_DIR",
            dir.path().to_string_lossy().to_string(),
        );
        Self {
            _dir: dir,
            _lock: lock,
        }
    }

    pub fn path(&self) -> &std::path::Path {
        self._dir.path()
    }
}

impl Drop for TempStateDirGuard {
    fn drop(&mut self) {
        std::env::remove_var("SPANDA_CONTROL_CENTER_STATE_DIR");
    }
}
