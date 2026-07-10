//! Spanda type system: primitives, generics, physical units, and domain types.

use crate::units::{self, PhysicalCategory};
use spanda_ast::nodes::{BinaryOp, SpandaType, UnitKind};
use std::collections::HashMap;

/// Generic type constructor arity.
#[derive(Debug, Clone, Copy)]
pub struct GenericDef {
    pub name: &'static str,
    pub arity: usize,
    pub namespace: Option<&'static str>,
}

/// Resolve a type name (optionally qualified) to a `SpandaType`.
pub fn resolve_type_name(name: &str) -> Result<SpandaType, String> {
    // Description:
    //     Resolve type name.
    //
    // Inputs:
    //     name: &str
    //         Caller-supplied name.
    //
    // Outputs:
    //     result: Result<SpandaType, String>
    //         Return value from `resolve_type_name`.
    //
    // Example:
    //     let result = spanda_typecheck::type_system::resolve_type_name(name);

    // Resolve the symbol name used below.
    let name = name.strip_prefix("std.").unwrap_or(name);
    let name = name.rsplit('.').next().unwrap_or(name);

    // Match on name and handle each case.
    match name {
        "Int" | "int" => Ok(SpandaType::Int),
        "Float" | "float" => Ok(SpandaType::Float),
        "Bool" | "bool" => Ok(SpandaType::Bool),
        "String" | "string" => Ok(SpandaType::String),
        "Char" | "char" => Ok(SpandaType::Char),
        "Bytes" | "bytes" => Ok(SpandaType::Bytes),
        "Null" | "null" => Ok(SpandaType::Null),
        "Void" | "void" => Ok(SpandaType::Void),
        "Time" => Ok(SpandaType::Named {
            name: "Time".into(),
        }),
        "Duration" => Ok(SpandaType::Number { unit: UnitKind::Ms }),
        "Timestamp" => Ok(SpandaType::Named {
            name: "Timestamp".into(),
        }),
        "Interval" => Ok(SpandaType::Named {
            name: "Interval".into(),
        }),
        "Distance" => Ok(SpandaType::Number { unit: UnitKind::M }),
        "Velocity" => Ok(SpandaType::Velocity),
        "Acceleration" => Ok(SpandaType::Number {
            unit: UnitKind::MPerS2,
        }),
        "Angle" => Ok(SpandaType::Number {
            unit: UnitKind::Rad,
        }),
        "AngularVelocity" => Ok(SpandaType::Number {
            unit: UnitKind::RadPerS,
        }),
        "Mass" | "Force" | "Power" | "Voltage" | "Current" | "Temperature" | "Pressure"
        | "Humidity" | "Illuminance" | "Luminance" | "Concentration" | "SoundLevel"
        | "MagneticField" | "RotationalSpeed" | "Torque" | "Energy" | "UvIndex" | "Ph"
        | "Conductivity" | "ParticulateMatter" | "Turbidity" | "Salinity" | "Radiation"
        | "SoilMoisture" => Ok(SpandaType::Named {
            name: name.to_string(),
        }),
        "Point2D" | "Point3D" | "Vector2D" | "Vector3D" | "Quaternion" | "Pose" => {
            Ok(SpandaType::Pose)
        }
        "Transform" => Ok(SpandaType::Transform),
        "Trajectory" | "Path" => Ok(SpandaType::Trajectory),
        "NavigationGoal"
        | "CostMap"
        | "MotionPlan"
        | "JointState"
        | "KinematicModel"
        | "CollisionModel"
        | "Map"
        | "OccupancyGrid"
        | "Landmark"
        | "LocalizationEstimate"
        | "MapLayer"
        | "Fleet"
        | "FleetTask"
        | "FleetCoordinator"
        | "Mission"
        | "Navigation"
        | "BatteryState"
        | "EnergyBudget"
        | "ChargingStation"
        | "HealthScore"
        | "MaintenanceAlert"
        | "FailurePrediction"
        | "Detection"
        | "Classification"
        | "Tracking"
        | "Segmentation"
        | "Arm"
        | "Gripper"
        | "EndEffector"
        | "Grasp"
        | "SwarmCoordinator"
        | "SwarmPolicy" => Ok(SpandaType::Named {
            name: name.to_string(),
        }),
        "Waypoint" | "MotionCommand" | "ControlSignal" | "PIDConfig" => Ok(SpandaType::Named {
            name: name.to_string(),
        }),
        "CameraFrame" | "Image" | "DepthImage" | "PointCloud" | "LidarScan" => Ok(SpandaType::Scan),
        "GpsFix" | "ImuData" | "AudioFrame" | "GnssFix" | "GeoPoint" | "GeoFence" | "Altitude"
        | "Heading" | "SpeedOverGround" | "SatelliteInfo" | "PositionAccuracy"
        | "NavigationStatus" => Ok(SpandaType::Named {
            name: name.to_string(),
        }),
        "WifiConnection"
        | "BluetoothConnection"
        | "BleConnection"
        | "CellularConnection"
        | "LTEConnection"
        | "FourGConnection"
        | "FiveGConnection"
        | "EthernetConnection"
        | "MeshConnection"
        | "NetworkStatus"
        | "SignalStrength"
        | "PacketLoss"
        | "RoamingStatus"
        | "SimIdentity" => Ok(SpandaType::Named {
            name: name.to_string(),
        }),
        "LLM" | "VisionModel" | "EmbeddingModel" | "Prompt" | "Completion" | "Embedding"
        | "Token" | "Context" | "Memory" | "Plan" | "ReasoningTrace" => Ok(SpandaType::Named {
            name: name.to_string(),
        }),
        "Agent" | "Goal" | "Task" | "Skill" | "Capability" | "Intent" => Ok(SpandaType::Named {
            name: name.to_string(),
        }),
        "ActionProposal" => Ok(SpandaType::Named {
            name: "ActionProposal".into(),
        }),
        "SafeAction" => Ok(SpandaType::Named {
            name: "SafeAction".into(),
        }),
        "UntrustedLinear" | "UntrustedAngular" => Ok(SpandaType::Named {
            name: name.to_string(),
        }),
        "Command" | "Conversation" | "Speech" | "Gesture" | "Emotion" | "Feedback" | "Approval" => {
            Ok(SpandaType::Named {
                name: name.to_string(),
            })
        }
        "Identity" | "RobotIdentity" | "Signature" | "Permission" | "TrustLevel" | "Hash"
        | "EncryptedMessage" | "SignedMessage" | "VerifiedMessage" | "TrustedSource"
        | "Certificate" | "PublicKey" | "PrivateKey" | "SessionKey" | "AuditEvent" | "AuditLog"
        | "ProvenanceRecord" | "MissionRecord" | "RecordId" => Ok(SpandaType::Named {
            name: name.to_string(),
        }),
        "Robot" | "Sensor" | "Actuator" | "Event" | "Bus" | "CompatibilityReport" => {
            Ok(SpandaType::Named {
                name: name.to_string(),
            })
        }
        "Result" | "Option" | "Error" | "File" | "Reader" | "Writer" | "Logger" | "LogLevel" => {
            Ok(SpandaType::Named {
                name: name.to_string(),
            })
        }
        "Confidence" | "Prediction" | "Probability" => Ok(SpandaType::Named {
            name: name.to_string(),
        }),
        "Risk" | "Hazard" | "SafetyConstraint" | "EmergencyStop" => Ok(SpandaType::Named {
            name: name.to_string(),
        }),
        "Twin" | "SimulationState" | "Telemetry" | "Replay" | "Fault" | "Scenario" => {
            Ok(SpandaType::Named {
                name: name.to_string(),
            })
        }
        "Transport"
        | "QosProfile"
        | "QoS"
        | "Bandwidth"
        | "Latency"
        | "TopicPath"
        | "ServiceEndpoint"
        | "MessageEnvelope"
        | "DiscoveryFilter"
        | "NetworkRequirements"
        | "Reliability"
        | "HistoryPolicy"
        | "CommBus"
        | "Endpoint" => Ok(SpandaType::Named {
            name: name.to_string(),
        }),
        "KnowledgeGraph" | "Belief" | "Observation" | "WorldModel" | "Policy" | "Reward"
        | "StateEstimate" | "SensorFusion" | "FusedObservation" => Ok(SpandaType::Named {
            name: name.to_string(),
        }),
        "Scan" => Ok(SpandaType::Scan),
        "Regex" => Ok(SpandaType::Regex),
        "Match" => Ok(SpandaType::Match),
        "Capture" => Ok(SpandaType::Capture),
        "CaptureGroup" => Ok(SpandaType::CaptureGroup),
        other if is_known_domain_type(other) => Ok(SpandaType::Named {
            name: other.to_string(),
        }),
        other => Err(format!("Unknown type '{other}'")),
    }
}

