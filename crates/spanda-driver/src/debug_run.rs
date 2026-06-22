//! Debugger run helper built on the debug session machine.
//!
use spanda_debug::{DebugOptions, DebugSession};
use spanda_error::SpandaError;

use crate::debug_session::{DebugMachine, DebugStepKind};

pub fn run_debug(source: &str, options: DebugOptions) -> Result<DebugSession, SpandaError> {
    // Start a debug session and run until the first pause or completion.
    //
    // Parameters:
    // - `source` — full `.sd` source text
    // - `options` — breakpoints and step flags
    //
    // Returns:
    // Debug session with recorded pauses.
    //
    // Options:
    // None.
    //
    // Example:
    // let session = run_debug(source, DebugOptions::default())?;

    let step = if options.step {
        DebugStepKind::StepOver
    } else {
        DebugStepKind::Continue
    };
    let mut machine = DebugMachine::start(source, options)?;
    machine.run_until_pause(step)
}
