//! World model runtime method dispatch.
//!

use super::{Interpreter, RobotBackend, RuntimeError, RuntimeValue};
use spanda_ast::nodes::{Expr, UnitKind};
use spanda_error::SpandaError;

impl<B: RobotBackend> Interpreter<B> {
    pub(super) fn eval_world_model_method(
        &mut self,
        method: &str,
        args: &[Expr],
        _named_args: &[spanda_ast::nodes::NamedArg],
        line: u32,
    ) -> Result<RuntimeValue, SpandaError> {
        // Description:
        //     Eval world model method.
        //
        // Inputs:
        //     &mut self: input value
        //         Caller-supplied &mut self.
        //     ethod: &str
        //         Caller-supplied ethod.
        //     args: &[Expr]
        //         Caller-supplied args.
        //     _named_args: &[spanda_ast::nodes::NamedArg]
        //         Caller-supplied named args.
        //     line: u32
        //         Caller-supplied line.
        //
        // Outputs:
        //     result: Result<RuntimeValue, SpandaError>
        //         Return value from `eval_world_model_method`.
        //
        // Example:

        //     let result = spanda_interpreter::runtime_world_model::eval_world_model_method(&mut self, ethod, args, _named_args, line);

        match method {
            "update" => {
                let observation = if let Some(arg) = args.first() {
                    self.eval_expr(arg)?
                } else {
                    return Err(RuntimeError::new(
                        "world_model.update requires an observation",
                        line,
                    )
                    .into());
                };
                let confidence = self.world_model.update(&observation);
                self.log(format!("world_model.update -> belief {confidence:.2}"));
                Ok(RuntimeValue::Number {
                    value: confidence,
                    unit: UnitKind::None,
                })
            }
            "belief" => Ok(RuntimeValue::Number {
                value: self.world_model.belief_confidence(),
                unit: UnitKind::None,
            }),
            "export" => {
                let json = self.world_model.export_json().to_string();
                Ok(RuntimeValue::String { value: json })
            }
            other => {
                Err(RuntimeError::new(format!("unknown world_model method: {other}"), line).into())
            }
        }
    }
}
