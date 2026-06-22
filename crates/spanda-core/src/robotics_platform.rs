//! Robotics platform primitives re-exported from workspace crates.
//!
pub use spanda_ast::robotics_decl::*;
pub use spanda_runtime::robotics::{
    FleetRegistry, MissionRuntime, MissionState, ProgramSafetyZoneRegistry,
};
pub use spanda_runtime_host::robotics_validation::{
    validate_certification_standard, validate_fleet_members, validate_mission_decl,
    validate_swarm_fleet,
};
