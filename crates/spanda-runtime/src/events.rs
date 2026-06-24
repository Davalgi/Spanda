//! Declarative event dispatch for Spanda programs.
//!
//! Maps event names declared in source to handler statement blocks. The runtime
//! [`crate::runtime::Interpreter`] registers handlers at load time and dispatches
//! them when triggers or explicit `emit` statements fire matching events.

use spanda_ast::nodes::Stmt;
use std::collections::HashMap;
/// Event bus mapping declared events to handler bodies.
///
/// Stores a name-to-statements table populated from `on event` declarations in
/// a Spanda program. Handlers are executed by the interpreter when a matching
/// event is dispatched.
///
/// # Parameters
///
/// None — construct with [`EventBus::new`].
///
/// # Returns
///
/// N/A (type definition).
///
/// # Options
///
/// None.
///
/// # Example
///
/// ```
/// use spanda_ast::nodes::Stmt;
/// use spanda_runtime::events::EventBus;
///
/// let mut bus = EventBus::new();
/// bus.register("obstacle_detected".into(), vec![]);
/// assert!(bus.handler_body("obstacle_detected").is_some());
/// ```
pub struct EventBus {
    handlers: HashMap<String, Vec<Stmt>>,
}

impl EventBus {
    pub fn new() -> Self {
        // Description:
        //     Construct a new instance.
        //
        // Inputs:
        //     None.
        //
        // Outputs:
        //     result: Self
        //         Return value from `new`.
        //
        // Example:
        //     let value = spanda_runtime::events::new();

        // assert!(bus.handler_body("any").is_none());
        Self {
            handlers: HashMap::new(),
        }
    }

    pub fn register(&mut self, event: String, body: Vec<Stmt>) {
        // Description:
        //     Register.
        //
        // Inputs:
        //     &mut self: input value
        //         Caller-supplied &mut self.
        //     even: String
        //         Caller-supplied even.
        //     body: Vec<Stmt>
        //         Caller-supplied body.
        //
        // Outputs:
        //     None.
        //
        // Example:
        //     let result = spanda_runtime::events::register(&mut self, even, body);

        // bus.register("tick".into(), vec![]);
        self.handlers.insert(event, body);
    }

    pub fn handler_body(&self, event: &str) -> Option<&[Stmt]> {
        // Description:
        //     Handler body.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //     even: &str
        //         Caller-supplied even.
        //
        // Outputs:
        //     result: Option<&[Stmt]>
        //         Return value from `handler_body`.
        //
        // Example:
        //     let result = spanda_runtime::events::handler_body(&self, even);

        // assert!(bus.handler_body("missing").is_none());
        self.handlers.get(event).map(|v| v.as_slice())
    }
}

impl Default for EventBus {
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
        //     let result = spanda_runtime::events::default();

        // Build the result via new.
        Self::new()
    }
}
