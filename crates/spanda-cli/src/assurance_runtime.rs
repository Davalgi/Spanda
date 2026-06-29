//! CLI-injected assurance runtime wiring full assurance into the interpreter.
//!
use spanda_assurance::AssuranceBackedRuntime;
use spanda_runtime::assurance_runtime::SharedAssuranceRuntime;

/// Shared assurance runtime for default `spanda` CLI runs.
pub fn default_assurance_runtime() -> SharedAssuranceRuntime {
    std::sync::Arc::new(AssuranceBackedRuntime)
}
