//! Mission trace replay and playback helpers.
//!
use spanda_error::SpandaError;
use spanda_interpreter::{
    create_default_simulator, RobotBackend, RunOptions, RunResult, SimulatorConfig,
};
use spanda_runtime::replay::{
    playback_frames, verify_traces, MissionTrace, PlaybackReport, TraceVerification,
};
use spanda_runtime::robot_state::RobotState;

use crate::run::run;

pub fn replay_mission(
    source: &str,
    trace_path: &str,
    mut options: RunOptions,
) -> Result<(RunResult, TraceVerification), SpandaError> {
    // Description:
    //     Replay mission.
    //
    // Inputs:
    //     source: &str
    //         Caller-supplied source.
    //     race_path: &str
    //         Caller-supplied race path.
    //     options: RunOptions
    //         Caller-supplied options.
    //
    // Outputs:
    //     result: Result<(RunResult, TraceVerification), SpandaError>
    //         Return value from `replay_mission`.
    //
    // Example:

    //     let result = spanda_driver::replay::replay_mission(source, race_path, options);

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
    // Description:
    //     Playback mission.
    //
    // Inputs:
    //     race_path: &str
    //         Caller-supplied race path.
    //     options: RunOptions
    //         Caller-supplied options.
    //
    // Outputs:
    //     result: Result<(PlaybackReport, RobotState), SpandaError>
    //         Return value from `playback_mission`.
    //
    // Example:

    //     let result = spanda_driver::replay::playback_mission(race_path, options);

    let trace = MissionTrace::load(trace_path)?;
    let from_ms = options.replay_from_ms.unwrap_or(0.0);
    let frames: Vec<_> = trace.frames_from(from_ms).to_vec();
    let mut sim = create_default_simulator(SimulatorConfig::default());
    let report = playback_frames(&frames, &mut sim, options.playback_wall_clock);
    Ok((report, sim.get_state()))
}
