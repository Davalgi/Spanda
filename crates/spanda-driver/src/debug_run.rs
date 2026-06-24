//! Debugger run helper built on the debug session machine.
//!
use spanda_debug::{DebugOptions, DebugSession};
use spanda_error::SpandaError;

use crate::debug_session::{DebugMachine, DebugStepKind};

pub fn run_debug(source: &str, options: DebugOptions) -> Result<DebugSession, SpandaError> {
    // Description:
    //     Run debug.
    //
    // Inputs:
    //     source: &str
    //         Caller-supplied source.
    //     options: DebugOptions
    //         Caller-supplied options.
    //
    // Outputs:
    //     result: Result<DebugSession, SpandaError>
    //         Return value from `run_debug`.
    //
    // Example:

    //     let result = spanda_driver::debug_run::run_debug(source, options);

    let step = if options.step {
        DebugStepKind::StepOver
    } else {
        DebugStepKind::Continue
    };
    let mut machine = DebugMachine::start(source, options)?;
    machine.run_until_pause(step)
}