pub fn resolve_generic_type(name: &str, args: &[SpandaType]) -> Result<SpandaType, String> {
    // Description:
    //     Resolve generic type.
    //
    // Inputs:
    //     name: &str
    //         Caller-supplied name.
    //     args: &[SpandaType]
    //         Caller-supplied args.
    //
    // Outputs:
    //     result: Result<SpandaType, String>
    //         Return value from `resolve_generic_type`.
    //
    // Example:
    //     let result = spanda_typecheck::type_system::resolve_generic_type(name, args);

    // Compute base for the following logic.
    let base = name.rsplit('.').next().unwrap_or(name);
    let expected = generic_arity(base).ok_or_else(|| format!("Unknown generic type '{base}'"))?;

    // Take the branch when len differs from expected.
    if args.len() != expected {
        return Err(format!(
            "Type '{base}' expects {expected} type argument(s), got {}",
            args.len()
        ));
    }
    Ok(SpandaType::Generic {
        name: base.to_string(),
        type_args: args.to_vec(),
    })
}

pub fn generic_arity(name: &str) -> Option<usize> {
    // Description:
    //     Generic arity.
    //
    // Inputs:
    //     name: &str
    //         Caller-supplied name.
    //
    // Outputs:
    //     result: Option<usize>
    //         Return value from `generic_arity`.
    //
    // Example:
    //     let result = spanda_typecheck::type_system::generic_arity(name);

    // Match on name and handle each case.
    match name {
        "Array" | "Set" | "Queue" | "Stack" | "Topic" | "Message" | "Twin" | "Future" => Some(1),
        "Map" | "Service" | "Tuple" | "Result" => Some(2),
        "Option" => Some(1),
        "Action" => Some(3),
        "Endpoint" => Some(1),
        _ => None,
    }
}

