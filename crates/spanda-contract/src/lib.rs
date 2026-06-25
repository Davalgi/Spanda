//! Mission contract static verification for Spanda programs.
//!
//! Composes existing `mission_plan`, robot `mission`, `safety`, `continuity_policy`,
//! and `recovery_policy` declarations into verifiable contract reports without
//! requiring a separate contract syntax yet.

mod report;
mod verify;

pub use report::{
    format_contract_report, ContractCheck, ContractVerificationReport, MissionContractReport,
};
pub use verify::verify_contract;
