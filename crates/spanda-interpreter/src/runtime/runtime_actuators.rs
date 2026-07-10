//! Actuator execute/safe-motion dispatch for the interpreter.
//!

use super::{
    get_number, get_trajectory_waypoints, Interpreter, IntoSpandaError, MotionCommand,
    RobotBackend, RuntimeError, RuntimeValue,
};
use spanda_ai::{is_action_proposal, is_safe_action};
use spanda_ast::nodes::Expr;
use spanda_error::SpandaError;
use spanda_safety::Pose2d;

impl<B: RobotBackend> Interpreter<B> {
    pub(super) fn execute_actuator_method(
        &mut self,
        name: &str,
        _actuator_type: &str,
        method: &str,
        args: &[Expr],
        named_args: &[spanda_ast::nodes::NamedArg],
        line: u32,
    ) -> Result<RuntimeValue, SpandaError> {
        // Description:
        //     Execute actuator method.
        //
        // Inputs:
        //     &mut self: input value
        //         Caller-supplied &mut self.
        //     name: &str
        //         Caller-supplied name.
        //     _actuator_type: &str
        //         Caller-supplied actuator type.
        //     ethod: &str
        //         Caller-supplied ethod.
        //     args: &[Expr]
        //         Caller-supplied args.
        //     named_args: &[spanda_ast::nodes::NamedArg]
        //         Caller-supplied named args.
        //     line: u32
        //         Caller-supplied line.
        //
        // Outputs:
        //     result: Result<RuntimeValue, SpandaError>
        //         Return value from `execute_actuator_method`.
        //
        // Example:
        //     let result = spanda_interpreter::runtime_actuators::execute_actuator_method(&mut self, name, _actuator_type, ethod, args, named_args, line);

        // Compute motion methods for the following logic.
        let motion_methods = [
            "drive",
            "move_to",
            "set_thrust",
            "grip",
            "release",
            "open",
            "hover",
            "follow",
        ];

        // Check membership before continuing.
        if (motion_methods.contains(&method) || method == "stop")
            && !self.check_safety_before_motion()
        {
            // Emit output when on motion blocked provides a cb.
            if let Some(cb) = &self.options.on_motion_blocked {
                cb("Safety rule triggered — motion blocked".into());
            }
            self.backend.execute_motion(MotionCommand::Stop {
                actuator: name.to_string(),
            });
            return Ok(RuntimeValue::Void);
        }

        // Match on method and handle each case.
        match method {
            "stop" => {
                self.backend.execute_motion(MotionCommand::Stop {
                    actuator: name.to_string(),
                });
            }
            "drive" => {
                let linear = get_number(&self.get_named_arg_value(named_args, "linear")?, 0.0);
                let angular = get_number(&self.get_named_arg_value(named_args, "angular")?, 0.0);
                if let Some(reason) = self.check_runtime_policy_before_motion(linear.abs()) {
                    return self.block_motion_for_policy(name, reason, line);
                }
                let pose = self.backend.get_state().pose;
                let pose2d = Pose2d {
                    x: pose.x,
                    y: pose.y,
                };
                let (clamped_linear, clamped_angular) = match self.safety_monitor.as_ref() {
                    Some(m) => (
                        m.clamp_speed_at_pose(linear, &pose2d),
                        m.clamp_angular(angular),
                    ),
                    None => (linear, angular),
                };
                self.backend.execute_motion(MotionCommand::Drive {
                    linear: clamped_linear,
                    angular: clamped_angular,
                    actuator: name.to_string(),
                });
            }
            "follow" => {
                let path_val = self.get_named_arg_value(named_args, "path")?;
                let waypoints = get_trajectory_waypoints(&path_val).unwrap_or_default();
                // Default follow cruise matches the historical simulator constant.
                const DEFAULT_FOLLOW_SPEED: f64 = 0.5;
                if let Some(reason) = self.check_runtime_policy_before_motion(DEFAULT_FOLLOW_SPEED)
                {
                    return self.block_motion_for_policy(name, reason, line);
                }
                // Pass the unclamped request; reclamp at the current pose (and each tick).
                self.backend.execute_motion(MotionCommand::Follow {
                    waypoints,
                    max_linear: DEFAULT_FOLLOW_SPEED,
                    actuator: name.to_string(),
                });
                self.reclamp_active_follow_cruise();
            }
            "move_to" => {
                let x = get_number(&self.get_named_arg_value(named_args, "x")?, 0.0);
                let y = get_number(&self.get_named_arg_value(named_args, "y")?, 0.0);
                let z = get_number(&self.get_named_arg_value(named_args, "z")?, 0.0);
                self.backend.execute_motion(MotionCommand::MoveTo {
                    x,
                    y,
                    z,
                    actuator: name.to_string(),
                });
            }
            "grip" => {
                self.backend.execute_motion(MotionCommand::Grip {
                    actuator: name.to_string(),
                });
            }
            "release" => {
                self.backend.execute_motion(MotionCommand::Release {
                    actuator: name.to_string(),
                });
            }
            "open" => {
                self.backend.execute_motion(MotionCommand::Open {
                    actuator: name.to_string(),
                });
            }
            "set_thrust" => {
                let thrust = get_number(&self.get_named_arg_value(named_args, "thrust")?, 0.0);
                self.backend.execute_motion(MotionCommand::SetThrust {
                    thrust,
                    actuator: name.to_string(),
                });
            }
            "hover" => {
                self.backend.execute_motion(MotionCommand::Hover {
                    actuator: name.to_string(),
                });
            }
            "execute" => {
                // Emit output when as deref provides a agent.
                if let Some(agent) = self.current_agent.clone() {
                    self.check_agent_capability(&agent, "propose_motion", None, line)?;
                }
                let action_val = if let Some(first) = args.first() {
                    self.eval_expr(first)?
                } else {
                    self.get_named_arg_value(named_args, "action")?
                };

                // Take the branch when is safe action is false.
                if !is_safe_action(&action_val) {
                    // Take this path when is action proposal(&action val).
                    if is_action_proposal(&action_val) {
                        return Err(RuntimeError::new(
                            "Unsafe AI action: ActionProposal cannot execute actuators — call safety.validate() first",
                            line,
                        )
                        .into_spanda());
                    }
                    return Err(RuntimeError::new(
                        "Actuator execute() requires SafeAction from safety.validate()",
                        line,
                    )
                    .into_spanda());
                }

                // Take the branch when check safety before motion is false.
                if !self.check_safety_before_motion() {
                    // Emit output when on motion blocked provides a cb.
                    if let Some(cb) = &self.options.on_motion_blocked {
                        cb("Safety rule triggered — motion blocked".into());
                    }
                    self.backend.execute_motion(MotionCommand::Stop {
                        actuator: name.to_string(),
                    });
                    return Ok(RuntimeValue::Void);
                }

                // Take this path when let RuntimeValue::SafeAction { linear, angular } = action val.
                if let RuntimeValue::SafeAction { linear, angular } = action_val {
                    // Re-clamp SafeAction speeds so execute() cannot exceed current envelope.
                    let pose = self.backend.get_state().pose;
                    let pose2d = Pose2d {
                        x: pose.x,
                        y: pose.y,
                    };
                    let (clamped_linear, clamped_angular) = match self.safety_monitor.as_ref() {
                        Some(m) => (
                            m.clamp_speed_at_pose(linear, &pose2d),
                            m.clamp_angular(angular),
                        ),
                        None => (linear, angular),
                    };
                    self.backend.execute_motion(MotionCommand::Drive {
                        linear: clamped_linear,
                        angular: clamped_angular,
                        actuator: name.to_string(),
                    });
                }
            }
            _ => {}
        }
        self.log(format!("{name}.{method}()"));
        Ok(RuntimeValue::Void)
    }
}