fn is_known_domain_type(name: &str) -> bool {
    // Description:
    //     Is known domain type.
    //
    // Inputs:
    //     name: &str
    //         Caller-supplied name.
    //
    // Outputs:
    //     result: bool
    //         Return value from `is_known_domain_type`.
    //
    // Example:
    //     let result = spanda_typecheck::type_system::is_known_domain_type(name);

    // Produce contains as the result.
    KNOWN_DOMAIN_TYPES.contains(&name)
}

const KNOWN_DOMAIN_TYPES: &[&str] = &[
    "Mass",
    "Force",
    "Power",
    "Voltage",
    "Current",
    "Temperature",
    "Pressure",
    "Humidity",
    "Illuminance",
    "Luminance",
    "Concentration",
    "SoundLevel",
    "MagneticField",
    "RotationalSpeed",
    "Torque",
    "Energy",
    "UvIndex",
    "Ph",
    "Conductivity",
    "ParticulateMatter",
    "Turbidity",
    "Salinity",
    "Radiation",
    "SoilMoisture",
    "Time",
    "Timestamp",
    "Interval",
    "Waypoint",
    "MotionCommand",
    "ControlSignal",
    "PIDConfig",
    "GpsFix",
    "ImuData",
    "AudioFrame",
    "Prompt",
    "Completion",
    "Embedding",
    "Token",
    "Context",
    "Memory",
    "Plan",
    "ReasoningTrace",
    "Agent",
    "Goal",
    "Task",
    "Skill",
    "Capability",
    "Intent",
    "Approval",
    "Command",
    "Conversation",
    "Speech",
    "Gesture",
    "Emotion",
    "Feedback",
    "Confidence",
    "Prediction",
    "Probability",
    "Risk",
    "Hazard",
    "SafetyConstraint",
    "Twin",
    "SimulationState",
    "Telemetry",
    "Replay",
    "Fault",
    "Scenario",
    "KnowledgeGraph",
    "Belief",
    "Observation",
    "WorldModel",
    "Policy",
    "Reward",
    "StateEstimate",
    "SensorFusion",
    "FusedObservation",
    "LLM",
    "VisionModel",
    "EmbeddingModel",
    "CameraFrame",
    "Image",
    "DepthImage",
    "PointCloud",
    "LidarScan",
    "Transport",
    "QosProfile",
    "QoS",
    "Bandwidth",
    "Latency",
    "TopicPath",
    "ServiceEndpoint",
    "MessageEnvelope",
    "DiscoveryFilter",
    "NetworkRequirements",
    "Reliability",
    "HistoryPolicy",
    "CommBus",
    "Endpoint",
    "Identity",
    "RobotIdentity",
    "Signature",
    "Permission",
    "TrustLevel",
    "Hash",
    "AuditEvent",
    "AuditLog",
    "ProvenanceRecord",
    "MissionRecord",
    "RecordId",
    "Robot",
    "Sensor",
    "Actuator",
    "Event",
    "Bus",
    "CompatibilityReport",
    "Result",
    "Option",
    "Error",
    "File",
    "Reader",
    "Writer",
    "Logger",
    "LogLevel",
    "Simulator",
    "Motor",
    "Servo",
    "Gripper",
    "DriveUnit",
    "HardwareProfile",
];

