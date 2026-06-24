//! Test-only helpers shared across crate modules.

use std::sync::{Mutex, MutexGuard};

static ENV_LOCK: Mutex<()> = Mutex::new(());

/// Serialize tests that mutate process environment variables.
pub fn env_lock() -> MutexGuard<'static, ()> {
    // Description:
    //     Env lock.
    //
    // Inputs:
    //     None.
    //
    // Outputs:
    //     result: MutexGuard<'static, ()>
    //         Return value from `env_lock`.
    //
    // Example:

    //     let result = spanda_package::testing::env_lock();

    ENV_LOCK
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}
