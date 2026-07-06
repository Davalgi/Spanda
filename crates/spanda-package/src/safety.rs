//! safety support for Spanda.
//!
use serde::{Deserialize, Serialize};
use std::str::FromStr;

/// Package trust / safety level for deployment gating.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum SafetyLevel {
    #[default]
    Experimental,
    SimulationOnly,
    #[serde(alias = "hardware")]
    HardwareSafe,
    Certified,
}

impl SafetyLevel {
    pub fn all() -> &'static [SafetyLevel] {
        // Description:
        //     All.
        //
        // Inputs:
        //     None.
        //
        // Outputs:
        //     result: &'static [SafetyLevel]
        //         Return value from `all`.
        //
        // Example:
        //     let result = spanda_package::safety::all();

        // Return the static list of known values.
        &[
            Self::Experimental,
            Self::SimulationOnly,
            Self::HardwareSafe,
            Self::Certified,
        ]
    }

    pub fn as_str(&self) -> &'static str {
        // Description:
        //     As str.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //
        // Outputs:
        //     result: &'static str
        //         Return value from `as_str`.
        //
        // Example:
        //     let result = spanda_package::safety::as_str(&self);

        // Dispatch based on the enum variant or current state.
        match self {
            Self::Experimental => "experimental",
            Self::SimulationOnly => "simulation_only",
            Self::HardwareSafe => "hardware_safe",
            Self::Certified => "certified",
        }
    }

    /// Whether this level may control physical actuators on real hardware.
    pub fn can_control_actuators_default(&self) -> bool {
        // Description:
        //     Can control actuators default.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //
        // Outputs:
        //     result: bool
        //         Return value from `can_control_actuators_default`.
        //
        // Example:
        //     let result = spanda_package::safety::can_control_actuators_default(&self);

        // Produce Certified) as the result.
        matches!(self, Self::HardwareSafe | Self::Certified)
    }

    /// Whether packages at this level require manual review before deployment.
    pub fn requires_review_default(&self) -> bool {
        // Description:
        //     Requires review default.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //
        // Outputs:
        //     result: bool
        //         Return value from `requires_review_default`.
        //
        // Example:
        //     let result = spanda_package::safety::requires_review_default(&self);

        // Produce SimulationOnly) as the result.
        matches!(self, Self::Experimental | Self::SimulationOnly)
    }
}

impl FromStr for SafetyLevel {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Description:
        //     From str.
        //
        // Inputs:
        //     s: &str
        //         Caller-supplied s.
        //
        // Outputs:
        //     result: Result<Self, Self::Err>
        //         Return value from `from_str`.
        //
        // Example:
        //     let result = spanda_package::safety::from_str(s);

        // Match on s and handle each case.
        match s {
            "experimental" => Ok(Self::Experimental),
            "simulation_only" => Ok(Self::SimulationOnly),
            "hardware" | "hardware_safe" => Ok(Self::HardwareSafe),
            "certified" => Ok(Self::Certified),
            other => Err(format!("unknown safety level '{other}'")),
        }
    }
}

/// Safety metadata from `[safety]` in spanda.toml.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SafetyMetadata {
    #[serde(default)]
    pub level: SafetyLevel,
    #[serde(default = "default_true")]
    pub requires_review: bool,
    #[serde(default)]
    pub can_control_actuators: bool,
}

fn default_true() -> bool {
    // Description:
    //     Default true.
    //
    // Inputs:
    //     None.
    //
    // Outputs:
    //     result: bool
    //         Return value from `default_true`.
    //
    // Example:
    //     let result = spanda_package::safety::default_true();

    // Produce true as the result.
    true
}

impl Default for SafetyMetadata {
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
        //     let result = spanda_package::safety::default();

        // Compute level for the following logic.
        let level = SafetyLevel::Experimental;
        Self {
            level,
            requires_review: level.requires_review_default(),
            can_control_actuators: level.can_control_actuators_default(),
        }
    }
}

impl SafetyMetadata {
    pub fn normalize(&mut self) {
        // Description:
        //     Normalize.
        //
        // Inputs:
        //     &mut self: input value
        //         Caller-supplied &mut self.
        //
        // Outputs:
        //     None.
        //
        // Example:
        //     let result = spanda_package::safety::normalize(&mut self);

        // take the branch when level equals can control actuators.
        if self.level == SafetyLevel::default() && !self.can_control_actuators {
            self.requires_review = self.level.requires_review_default();
        }
    }
}