/// Physical category used to reject invalid unit operations.
pub fn physical_category(ty: &SpandaType) -> PhysicalCategory {
    // Description:
    //     Physical category.
    //
    // Inputs:
    //     y: &SpandaType
    //         Caller-supplied y.
    //
    // Outputs:
    //     result: PhysicalCategory
    //         Return value from `physical_category`.
    //
    // Example:
    //     let result = spanda_typecheck::type_system::physical_category(y);

    // Match on ty and handle each case.
    match ty {
        SpandaType::Int | SpandaType::Float => PhysicalCategory::Scalar,
        SpandaType::Number { unit, .. } => units::unit_category(*unit),
        SpandaType::Velocity => PhysicalCategory::Velocity,
        SpandaType::Pose => PhysicalCategory::Distance,
        SpandaType::Named { name } => match name.as_str() {
            "Distance" => PhysicalCategory::Distance,
            "Duration" | "Time" | "Timestamp" | "Interval" => PhysicalCategory::Duration,
            "Velocity" => PhysicalCategory::Velocity,
            "Acceleration" => PhysicalCategory::Acceleration,
            "Angle" | "AngularVelocity" => PhysicalCategory::AngularVelocity,
            "Mass" => PhysicalCategory::Mass,
            "Force" => PhysicalCategory::Force,
            "Power" => PhysicalCategory::Power,
            "Voltage" => PhysicalCategory::Voltage,
            "Current" => PhysicalCategory::Current,
            "Temperature" => PhysicalCategory::Temperature,
            "Pressure" => PhysicalCategory::Pressure,
            "Humidity" => PhysicalCategory::Humidity,
            "Illuminance" => PhysicalCategory::Illuminance,
            "Luminance" => PhysicalCategory::Luminance,
            "Concentration" => PhysicalCategory::Concentration,
            "SoundLevel" => PhysicalCategory::SoundLevel,
            "MagneticField" => PhysicalCategory::MagneticField,
            "RotationalSpeed" => PhysicalCategory::RotationalSpeed,
            "Torque" => PhysicalCategory::Torque,
            "Energy" => PhysicalCategory::Energy,
            "UvIndex" => PhysicalCategory::UvIndex,
            "Ph" => PhysicalCategory::Ph,
            "Conductivity" => PhysicalCategory::Conductivity,
            "ParticulateMatter" => PhysicalCategory::ParticulateMatter,
            "Turbidity" => PhysicalCategory::Turbidity,
            "Salinity" => PhysicalCategory::Salinity,
            "Radiation" => PhysicalCategory::Radiation,
            "SoilMoisture" => PhysicalCategory::SoilMoisture,
            _ => PhysicalCategory::Scalar,
        },
        _ => PhysicalCategory::Scalar,
    }
}

/// Returns `None` when the operation is invalid (e.g. distance + duration).
pub fn binary_physical_op_allowed(op: BinaryOp, left: &SpandaType, right: &SpandaType) -> bool {
    // Description:
    //     Binary physical op allowed.
    //
    // Inputs:
    //     op: BinaryOp
    //         Caller-supplied op.
    //     lef: &SpandaType
    //         Caller-supplied lef.
    //     righ: &SpandaType
    //         Caller-supplied righ.
    //
    // Outputs:
    //     result: bool
    //         Return value from `binary_physical_op_allowed`.
    //
    // Example:
    //     let result = spanda_typecheck::type_system::binary_physical_op_allowed(op, lef, righ);

    // Import the items needed by the logic below.
    use spanda_ast::nodes::BinaryOp;
    let cat_l = physical_category(left);
    let cat_r = physical_category(right);

    // Match on op and handle each case.
    match op {
        BinaryOp::Add | BinaryOp::Sub => {
            // Take the branch when cat l equals Scalar.
            if cat_l == PhysicalCategory::Scalar && cat_r == PhysicalCategory::Scalar {
                return true;
            }
            cat_l == cat_r && cat_l != PhysicalCategory::Scalar
        }
        BinaryOp::Lt
        | BinaryOp::Lte
        | BinaryOp::Gt
        | BinaryOp::Gte
        | BinaryOp::Eq
        | BinaryOp::Neq => cat_l == cat_r,
        BinaryOp::Mul | BinaryOp::Div => true,
        BinaryOp::And | BinaryOp::Or => {
            matches!(left, SpandaType::Bool) && matches!(right, SpandaType::Bool)
        }
    }
}

