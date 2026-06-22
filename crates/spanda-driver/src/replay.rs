//! Mission trace replay and playback helpers.
//!
use spanda_error::SpandaError;
use spanda_interpreter::{
    create_default_simulator, RobotBackend, RunOptions, RunResult, SimulatorConfig,
};
use spanda_runtime::replay::{playback_frames, verify_traces, MissionTrace, PlaybackReport, TraceVerification};
use spanda_runtime::robot_state::RobotState;

use crate::run::run;

pub fn replay_mission(
    source: &str,
    trace_path: &str,
    mut options: RunOptions,
) -> Result<(RunResult, TraceVerification), SpandaError> {
    // Re-run a program and verify the recorded mission trace matches a reference trace.
    //
    // Parameters:
    // - `source` — `.sd` program source text
    // - `trace_path` — reference `.trace` file path
    // - `options` — run options; `replay_from_ms` selects comparison offset
    //
    // Returns:
    // Run result plus deterministic trace verification report.
    //
    // Options:
    // None.
    //
    // Example:
    // let (result, report) = replay_mission(source, "mission.trace", RunOptions::default())?;

    let expected = MissionTrace::load(trace_path)?;
    let from_ms = options.replay_from_ms.unwrap_or(0.0);
    options.record_trace = true;
    options.replay_deterministic = true;
    if options.trace_source.is_none() {
        options.trace_source = Some(expected.source.clone());
    }
    let result = run(source, options)?;
    let actual = result
        .mission_trace
        .as_ref()
        .ok_or_else(|| SpandaError::Runtime {
            message: "Replay run did not produce a mission trace".into(),
            line: 0,
        })?;
    let verification = verify_traces(&expected, actual, from_ms);
    Ok((result, verification))
}

pub fn playback_mission(
    trace_path: &str,
    options: RunOptions,
) -> Result<(PlaybackReport, RobotState), SpandaError> {
    // Play back recorded mission frames without re-executing program logic.
    //
    // Parameters:
    // - `trace_path` — input `.trace` file
    // - `options` — playback offset and wall-clock pacing options
    //
    // Returns:
    // Playback report and final robot state after applying snapshots.
    //
    // Options:
    // None.
    //
    // Example:
    // let (report, state) = playback_mission("mission.trace", RunOptions::default())?;

    let trace = MissionTrace::load(trace_path)?;
    let from_ms = options.replay_from_ms.unwrap_or(0.0);
    let frames: Vec<_> = trace.frames_from(from_ms).to_vec();
    let mut sim = create_default_simulator(SimulatorConfig::default());
    let report = playback_frames(&frames, &mut sim, options.playback_wall_clock);
    Ok((report, sim.get_state()))
}
