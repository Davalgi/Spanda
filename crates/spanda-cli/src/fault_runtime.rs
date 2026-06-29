//! CLI-injected fault runtime wiring runtime-faults into the interpreter.
//!
use spanda_runtime::fault_runtime::SharedFaultRuntime;
use spanda_runtime_faults::FaultBackedRuntime;

/// Shared fault runtime for default `spanda` CLI runs.
pub fn default_fault_runtime() -> SharedFaultRuntime {
    std::sync::Arc::new(FaultBackedRuntime)
}
