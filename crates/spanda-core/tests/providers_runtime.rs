//! Runtime provider registry wiring tests.
//!
use spanda_core::runtime::{Interpreter, InterpreterOptions};
use spanda_core::simulator::{create_default_simulator, SimulatorConfig};
use spanda_providers::ProviderBackedRuntime;
use std::sync::Arc;

#[test]
fn interpreter_bootstraps_provider_registry_by_default() {
    // Description:
    //     Interpreter bootstraps provider registry by default.
    //
    // Inputs:
    //     None.
    //
    // Outputs:
    //     None.
    //
    // Example:

    //     let result = spanda_core::providers_runtime::interpreter_bootstraps_provider_registry_by_default();

    let sim = create_default_simulator(SimulatorConfig::default());
    let interp = Interpreter::new(
        sim,
        InterpreterOptions {
            provider_runtime: Some(Arc::new(ProviderBackedRuntime)),
            ..Default::default()
        },
    );
    assert!(interp.provider_registry().transport_count() >= 2);
}
