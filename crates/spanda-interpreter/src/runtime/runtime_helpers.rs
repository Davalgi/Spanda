//! Shared interpreter helpers used across runtime child modules.
//!

use super::{Interpreter, RobotBackend, RuntimeValue};
use spanda_ai::MemoryStore;
use spanda_ast::nodes::{AgentDecl, Expr};
use spanda_error::SpandaError;

impl<B: RobotBackend> Interpreter<B> {
    pub(super) fn goal_text_from_value(value: &RuntimeValue) -> Option<String> {
        // Description:
        //     Goal text from value.
        //
        // Inputs:
        //     value: &RuntimeValue
        //         Caller-supplied value.
        //
        // Outputs:
        //     result: Option<String>
        //         Return value from `goal_text_from_value`.
        //
        // Example:
        //     let result = spanda_interpreter::runtime_helpers::goal_text_from_value(value);

        // Match on value and handle each case.
        match value {
            RuntimeValue::Goal { text } => Some(text.clone()),
            RuntimeValue::String { value } => Some(value.clone()),
            _ => None,
        }
    }

    pub(super) fn resolve_reason_goal(
        &mut self,
        named_args: &[spanda_ast::nodes::NamedArg],
        line: u32,
    ) -> Result<Option<String>, SpandaError> {
        // Description:
        //     Resolve reason goal.
        //
        // Inputs:
        //     &mut self: input value
        //         Caller-supplied &mut self.
        //     named_args: &[spanda_ast::nodes::NamedArg]
        //         Caller-supplied named args.
        //     line: u32
        //         Caller-supplied line.
        //
        // Outputs:
        //     result: Result<Option<String>, SpandaError>
        //         Return value from `resolve_reason_goal`.
        //
        // Example:
        //     let result = spanda_interpreter::runtime_helpers::resolve_reason_goal(&mut self, named_args, line);

        // handle the success value from get named arg value.
        if let Ok(value) = self.get_named_arg_value(named_args, "goal") {
            // Keep entries that match the expected pattern.
            if !matches!(value, RuntimeValue::Void) {
                return Ok(Self::goal_text_from_value(&value));
            }
        }

        // Emit output when as deref provides a agent name.
        if let Some(agent_name) = self.current_agent.as_deref() {
            // Emit output when get provides a agent.
            if let Some(agent) = self.agents.get(agent_name) {
                let text = match &agent.decl {
                    AgentDecl::AgentDecl { goal, .. } => goal.clone(),
                };

                // Skip further work when !text is empty.
                if !text.is_empty() {
                    return Ok(Some(text));
                }
            }
        }
        let _ = line;
        Ok(None)
    }

    pub(super) fn enrich_reason_goal(&self, goal: Option<String>) -> Option<String> {
        // Description:
        //     Enrich reason goal.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //     goal: Option<String>
        //         Caller-supplied goal.
        //
        // Outputs:
        //     result: Option<String>
        //         Return value from `enrich_reason_goal`.
        //
        // Example:
        //     let result = spanda_interpreter::runtime_helpers::enrich_reason_goal(&self, goal);

        // Create mutable parts for accumulating results.
        let mut parts = Vec::new();

        // Emit output when is empty provides a g.
        if let Some(g) = goal.filter(|s| !s.is_empty()) {
            parts.push(g);
        }

        // Emit output when as deref provides a agent name.
        if let Some(agent_name) = self.current_agent.as_deref() {
            // Emit output when self provides a summary.
            if let Some(summary) = self
                .agents
                .get(agent_name)
                .and_then(|a| a.memory.as_ref())
                .and_then(MemoryStore::summary_for_prompt)
            {
                parts.push(summary);
            }
        }

        // Skip further work when parts is empty.
        if parts.is_empty() {
            None
        } else {
            Some(parts.join("\n"))
        }
    }

    pub(super) fn expr_path_string(expr: &Expr) -> String {
        // Description:
        //     Expr path string.
        //
        // Inputs:
        //     expr: &Expr
        //         Caller-supplied expr.
        //
        // Outputs:
        //     result: String
        //         Return value from `expr_path_string`.
        //
        // Example:
        //     let result = spanda_interpreter::runtime_helpers::expr_path_string(expr);

        // Match on expr and handle each case.
        match expr {
            Expr::IdentExpr { name, .. } => name.clone(),
            Expr::MemberExpr {
                object, property, ..
            } => {
                format!("{}.{}", Self::expr_path_string(object), property)
            }
            _ => String::new(),
        }
    }

    pub(super) fn runtime_value_payload(value: &RuntimeValue) -> String {
        // Description:
        //     Runtime value payload.
        //
        // Inputs:
        //     value: &RuntimeValue
        //         Caller-supplied value.
        //
        // Outputs:
        //     result: String
        //         Return value from `runtime_value_payload`.
        //
        // Example:
        //     let result = spanda_interpreter::runtime_helpers::runtime_value_payload(value);

        // Match on value and handle each case.
        match value {
            RuntimeValue::String { value } => value.clone(),
            RuntimeValue::Number { value, .. } => value.to_string(),
            RuntimeValue::Bool { value } => value.to_string(),
            RuntimeValue::Pose { x, y, theta, z } => {
                format!(r#"{{"x":{x},"y":{y},"theta":{theta},"z":{z}}}"#)
            }
            RuntimeValue::SafeAction { linear, angular } => {
                format!(r#"{{"linear":{linear},"angular":{angular}}}"#)
            }
            RuntimeValue::ActionProposal {
                linear,
                angular,
                source,
                ..
            } => format!(r#"{{"linear":{linear},"angular":{angular},"source":"{source}"}}"#),
            _ => format!("{value:?}"),
        }
    }
}
