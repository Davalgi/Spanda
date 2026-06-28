//! Evaluate readiness for a Spanda program via the official SDK.
//!
//! Run Control Center first:
//!   spanda control-center serve --config examples/robotics --program examples/robotics/rover.sd
//!
//! Then:
//!   cargo run --example readiness -p spanda-sdk

use spanda_sdk::SpandaClient;

fn main() {
    let client = SpandaClient::local();
    match client.readiness("examples/robotics/rover.sd") {
        Ok(report) => {
            if let Some(score) = report.score {
                println!("Readiness score: {score}");
            } else {
                println!("Readiness: {report:?}");
            }
        }
        Err(err) => eprintln!("readiness failed: {err}"),
    }
}
