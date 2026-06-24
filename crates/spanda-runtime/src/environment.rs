//! Variable binding environment for the Spanda interpreter.
//!
use crate::value::{format_runtime_value, RuntimeValue};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Environment {
    bindings: HashMap<String, RuntimeValue>,
}

impl Environment {
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
        //     let value = spanda_runtime::environment::new();

        // Assemble the struct fields and return it.
        Self {
            bindings: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: impl Into<String>, value: RuntimeValue) {
        // Description:
        //     Define.
        //
        // Inputs:
        //     &mut self: input value
        //         Caller-supplied &mut self.
        //     name: impl Into<String>
        //         Caller-supplied name.
        //     value: RuntimeValue
        //         Caller-supplied value.
        //
        // Outputs:
        //     None.
        //
        // Example:
        //     let result = spanda_runtime::environment::define(&mut self, name, value);

        // Append into self.
        self.bindings.insert(name.into(), value);
    }

    pub fn get(&self, name: &str) -> Option<&RuntimeValue> {
        // Description:
        //     Get.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //     name: &str
        //         Caller-supplied name.
        //
        // Outputs:
        //     result: Option<&RuntimeValue>
        //         Return value from `get`.
        //
        // Example:
        //     let result = spanda_runtime::environment::get(&self, name);

        // Call get on the current instance.
        self.bindings.get(name)
    }

    pub fn remove(&mut self, name: &str) -> Option<RuntimeValue> {
        // Description:
        //     Remove.
        //
        // Inputs:
        //     &mut self: input value
        //         Caller-supplied &mut self.
        //     name: &str
        //         Caller-supplied name.
        //
        // Outputs:
        //     result: Option<RuntimeValue>
        //         Return value from `remove`.
        //
        // Example:

        //     let result = spanda_runtime::environment::remove(&mut self, name);

        self.bindings.remove(name)
    }

    pub fn set(&mut self, name: impl Into<String>, value: RuntimeValue) {
        // Description:
        //     Set.
        //
        // Inputs:
        //     &mut self: input value
        //         Caller-supplied &mut self.
        //     name: impl Into<String>
        //         Caller-supplied name.
        //     value: RuntimeValue
        //         Caller-supplied value.
        //
        // Outputs:
        //     None.
        //
        // Example:
        //     let result = spanda_runtime::environment::set(&mut self, name, value);

        // Append into self.
        self.bindings.insert(name.into(), value);
    }

    pub fn clone_bindings(&self) -> Self {
        // Description:
        //     Clone bindings.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //
        // Outputs:
        //     result: Self
        //         Return value from `clone_bindings`.
        //
        // Example:
        //     let result = spanda_runtime::environment::clone_bindings(&self);

        // Assemble the struct fields and return it.
        Self {
            bindings: self.bindings.clone(),
        }
    }

    pub fn snapshot_display(&self) -> std::collections::HashMap<String, String> {
        // Description:
        //     Snapshot display.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //
        // Outputs:
        //     result: std::collections::HashMap<String, String>
        //         Return value from `snapshot_display`.
        //
        // Example:
        //     let result = spanda_runtime::environment::snapshot_display(&self);

        // Call bindings on the current instance.
        self.bindings
            .iter()
            .map(|(name, value)| (name.clone(), format_runtime_value(value)))
            .collect()
    }
}

impl Default for Environment {
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
        //     let result = spanda_runtime::environment::default();

        // Build the result via new.
        Self::new()
    }
}
