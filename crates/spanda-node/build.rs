//! build support for Spanda.
//!
extern crate napi_build;

fn main() {
    // Description:
    //     Main.
    //
    // Inputs:
    //     None.
    //
    // Outputs:
    //     None.
    //
    // Example:
    //     let result = spanda_node::build::main();

    // Produce setup as the result.
    napi_build::setup();
}
