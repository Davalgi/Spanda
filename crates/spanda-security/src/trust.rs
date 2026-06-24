//! trust support for Spanda.
//!
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::str::FromStr;

/// Runtime trust tier for devices, packages, and communication endpoints.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum TrustLevel {
    #[default]
    Untrusted,
    Restricted,
    Trusted,
    Certified,
}

impl TrustLevel {
    pub fn all() -> &'static [TrustLevel] {
        // Description:
        //     All.
        //
        // Inputs:
        //     None.
        //
        // Outputs:
        //     result: &'static [TrustLevel]
        //         Return value from `all`.
        //
        // Example:
        //     let result = spanda_security::trust::all();

        // Return the static list of known values.
        &[
            Self::Untrusted,
            Self::Restricted,
            Self::Trusted,
            Self::Certified,
        ]
    }

    pub fn rank(self) -> u8 {
        // Description:
        //     Rank.
        //
        // Inputs:
        //     None.
        //
        // Outputs:
        //     result: u8
        //         Return value from `rank`.
        //
        // Example:
        //     let result = instance.rank();

        // Dispatch based on the enum variant or current state.
        match self {
            Self::Untrusted => 0,
            Self::Restricted => 1,
            Self::Trusted => 2,
            Self::Certified => 3,
        }
    }

    pub fn as_str(self) -> &'static str {
        // Description:
        //     As str.
        //
        // Inputs:
        //     None.
        //
        // Outputs:
        //     result: &'static str
        //         Return value from `as_str`.
        //
        // Example:
        //     let result = instance.as_str();

        // Dispatch based on the enum variant or current state.
        match self {
            Self::Untrusted => "untrusted",
            Self::Restricted => "restricted",
            Self::Trusted => "trusted",
            Self::Certified => "certified",
        }
    }

    pub fn satisfies(self, required: TrustLevel) -> bool {
        // Description:
        //     Satisfies.
        //
        // Inputs:
        //     required: TrustLevel
        //         Caller-supplied required.
        //
        // Outputs:
        //     result: bool
        //         Return value from `satisfies`.
        //
        // Example:
        //     let result = instance.satisfies(required);

        // Call rank on the current instance.
        self.rank().cmp(&required.rank()) != Ordering::Less
    }
}

impl FromStr for TrustLevel {
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
        //     let result = spanda_security::trust::from_str(s);

        // Match on s and handle each case.
        match s {
            "untrusted" => Ok(Self::Untrusted),
            "restricted" => Ok(Self::Restricted),
            "trusted" => Ok(Self::Trusted),
            "certified" => Ok(Self::Certified),
            other => Err(format!("unknown trust level '{other}'")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trust_ordering() {
        // Description:
        //     Trust ordering.
        //
        // Inputs:
        //     None.
        //
        // Outputs:
        //     None.
        //
        // Example:

        //     let result = spanda_security::trust::trust_ordering();

        assert!(TrustLevel::Certified.satisfies(TrustLevel::Trusted));
        assert!(!TrustLevel::Restricted.satisfies(TrustLevel::Trusted));
    }
}
