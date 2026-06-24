//! Safety zone evaluation and motion gating.
//!

use super::{get_number, Interpreter, RobotBackend};
use spanda_ast::nodes::{SafetyZoneDecl, ZoneShape};
use spanda_error::SpandaError;
use spanda_safety::{SafetyZoneRuntime, SafetyZoneShape};

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
}