pub fn is_action_proposal_type(ty: &SpandaType) -> bool {
    // Description:
    //     Is action proposal type.
    //
    // Inputs:
    //     y: &SpandaType
    //         Caller-supplied y.
    //
    // Outputs:
    //     result: bool
    //         Return value from `is_action_proposal_type`.
    //
    // Example:
    //     let result = spanda_typecheck::type_system::is_action_proposal_type(y);

    // Produce matches! as the result.
    matches!(
        ty,
        SpandaType::Named { name } if name == "ActionProposal"
    )
}

pub fn is_safe_action_type(ty: &SpandaType) -> bool {
    // Description:
    //     Is safe action type.
    //
    // Inputs:
    //     y: &SpandaType
    //         Caller-supplied y.
    //
    // Outputs:
    //     result: bool
    //         Return value from `is_safe_action_type`.
    //
    // Example:
    //     let result = spanda_typecheck::type_system::is_safe_action_type(y);

    // Produce matches! as the result.
    matches!(
        ty,
        SpandaType::Named { name } if name == "SafeAction"
    )
}

pub fn is_untrusted_motion_component(ty: &SpandaType) -> bool {
    // True when `ty` is an opaque ActionProposal motion component.
    //
    // Parameters:
    // - `ty` — candidate type from member access or a let-bound alias
    //
    // Returns:
    // Whether the type is `UntrustedLinear` or `UntrustedAngular`.
    //
    // Options:
    // None.
    //
    // Example:
    // assert!(is_untrusted_motion_component(&SpandaType::Named {
    //     name: "UntrustedLinear".into(),
    // }));

    // Match opaque AI motion component names produced by ActionProposal fields.
    matches!(
        ty,
        SpandaType::Named { name } if name == "UntrustedLinear" || name == "UntrustedAngular"
    )
}

