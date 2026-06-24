//! simulator support for Spanda.
//!
use crate::runtime::{MotionCommand, PoseValue, RobotBackend, RuntimeValue};
use spanda_hal::hal::{create_sim_hal, HalBackend, SimHalBackend};
use spanda_runtime::robot_state::{PoseState, RobotState, VelocityState};

#[derive(Debug, Clone)]
pub struct Obstacle {
    pub x: f64,
    pub y: f64,
    pub radius: f64,
}

#[derive(Debug, Clone)]
pub struct SimulatorConfig {
    pub obstacles: Vec<Obstacle>,
    pub initial_pose: PoseState,
    pub lidar_range: f64,
}

impl Default for SimulatorConfig {
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
        //     let result = spanda_interpreter::simulator::default();

        // Assemble the struct fields and return it.
        Self {
            obstacles: vec![
                Obstacle {
                    x: 2.0,
                    y: 0.0,
                    radius: 0.3,
                },
                Obstacle {
                    x: -1.0,
                    y: 1.5,
                    radius: 0.25,
                },
            ],
            initial_pose: PoseState {
                x: 0.0,
                y: 0.0,
                theta: 0.0,
                z: Some(0.0),
            },
            lidar_range: 10.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PublishedMessage {
    pub topic: String,
    pub message_type: String,
    pub value: RuntimeValue,
}

pub struct Simulator {
    pose: PoseState,
    velocity: VelocityState,
    emergency_stop: bool,
    obstacles: Vec<Obstacle>,
    lidar_range: f64,
    arm_position: (f64, f64, f64),
    gripper_closed: bool,
    thrust: f64,
    event_log: Vec<String>,
    published: Vec<PublishedMessage>,
    follow_queue: Vec<PoseValue>,
    service_log: Vec<String>,
    action_log: Vec<String>,
    hal: SimHalBackend,
}

impl Simulator {
    pub fn new(config: SimulatorConfig) -> Self {
        // Description:
        //     Construct a new instance.
        //
        // Inputs:
        //     config: SimulatorConfig
        //         Caller-supplied config.
        //
        // Outputs:
        //     result: Self
        //         Return value from `new`.
        //
        // Example:
        //     let value = spanda_interpreter::simulator::new(config);

        // Assemble the struct fields and return it.
        Self {
            pose: config.initial_pose,
            velocity: VelocityState {
                linear: 0.0,
                angular: 0.0,
            },
            emergency_stop: false,
            obstacles: config.obstacles,
            lidar_range: config.lidar_range,
            arm_position: (0.0, 0.0, 0.5),
            gripper_closed: false,
            thrust: 0.0,
            event_log: Vec::new(),
            published: Vec::new(),
            follow_queue: Vec::new(),
            service_log: Vec::new(),
            action_log: Vec::new(),
            hal: create_sim_hal(),
        }
    }

    pub fn get_event_log(&self) -> Vec<String> {
        // Description:
        //     Get event log.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //
        // Outputs:
        //     result: Vec<String>
        //         Return value from `get_event_log`.
        //
        // Example:
        //     let result = spanda_interpreter::simulator::get_event_log(&self);

        // Call clone on the current instance.
        self.event_log.clone()
    }

    pub fn get_arm_position(&self) -> (f64, f64, f64) {
        // Description:
        //     Get arm position.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //
        // Outputs:
        //     result: (f64, f64, f64)
        //         Return value from `get_arm_position`.
        //
        // Example:
        //     let result = spanda_interpreter::simulator::get_arm_position(&self);

        // Call arm position on the current instance.
        self.arm_position
    }

    pub fn get_service_log(&self) -> Vec<String> {
        // Description:
        //     Get service log.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //
        // Outputs:
        //     result: Vec<String>
        //         Return value from `get_service_log`.
        //
        // Example:
        //     let result = spanda_interpreter::simulator::get_service_log(&self);

        // Call clone on the current instance.
        self.service_log.clone()
    }

    pub fn get_action_log(&self) -> Vec<String> {
        // Description:
        //     Get action log.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //
        // Outputs:
        //     result: Vec<String>
        //         Return value from `get_action_log`.
        //
        // Example:
        //     let result = spanda_interpreter::simulator::get_action_log(&self);

        // Call clone on the current instance.
        self.action_log.clone()
    }

    pub fn get_published_topics(&self) -> Vec<PublishedMessage> {
        // Description:
        //     Get published topics.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //
        // Outputs:
        //     result: Vec<PublishedMessage>
        //         Return value from `get_published_topics`.
        //
        // Example:
        //     let result = spanda_interpreter::simulator::get_published_topics(&self);

        // Call clone on the current instance.
        self.published.clone()
    }

    fn simulate_lidar(&self) -> f64 {
        // Description:
        //     Simulate lidar.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //
        // Outputs:
        //     result: f64
        //         Return value from `simulate_lidar`.
        //
        // Example:
        //     let result = spanda_interpreter::simulator::simulate_lidar(&self);

        // Create mutable nearest for accumulating results.
        let mut nearest = self.lidar_range;

        // Process each obstacle.
        for obs in &self.obstacles {
            let dx = obs.x - self.pose.x;
            let dy = obs.y - self.pose.y;
            let dist = (dx * dx + dy * dy).sqrt() - obs.radius;

            // Take this path when dist > 0.0 && dist < nearest.
            if dist > 0.0 && dist < nearest {
                nearest = dist;
            }
        }
        let wall_dist = 5.0 - self.pose.x.abs();

        // Take this path when wall dist > 0.0 && wall dist < nearest.
        if wall_dist > 0.0 && wall_dist < nearest {
            nearest = wall_dist;
        }
        nearest.max(0.01)
    }
}

impl RobotBackend for Simulator {
    fn read_sensor(
        &mut self,
        _sensor_name: &str,
        sensor_type: &str,
        _topic: Option<&str>,
    ) -> RuntimeValue {
        // Description:
        //     Read sensor.
        //
        // Inputs:
        //     &mut self: input value
        //         Caller-supplied &mut self.
        //     _sensor_name: &str
        //         Caller-supplied sensor name.
        //     sensor_type: &str
        //         Caller-supplied sensor type.
        //     _topic: Option<&str>
        //         Caller-supplied topic.
        //
        // Outputs:
        //     result: RuntimeValue
        //         Return value from `read_sensor`.
        //
        // Example:
        //     let result = spanda_interpreter::simulator::read_sensor(&mut self, _sensor_name, sensor_type, _topic);

        // Import the items needed by the logic below.
        use spanda_ast::nodes::UnitKind;
        use std::collections::HashMap;

        // Match on sensor type and handle each case.
        match sensor_type {
            "Lidar" => RuntimeValue::Scan {
                nearest_distance: self.simulate_lidar(),
            },
            "IMU" => RuntimeValue::Object {
                type_name: "IMUReading".into(),
                fields: HashMap::from([
                    (
                        "roll".into(),
                        RuntimeValue::Number {
                            value: 0.0,
                            unit: UnitKind::Rad,
                        },
                    ),
                    (
                        "pitch".into(),
                        RuntimeValue::Number {
                            value: 0.0,
                            unit: UnitKind::Rad,
                        },
                    ),
                    (
                        "yaw".into(),
                        RuntimeValue::Number {
                            value: self.pose.theta,
                            unit: UnitKind::Rad,
                        },
                    ),
                ]),
            },
            "AltitudeSensor" => RuntimeValue::Number {
                value: self.pose.z.unwrap_or(0.0),
                unit: UnitKind::M,
            },
            "GPS" | "GNSS" => {
                let alt = self.pose.z.unwrap_or(0.0);
                let fix_quality = if self.emergency_stop { 0.0 } else { 1.0 };
                spanda_connectivity_runtime::runtime_gps_fix(
                    self.pose.x,
                    self.pose.y,
                    alt,
                    fix_quality,
                )
            }
            "ForceTorque" => RuntimeValue::Object {
                type_name: "ForceTorqueReading".into(),
                fields: HashMap::from([(
                    "force".into(),
                    RuntimeValue::Number {
                        value: if self.gripper_closed { 5.0 } else { 0.0 },
                        unit: UnitKind::None,
                    },
                )]),
            },
            "Camera" => RuntimeValue::Object {
                type_name: "CameraFrame".into(),
                fields: HashMap::from([
                    (
                        "width".into(),
                        RuntimeValue::Number {
                            value: 640.0,
                            unit: UnitKind::None,
                        },
                    ),
                    (
                        "height".into(),
                        RuntimeValue::Number {
                            value: 480.0,
                            unit: UnitKind::None,
                        },
                    ),
                ]),
            },
            _ => RuntimeValue::Void,
        }
    }

    fn publish_topic(&mut self, topic_path: &str, message_type: &str, value: RuntimeValue) {
        // Description:
        //     Publish topic.
        //
        // Inputs:
        //     &mut self: input value
        //         Caller-supplied &mut self.
        //     opic_path: &str
        //         Caller-supplied opic path.
        //     essage_type: &str
        //         Caller-supplied essage type.
        //     value: RuntimeValue
        //         Caller-supplied value.
        //
        // Outputs:
        //     None.
        //
        // Example:
        //     let result = spanda_interpreter::simulator::publish_topic(&mut self, opic_path, essage_type, value);

        // take this path when let RuntimeValue::Velocity { linear, angular } = &value.
        if let RuntimeValue::Velocity { linear, angular } = &value {
            self.velocity = VelocityState {
                linear: *linear,
                angular: *angular,
            };
        }
        self.published.push(PublishedMessage {
            topic: topic_path.to_string(),
            message_type: message_type.to_string(),
            value,
        });
        self.event_log
            .push(format!("publish({topic_path}, {message_type})"));
    }

    fn call_service(&mut self, service_name: &str, service_type: &str) -> RuntimeValue {
        // Description:
        //     Call service.
        //
        // Inputs:
        //     &mut self: input value
        //         Caller-supplied &mut self.
        //     service_name: &str
        //         Caller-supplied service name.
        //     service_type: &str
        //         Caller-supplied service type.
        //
        // Outputs:
        //     result: RuntimeValue
        //         Return value from `call_service`.
        //
        // Example:
        //     let result = spanda_interpreter::simulator::call_service(&mut self, service_name, service_type);

        // Call service log on the current instance.
        self.service_log
            .push(format!("{service_name}:{service_type}"));
        self.event_log.push(format!("service({service_name})"));
        RuntimeValue::Bool { value: true }
    }

    fn send_action(
        &mut self,
        action_name: &str,
        action_type: &str,
        goal: RuntimeValue,
    ) -> RuntimeValue {
        // Description:
        //     Send action.
        //
        // Inputs:
        //     &mut self: input value
        //         Caller-supplied &mut self.
        //     action_name: &str
        //         Caller-supplied action name.
        //     action_type: &str
        //         Caller-supplied action type.
        //     goal: RuntimeValue
        //         Caller-supplied goal.
        //
        // Outputs:
        //     result: RuntimeValue
        //         Return value from `send_action`.
        //
        // Example:
        //     let result = spanda_interpreter::simulator::send_action(&mut self, action_name, action_type, goal);

        // Append into self.
        self.action_log.push(format!("{action_name}:{action_type}"));
        self.event_log.push(format!("action({action_name})"));

        // Match on goal and handle each case.
        match goal {
            RuntimeValue::Pose { x, y, theta, z } => {
                self.pose = PoseState {
                    x,
                    y,
                    theta,
                    z: Some(z),
                };
            }
            RuntimeValue::Trajectory { waypoints } if !waypoints.is_empty() => {
                self.follow_queue = waypoints;
            }
            _ => {}
        }
        RuntimeValue::Bool { value: true }
    }

    fn execute_motion(&mut self, cmd: MotionCommand) {
        // Description:
        //     Execute motion.
        //
        // Inputs:
        //     &mut self: input value
        //         Caller-supplied &mut self.
        //     cmd: MotionCommand
        //         Caller-supplied cmd.
        //
        // Outputs:
        //     None.
        //
        // Example:
        //     let result = spanda_interpreter::simulator::execute_motion(&mut self, cmd);

        // keep entries that match the expected pattern.
        if self.emergency_stop && !matches!(cmd, MotionCommand::Stop { .. }) {
            self.velocity = VelocityState {
                linear: 0.0,
                angular: 0.0,
            };
            return;
        }

        // Match on cmd and handle each case.
        match cmd {
            MotionCommand::Drive {
                linear, angular, ..
            } => {
                self.velocity = VelocityState { linear, angular };
                self.event_log
                    .push(format!("drive({:.2} m/s, {:.2} rad/s)", linear, angular));
            }
            MotionCommand::Follow { waypoints, .. } => {
                self.follow_queue = waypoints;
                self.event_log
                    .push(format!("follow({} waypoints)", self.follow_queue.len()));
            }
            MotionCommand::Stop { .. } => {
                self.velocity = VelocityState {
                    linear: 0.0,
                    angular: 0.0,
                };
                self.follow_queue.clear();
                self.event_log.push("stop()".into());
            }
            MotionCommand::MoveTo { x, y, z, .. } => {
                self.arm_position = (x, y, z);
                self.event_log.push(format!("move_to({x}, {y}, {z})"));
            }
            MotionCommand::Grip { .. } => {
                self.gripper_closed = true;
                self.event_log.push("grip()".into());
            }
            MotionCommand::Release { .. } => {
                self.gripper_closed = false;
                self.event_log.push("release()".into());
            }
            MotionCommand::Open { .. } => {
                self.gripper_closed = false;
                self.event_log.push("open()".into());
            }
            MotionCommand::SetThrust { thrust, .. } => {
                self.thrust = thrust;
                self.event_log.push(format!("set_thrust({thrust})"));
            }
            MotionCommand::Hover { .. } => {
                self.thrust = 0.5;
                self.velocity = VelocityState {
                    linear: 0.0,
                    angular: 0.0,
                };
                self.event_log.push("hover()".into());
            }
        }
    }

    fn tick(&mut self, dt_ms: f64) {
        // Description:
        //     Tick.
        //
        // Inputs:
        //     &mut self: input value
        //         Caller-supplied &mut self.
        //     dt_ms: f64
        //         Caller-supplied dt ms.
        //
        // Outputs:
        //     None.
        //
        // Example:
        //     let result = spanda_interpreter::simulator::tick(&mut self, dt_ms);

        // take this path when self.emergency stop.
        if self.emergency_stop {
            self.velocity = VelocityState {
                linear: 0.0,
                angular: 0.0,
            };
            return;
        }
        let dt = dt_ms / 1000.0;

        // Emit output when cloned provides a target.
        if let Some(target) = self.follow_queue.first().cloned() {
            let dx = target.x - self.pose.x;
            let dy = target.y - self.pose.y;
            let dist = (dx * dx + dy * dy).sqrt();

            // Take this path when dist < 0.05.
            if dist < 0.05 {
                self.follow_queue.remove(0);
                self.pose.x = target.x;
                self.pose.y = target.y;
                self.pose.theta = target.theta;
            } else {
                let speed = 0.5;
                self.pose.x += (dx / dist) * speed * dt;
                self.pose.y += (dy / dist) * speed * dt;
                self.pose.theta = dy.atan2(dx);
                self.velocity = VelocityState {
                    linear: speed,
                    angular: 0.0,
                };
            }
            return;
        }

        // Take this path when self.thrust > 0.0.
        if self.thrust > 0.0 {
            let climb_rate = (self.thrust - 0.5) * 2.0;
            let z = self.pose.z.unwrap_or(0.0);
            self.pose.z = Some((z + climb_rate * dt).max(0.0));
        }
        let new_theta = self.pose.theta + self.velocity.angular * dt;
        let new_x = self.pose.x + self.velocity.linear * self.pose.theta.cos() * dt;
        let new_y = self.pose.y + self.velocity.linear * self.pose.theta.sin() * dt;
        self.pose.x = new_x;
        self.pose.y = new_y;
        self.pose.theta = new_theta;
    }

    fn get_state(&self) -> RobotState {
        // Description:
        //     Get state.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //
        // Outputs:
        //     result: RobotState
        //         Return value from `get_state`.
        //
        // Example:
        //     let result = spanda_interpreter::simulator::get_state(&self);

        // Produce RobotState as the result.
        RobotState {
            pose: self.pose.clone(),
            velocity: self.velocity.clone(),
            emergency_stop: self.emergency_stop,
        }
    }

    fn set_emergency_stop(&mut self, value: bool) {
        // Description:
        //     Set emergency stop.
        //
        // Inputs:
        //     &mut self: input value
        //         Caller-supplied &mut self.
        //     value: bool
        //         Caller-supplied value.
        //
        // Outputs:
        //     None.
        //
        // Example:
        //     let result = spanda_interpreter::simulator::set_emergency_stop(&mut self, value);

        // Call emergency stop = value; on the current instance.
        self.emergency_stop = value;

        // Take this path when value.
        if value {
            self.velocity = VelocityState {
                linear: 0.0,
                angular: 0.0,
            };
            self.follow_queue.clear();
        }
    }

    fn get_hal(&mut self) -> Option<&mut dyn HalBackend> {
        // Description:
        //     Get hal.
        //
        // Inputs:
        //     &mut self: input value
        //         Caller-supplied &mut self.
        //
        // Outputs:
        //     result: Option<&mut dyn HalBackend>
        //         Return value from `get_hal`.
        //
        // Example:
        //     let result = spanda_interpreter::simulator::get_hal(&mut self);

        // Produce hal) as the result.
        Some(&mut self.hal)
    }

    fn event_log(&self) -> Vec<String> {
        // Description:
        //     Event log.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //
        // Outputs:
        //     result: Vec<String>
        //         Return value from `event_log`.
        //
        // Example:
        //     let result = spanda_interpreter::simulator::event_log(&self);

        // Call get event log on the current instance.
        self.get_event_log()
    }
}

pub fn create_default_simulator(config: SimulatorConfig) -> Simulator {
    // Description:
    //     Create default simulator.
    //
    // Inputs:
    //     config: SimulatorConfig
    //         Caller-supplied config.
    //
    // Outputs:
    //     result: Simulator
    //         Return value from `create_default_simulator`.
    //
    // Example:
    //     let result = spanda_interpreter::simulator::create_default_simulator(config);

    // Produce new as the result.
    Simulator::new(config)
}

impl spanda_runtime::replay::ReplayStateTarget for Simulator {
    fn apply_replay_state(&mut self, snapshot: &spanda_runtime::replay::ReplayStateSnapshot) {
        // Description:
        //     Apply replay state.
        //
        // Inputs:
        //     &mut self: value
        //         Caller-supplied &mut self.
        //     snapsho: &spanda_runtime::replay::ReplayStateSnapshot
        //         Caller-supplied snapsho.
        //
        // Outputs:
        //     None.
        //
        // Example:
        //     let result = spanda_interpreter::simulator::apply_replay_state(&mut self, snapsho);
        self.pose = snapshot.pose.clone();
        self.velocity = snapshot.velocity.clone();
        self.emergency_stop = snapshot.emergency_stop;
        self.follow_queue.clear();
        self.thrust = 0.0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn updates_pose_on_drive() {
        // Description:
        //     Updates pose on drive.
        //
        // Inputs:
        //     None.
        //
        // Outputs:
        //     None.
        //
        // Example:

        //     let result = spanda_interpreter::simulator::updates_pose_on_drive();

        let mut sim = create_default_simulator(SimulatorConfig::default());
        sim.execute_motion(MotionCommand::Drive {
            linear: 1.0,
            angular: 0.0,
            actuator: "wheels".into(),
        });
        sim.tick(1000.0);
        assert!((sim.get_state().pose.x - 1.0).abs() < 0.1);
    }

    #[test]
    fn simulates_lidar_nearest_distance() {
        // Description:
        //     Simulates lidar nearest distance.
        //
        // Inputs:
        //     None.
        //
        // Outputs:
        //     None.
        //
        // Example:

        //     let result = spanda_interpreter::simulator::simulates_lidar_nearest_distance();

        let mut sim = create_default_simulator(SimulatorConfig {
            initial_pose: PoseState {
                x: 0.0,
                y: 0.0,
                theta: 0.0,
                z: Some(0.0),
            },
            obstacles: vec![Obstacle {
                x: 3.0,
                y: 0.0,
                radius: 0.5,
            }],
            ..Default::default()
        });
        let reading = sim.read_sensor("lidar", "Lidar", None);
        if let RuntimeValue::Scan { nearest_distance } = reading {
            assert!((nearest_distance - 2.5).abs() < 0.1);
        } else {
            panic!("expected scan");
        }
    }

    #[test]
    fn stops_motion_on_emergency_stop() {
        // Description:
        //     Stops motion on emergency stop.
        //
        // Inputs:
        //     None.
        //
        // Outputs:
        //     None.
        //
        // Example:

        //     let result = spanda_interpreter::simulator::stops_motion_on_emergency_stop();

        let mut sim = create_default_simulator(SimulatorConfig::default());
        sim.execute_motion(MotionCommand::Drive {
            linear: 1.0,
            angular: 0.0,
            actuator: "wheels".into(),
        });
        sim.set_emergency_stop(true);
        sim.tick(1000.0);
        assert_eq!(sim.get_state().velocity.linear, 0.0);
    }

    #[test]
    fn simulates_drone_altitude_with_thrust() {
        // Description:
        //     Simulates drone altitude with thrust.
        //
        // Inputs:
        //     None.
        //
        // Outputs:
        //     None.
        //
        // Example:

        //     let result = spanda_interpreter::simulator::simulates_drone_altitude_with_thrust();

        let mut sim = create_default_simulator(SimulatorConfig {
            initial_pose: PoseState {
                x: 0.0,
                y: 0.0,
                theta: 0.0,
                z: Some(1.0),
            },
            ..Default::default()
        });
        sim.execute_motion(MotionCommand::SetThrust {
            thrust: 0.8,
            actuator: "rotors".into(),
        });
        sim.tick(500.0);
        assert!(sim.get_state().pose.z.unwrap_or(0.0) > 1.0);
    }

    #[test]
    fn tracks_arm_move_to() {
        // Description:
        //     Tracks arm move to.
        //
        // Inputs:
        //     None.
        //
        // Outputs:
        //     None.
        //
        // Example:

        //     let result = spanda_interpreter::simulator::tracks_arm_move_to();

        let mut sim = create_default_simulator(SimulatorConfig::default());
        sim.execute_motion(MotionCommand::MoveTo {
            x: 0.5,
            y: 0.3,
            z: 0.2,
            actuator: "arm".into(),
        });
        assert_eq!(sim.get_arm_position(), (0.5, 0.3, 0.2));
    }

    #[test]
    fn logs_motion_events() {
        // Description:
        //     Logs motion events.
        //
        // Inputs:
        //     None.
        //
        // Outputs:
        //     None.
        //
        // Example:

        //     let result = spanda_interpreter::simulator::logs_motion_events();

        let mut sim = create_default_simulator(SimulatorConfig::default());
        sim.execute_motion(MotionCommand::Stop {
            actuator: "wheels".into(),
        });
        assert!(sim.get_event_log().iter().any(|e| e == "stop()"));
    }
}
