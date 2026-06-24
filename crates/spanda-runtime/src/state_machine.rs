//! Runtime execution of declarative state machines.
//!
//! Tracks the active state of a `state_machine` block and enforces allowed
//! `(from, to)` transitions declared in source. Used by the interpreter for
//! `enter` statements and state-entry/exit triggers.
/// Runtime state for a declared state machine with validated transitions.
///
/// Holds the machine name, current state, the full state list, and the
/// directed transition graph from the AST.
///
/// # Parameters
///
/// None — construct with [`StateMachineRuntime::new`].
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
/// use spanda_runtime::state_machine::StateMachineRuntime;
///
/// let mut sm = StateMachineRuntime::new(
///     "Nav".into(),
///     vec!["Idle".into(), "Moving".into()],
///     vec![("Idle".into(), "Moving".into())],
/// );
/// assert_eq!(sm.current, "Idle");
/// ```
pub struct StateMachineRuntime {
    pub name: String,
    pub current: String,
    states: Vec<String>,
    transitions: Vec<(String, String)>,
}

impl StateMachineRuntime {
    pub fn new(name: String, states: Vec<String>, transitions: Vec<(String, String)>) -> Self {
        // Description:
        //     Construct a new instance.
        //
        // Inputs:
        //     name: String
        //         Caller-supplied name.
        //     states: Vec<String>
        //         Caller-supplied states.
        //     ransitions: Vec<(String, String)>
        //         Caller-supplied ransitions.
        //
        // Outputs:
        //     result: Self
        //         Return value from `new`.
        //
        // Example:

        //     let value = spanda_runtime::state_machine::new(name, states, ransitions);
        let current = states.first().cloned().unwrap_or_default();
        Self {
            name,
            current,
            states,
            transitions,
        }
    }

    pub fn try_enter(&mut self, target: &str) -> Option<String> {
        // Description:
        //     Try enter.
        //
        // Inputs:
        //     &mut self: input value
        //         Caller-supplied &mut self.
        //     arge: &str
        //         Caller-supplied arge.
        //
        // Outputs:
        //     result: Option<String>
        //         Return value from `try_enter`.
        //
        // Example:

        //     let result = spanda_runtime::state_machine::try_enter(&mut self, arge);
        if !self.states.iter().any(|s| s == target) {
            return None;
        }
        let allowed = self
            .transitions
            .iter()
            .any(|(from, to)| from == &self.current && to == target);

        // Take the branch when allowed is false.
        if !allowed {
            return None;
        }
        let previous = self.current.clone();
        self.current = target.to_string();
        Some(previous)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn try_enter_follows_declared_transitions() {
        // Description:
        //     Try enter follows declared transitions.
        //
        // Inputs:
        //     None.
        //
        // Outputs:
        //     None.
        //
        // Example:

        //     let result = spanda_runtime::state_machine::try_enter_follows_declared_transitions();

        let mut sm = StateMachineRuntime::new(
            "Flow".into(),
            vec!["Idle".into(), "Loading".into()],
            vec![("Idle".into(), "Loading".into())],
        );
        assert_eq!(sm.current, "Idle");
        assert_eq!(sm.try_enter("Loading"), Some("Idle".into()));
        assert_eq!(sm.current, "Loading");
        assert_eq!(sm.try_enter("Idle"), None);
    }
}
