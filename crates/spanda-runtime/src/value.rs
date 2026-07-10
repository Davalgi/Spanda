//! Interpreter runtime value representations.
//!
use crate::robotics::{FleetRegistry, MissionRuntime};
use spanda_ast::nodes::UnitKind;
use spanda_ast::{CaptureResult, RegexPattern};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct PoseValue {
    pub x: f64,
    pub y: f64,
    pub theta: f64,
    pub z: f64,
}

impl Default for PoseValue {
    fn default() -> Self {
        // Description:
        //     Provide the default value for this type.
        //
        // Inputs:
        //     None.
        //
        // Outputs:
        //     result: Self
        //         Return value from `default`.
        //
        // Example:
        //     let result = spanda_runtime::value::default();

        // Assemble the struct fields and return it.
        Self {
            x: 0.0,
            y: 0.0,
            theta: 0.0,
            z: 0.0,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum RuntimeValue {
    Number {
        value: f64,
        unit: UnitKind,
    },
    Bool {
        value: bool,
    },
    String {
        value: String,
    },
    Regex {
        pattern: RegexPattern,
    },
    Capture {
        result: CaptureResult,
    },
    Void,
    Scan {
        nearest_distance: f64,
    },
    Pose {
        x: f64,
        y: f64,
        theta: f64,
        z: f64,
    },
    Velocity {
        linear: f64,
        angular: f64,
    },
    Trajectory {
        waypoints: Vec<PoseValue>,
    },
    Transform {
        from_frame: String,
        to_frame: String,
        pose: PoseValue,
    },
    Object {
        type_name: String,
        fields: HashMap<String, RuntimeValue>,
    },
    Enum {
        enum_name: String,
        variant: String,
        payloads: Vec<RuntimeValue>,
    },
    Sensor {
        name: String,
        sensor_type: String,
        library: Option<String>,
        hal_binding: Option<String>,
        topic: Option<String>,
    },
    Actuator {
        name: String,
        actuator_type: String,
    },
    Topic {
        name: String,
        message_type: String,
        topic_path: String,
    },
    Service {
        name: String,
        service_type: String,
    },
    Action {
        name: String,
        action_type: String,
    },
    Robot,
    Agent {
        name: String,
    },
    TraitObject {
        trait_name: String,
        agent: String,
    },
    Twin {
        name: String,
    },
    SafetyCtx,
    AuditCtx,
    LedgerCtx,
    WorldModelCtx,
    Identity {
        id: String,
        public_key: String,
    },
    Secret {
        name: String,
    },
    AiModel {
        name: String,
        model_type: String,
        provider: String,
    },
    ActionProposal {
        linear: f64,
        angular: f64,
        source: String,
        trace: Vec<String>,
    },
    SafeAction {
        linear: f64,
        angular: f64,
    },
    Goal {
        text: String,
    },
    SensorFusion {
        sensors: Vec<String>,
        estimator: Option<String>,

        /// Full fusion input paths from `state_estimator` (e.g. `gps.fix`); empty for `observe { }`.
        fusion_inputs: Vec<String>,
    },
    MissionControl {
        runtime: MissionRuntime,
    },
    NavigationControl {
        goal: Option<String>,
    },
    SlamControl,
    FleetControl {
        registry: FleetRegistry,
    },
    Completion {
        text: String,
        model: Option<String>,
    },
    Embedding {
        dimensions: usize,
        vector: Vec<f64>,
    },
    Result {
        ok: bool,
        value: Box<RuntimeValue>,
    },
    Option {
        present: bool,
        value: Option<Box<RuntimeValue>>,
    },
    Bytes {
        data: Vec<u8>,
    },
    Null,
    Future {
        func_name: String,
        args: Vec<RuntimeValue>,
        resolved: Option<Box<RuntimeValue>>,
    },
    TaskHandle {
        id: u64,
    },
    Channel {
        id: u64,
    },
}

impl RuntimeValue {
    pub fn number(value: f64, unit: UnitKind) -> Self {
        // Description:
        //     Number.
        //
        // Inputs:
        //     value: f64
        //         Caller-supplied value.
        //     ni: UnitKind
        //         Caller-supplied ni.
        //
        // Outputs:
        //     result: Self
        //         Return value from `number`.
        //
        // Example:
        //     let result = spanda_runtime::value::number(value, ni);

        // Build a Number runtime value.
        RuntimeValue::Number { value, unit }
    }

    pub fn string(value: impl Into<String>) -> Self {
        // Description:
        //     String.
        //
        // Inputs:
        //     value: impl Into<String>
        //         Caller-supplied value.
        //
        // Outputs:
        //     result: Self
        //         Return value from `string`.
        //
        // Example:
        //     let result = spanda_runtime::value::string(value);

        // Build a String runtime value.
        RuntimeValue::String {
            value: value.into(),
        }
    }

    pub fn object(type_name: impl Into<String>, fields: HashMap<String, RuntimeValue>) -> Self {
        // Description:
        //     Object.
        //
        // Inputs:
        //     ype_name: impl Into<String>
        //         Caller-supplied ype name.
        //     fields: HashMap<String, RuntimeValue>
        //         Caller-supplied fields.
        //
        // Outputs:
        //     result: Self
        //         Return value from `object`.
        //
        // Example:
        //     let result = spanda_runtime::value::object(ype_name, fields);

        // Build a Object runtime value.
        RuntimeValue::Object {
            type_name: type_name.into(),
            fields,
        }
    }

    pub fn scan(nearest_distance: f64) -> Self {
        // Description:
        //     Scan.
        //
        // Inputs:
        //     nearest_distance: f64
        //         Caller-supplied nearest distance.
        //
        // Outputs:
        //     result: Self
        //         Return value from `scan`.
        //
        // Example:
        //     let result = spanda_runtime::value::scan(nearest_distance);

        // Build a Scan runtime value.
        RuntimeValue::Scan { nearest_distance }
    }

    pub fn as_number(&self) -> Option<f64> {
        // Description:
        //     As number.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //
        // Outputs:
        //     result: Option<f64>
        //         Return value from `as_number`.
        //
        // Example:
        //     let result = spanda_runtime::value::as_number(&self);

        // Dispatch based on the enum variant or current state.
        match self {
            RuntimeValue::Number { value, .. } => Some(*value),
            _ => None,
        }
    }

    pub fn as_string(&self) -> Option<&str> {
        // Description:
        //     As string.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //
        // Outputs:
        //     result: Option<&str>
        //         Return value from `as_string`.
        //
        // Example:
        //     let result = spanda_runtime::value::as_string(&self);

        // Dispatch based on the enum variant or current state.
        match self {
            RuntimeValue::String { value } => Some(value),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub enum MotionCommand {
    Drive {
        linear: f64,
        angular: f64,
        actuator: String,
    },
    Stop {
        actuator: String,
    },
    MoveTo {
        x: f64,
        y: f64,
        z: f64,
        actuator: String,
    },
    Follow {
        waypoints: Vec<PoseValue>,
        /// Requested cruise speed for trajectory tracking (m/s). Callers should pass the
        /// unclamped request; the interpreter re-clamps per pose/tick via the safety monitor.
        max_linear: f64,
        actuator: String,
    },
    Grip {
        actuator: String,
    },
    Release {
        actuator: String,
    },
    Open {
        actuator: String,
    },
    SetThrust {
        thrust: f64,
        actuator: String,
    },
    Hover {
        actuator: String,
    },
}
pub fn format_runtime_value(value: &RuntimeValue) -> String {
    // Description:
    //     Format runtime value.
    //
    // Inputs:
    //     value: &RuntimeValue
    //         Caller-supplied value.
    //
    // Outputs:
    //     result: String
    //         Return value from `format_runtime_value`.
    //
    // Example:
    //     let result = spanda_runtime::value::format_runtime_value(value);

    // Match on value and handle each case.
    match value {
        RuntimeValue::Number { value, unit } => {
            // Take the branch when *unit equals None.
            if *unit == UnitKind::None {
                value.to_string()
            } else {
                format!("{value} {}", unit.as_str())
            }
        }
        RuntimeValue::Bool { value } => value.to_string(),
        RuntimeValue::String { value } => value.clone(),
        RuntimeValue::Void => "void".into(),
        RuntimeValue::Enum {
            variant, payloads, ..
        } => {
            // Skip further work when payloads is empty.
            if payloads.is_empty() {
                variant.clone()
            } else {
                format!(
                    "{variant}({})",
                    payloads
                        .iter()
                        .map(format_runtime_value)
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            }
        }
        RuntimeValue::TraitObject { trait_name, agent } => {
            format!("dyn {trait_name}@{agent}")
        }
        RuntimeValue::Agent { name } => format!("agent {name}"),
        RuntimeValue::Object { type_name, fields } => format!(
            "{type_name} {{ {} }}",
            fields
                .iter()
                .map(|(k, v)| format!("{k}: {}", format_runtime_value(v)))
                .collect::<Vec<_>>()
                .join(", ")
        ),
        other => format!("{other:?}"),
    }
}
pub fn runtime_pose(x: f64, y: f64, theta: f64, z: f64) -> RuntimeValue {
    // Description:
    //     Runtime pose.
    //
    // Inputs:
    //     x: f64
    //         Caller-supplied x.
    //     y: f64
    //         Caller-supplied y.
    //     heta: f64
    //         Caller-supplied heta.
    //     z: f64
    //         Caller-supplied z.
    //
    // Outputs:
    //     result: RuntimeValue
    //         Return value from `runtime_pose`.
    //
    // Example:
    //     let result = spanda_runtime::value::runtime_pose(x, y, heta, z);

    // Build a Pose runtime value.
    RuntimeValue::Pose { x, y, theta, z }
}

pub fn runtime_velocity(linear: f64, angular: f64) -> RuntimeValue {
    // Description:
    //     Runtime velocity.
    //
    // Inputs:
    //     linear: f64
    //         Caller-supplied linear.
    //     angular: f64
    //         Caller-supplied angular.
    //
    // Outputs:
    //     result: RuntimeValue
    //         Return value from `runtime_velocity`.
    //
    // Example:
    //     let result = spanda_runtime::value::runtime_velocity(linear, angular);

    // Build a Velocity runtime value.
    RuntimeValue::Velocity { linear, angular }
}

pub fn runtime_trajectory(waypoints: Vec<PoseValue>) -> RuntimeValue {
    // Description:
    //     Runtime trajectory.
    //
    // Inputs:
    //     waypoints: Vec<PoseValue>
    //         Caller-supplied waypoints.
    //
    // Outputs:
    //     result: RuntimeValue
    //         Return value from `runtime_trajectory`.
    //
    // Example:
    //     let result = spanda_runtime::value::runtime_trajectory(waypoints);

    // Build a Trajectory runtime value.
    RuntimeValue::Trajectory { waypoints }
}
pub fn get_pose_fields(val: &RuntimeValue) -> Option<PoseValue> {
    // Description:
    //     Get pose fields.
    //
    // Inputs:
    //     val: &RuntimeValue
    //         Caller-supplied val.
    //
    // Outputs:
    //     result: Option<PoseValue>
    //         Return value from `get_pose_fields`.
    //
    // Example:
    //     let result = spanda_runtime::value::get_pose_fields(val);

    // Match on val and handle each case.
    match val {
        RuntimeValue::Pose { x, y, theta, z } => Some(PoseValue {
            x: *x,
            y: *y,
            theta: *theta,
            z: *z,
        }),
        _ => None,
    }
}

pub fn get_velocity_fields(val: &RuntimeValue) -> Option<(f64, f64)> {
    // Description:
    //     Get velocity fields.
    //
    // Inputs:
    //     val: &RuntimeValue
    //         Caller-supplied val.
    //
    // Outputs:
    //     result: Option<(f64, f64)>
    //         Return value from `get_velocity_fields`.
    //
    // Example:
    //     let result = spanda_runtime::value::get_velocity_fields(val);

    // Match on val and handle each case.
    match val {
        RuntimeValue::Velocity { linear, angular } => Some((*linear, *angular)),
        _ => None,
    }
}

pub fn get_trajectory_waypoints(val: &RuntimeValue) -> Option<Vec<PoseValue>> {
    // Description:
    //     Get trajectory waypoints.
    //
    // Inputs:
    //     val: &RuntimeValue
    //         Caller-supplied val.
    //
    // Outputs:
    //     result: Option<Vec<PoseValue>>
    //         Return value from `get_trajectory_waypoints`.
    //
    // Example:
    //     let result = spanda_runtime::value::get_trajectory_waypoints(val);

    // Match on val and handle each case.
    match val {
        RuntimeValue::Trajectory { waypoints } => Some(waypoints.clone()),
        _ => None,
    }
}

pub fn get_number(val: &RuntimeValue, default: f64) -> f64 {
    // Description:
    //     Get number.
    //
    // Inputs:
    //     val: &RuntimeValue
    //         Caller-supplied val.
    //     defaul: f64
    //         Caller-supplied defaul.
    //
    // Outputs:
    //     result: f64
    //         Return value from `get_number`.
    //
    // Example:
    //     let result = spanda_runtime::value::get_number(val, defaul);

    // Produce unwrap or as the result.
    val.as_number().unwrap_or(default)
}

pub fn get_string(val: &RuntimeValue, default: &str) -> String {
    // Description:
    //     Get string.
    //
    // Inputs:
    //     val: &RuntimeValue
    //         Caller-supplied val.
    //     defaul: &str
    //         Caller-supplied defaul.
    //
    // Outputs:
    //     result: String
    //         Return value from `get_string`.
    //
    // Example:
    //     let result = spanda_runtime::value::get_string(val, defaul);

    // Produce to string as the result.
    val.as_string().unwrap_or(default).to_string()
}
