//! Deterministic mission trace recording and replay for simulation runs.
//!
use crate::error::RuntimeError;
use crate::path_util::normalize_trace_source;
use crate::robot_state::{PoseState, VelocityState};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

/// Robot state captured for frame-by-frame playback without re-running program logic.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReplayStateSnapshot {
    pub pose: PoseState,
    pub velocity: VelocityState,
    pub emergency_stop: bool,
    #[serde(default)]
    pub active_mode: Option<String>,
}

/// One recorded simulation frame for deterministic replay.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TraceFrame {
    pub sim_time_ms: f64,
    pub event: String,
    #[serde(default)]
    pub payload: serde_json::Value,
    #[serde(default)]
    pub state: Option<ReplayStateSnapshot>,
}

/// Full mission trace file format.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MissionTrace {
    pub version: u32,
    pub source: String,
    #[serde(default)]
    pub deterministic: bool,
    pub frames: Vec<TraceFrame>,
}

impl MissionTrace {
    pub fn new(source: impl Into<String>) -> Self {
        // Description:
        //     Construct a new instance.
        //
        // Inputs:
        //     source: impl Into<String>
        //         Caller-supplied source.
        //
        // Outputs:
        //     result: Self
        //         Return value from `new`.
        //
        // Example:
        //     let value = spanda_runtime::replay::new(source);

        // Initialize metadata with an empty frame list.
        Self {
            version: 1,
            source: source.into(),
            deterministic: true,
            frames: Vec::new(),
        }
    }

    pub fn record(
        &mut self,
        sim_time_ms: f64,
        event: impl Into<String>,
        payload: serde_json::Value,
    ) {
        // Description:
        //     Record.
        //
        // Inputs:
        //     &mut self: input value
        //         Caller-supplied &mut self.
        //     sim_time_ms: f64
        //         Caller-supplied sim time ms.
        //     even: impl Into<String>
        //         Caller-supplied even.
        //     payload: serde_json::Value
        //         Caller-supplied payload.
        //
        // Outputs:
        //     None.
        //
        // Example:
        //     let result = spanda_runtime::replay::record(&mut self, sim_time_ms, even, payload);

        // Push the frame in arrival order for deterministic playback.
        self.record_with_state(sim_time_ms, event, payload, None);
    }

    pub fn record_with_state(
        &mut self,
        sim_time_ms: f64,
        event: impl Into<String>,
        payload: serde_json::Value,
        state: Option<ReplayStateSnapshot>,
    ) {
        // Description:
        //     Record with state.
        //
        // Inputs:
        //     &mut self: input value
        //         Caller-supplied &mut self.
        //     sim_time_ms: f64
        //         Caller-supplied sim time ms.
        //     even: impl Into<String>
        //         Caller-supplied even.
        //     payload: serde_json::Value
        //         Caller-supplied payload.
        //     state: Option<ReplayStateSnapshot>
        //         Caller-supplied state.
        //
        // Outputs:
        //     None.
        //
        // Example:

        //     let result = spanda_runtime::replay::record_with_state(&mut self, sim_time_ms, even, payload, state);

        self.frames.push(TraceFrame {
            sim_time_ms,
            event: event.into(),
            payload,
            state,
        });
        if self.frames.iter().any(|f| f.state.is_some()) {
            self.version = 2;
        }
    }

    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<(), RuntimeError> {
        // Description:
        //     Save.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //     path: P
        //         Caller-supplied path.
        //
        // Outputs:
        //     result: Result<(), RuntimeError>
        //         Return value from `save`.
        //
        // Example:
        //     let result = spanda_runtime::replay::save(&self, path);

        // Rewrite absolute source labels so committed traces stay machine-portable.
        let mut portable = self.clone();
        portable.source = normalize_trace_source(path.as_ref(), &self.source);

        // Encode as pretty JSON for human inspection and tooling.
        let json = serde_json::to_string_pretty(&portable)
            .map_err(|err| RuntimeError::new(format!("Failed to encode trace: {err}"), 0))?;
        fs::write(path, json)
            .map_err(|err| RuntimeError::new(format!("Failed to write trace file: {err}"), 0))
    }

    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, RuntimeError> {
        // Description:
        //     Load.
        //
        // Inputs:
        //     path: P
        //         Caller-supplied path.
        //
        // Outputs:
        //     result: Result<Self, RuntimeError>
        //         Return value from `load`.
        //
        // Example:
        //     let result = spanda_runtime::replay::load(path);

        // Read and decode the JSON trace file.
        let text = fs::read_to_string(path.as_ref())
            .map_err(|err| RuntimeError::new(format!("Failed to read trace file: {err}"), 0))?;
        serde_json::from_str(&text)
            .map_err(|err| RuntimeError::new(format!("Invalid trace file: {err}"), 0))
    }

    pub fn frames_from(&self, offset_ms: f64) -> &[TraceFrame] {
        // Description:
        //     Frames from.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //     offset_ms: f64
        //         Caller-supplied offset ms.
        //
        // Outputs:
        //     result: &[TraceFrame]
        //         Return value from `frames_from`.
        //
        // Example:
        //     let result = spanda_runtime::replay::frames_from(&self, offset_ms);

        // Find the first frame at or after the offset timestamp.
        let idx = self
            .frames
            .iter()
            .position(|frame| frame.sim_time_ms >= offset_ms)
            .unwrap_or(self.frames.len());
        &self.frames[idx..]
    }
}