pub fn std_namespaces() -> HashMap<&'static str, &'static [&'static str]> {
    // Description:
    //     Std namespaces.
    //
    // Inputs:
    //     None.
    //
    // Outputs:
    //     result: HashMap<&'static str, &'static [&'static str]>
    //         Return value from `std_namespaces`.
    //
    // Example:
    //     let result = spanda_typecheck::type_system::std_namespaces();

    // Create mutable m for accumulating results.
    let mut m = HashMap::new();
    m.insert(
        "std.time",
        &["Time", "Duration", "Timestamp", "Interval"][..],
    );
    m.insert(
        "std.units",
        &[
            "Distance",
            "Velocity",
            "Acceleration",
            "Angle",
            "AngularVelocity",
            "Mass",
            "Force",
            "Power",
            "Voltage",
            "Current",
            "Temperature",
            "Pressure",
            "Humidity",
            "Illuminance",
            "Luminance",
            "Concentration",
            "SoundLevel",
            "MagneticField",
            "RotationalSpeed",
            "Torque",
            "Energy",
            "UvIndex",
            "Ph",
            "Conductivity",
            "ParticulateMatter",
            "Turbidity",
            "Salinity",
            "Radiation",
            "SoilMoisture",
        ][..],
    );
    m.insert(
        "std.spatial",
        &[
            "Point2D",
            "Point3D",
            "Vector2D",
            "Vector3D",
            "Quaternion",
            "Pose",
            "Transform",
            "Trajectory",
            "Path",
            "Waypoint",
        ][..],
    );
    m.insert(
        "std.ai",
        &[
            "LLM",
            "VisionModel",
            "EmbeddingModel",
            "Prompt",
            "Completion",
            "Embedding",
            "Token",
            "Context",
            "Memory",
            "Plan",
            "ReasoningTrace",
        ][..],
    );
    m.insert(
        "std.robotics",
        &[
            "MotionCommand",
            "ControlSignal",
            "PIDConfig",
            "ActionProposal",
            "SafeAction",
            "Agent",
            "Goal",
            "Task",
            "Skill",
            "Capability",
            "Intent",
        ][..],
    );
    m.insert(
        "std.sensors",
        &[
            "CameraFrame",
            "Image",
            "DepthImage",
            "PointCloud",
            "LidarScan",
            "GpsFix",
            "ImuData",
            "AudioFrame",
        ][..],
    );
    m.insert(
        "std.safety",
        &[
            "Risk",
            "Hazard",
            "SafetyConstraint",
            "EmergencyStop",
            "SafeAction",
        ][..],
    );
    m.insert(
        "std.twin",
        &[
            "Twin",
            "SimulationState",
            "Telemetry",
            "Replay",
            "Fault",
            "Scenario",
        ][..],
    );
    m.insert(
        "std.hri",
        &[
            "Command",
            "Conversation",
            "Speech",
            "Gesture",
            "Emotion",
            "Feedback",
        ][..],
    );
    m.insert(
        "std.actuators",
        &[
            "Actuator",
            "Motor",
            "Servo",
            "Gripper",
            "DriveUnit",
            "JointCommand",
            "TorqueCommand",
            "VelocityCommand",
        ][..],
    );
    m.insert(
        "std.communication",
        &[
            "Transport",
            "QosProfile",
            "QoS",
            "Bandwidth",
            "Latency",
            "TopicPath",
            "ServiceEndpoint",
            "MessageEnvelope",
            "DiscoveryFilter",
            "NetworkRequirements",
            "Reliability",
            "HistoryPolicy",
            "CommBus",
            "Endpoint",
            "Topic",
            "Message",
            "Service",
            "Action",
        ][..],
    );
    m.insert(
        "std.hardware",
        &[
            "HardwareProfile",
            "SensorSpec",
            "ActuatorSpec",
            "BusConfig",
            "PinConfig",
            "DeviceTree",
            "Peripheral",
            "Interface",
        ][..],
    );
    m.insert(
        "std.sim",
        &[
            "Simulator",
            "WorldState",
            "PhysicsConfig",
            "Scene",
            "Entity",
            "SensorModel",
            "ActuatorModel",
            "Tick",
            "ReplayBuffer",
        ][..],
    );
    m.insert(
        "std.network",
        &[
            "Transport",
            "QosProfile",
            "QoS",
            "Bandwidth",
            "Latency",
            "TopicPath",
            "ServiceEndpoint",
            "MessageEnvelope",
            "DiscoveryFilter",
            "NetworkRequirements",
            "Reliability",
            "HistoryPolicy",
            "CommBus",
            "Endpoint",
            "Topic",
            "Message",
            "Service",
            "Action",
        ][..],
    );
    m.insert(
        "std.policies.homeostasis",
        &[
            "HomeostasisPolicy",
            "StabilityMetric",
            "StabilityRange",
            "DriftSignal",
            "CorrectionAction",
            "StabilityReport",
        ][..],
    );
    m.insert(
        "std.policies.attention",
        &[
            "AttentionPolicy",
            "AttentionScore",
            "AttentionWindow",
            "EventPriority",
            "SuppressionRule",
            "SignalPriority",
        ][..],
    );
    m.insert("std.core", &["Result", "Option", "Error", "Void"][..]);
    m.insert("std.math", &["Float", "Int"][..]);
    m.insert(
        "std.collections",
        &["Array", "Map", "Set", "Queue", "Stack", "Tuple"][..],
    );
    m.insert("std.result", &["Result", "Option", "Error"][..]);
    m.insert("std.io", &["File", "Reader", "Writer", "Bytes"][..]);
    m.insert("std.log", &["Logger", "LogLevel"][..]);
    m.insert(
        "std.security",
        &[
            "Identity",
            "RobotIdentity",
            "Signature",
            "Permission",
            "Capability",
            "TrustLevel",
        ][..],
    );
    m.insert(
        "std.audit",
        &[
            "AuditEvent",
            "AuditLog",
            "ProvenanceRecord",
            "MissionRecord",
            "RecordId",
        ][..],
    );
    m.insert("std.crypto", &["Hash", "Signature"][..]);
    m.insert(
        "std.positioning",
        &[
            "GpsFix",
            "GnssFix",
            "GeoPoint",
            "GeoFence",
            "Altitude",
            "Heading",
            "SpeedOverGround",
            "SatelliteInfo",
            "PositionAccuracy",
            "NavigationStatus",
        ][..],
    );
    m.insert(
        "std.connectivity",
        &[
            "WifiConnection",
            "BluetoothConnection",
            "BleConnection",
            "CellularConnection",
            "LTEConnection",
            "FourGConnection",
            "FiveGConnection",
            "EthernetConnection",
            "MeshConnection",
            "NetworkStatus",
            "SignalStrength",
            "Bandwidth",
            "Latency",
            "PacketLoss",
            "RoamingStatus",
            "SimIdentity",
        ][..],
    );
    m.insert(
        "std.wifi",
        &["WifiConnection", "SignalStrength", "NetworkStatus"][..],
    );
    m.insert(
        "std.bluetooth",
        &["BluetoothConnection", "BleConnection", "BleService"][..],
    );
    m.insert(
        "std.cellular",
        &[
            "CellularConnection",
            "LTEConnection",
            "FourGConnection",
            "FiveGConnection",
            "RoamingStatus",
            "SimIdentity",
        ][..],
    );
    m.insert("std.geofence", &["GeoFence", "GeoPoint"][..]);
    m.insert(
        "std.robotics",
        &[
            "Robot",
            "Sensor",
            "Actuator",
            "MotionCommand",
            "ControlSignal",
            "PIDConfig",
            "ActionProposal",
            "SafeAction",
            "Agent",
            "Goal",
            "Task",
            "Skill",
            "Capability",
            "Intent",
        ][..],
    );
    m.insert(
        "std.ai",
        &[
            "LLM",
            "VisionModel",
            "EmbeddingModel",
            "Prompt",
            "Completion",
            "Embedding",
            "Token",
            "Context",
            "Memory",
            "Plan",
            "ReasoningTrace",
            "ActionProposal",
            "SafeAction",
        ][..],
    );
    m.insert(
        "std.communication",
        &[
            "Transport",
            "QosProfile",
            "QoS",
            "Bandwidth",
            "Latency",
            "TopicPath",
            "ServiceEndpoint",
            "MessageEnvelope",
            "DiscoveryFilter",
            "NetworkRequirements",
            "Reliability",
            "HistoryPolicy",
            "CommBus",
            "Endpoint",
            "Topic",
            "Message",
            "Service",
            "Action",
            "Event",
            "Bus",
        ][..],
    );
    m.insert(
        "std.hardware",
        &[
            "HardwareProfile",
            "CompatibilityReport",
            "SensorSpec",
            "ActuatorSpec",
            "BusConfig",
            "PinConfig",
            "DeviceTree",
            "Peripheral",
            "Interface",
        ][..],
    );
    m.insert(
        "std.sim",
        &[
            "Simulator",
            "Scenario",
            "Fault",
            "Replay",
            "WorldState",
            "PhysicsConfig",
            "Scene",
            "Entity",
            "SensorModel",
            "ActuatorModel",
            "Tick",
            "ReplayBuffer",
        ][..],
    );
    m.insert(
        "std.hri",
        &[
            "Command",
            "Conversation",
            "Speech",
            "Gesture",
            "Emotion",
            "Feedback",
            "Intent",
            "Approval",
        ][..],
    );
    m.insert(
        "std.navigation",
        &[
            "NavigationGoal",
            "Path",
            "Waypoint",
            "Trajectory",
            "CostMap",
        ][..],
    );
    m.insert(
        "std.fusion",
        &[
            "FusedObservation",
            "StateEstimate",
            "Confidence",
            "SensorFusion",
        ][..],
    );
    m.insert(
        "std.slam",
        &[
            "Map",
            "OccupancyGrid",
            "Landmark",
            "LocalizationEstimate",
            "MapLayer",
        ][..],
    );
    m.insert(
        "std.manipulation",
        &["Arm", "Gripper", "EndEffector", "Grasp", "Pick", "Place"][..],
    );
    m.insert(
        "std.maintenance",
        &["HealthScore", "MaintenanceAlert", "FailurePrediction"][..],
    );
    m.insert(
        "std.environment",
        &[
            "Weather",
            "Temperature",
            "Humidity",
            "AirQuality",
            "LightLevel",
        ][..],
    );
    m
}

