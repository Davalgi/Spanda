//! Runtime safety monitor, geofenced zones, and motion validation for Spanda.
//!
use spanda_runtime::environment::Environment;
use spanda_runtime::robot_state::RobotState;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct SafetyZoneRuntime {
    pub name: String,
    pub shape: SafetyZoneShape,
    pub x: f64,
    pub y: f64,
    pub radius: Option<f64>,
    pub width: Option<f64>,
    pub height: Option<f64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SafetyZoneShape {
    Circle,
    Rect,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SafetyEvaluation {
    pub allowed: bool,
    pub reason: Option<String>,
    pub emergency_stop: bool,
}

pub type StopIfRule = Box<dyn Fn(&Environment) -> bool>;

pub struct SafetyConfig {
    pub max_speed: f64,
    /// Optional turn-rate cap in rad/s (`f64::INFINITY` = unbounded).
    pub max_angular: f64,
    pub stop_if_rules: Vec<StopIfRule>,
    pub zones: Vec<SafetyZoneRuntime>,
    pub zone_speed_caps: HashMap<String, f64>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ValidatedMotion {
    pub linear: f64,
    pub angular: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ValidateActionResult {
    Ok(ValidatedMotion),
    Err { reason: String },
}

pub struct SafetyMonitor {
    config: SafetyConfig,
    emergency_stop: bool,
}

impl SafetyMonitor {
    pub fn new(config: SafetyConfig) -> Self {
        // Description:
        //     Construct a new instance.
        //
        // Inputs:
        //     config: SafetyConfig
        //         Caller-supplied config.
        //
        // Outputs:
        //     result: Self
        //         Return value from `new`.
        //
        // Example:
        //     let value = spanda_safety::new(config);

        // Assemble the struct fields and return it.
        Self {
            config,
            emergency_stop: false,
        }
    }

    pub fn evaluate_before_motion(&mut self, env: &Environment, pose: &Pose2d) -> SafetyEvaluation {
        // Description:
        //     Evaluate before motion.
        //
        // Inputs:
        //     &mut self: input value
        //         Caller-supplied &mut self.
        //     env: &Environment
        //         Caller-supplied env.
        //     pose: &Pose2d
        //         Caller-supplied pose.
        //
        // Outputs:
        //     result: SafetyEvaluation
        //         Return value from `evaluate_before_motion`.
        //
        // Example:
        //     let result = spanda_safety::evaluate_before_motion(&mut self, env, pose);

        // Compute peek for the following logic.
        let peek = self.peek_before_motion(env, pose);

        // Take the branch when emergency stop is false.
        if !peek.allowed && peek.emergency_stop {
            self.emergency_stop = true;
        }
        peek
    }

    pub fn peek_before_motion(&self, env: &Environment, pose: &Pose2d) -> SafetyEvaluation {
        // Description:
        //     Peek before motion.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //     env: &Environment
        //         Caller-supplied env.
        //     pose: &Pose2d
        //         Caller-supplied pose.
        //
        // Outputs:
        //     result: SafetyEvaluation
        //         Return value from `peek_before_motion`.
        //
        // Example:
        //     let result = spanda_safety::peek_before_motion(&self, env, pose);

        // take this path when self.emergency stop.
        if self.emergency_stop {
            return SafetyEvaluation {
                allowed: false,
                reason: Some("Emergency stop active".to_string()),
                emergency_stop: true,
            };
        }

        // Process each stop if rule.
        for rule in &self.config.stop_if_rules {
            // Take this path when rule(env).
            if rule(env) {
                return SafetyEvaluation {
                    allowed: false,
                    reason: Some("stop_if safety rule triggered".to_string()),
                    emergency_stop: true,
                };
            }
        }

        // Process each zone.
        for zone in &self.config.zones {
            // Take this path when Self::is point in zone(pose.x, pose.y, zone).
            if Self::is_point_in_zone(pose.x, pose.y, zone) {
                // Allow motion inside zones that only declare a program speed cap.
                if self.config.zone_speed_caps.contains_key(&zone.name) {
                    continue;
                }
                return SafetyEvaluation {
                    allowed: false,
                    reason: Some(format!("Robot entered safety zone '{}'", zone.name)),
                    emergency_stop: true,
                };
            }
        }
        SafetyEvaluation {
            allowed: true,
            reason: None,
            emergency_stop: false,
        }
    }

    pub fn validate_action_proposal(
        &self,
        linear: f64,
        angular: f64,
        env: &Environment,
        pose: &Pose2d,
    ) -> ValidateActionResult {
        // Description:
        //     Validate action proposal.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //     linear: f64
        //         Caller-supplied linear.
        //     angular: f64
        //         Caller-supplied angular.
        //     env: &Environment
        //         Caller-supplied env.
        //     pose: &Pose2d
        //         Caller-supplied pose.
        //
        // Outputs:
        //     result: ValidateActionResult
        //         Return value from `validate_action_proposal`.
        //
        // Example:
        //     let result = spanda_safety::validate_action_proposal(&self, linear, angular, env, pose);

        // Compute peek for the following logic.
        let peek = self.peek_before_motion(env, pose);

        // Take the branch when allowed is false.
        if !peek.allowed {
            return ValidateActionResult::Err {
                reason: peek
                    .reason
                    .unwrap_or_else(|| "Safety validation failed".to_string()),
            };
        }
        ValidateActionResult::Ok(ValidatedMotion {
            linear: self.clamp_speed_at_pose(linear, pose),
            angular: self.clamp_angular(angular),
        })
    }

    pub fn effective_max_speed(&self, pose: &Pose2d) -> f64 {
        // Description:

        //     Effective max speed.

        //

        // Inputs:

        //     &self: value

        //         Caller-supplied &self.

        //     pose: &Pose2d

        //         Caller-supplied pose.

        //

        // Outputs:

        //     result: f64

        //         Return value from `effective_max_speed`.

        //

        // Example:

        //     let result = spanda_safety::effective_max_speed(&self, pose);
        let mut cap = self.config.max_speed;
        for zone in &self.config.zones {
            if Self::is_point_in_zone(pose.x, pose.y, zone) {
                if let Some(zone_cap) = self.config.zone_speed_caps.get(&zone.name) {
                    cap = cap.min(*zone_cap);
                }
            }
        }
        cap
    }

    pub fn clamp_speed_at_pose(&self, requested: f64, pose: &Pose2d) -> f64 {
        // Description:

        //     Clamp speed at pose.

        //

        // Inputs:

        //     &self: value

        //         Caller-supplied &self.

        //     requested: f64

        //         Caller-supplied requested.

        //     pose: &Pose2d

        //         Caller-supplied pose.

        //

        // Outputs:

        //     result: f64

        //         Return value from `clamp_speed_at_pose`.

        //

        // Example:

        //     let result = spanda_safety::clamp_speed_at_pose(&self, requested, pose);
        let sign = if requested == 0.0 {
            1.0
        } else {
            requested.signum()
        };
        requested.abs().min(self.effective_max_speed(pose)) * sign
    }

    pub fn clamp_angular(&self, requested: f64) -> f64 {
        // Clamp angular velocity to `safety.max_angular` when configured.
        //
        // Parameters:
        // - `requested` — requested angular velocity in rad/s
        //
        // Returns:
        // The signed angular velocity limited by `max_angular`.
        //
        // Options:
        // None.
        //
        // Example:
        // assert_eq!(monitor.clamp_angular(2.0), 1.0);

        // Preserve sign while applying the absolute turn-rate cap.
        let sign = if requested == 0.0 {
            1.0
        } else {
            requested.signum()
        };
        requested.abs().min(self.config.max_angular) * sign
    }

    pub fn is_in_zone(&self, zone_name: &str, pose: &Pose2d) -> bool {
        // Description:
        //     Is in zone.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //     zone_name: &str
        //         Caller-supplied zone name.
        //     pose: &Pose2d
        //         Caller-supplied pose.
        //
        // Outputs:
        //     result: bool
        //         Return value from `is_in_zone`.
        //
        // Example:
        //     let result = spanda_safety::is_in_zone(&self, zone_name, pose);

        // Compute Some for the following logic.
        let Some(zone) = self.config.zones.iter().find(|z| z.name == zone_name) else {
            return false;
        };
        Self::is_point_in_zone(pose.x, pose.y, zone)
    }

    pub fn clamp_speed(&self, requested: f64) -> f64 {
        // Description:

        //     Clamp speed.

        //

        // Inputs:

        //     &self: value

        //         Caller-supplied &self.

        //     requested: f64

        //         Caller-supplied requested.

        //

        // Outputs:

        //     result: f64

        //         Return value from `clamp_speed`.

        //

        // Example:

        //     let result = spanda_safety::clamp_speed(&self, requested);
        self.clamp_speed_at_pose(requested, &Pose2d { x: 0.0, y: 0.0 })
    }

    pub fn is_emergency_stop(&self) -> bool {
        // Description:
        //     Is emergency stop.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //
        // Outputs:
        //     result: bool
        //         Return value from `is_emergency_stop`.
        //
        // Example:
        //     let result = spanda_safety::is_emergency_stop(&self);

        // Call emergency stop on the current instance.
        self.emergency_stop
    }

    pub fn set_emergency_stop(&mut self, active: bool) {
        // Description:
        //     Set emergency stop.
        //
        // Inputs:
        //     &mut self: input value
        //         Caller-supplied &mut self.
        //     active: bool
        //         Caller-supplied active.
        //
        // Outputs:
        //     None.
        //
        // Example:
        //     let result = spanda_safety::set_emergency_stop(&mut self, active);

        // Call emergency stop = active; on the current instance.
        self.emergency_stop = active;
    }

    pub fn apply_speed_cap(&mut self, cap_mps: f64) {
        // Description:
        //     Apply speed cap.
        //
        // Inputs:
        //     &mut self: input value
        //         Caller-supplied &mut self.
        //     cap_mps: f64
        //         Caller-supplied cap mps.
        //
        // Outputs:
        //     None.
        //
        // Example:
        //     let result = spanda_safety::apply_speed_cap(&mut self, cap_mps);

        // Keep the lower of the current cap and the recovery limit.
        self.config.max_speed = self.config.max_speed.min(cap_mps);
    }

    pub fn reset(&mut self) {
        // Description:
        //     Reset.
        //
        // Inputs:
        //     &mut self: input value
        //         Caller-supplied &mut self.
        //
        // Outputs:
        //     None.
        //
        // Example:
        //     let result = spanda_safety::reset(&mut self);

        // Call emergency stop = false; on the current instance.
        self.emergency_stop = false;
    }

    fn is_point_in_zone(x: f64, y: f64, zone: &SafetyZoneRuntime) -> bool {
        // Description:
        //     Is point in zone.
        //
        // Inputs:
        //     x: f64
        //         Caller-supplied x.
        //     y: f64
        //         Caller-supplied y.
        //     zone: &SafetyZoneRuntime
        //         Caller-supplied zone.
        //
        // Outputs:
        //     result: bool
        //         Return value from `is_point_in_zone`.
        //
        // Example:
        //     let result = spanda_safety::is_point_in_zone(x, y, zone);

        // Match on shape and handle each case.
        match zone.shape {
            SafetyZoneShape::Circle => {
                // Emit output when radius provides a radius.
                if let Some(radius) = zone.radius {
                    let dx = x - zone.x;
                    let dy = y - zone.y;
                    (dx * dx + dy * dy).sqrt() <= radius
                } else {
                    false
                }
            }
            SafetyZoneShape::Rect => {
                // Take this path when let (Some(width), Some(height)) = (zone.width, zone.height).
                if let (Some(width), Some(height)) = (zone.width, zone.height) {
                    x >= zone.x && x <= zone.x + width && y >= zone.y && y <= zone.y + height
                } else {
                    false
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pose2d {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Pose3d {
    pub x: f64,
    pub y: f64,
    pub theta: f64,
    pub z: f64,
}

pub fn create_safety_config_from_robot(
    max_speed: f64,
    stop_if_rules: Vec<StopIfRule>,
    zones: Vec<SafetyZoneRuntime>,
    zone_speed_caps: HashMap<String, f64>,
) -> SafetyConfig {
    // Description:
    //     Create safety config from robot.
    //
    // Inputs:
    //     ax_speed: f64
    //         Caller-supplied ax speed.
    //     stop_if_rules: Vec<StopIfRule>
    //         Caller-supplied stop if rules.
    //     zones: Vec<SafetyZoneRuntime>
    //         Caller-supplied zones.
    //     zone_speed_caps: HashMap<String, f64>
    //         Caller-supplied zone speed caps.
    //
    // Outputs:
    //     result: SafetyConfig
    //         Return value from `create_safety_config_from_robot`.
    //
    // Example:
    //     let result = spanda_safety::create_safety_config_from_robot(ax_speed, stop_if_rules, zones, zone_speed_caps);

    create_safety_config_from_robot_with_angular(
        max_speed,
        f64::INFINITY,
        stop_if_rules,
        zones,
        zone_speed_caps,
    )
}

pub fn create_safety_config_from_robot_with_angular(
    max_speed: f64,
    max_angular: f64,
    stop_if_rules: Vec<StopIfRule>,
    zones: Vec<SafetyZoneRuntime>,
    zone_speed_caps: HashMap<String, f64>,
) -> SafetyConfig {
    // Build a SafetyConfig including an optional angular velocity cap.
    //
    // Parameters:
    // - `max_speed` — linear speed cap (m/s)
    // - `max_angular` — angular velocity cap (rad/s); use INFINITY for unbounded
    // - `stop_if_rules` — emergency stop predicates
    // - `zones` — runtime safety zones
    // - `zone_speed_caps` — named zone linear speed caps
    //
    // Returns:
    // A populated `SafetyConfig`.
    //
    // Options:
    // None.
    //
    // Example:
    // let cfg = create_safety_config_from_robot_with_angular(1.0, 0.5, vec![], vec![], HashMap::new());

    // Produce SafetyConfig as the result.
    SafetyConfig {
        max_speed,
        max_angular,
        stop_if_rules,
        zones,
        zone_speed_caps,
    }
}

pub fn apply_emergency_stop(state: RobotState) -> RobotState {
    // Description:
    //     Apply emergency stop.
    //
    // Inputs:
    //     state: RobotState
    //         Caller-supplied state.
    //
    // Outputs:
    //     result: RobotState
    //         Return value from `apply_emergency_stop`.
    //
    // Example:
    //     let result = spanda_safety::apply_emergency_stop(state);

    // Produce RobotState as the result.
    RobotState {
        emergency_stop: true,
        velocity: spanda_runtime::robot_state::VelocityState {
            linear: 0.0,
            angular: 0.0,
        },
        ..state
    }
}

pub fn interpolate_poses(
    from: &spanda_runtime::robot_state::PoseState,
    to: &spanda_runtime::robot_state::PoseState,
    steps: f64,
) -> Vec<Pose3d> {
    // Description:
    //     Interpolate poses.
    //
    // Inputs:
    //     fro: &spanda_runtime::robot_state::PoseState
    //         Caller-supplied fro.
    //     o: &spanda_runtime::robot_state::PoseState
    //         Caller-supplied o.
    //     steps: f64
    //         Caller-supplied steps.
    //
    // Outputs:
    //     result: Vec<Pose3d>
    //         Return value from `interpolate_poses`.
    //
    // Example:
    //     let result = spanda_safety::interpolate_poses(fro, o, steps);

    // Compute count for the following logic.
    let count = steps.max(2.0).floor() as usize;
    let from_z = from.z.unwrap_or(0.0);
    let to_z = to.z.unwrap_or(0.0);
    let mut waypoints = Vec::with_capacity(count);

    // Iterate over count.
    for i in 0..count {
        let t = i as f64 / (count as f64 - 1.0);
        waypoints.push(Pose3d {
            x: from.x + (to.x - from.x) * t,
            y: from.y + (to.y - from.y) * t,
            theta: from.theta + (to.theta - from.theta) * t,
            z: from_z + (to_z - from_z) * t,
        });
    }
    waypoints
}

#[cfg(test)]
mod tests {
    use super::*;
    use spanda_ast::nodes::UnitKind;
    use spanda_runtime::value::RuntimeValue;

    #[test]
    fn blocks_motion_when_stop_if_triggers() {
        // Description:
        //     Blocks motion when stop if triggers.
        //
        // Inputs:
        //     None.
        //
        // Outputs:
        //     None.
        //
        // Example:

        //     let result = spanda_safety::blocks_motion_when_stop_if_triggers();

        let mut env = Environment::new();
        env.define("obstacle", RuntimeValue::number(0.3, UnitKind::M));

        let mut monitor = SafetyMonitor::new(create_safety_config_from_robot(
            1.5,
            vec![Box::new(|e| {
                matches!(
                    e.get("obstacle"),
                    Some(RuntimeValue::Number { value, .. }) if *value < 0.5
                )
            })],
            vec![],
            HashMap::new(),
        ));

        let result = monitor.evaluate_before_motion(&env, &Pose2d { x: 0.0, y: 0.0 });
        assert!(!result.allowed);
        assert!(result.emergency_stop);
    }

    #[test]
    fn allows_motion_when_rules_pass() {
        // Description:
        //     Allows motion when rules pass.
        //
        // Inputs:
        //     None.
        //
        // Outputs:
        //     None.
        //
        // Example:

        //     let result = spanda_safety::allows_motion_when_rules_pass();

        let mut env = Environment::new();
        env.define("obstacle", RuntimeValue::number(2.0, UnitKind::M));

        let mut monitor = SafetyMonitor::new(create_safety_config_from_robot(
            1.5,
            vec![Box::new(|e| {
                matches!(
                    e.get("obstacle"),
                    Some(RuntimeValue::Number { value, .. }) if *value < 0.5
                )
            })],
            vec![],
            HashMap::new(),
        ));

        let result = monitor.evaluate_before_motion(&env, &Pose2d { x: 0.0, y: 0.0 });
        assert!(result.allowed);
    }

    #[test]
    fn detects_safety_zone_entry() {
        // Description:
        //     Detects safety zone entry.
        //
        // Inputs:
        //     None.
        //
        // Outputs:
        //     None.
        //
        // Example:

        //     let result = spanda_safety::detects_safety_zone_entry();

        let monitor = SafetyMonitor::new(create_safety_config_from_robot(
            1.5,
            vec![],
            vec![SafetyZoneRuntime {
                name: "keepout".to_string(),
                shape: SafetyZoneShape::Circle,
                x: 0.0,
                y: 0.0,
                radius: Some(1.0),
                width: None,
                height: None,
            }],
            HashMap::new(),
        ));
        assert!(monitor.is_in_zone("keepout", &Pose2d { x: 0.5, y: 0.0 }));
        assert!(!monitor.is_in_zone("keepout", &Pose2d { x: 5.0, y: 5.0 }));
    }

    #[test]
    fn clamps_speed_to_max() {
        // Description:
        //     Clamps speed to max.
        //
        // Inputs:
        //     None.
        //
        // Outputs:
        //     None.
        //
        // Example:

        //     let result = spanda_safety::clamps_speed_to_max();

        let monitor = SafetyMonitor::new(create_safety_config_from_robot(
            1.0,
            vec![],
            vec![],
            HashMap::new(),
        ));
        assert_eq!(monitor.clamp_speed(2.0), 1.0);
        assert_eq!(monitor.clamp_speed(-3.0), -1.0);
    }

    #[test]
    fn clamps_angular_to_max_angular() {
        // Angular velocity is limited by safety.max_angular.
        //
        // Parameters:
        // None.
        //
        // Returns:
        // None.
        //
        // Options:
        // None.
        //
        // Example:
        // clamps_angular_to_max_angular();

        let monitor = SafetyMonitor::new(create_safety_config_from_robot_with_angular(
            1.0,
            0.5,
            vec![],
            vec![],
            HashMap::new(),
        ));
        assert_eq!(monitor.clamp_angular(2.0), 0.5);
        assert_eq!(monitor.clamp_angular(-3.0), -0.5);
        let validated = monitor.validate_action_proposal(
            0.2,
            1.5,
            &Environment::new(),
            &Pose2d { x: 0.0, y: 0.0 },
        );
        match validated {
            ValidateActionResult::Ok(motion) => {
                assert_eq!(motion.angular, 0.5);
            }
            ValidateActionResult::Err { reason } => panic!("unexpected reject: {reason}"),
        }
    }

    #[test]
    fn clamps_speed_to_program_zone_cap() {
        // Description:
        //     Clamps speed to program zone cap.
        //
        // Inputs:
        //     None.
        //
        // Outputs:
        //     None.
        //
        // Example:
        //     let result = spanda_safety::clamps_speed_to_program_zone_cap();
        let mut caps = HashMap::new();
        caps.insert("HumanArea".into(), 0.5);
        let monitor = SafetyMonitor::new(create_safety_config_from_robot(
            1.0,
            vec![],
            vec![SafetyZoneRuntime {
                name: "HumanArea".into(),
                shape: SafetyZoneShape::Circle,
                x: 0.0,
                y: 0.0,
                radius: Some(2.0),
                width: None,
                height: None,
            }],
            caps,
        ));
        let inside = Pose2d { x: 0.0, y: 0.0 };
        let outside = Pose2d { x: 10.0, y: 10.0 };
        assert_eq!(monitor.clamp_speed_at_pose(0.8, &inside), 0.5);
        assert_eq!(monitor.clamp_speed_at_pose(0.8, &outside), 0.8);
    }

    #[test]
    fn allows_motion_inside_speed_cap_zone() {
        // Description:
        //     Allows motion inside speed cap zone.
        //
        // Inputs:
        //     None.
        //
        // Outputs:
        //     None.
        //
        // Example:
        //     let result = spanda_safety::allows_motion_inside_speed_cap_zone();
        let mut caps = HashMap::new();
        caps.insert("HumanArea".into(), 0.5);
        let mut monitor = SafetyMonitor::new(create_safety_config_from_robot(
            1.0,
            vec![],
            vec![SafetyZoneRuntime {
                name: "HumanArea".into(),
                shape: SafetyZoneShape::Circle,
                x: 0.0,
                y: 0.0,
                radius: Some(2.0),
                width: None,
                height: None,
            }],
            caps,
        ));
        let inside = Pose2d { x: 0.0, y: 0.0 };
        let result = monitor.evaluate_before_motion(&Environment::new(), &inside);
        assert!(result.allowed);
    }
}
