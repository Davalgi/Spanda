//! Robotics platform primitives: mission lifecycle, fleet grouping, and navigation helpers.
//!
//! Core language constructs (`mission`, `fleet`, `safety_zone`) are parsed into AST nodes in
//! [`crate::foundations`]. This module holds shared runtime state and validation helpers.

pub use spanda_ast::robotics_decl::*;
pub use spanda_runtime::robotics::{
    FleetRegistry, MissionRuntime, MissionState, ProgramSafetyZoneRegistry,
};
pub use spanda_runtime_host::robotics_validation::{
    validate_fleet_members, validate_mission_decl, validate_swarm_fleet,
};

/// Validate certification standard identifiers at parse/type-check time.
pub fn validate_certification_standard(name: &str) -> Option<String> {
    if CertificationStandard::parse_ident(name).is_some() {
        return None;
    }
    Some(format!(
        "unknown certification standard '{name}' (expected ISO13849, IEC61508, or ISO26262)"
    ))
}
