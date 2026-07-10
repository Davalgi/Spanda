//! Safety zone evaluation and motion gating.
//!

use super::{get_number, Interpreter, MotionCommand, RobotBackend};
use spanda_ast::nodes::{SafetyZoneDecl, ZoneShape};
use spanda_error::SpandaError;
use spanda_safety::{Pose2d, SafetyZoneRuntime, SafetyZoneShape, ValidateActionResult};

impl<B: RobotBackend> Interpreter<B> {
    pub(super) fn eval_safety_zone(
        &mut self,
        zone: &SafetyZoneDecl,
    ) -> Result<SafetyZoneRuntime, SpandaError> {
        // Description:
        //     Eval safety zone.
        //
        // Inputs:
        //     &mut self: input value
        //         Caller-supplied &mut self.
        //     zone: &SafetyZoneDecl
        //         Caller-supplied zone.
        //
        // Outputs:
        //     result: Result<SafetyZoneRuntime, SpandaError>
        //         Return value from `eval_safety_zone`.
        //
        // Example:
        //     let result = spanda_interpreter::runtime_safety::eval_safety_zone(&mut self, zone);

        // Compute SafetyZoneDecl for the following logic.
        let SafetyZoneDecl::SafetyZoneDecl {
            name,
            shape,
            x,
            y,
            radius,
            width,
            height,
            ..
        } = zone;
        let mut runtime = SafetyZoneRuntime {
            name: name.clone(),
            shape: match shape {
                ZoneShape::Circle => SafetyZoneShape::Circle,
                ZoneShape::Rect => SafetyZoneShape::Rect,
            },
            x: get_number(&self.eval_expr(x)?, 0.0),
            y: get_number(&self.eval_expr(y)?, 0.0),
            radius: None,
            width: None,
            height: None,
        };

        // Take the branch when *shape equals Circle.
        if *shape == ZoneShape::Circle {
            // Emit output when radius provides a r.
            if let Some(r) = radius {
                runtime.radius = Some(get_number(&self.eval_expr(r)?, 0.0));
            }
        }

        // Take the branch when *shape equals Rect.
        if *shape == ZoneShape::Rect {
            // Emit output when width provides a w.
            if let Some(w) = width {
                runtime.width = Some(get_number(&self.eval_expr(w)?, 0.0));
            }

            // Emit output when height provides a h.
            if let Some(h) = height {
                runtime.height = Some(get_number(&self.eval_expr(h)?, 0.0));
            }
        }
        Ok(runtime)
    }

    pub(super) fn reclamp_active_follow_cruise(&mut self) {
        // Re-validate follow cruise like `safety.validate` at the current pose each tick.
        //
        // Parameters:
        // None.
        //
        // Returns:
        // Nothing.
        //
        // Options:
        // None.
        //
        // Example:
        // self.reclamp_active_follow_cruise();

        const DEFAULT_FOLLOW_SPEED: f64 = 0.5;
        let Some(monitor) = self.safety_monitor.as_ref() else {
            return;
        };
        let pose = self.backend.get_state().pose;
        let pose2d = Pose2d {
            x: pose.x,
            y: pose.y,
        };
        // Re-derive a SafeAction-equivalent envelope for the follow cruise segment.
        let result =
            monitor.validate_action_proposal(DEFAULT_FOLLOW_SPEED, 0.0, &self.env, &pose2d);
        match result {
            ValidateActionResult::Ok(motion) => {
                self.backend.reclamp_follow_cruise(motion.linear.abs());
            }
            ValidateActionResult::Err { .. } => {
                // Stop following when stop_if / e-stop / policy rejects the segment.
                self.backend.execute_motion(MotionCommand::Stop {
                    actuator: "follow".into(),
                });
            }
        }
    }
}
