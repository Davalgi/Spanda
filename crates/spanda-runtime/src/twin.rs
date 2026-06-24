//! twin support for Spanda.
//!
use crate::value::RuntimeValue;
use std::collections::HashMap;

/// Shadow state for a digital twin with optional replay buffer.
pub struct TwinRuntime {
    pub name: String,
    pub mirrors: Vec<String>,
    pub replay: bool,
    pub telemetry_sync: bool,
    pub faults_sync: bool,
    pub events_sync: bool,
    pub shadow: HashMap<String, RuntimeValue>,
    replay_buffer: Vec<HashMap<String, RuntimeValue>>,
}

impl TwinRuntime {
    pub fn new(name: String, mirrors: Vec<String>, replay: bool) -> Self {
        // Description:
        //     Construct a new instance.
        //
        // Inputs:
        //     name: String
        //         Caller-supplied name.
        //     irrors: Vec<String>
        //         Caller-supplied irrors.
        //     replay: bool
        //         Caller-supplied replay.
        //
        // Outputs:
        //     result: Self
        //         Return value from `new`.
        //
        // Example:
        //     let value = spanda_runtime::twin::new(name, irrors, replay);

        // Assemble the struct fields and return it.
        Self {
            name,
            mirrors,
            replay,
            telemetry_sync: false,
            faults_sync: false,
            events_sync: false,
            shadow: HashMap::new(),
            replay_buffer: Vec::new(),
        }
    }

    pub fn with_sync(mut self, telemetry: bool, replay: bool, faults: bool, events: bool) -> Self {
        // Description:
        //     With sync.
        //
        // Inputs:
        //     mut self: input value
        //         Caller-supplied mut self.
        //     elemetry: bool
        //         Caller-supplied elemetry.
        //     replay: bool
        //         Caller-supplied replay.
        //     faults: bool
        //         Caller-supplied faults.
        //     events: bool
        //         Caller-supplied events.
        //
        // Outputs:
        //     result: Self
        //         Return value from `with_sync`.
        //
        // Example:
        //     let result = spanda_runtime::twin::with_sync(mut self, elemetry, replay, faults, events);

        // Call telemetry sync = telemetry; on the current instance.
        self.telemetry_sync = telemetry;

        // Take this path when replay.
        if replay {
            self.replay = true;
        }
        self.faults_sync = faults;
        self.events_sync = events;

        // Take this path when telemetry.
        if telemetry {
            // Check each struct field.
            for field in ["pose", "velocity"] {
                // Take the branch when any equals field).
                if !self.mirrors.iter().any(|m| m == field) {
                    self.mirrors.push(field.to_string());
                }
            }
        }
        self
    }

    pub fn snapshot(&mut self, field: &str, value: RuntimeValue) {
        // Description:
        //     Snapshot.
        //
        // Inputs:
        //     &mut self: input value
        //         Caller-supplied &mut self.
        //     field: &str
        //         Caller-supplied field.
        //     value: RuntimeValue
        //         Caller-supplied value.
        //
        // Outputs:
        //     None.
        //
        // Example:
        //     let result = spanda_runtime::twin::snapshot(&mut self, field, value);

        // take the branch when any equals field).
        if self.mirrors.iter().any(|m| m == field) {
            self.shadow.insert(field.to_string(), value);
        }
    }

    pub fn commit_frame(&mut self) {
        // Description:
        //     Commit frame.
        //
        // Inputs:
        //     &mut self: input value
        //         Caller-supplied &mut self.
        //
        // Outputs:
        //     None.
        //
        // Example:
        //     let result = spanda_runtime::twin::commit_frame(&mut self);

        // skip further work when shadow is empty.
        if self.replay && !self.shadow.is_empty() {
            self.replay_buffer.push(self.shadow.clone());
        }
    }

    pub fn replay_frame_count(&self) -> usize {
        // Description:
        //     Replay frame count.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //
        // Outputs:
        //     result: usize
        //         Return value from `replay_frame_count`.
        //
        // Example:
        //     let result = spanda_runtime::twin::replay_frame_count(&self);

        // Call len on the current instance.
        self.replay_buffer.len()
    }

    pub fn shadow_field(&self, field: &str) -> Option<&RuntimeValue> {
        // Description:
        //     Shadow field.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //     field: &str
        //         Caller-supplied field.
        //
        // Outputs:
        //     result: Option<&RuntimeValue>
        //         Return value from `shadow_field`.
        //
        // Example:
        //     let result = spanda_runtime::twin::shadow_field(&self, field);

        // take the branch when any equals field).
        if self.mirrors.iter().any(|m| m == field) {
            self.shadow.get(field)
        } else {
            None
        }
    }

    pub fn replay_field(&self, index: usize, field: &str) -> Option<&RuntimeValue> {
        // Description:
        //     Replay field.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //     index: usize
        //         Caller-supplied index.
        //     field: &str
        //         Caller-supplied field.
        //
        // Outputs:
        //     result: Option<&RuntimeValue>
        //         Return value from `replay_field`.
        //
        // Example:
        //     let result = spanda_runtime::twin::replay_field(&self, index, field);

        // take the branch when any equals field).
        if !self.replay || !self.mirrors.iter().any(|m| m == field) {
            return None;
        }
        self.replay_buffer.get(index)?.get(field)
    }