/// Built-in AI provider names accepted in `ai_model { provider: … }` literals.
pub const KNOWN_AI_PROVIDERS: &[&str] = &["mock", "openai", "anthropic", "onnx"];

/// Formats accepted by `serialize` / `deserialize` literals.
pub const KNOWN_SERIALIZE_FORMATS: &[&str] = &["json", "yaml", "binary"];

/// Targets accepted by `spanda codegen --target` / `CodegenTarget.*` config literals.
pub const KNOWN_CODEGEN_TARGETS: &[&str] = &["native", "wasm", "esp32"];

/// Closed typed-enum names for config / format literals.
pub const TYPED_CONFIG_ENUMS: &[&str] = &["AiProvider", "SerializeFormat", "CodegenTarget"];

pub fn is_known_ai_provider(name: &str) -> bool {
    // Return true when `name` matches a built-in AI provider (case-insensitive).
    //
    // Parameters:
    // - `name` — provider string from an `ai_model` config entry
    //
    // Returns:
    // `true` for mock, openai, anthropic, or onnx.
    //
    // Options:
    // None.
    //
    // Example:
    // assert!(is_known_ai_provider("OpenAI"));

    // Compare against the built-in provider list without regard to case.
    KNOWN_AI_PROVIDERS
        .iter()
        .any(|known| known.eq_ignore_ascii_case(name))
}

