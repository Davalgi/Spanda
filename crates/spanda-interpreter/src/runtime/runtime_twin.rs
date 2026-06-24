//! Digital twin runtime method dispatch for the interpreter.
//!

use super::{
    get_number, get_string, Interpreter, IntoSpandaError, RobotBackend, RuntimeError, RuntimeValue,
};
use spanda_ast::nodes::{Expr, LiteralValue, UnitKind};
use spanda_error::SpandaError;
use spanda_runtime::twin::TwinRuntime;

impl<B: RobotBackend> Interpreter<B> {
    fn twin_runtime(&self, line: u32) -> Result<&TwinRuntime, SpandaError> {
        // Description:
        //     Twin runtime.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //     line: u32
        //         Caller-supplied line.
        //
        // Outputs:
        //     result: Result<&TwinRuntime, SpandaError>
        //         Return value from `twin_runtime`.
        //
        // Example:

        //     let result = spanda_interpreter::runtime_twin::twin_runtime(&self, line);

        self.twin
            .as_ref()
            .ok_or_else(|| RuntimeError::new("No digital twin configured", line).into_spanda())
    }

    pub(super) fn eval_twin_method(
        &mut self,
        method: &str,
        args: &[Expr],
        named_args: &[spanda_ast::nodes::NamedArg],
        line: u32,
    ) -> Result<RuntimeValue, SpandaError> {
        // Description:
        //     Eval twin method.
        //
        // Inputs:
        //     &mut self: input value
        //         Caller-supplied &mut self.
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
        //         Return value from `eval_twin_method`.
        //
        // Example:
        //     let result = spanda_interpreter::runtime_twin::eval_twin_method(&mut self, ethod, args, named_args, line);

        // Require a configured twin before dispatching methods.
        let _ = self.twin_runtime(line)?;
        self.refresh_twin_shadow_from_backend();

        // Match on method and handle each case.
        match method {
            "frame_count" => {
                let count = self.twin_runtime(line)?.replay_frame_count();
                Ok(RuntimeValue::Number {
                    value: count as f64,
                    unit: UnitKind::None,
                })
            }
            "mirror" => {
                let field = self.twin_field_name(args, named_args, line)?;
                self.twin_runtime(line)?
                    .shadow_field(&field)
                    .cloned()
                    .ok_or_else(|| {
                        RuntimeError::new(
                            format!("Twin has no mirrored shadow field '{field}'"),
                            line,
                        )
                        .into_spanda()
                    })
            }
            "replay" => {
                let twin = self.twin_runtime(line)?;

                // Take the branch when replay is false.
                if !twin.replay {
                    return Err(RuntimeError::new(
                        "Twin replay is disabled — set replay true in twin block",
                        line,
                    )
                    .into_spanda());
                }
                let index =
                    get_number(&self.get_named_arg_value(named_args, "index")?, 0.0) as usize;
                let field = self.twin_field_name(args, named_args, line)?;
                self.twin_runtime(line)?
                    .replay_field(index, &field)
                    .cloned()
                    .ok_or_else(|| {
                        RuntimeError::new(
                            format!("Twin replay frame {index} has no field '{field}'"),
                            line,
                        )
                        .into_spanda()
                    })
            }
            method => {
                let twin = self.twin_runtime(line)?;

                // Take this path when the twin mirrors the requested field.
                if twin.mirrors.iter().any(|m| m == method) {
                    twin.shadow_field(method).cloned().ok_or_else(|| {
                        RuntimeError::new(
                            format!("Twin shadow field '{method}' not yet mirrored"),
                            line,
                        )
                        .into_spanda()
                    })
                } else {
                    Ok(RuntimeValue::Void)
                }
            }
        }
    }

    fn twin_field_name(
        &mut self,
        args: &[Expr],
        named_args: &[spanda_ast::nodes::NamedArg],
        line: u32,
    ) -> Result<String, SpandaError> {
        // Description:
        //     Twin field name.
        //
        // Inputs:
        //     &mut self: input value
        //         Caller-supplied &mut self.
        //     args: &[Expr]
        //         Caller-supplied args.
        //     named_args: &[spanda_ast::nodes::NamedArg]
        //         Caller-supplied named args.
        //     line: u32
        //         Caller-supplied line.
        //
        // Outputs:
        //     result: Result<String, SpandaError>
        //         Return value from `twin_field_name`.
        //
        // Example:
        //     let result = spanda_interpreter::runtime_twin::twin_field_name(&mut self, args, named_args, line);

        // Apply each command-line argument.
        for arg in named_args {
            // Take the branch when name equals "field".
            if arg.name == "field" {
                return self.twin_field_from_expr(&arg.value, line);
            }
        }

        // Emit output when first provides a arg.
        if let Some(arg) = args.first() {
            return self.twin_field_from_expr(arg, line);
        }
        Err(RuntimeError::new("Expected 'field' argument for twin method", line).into_spanda())
    }

    fn twin_field_from_expr(&mut self, expr: &Expr, _line: u32) -> Result<String, SpandaError> {
        // Description:
        //     Twin field from expr.
        //
        // Inputs:
        //     &mut self: input value
        //         Caller-supplied &mut self.
        //     expr: &Expr
        //         Caller-supplied expr.
        //     _line: u32
        //         Caller-supplied line.
        //
        // Outputs:
        //     result: Result<String, SpandaError>
        //         Return value from `twin_field_from_expr`.
        //
        // Example:
        //     let result = spanda_interpreter::runtime_twin::twin_field_from_expr(&mut self, expr, _line);

        // Match on expr and handle each case.
        match expr {
            Expr::LiteralExpr {
                value: LiteralValue::String(s),
                ..
            } => Ok(s.clone()),
            Expr::IdentExpr { name, .. } => Ok(name.clone()),
            _ => Ok(get_string(&self.eval_expr(expr)?, "")),
        }
    }
}