    /// Compare previous shadow against live mirrored values; true when divergence exceeds threshold.
    pub fn detect_divergence(&self, live: &HashMap<String, RuntimeValue>, threshold: f64) -> bool {
        // Description:
        //     Detect divergence.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //     live: &HashMap<String, RuntimeValue>
        //         Caller-supplied live.
        //     hreshold: f64
        //         Caller-supplied hreshold.
        //
        // Outputs:
        //     result: bool
        //         Return value from `detect_divergence`.
        //
        // Example:
        //     let result = spanda_runtime::twin::detect_divergence(&self, live, hreshold);

        // Check each struct field.
        for field in &self.mirrors {
            let Some(shadow_val) = self.shadow.get(field) else {
                continue;
            };
            let Some(live_val) = live.get(field) else {
                continue;
            };

            // Take this path when value distance(shadow val, live val) > threshold.
            if value_distance(shadow_val, live_val) > threshold {
                return true;
            }
        }
        false
    }

    pub fn live_mirrored_fields(
        pose: (f64, f64, f64, f64),
        velocity: (f64, f64),
        mirrors: &[String],
    ) -> HashMap<String, RuntimeValue> {
        // Description:

        //     Live mirrored fields.

        //

        // Inputs:

        //     pose: (f64, f64, f64, f64)

        //         Caller-supplied pose.

        //     velocity: (f64, f64)

        //         Caller-supplied velocity.

        //     irrors: &[String]

        //         Caller-supplied irrors.

        //

        // Outputs:

        //     result: HashMap<String, RuntimeValue>

        //         Return value from `live_mirrored_fields`.

        //

        // Example:

        //     let result = spanda_runtime::twin::live_mirrored_fields(pose, velocity, irrors);
        let mut live = HashMap::new();

        // Take the branch when any equals "pose").
        if mirrors.iter().any(|m| m == "pose") {
            live.insert(
                "pose".into(),
                RuntimeValue::Pose {
                    x: pose.0,
                    y: pose.1,
                    theta: pose.2,
                    z: pose.3,
                },
            );
        }

        // Take the branch when any equals "velocity").
        if mirrors.iter().any(|m| m == "velocity") {
            live.insert(
                "velocity".into(),
                RuntimeValue::Velocity {
                    linear: velocity.0,
                    angular: velocity.1,
                },
            );
        }
        live
    }
}

fn value_distance(a: &RuntimeValue, b: &RuntimeValue) -> f64 {
    // Description:
    //     Value distance.
    //
    // Inputs:
    //     a: &RuntimeValue
    //         Caller-supplied a.
    //     b: &RuntimeValue
    //         Caller-supplied b.
    //
    // Outputs:
    //     result: f64
    //         Return value from `value_distance`.
    //
    // Example:
    //     let result = spanda_runtime::twin::value_distance(a, b);

    // Match on value and handle each case.
    match (a, b) {
        (
            RuntimeValue::Pose {
                x: x1,
                y: y1,
                theta: _,
                z: z1,
            },
            RuntimeValue::Pose {
                x: x2,
                y: y2,
                theta: _,
                z: z2,
            },
        ) => {
            let dx = x1 - x2;
            let dy = y1 - y2;
            let dz = z1 - z2;
            (dx * dx + dy * dy + dz * dz).sqrt()
        }
        (
            RuntimeValue::Velocity {
                linear: l1,
                angular: a1,
            },
            RuntimeValue::Velocity {
                linear: l2,
                angular: a2,
            },
        ) => {
            let dl = l1 - l2;
            let da = a1 - a2;
            (dl * dl + da * da).sqrt()
        }
        (RuntimeValue::Number { value: v1, .. }, RuntimeValue::Number { value: v2, .. }) => {
            (v1 - v2).abs()
        }
        _ => 0.0,
    }
}

impl TwinRuntime {
    pub fn export_replay_json(&self) -> serde_json::Value {
        // Description:
        //     Export replay json.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //
        // Outputs:
        //     result: serde_json::Value
        //         Return value from `export_replay_json`.
        //
        // Example:

        //     let result = spanda_runtime::twin::export_replay_json(&self);

        use crate::serialize::runtime_to_json;
        let frames: Vec<serde_json::Value> = self
            .replay_buffer
            .iter()
            .enumerate()
            .map(|(index, frame)| {
                let fields: serde_json::Map<String, serde_json::Value> = frame
                    .iter()
                    .map(|(key, value)| (key.clone(), runtime_to_json(value)))
                    .collect();
                serde_json::json!({ "frame": index, "fields": fields })
            })
            .collect();
        serde_json::json!({
            "twin": self.name,
            "mirrors": self.mirrors,
            "frame_count": self.replay_frame_count(),
            "frames": frames,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::value::RuntimeValue;

    #[test]
    fn replay_field_returns_historical_snapshot() {
        // Description:
        //     Replay field returns historical snapshot.
        //
        // Inputs:
        //     None.
        //
        // Outputs:
        //     None.
        //
        // Example:

        //     let result = spanda_runtime::twin::replay_field_returns_historical_snapshot();

        let mut twin = TwinRuntime::new("T".into(), vec!["pose".into()], true);
        twin.snapshot(
            "pose",
            RuntimeValue::Pose {
                x: 1.0,
                y: 2.0,
                theta: 0.0,
                z: 0.0,
            },
        );
        twin.commit_frame();
        twin.snapshot(
            "pose",
            RuntimeValue::Pose {
                x: 3.0,
                y: 4.0,
                theta: 0.0,
                z: 0.0,
            },
        );
        twin.commit_frame();
        assert_eq!(twin.replay_frame_count(), 2);
        let export = twin.export_replay_json();
        assert_eq!(export["frame_count"], 2);
        assert!(export["frames"].as_array().unwrap().len() == 2);
        let first = twin.replay_field(0, "pose").unwrap();
        assert_eq!(
            first,
            &RuntimeValue::Pose {
                x: 1.0,
                y: 2.0,
                theta: 0.0,
                z: 0.0,
            }
        );
    }
}