pub fn parse_replay_offset(raw: &str) -> Result<f64, RuntimeError> {
    // Description:
    //     Parse replay offset.
    //
    // Inputs:
    //     raw: &str
    //         Caller-supplied raw.
    //
    // Outputs:
    //     result: Result<f64, RuntimeError>
    //         Return value from `parse_replay_offset`.
    //
    // Example:
    //     let result = spanda_runtime::replay::parse_replay_offset(raw);

    // Accept plain millisecond values directly.
    if let Ok(ms) = raw.parse::<f64>() {
        return Ok(ms);
    }

    // Parse `T+mm:ss` or `T+hh:mm:ss` formatted offsets.
    let value = raw.strip_prefix("T+").ok_or_else(|| {
        RuntimeError::new(
            format!("Invalid replay offset '{raw}'; expected T+mm:ss or milliseconds"),
            0,
        )
    })?;
    let parts: Vec<&str> = value.split(':').collect();
    let total_secs = match parts.as_slice() {
        [mins, secs] => {
            mins.parse::<f64>().unwrap_or(0.0) * 60.0 + secs.parse::<f64>().unwrap_or(0.0)
        }
        [hours, mins, secs] => {
            hours.parse::<f64>().unwrap_or(0.0) * 3600.0
                + mins.parse::<f64>().unwrap_or(0.0) * 60.0
                + secs.parse::<f64>().unwrap_or(0.0)
        }
        _ => {
            return Err(RuntimeError::new(
                format!("Invalid replay offset '{raw}'; expected T+mm:ss"),
                0,
            ))
        }
    };
    Ok(total_secs * 1000.0)
}

/// Result of comparing an expected mission trace to a fresh recorded run.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TraceVerification {
    pub ok: bool,
    pub matched: usize,
    pub mismatches: Vec<String>,
}

pub fn verify_traces(
    expected: &MissionTrace,
    actual: &MissionTrace,
    from_ms: f64,
) -> TraceVerification {
    // Description:
    //     Verify traces.
    //
    // Inputs:
    //     expected: &MissionTrace
    //         Caller-supplied expected.
    //     actual: &MissionTrace
    //         Caller-supplied actual.
    //     from_ms: f64
    //         Caller-supplied from ms.
    //
    // Outputs:
    //     result: TraceVerification
    //         Return value from `verify_traces`.
    //
    // Example:
    //     let result = spanda_runtime::replay::verify_traces(expected, actual, from_ms);

    // Align both traces from the requested offset.
    let exp = expected.frames_from(from_ms);
    let act = actual.frames_from(from_ms);
    let mut mismatches = Vec::new();
    let shared = exp.len().min(act.len());
    for index in 0..shared {
        if exp[index].event != act[index].event {
            mismatches.push(format!(
                "frame {index}: expected event '{}', got '{}'",
                exp[index].event, act[index].event
            ));
        } else if (exp[index].sim_time_ms - act[index].sim_time_ms).abs() > 0.001 {
            mismatches.push(format!(
                "frame {index} event '{}': expected t={:.3}ms, got t={:.3}ms",
                exp[index].event, exp[index].sim_time_ms, act[index].sim_time_ms
            ));
        }
    }
    if exp.len() != act.len() {
        mismatches.push(format!(
            "frame count mismatch: expected {}, got {}",
            exp.len(),
            act.len()
        ));
    }
    TraceVerification {
        ok: mismatches.is_empty(),
        matched: shared,
        mismatches,
    }
}

/// Target that can receive replayed state snapshots during playback.
pub trait ReplayStateTarget {
    fn apply_replay_state(&mut self, snapshot: &ReplayStateSnapshot);
}

/// Summary of a frame-by-frame playback run.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlaybackReport {
    pub frames_applied: usize,
    pub states_applied: usize,
    pub events: Vec<String>,
}

pub fn playback_frames<T: ReplayStateTarget>(
    frames: &[TraceFrame],
    target: &mut T,
    wall_clock: bool,
) -> PlaybackReport {
    // Description:
    //     Playback frames.
    //
    // Inputs:
    //     frames: &[TraceFrame]
    //         Caller-supplied frames.
    //     arge: &mut T
    //         Caller-supplied arge.
    //     wall_clock: bool
    //         Caller-supplied wall clock.
    //
    // Outputs:
    //     result: PlaybackReport
    //         Return value from `playback_frames`.
    //
    // Example:

    //     let result = spanda_runtime::replay::playback_frames(frames, arge, wall_clock);

    let mut states_applied = 0usize;
    let mut events = Vec::new();
    let mut prev_sim_ms = 0.0;

    for frame in frames {
        if wall_clock {
            let delta_ms = frame.sim_time_ms - prev_sim_ms;
            if delta_ms > 0.0 {
                std::thread::sleep(std::time::Duration::from_secs_f64(delta_ms / 1000.0));
            }
            prev_sim_ms = frame.sim_time_ms;
        }
        if let Some(state) = &frame.state {
            target.apply_replay_state(state);
            states_applied += 1;
        }
        events.push(frame.event.clone());
    }

    PlaybackReport {
        frames_applied: frames.len(),
        states_applied,
        events,
    }
}