pub fn is_known_serialize_format(name: &str) -> bool {
    // Return true when `name` is a supported serialize/deserialize format.
    //
    // Parameters:
    // - `name` — format string literal passed to `serialize` / `deserialize`
    //
    // Returns:
    // `true` for json, yaml, or binary.
    //
    // Options:
    // None.
    //
    // Example:
    // assert!(is_known_serialize_format("JSON"));

    KNOWN_SERIALIZE_FORMATS
        .iter()
        .any(|known| known.eq_ignore_ascii_case(name))
}

pub fn is_known_codegen_target(name: &str) -> bool {
    // Return true when `name` is a supported codegen target.
    //
    // Parameters:
    // - `name` — target string (`native` / `wasm` / `esp32`)
    //
    // Returns:
    // `true` for known targets.
    //
    // Options:
    // None.
    //
    // Example:
    // assert!(is_known_codegen_target("wasm"));

    KNOWN_CODEGEN_TARGETS
        .iter()
        .any(|known| known.eq_ignore_ascii_case(name))
}

pub fn typed_enum_variant_value<'a>(
    enum_name: &str,
    variant: &'a str,
) -> Result<&'a str, String> {
    // Validate a closed typed-enum literal and return the variant name.
    //
    // Parameters:
    // - `enum_name` — `AiProvider`, `SerializeFormat`, or `CodegenTarget`
    // - `variant` — variant identifier (e.g. `mock`, `json`)
    //
    // Returns:
    // The variant string when valid, or an error message.
    //
    // Options:
    // None.
    //
    // Example:
    // assert_eq!(typed_enum_variant_value("AiProvider", "mock").unwrap(), "mock");

    match enum_name {
        "AiProvider" => {
            if is_known_ai_provider(variant) {
                Ok(variant)
            } else {
                Err(format!(
                    "Unknown AiProvider variant '{variant}' (use {})",
                    KNOWN_AI_PROVIDERS.join(", ")
                ))
            }
        }
        "SerializeFormat" => {
            if is_known_serialize_format(variant) {
                Ok(variant)
            } else {
                Err(format!(
                    "Unknown SerializeFormat variant '{variant}' (use {})",
                    KNOWN_SERIALIZE_FORMATS.join(", ")
                ))
            }
        }
        "CodegenTarget" => {
            if is_known_codegen_target(variant) {
                Ok(variant)
            } else {
                Err(format!(
                    "Unknown CodegenTarget variant '{variant}' (use {})",
                    KNOWN_CODEGEN_TARGETS.join(", ")
                ))
            }
        }
        other => Err(format!(
            "Unknown typed enum '{other}' (use {})",
            TYPED_CONFIG_ENUMS.join(", ")
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_unknown_type() {
        // Description:
        //     Rejects unknown type.
        //
        // Inputs:
        //     None.
        //
        // Outputs:
        //     None.
        //
        // Example:

        //     let result = spanda_typecheck::type_system::rejects_unknown_type();

        assert!(resolve_type_name("NotARealType").is_err());
    }

    #[test]
    fn resolves_generics_with_arity() {
        // Description:
        //     Resolves generics with arity.
        //
        // Inputs:
        //     None.
        //
        // Outputs:
        //     None.
        //
        // Example:

        //     let result = spanda_typecheck::type_system::resolves_generics_with_arity();

        let goal = SpandaType::Named {
            name: "Goal".into(),
        };
        let err = resolve_generic_type("Array", &[]).unwrap_err();
        assert!(err.contains("expects 1"));
        let ok = resolve_generic_type("Array", &[goal]).unwrap();
        assert!(matches!(ok, SpandaType::Generic { .. }));
    }

    #[test]
    fn known_ai_providers_and_serialize_formats() {
        // Description:
        //     Known ai providers and serialize formats.
        //
        // Inputs:
        //     None.
        //
        // Outputs:
        //     None.
        //
        // Example:

        //     let result = spanda_typecheck::type_system::known_ai_providers_and_serialize_formats();

        assert!(is_known_ai_provider("mock"));
        assert!(is_known_ai_provider("OpenAI"));
        assert!(!is_known_ai_provider("not-a-provider"));
        assert!(is_known_serialize_format("json"));
        assert!(is_known_serialize_format("YAML"));
        assert!(!is_known_serialize_format("xml"));
    }

    #[test]
    fn rejects_mixed_physical_add() {
        // Description:
        //     Rejects mixed physical add.
        //
        // Inputs:
        //     None.
        //
        // Outputs:
        //     None.
        //
        // Example:

        //     let result = spanda_typecheck::type_system::rejects_mixed_physical_add();

        let vel = SpandaType::Velocity;
        let volt = SpandaType::Named {
            name: "Voltage".into(),
        };
        assert!(!binary_physical_op_allowed(BinaryOp::Add, &vel, &volt));
    }
}
