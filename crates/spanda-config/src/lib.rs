//! Cascading TOML configuration resolution for Spanda autonomous systems.
//!
//! This crate sits between project/package loading and verification/runtime.
//! Downstream consumers should use [`ResolvedSystemConfig`] rather than raw
//! TOML or JSON files.
//!
pub mod device_tree;
pub mod error;
pub mod json;
pub mod layer;
pub mod manifest;
pub mod mapping;
pub mod reports;
pub mod resolved;
pub mod resolver;
pub mod validation;

pub use device_tree::{ComputeNode, DeviceNode, DeviceTree, FleetNode, RobotNode};
pub use error::{ConfigError, ConfigResult};
pub use json::{load_config_value, parse_config_str};
pub use layer::{ConfigGraph, ConfigGraphEdge, ConfigLayer, ConfigMergeStrategy};
pub use manifest::{
    ConfigReferences, ExtendsSection, MergeStrategyHint, ProjectSection, SpandaManifest,
    MANIFEST_FILENAME,
};
pub use mapping::{ActuatorMapping, LogicalPhysicalMap, RobotMapping, SensorMapping};
pub use reports::{
    config_drift_report, format_report_text, generate_report_bundle, ConfigReportBundle,
};
pub use resolved::ResolvedSystemConfig;
pub use resolver::{diff_configs, merge_values, ConfigResolver, ResolverOptions};
pub use validation::{
    validate_device_tree, validate_logical_map, ConfigValidationReport, ValidationFinding,
    ValidationSeverity,
};
