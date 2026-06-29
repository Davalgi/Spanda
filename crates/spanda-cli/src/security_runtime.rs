//! CLI-injected security runtime wiring full security into the interpreter.
//!
use spanda_runtime::security_runtime::{SecurityRuntime, SecurityRuntimeFactory};
use spanda_security::SecurityBackedRuntime;

/// Security runtime factory for default `spanda` CLI runs.
pub fn default_security_runtime_factory() -> SecurityRuntimeFactory {
    || Box::new(SecurityBackedRuntime::new()) as Box<dyn SecurityRuntime>
}
