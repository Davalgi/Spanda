//! Module ownership classification for lean-core refactor tracking.
//!

/// Where a module or feature belongs in the lean-core architecture.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModuleOwnership {
    /// Language compiler, type system, safety contracts, runtime kernel.
    Core,

    /// Built-in `std.*` type definitions without vendor drivers.
    StandardLibrary,

    /// First-party optional package (registry under `packages/registry/`).
    OfficialPackage,

    /// Community or experimental package.
    ExperimentalPackage,

    /// Retained for backward compatibility; implementation moving to packages.
    CompatibilityShim,

    /// Scheduled for removal after migration period.
    Deprecated,
}

/// Record describing a core module's refactor status.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModuleClassification {
    pub module: &'static str,
    pub ownership: ModuleOwnership,
    pub target_package: Option<&'static str>,
    pub notes: &'static str,
}

/// Static audit table used by docs and migration tooling.
pub fn module_classifications() -> &'static [ModuleClassification] {
    // Description:
    //     Module classifications.
    //
    // Inputs:
    //     None.
    //
    // Outputs:
    //     result: &'static [ModuleClassification]
    //         Return value from `module_classifications`.
    //
    // Example:

    //     let result = spanda_runtime::classification::module_classifications();

    &[
        ModuleClassification {
            module: "lexer",
            ownership: ModuleOwnership::Core,
            target_package: None,
            notes: "Compiler front-end",
        },
        ModuleClassification {
            module: "parser",
            ownership: ModuleOwnership::Core,
            target_package: None,
            notes: "Compiler front-end",
        },
        ModuleClassification {
            module: "type_system",
            ownership: ModuleOwnership::Core,
            target_package: None,
            notes: "Type checker and std namespace registry",
        },
        ModuleClassification {
            module: "safety",
            ownership: ModuleOwnership::Core,
            target_package: None,
            notes: "ActionProposal / SafeAction gate",
        },
        ModuleClassification {
            module: "scheduler",
            ownership: ModuleOwnership::Core,
            target_package: None,
            notes: "Task and trigger scheduling interfaces",
        },
        ModuleClassification {
            module: "providers",
            ownership: ModuleOwnership::Core,
            target_package: None,
            notes: "Extension trait contracts for packages",
        },
        ModuleClassification {
            module: "connectivity_positioning",
            ownership: ModuleOwnership::CompatibilityShim,
            target_package: Some("spanda-gps / spanda-wifi / spanda-ble / spanda-cellular"),
            notes: "Type names stay in core; drivers move to connectivity packages",
        },
        ModuleClassification {
            module: "transport_mqtt",
            ownership: ModuleOwnership::Deprecated,
            target_package: Some("spanda-mqtt"),
            notes: "Removed from spanda-core; use spanda-transport-mqtt or spanda-transport-routing",
        },
        ModuleClassification {
            module: "transport",
            ownership: ModuleOwnership::Deprecated,
            target_package: Some("spanda-transport-routing"),
            notes: "Removed from spanda-core; use spanda_transport_routing::RoutingCommBus",
        },
        ModuleClassification {
            module: "transport_wire",
            ownership: ModuleOwnership::Deprecated,
            target_package: Some("spanda-transport"),
            notes: "Removed from spanda-core; use spanda_transport wire helpers",
        },
        ModuleClassification {
            module: "transport_security",
            ownership: ModuleOwnership::Deprecated,
            target_package: Some("spanda-transport"),
            notes: "Removed from spanda-core; use spanda_transport::security",
        },
        ModuleClassification {
            module: "transport_rclrs",
            ownership: ModuleOwnership::Deprecated,
            target_package: Some("spanda-ros2"),
            notes: "Removed from spanda-core; use spanda_transport_ros2::rclrs",
        },
        ModuleClassification {
            module: "transport_dds",
            ownership: ModuleOwnership::Deprecated,
            target_package: Some("spanda-dds"),
            notes: "Removed from spanda-core; use spanda-transport-dds or spanda-transport-routing",
        },
        ModuleClassification {
            module: "transport_websocket",
            ownership: ModuleOwnership::Deprecated,
            target_package: Some("spanda-mqtt"),
            notes: "Removed from spanda-core; use spanda-transport-websocket or spanda-transport-routing",
        },
        ModuleClassification {
            module: "transport_live",
            ownership: ModuleOwnership::Deprecated,
            target_package: Some("spanda-transport-routing"),
            notes: "Removed from spanda-core; use spanda_transport_routing::transport_live",
        },
        ModuleClassification {
            module: "nav2_adapter",
            ownership: ModuleOwnership::CompatibilityShim,
            target_package: Some("spanda-nav"),
            notes: "Nav2 bridge subprocess adapter",
        },
        ModuleClassification {
            module: "slam_adapter",
            ownership: ModuleOwnership::CompatibilityShim,
            target_package: Some("spanda-slam"),
            notes: "SLAM bridge subprocess adapter",
        },
        ModuleClassification {
            module: "ai",
            ownership: ModuleOwnership::CompatibilityShim,
            target_package: Some("spanda-opencv / spanda-yolo / spanda-openai"),
            notes: "AiProvider trait stays; vendor registries move to packages",
        },
        ModuleClassification {
            module: "fleet_orchestrator",
            ownership: ModuleOwnership::CompatibilityShim,
            target_package: Some("spanda-fleet"),
            notes: "Fleet orchestration CLI remains; heavy logic moves to package",
        },
        ModuleClassification {
            module: "deploy_service",
            ownership: ModuleOwnership::CompatibilityShim,
            target_package: Some("spanda-ota"),
            notes: "OTA deploy/rollout moves to spanda-ota",
        },
        ModuleClassification {
            module: "simulator",
            ownership: ModuleOwnership::Core,
            target_package: None,
            notes: "Default in-memory sim; Gazebo/Webots via simulation packages",
        },
    ]
}

/// Official first-party package names recognized by the lean-core model.
pub fn official_package_names() -> &'static [&'static str] {
    // Description:
    //     Official package names.
    //
    // Inputs:
    //     None.
    //
    // Outputs:
    //     result: &'static [&'static str]
    //         Return value from `official_package_names`.
    //
    // Example:

    //     let result = spanda_runtime::classification::official_package_names();

    &[
        "spanda-gps",
        "spanda-wifi",
        "spanda-ble",
        "spanda-cellular",
        "spanda-mqtt",
        "spanda-dds",
        "spanda-ros2",
        "spanda-slam",
        "spanda-nav",
        "spanda-opencv",
        "spanda-yolo",
        "spanda-moveit",
        "spanda-gazebo",
        "spanda-webots",
        "spanda-fleet",
        "spanda-ota",
        "spanda-maintenance",
        "spanda-ledger",
        "spanda-cloud",
        "spanda-openai",
    ]
}
