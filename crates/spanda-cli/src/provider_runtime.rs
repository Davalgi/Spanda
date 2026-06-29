//! CLI-injected provider runtime wiring official packages into the interpreter.
//!
use spanda_providers::ProviderBackedRuntime;
use spanda_runtime::provider_runtime::SharedProviderRuntime;

/// Shared provider runtime for default `spanda` CLI runs.
pub fn default_provider_runtime() -> SharedProviderRuntime {
    std::sync::Arc::new(ProviderBackedRuntime)
}
