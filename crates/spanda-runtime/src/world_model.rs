//! Minimal world-model belief state from recent observations.
//!

use crate::value::RuntimeValue;
use serde_json::{json, Value};

const MAX_OBSERVATIONS: usize = 32;

/// Rolling observation buffer with a simple confidence belief.
#[derive(Debug, Clone, Default)]
pub struct WorldModelRuntime {
    observations: Vec<Value>,
    belief_confidence: f64,
}

impl WorldModelRuntime {
    /// Create an empty world model.
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

        //     let value = spanda_runtime::world_model::new();

        Self::default()
    }

    /// Record one observation and refresh belief confidence.
    pub fn update(&mut self, observation: &RuntimeValue) -> f64 {
        // Description:
        //     Update.
        //
        // Inputs:
        //     &mut self: input value
        //         Caller-supplied &mut self.
        //     observation: &RuntimeValue
        //         Caller-supplied observation.
        //
        // Outputs:
        //     result: f64
        //         Return value from `update`.
        //
        // Example:

        //     let result = spanda_runtime::world_model::update(&mut self, observation);

        self.observations.push(runtime_value_to_json(observation));
        if self.observations.len() > MAX_OBSERVATIONS {
            let overflow = self.observations.len() - MAX_OBSERVATIONS;
            self.observations.drain(0..overflow);
        }
        self.belief_confidence = Self::confidence_from_observations(&self.observations);
        self.belief_confidence
    }

    /// Return the current belief confidence.
    pub fn belief_confidence(&self) -> f64 {
        // Description:
        //     Belief confidence.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //
        // Outputs:
        //     result: f64
        //         Return value from `belief_confidence`.
        //
        // Example:

        //     let result = spanda_runtime::world_model::belief_confidence(&self);

        self.belief_confidence
    }

    /// Export observations and belief for cloud upload or replay.
    pub fn export_json(&self) -> Value {
        // Description:
        //     Export json.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //
        // Outputs:
        //     result: Value
        //         Return value from `export_json`.
        //
        // Example:

        //     let result = spanda_runtime::world_model::export_json(&self);

        json!({
            "observations": self.observations,
            "belief_confidence": self.belief_confidence,
        })
    }

    fn confidence_from_observations(observations: &[Value]) -> f64 {
        // Description:
        //     Confidence from observations.
        //
        // Inputs:
        //     observations: &[Value]
        //         Caller-supplied observations.
        //
        // Outputs:
        //     result: f64
        //         Return value from `confidence_from_observations`.
        //
        // Example:

        //     let result = spanda_runtime::world_model::confidence_from_observations(observations);

        if observations.is_empty() {
            return 0.0;
        }
        let capped = observations.len().min(MAX_OBSERVATIONS) as f64;
        (capped / MAX_OBSERVATIONS as f64).clamp(0.1, 1.0)
    }
}

fn runtime_value_to_json(value: &RuntimeValue) -> Value {
    // Description:
    //     Runtime value to json.
    //
    // Inputs:
    //     value: &RuntimeValue
    //         Caller-supplied value.
    //
    // Outputs:
    //     result: Value
    //         Return value from `runtime_value_to_json`.
    //
    // Example:

    //     let result = spanda_runtime::world_model::runtime_value_to_json(value);

    match value {
        RuntimeValue::Number { value, unit } => {
            json!({ "kind": "number", "value": value, "unit": format!("{unit:?}") })
        }
        RuntimeValue::Bool { value } => json!({ "kind": "bool", "value": value }),
        RuntimeValue::String { value } => json!({ "kind": "string", "value": value }),
        RuntimeValue::Object { type_name, fields } => {
            let mut map = serde_json::Map::new();
            map.insert("kind".into(), json!("object"));
            map.insert("type_name".into(), json!(type_name));
            for (key, field_value) in fields {
                map.insert(key.clone(), runtime_value_to_json(field_value));
            }
            Value::Object(map)
        }
        other => json!({ "kind": "opaque", "value": format!("{other:?}") }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use spanda_ast::nodes::UnitKind;

    #[test]
    fn update_increases_belief_with_observations() {
        // Description:
        //     Update increases belief with observations.
        //
        // Inputs:
        //     None.
        //
        // Outputs:
        //     None.
        //
        // Example:

        //     let result = spanda_runtime::world_model::update_increases_belief_with_observations();

        let mut model = WorldModelRuntime::new();
        assert_eq!(model.belief_confidence(), 0.0);
        let obs = RuntimeValue::Number {
            value: 1.0,
            unit: UnitKind::None,
        };
        let belief = model.update(&obs);
        assert!(belief > 0.0);
        assert_eq!(model.belief_confidence(), belief);
    }
}
