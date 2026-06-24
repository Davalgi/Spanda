//! Framework adapter import verification for `spanda verify`.

use crate::compat::{CompatItem, CompatSeverity};
use spanda_ast::nodes::ImportDecl;

fn pass(category: &str, message: impl Into<String>, line: u32, column: u32) -> CompatItem {
    // Description:
    //     Pass.
    //
    // Inputs:
    //     category: &str
    //         Caller-supplied category.
    //     essage: impl Into<String>
    //         Caller-supplied essage.
    //     line: u32
    //         Caller-supplied line.
    //     column: u32
    //         Caller-supplied column.
    //
    // Outputs:
    //     result: CompatItem
    //         Return value from `pass`.
    //
    // Example:

    //     let result = spanda_hardware::adapter_verify::pass(category, essage, line, column);

    CompatItem {
        category: category.into(),
        message: message.into(),
        severity: CompatSeverity::Pass,
        line,
        column,
    }
}

const FRAMEWORK_IMPORT_PACKAGES: &[(&str, &str)] = &[
    ("robotics.ros2", "spanda-ros2"),
    ("communication.mqtt", "spanda-mqtt"),
    ("vision.opencv", "spanda-opencv"),
    ("vision.yolo", "spanda-yolo"),
    ("navigation.slam", "spanda-slam"),
    ("navigation.path_planning", "spanda-nav"),
    ("navigation.nav2", "spanda-nav2"),
    ("navigation.cartographer", "spanda-cartographer"),
    ("navigation.rtabmap", "spanda-rtabmap"),
    ("vision.detectron", "spanda-detectron"),
    ("manipulation.grasp", "spanda-manipulation"),
    ("hri.dialogue", "spanda-hri"),
    ("twin.sync", "spanda-digital-twin"),
    ("sim.gazebo", "spanda-sim-gazebo"),
    ("sim.webots", "spanda-sim-webots"),
    ("connectivity.ble", "spanda-ble"),
    ("positioning.gps", "spanda-gps"),
    ("connectivity.lte", "spanda-lte"),
];

/// Report registry adapter mappings for framework import paths declared in a program.
pub fn verify_framework_imports(imports: &[ImportDecl]) -> Vec<CompatItem> {
    // Description:
    //     Verify framework imports.
    //
    // Inputs:
    //     imports: &[ImportDecl]
    //         Caller-supplied imports.
    //
    // Outputs:
    //     result: Vec<CompatItem>
    //         Return value from `verify_framework_imports`.
    //
    // Example:
    //     let result = spanda_hardware::adapter_verify::verify_framework_imports(imports);
    let mut items = Vec::new();
    for imp in imports {
        let ImportDecl::ImportDecl { path, span, .. } = imp;
        for (import_path, package_name) in FRAMEWORK_IMPORT_PACKAGES {
            if path == *import_path {
                let detail = adapter_capability_summary(import_path);
                items.push(pass(
                    "adapter",
                    format!("Framework import '{path}' maps to {package_name} — {detail}",),
                    span.start.line,
                    span.start.column,
                ));
                break;
            }
        }
    }
    items
}

fn adapter_capability_summary(import_path: &str) -> &'static str {
    // Description:
    //     Adapter capability summary.
    //
    // Inputs:
    //     import_path: &str
    //         Caller-supplied import path.
    //
    // Outputs:
    //     result: &'static str
    //         Return value from `adapter_capability_summary`.
    //
    // Example:

    //     let result = spanda_hardware::adapter_verify::adapter_capability_summary(import_path);

    match import_path {
        "navigation.nav2" => "provides Nav2Adapter/navigate; requires topic.publish + ros2.bridge",
        "navigation.cartographer" => "provides CartographerSlam/slam.*; requires sensor.read",
        "navigation.rtabmap" => "provides RtabmapSlam/slam.*; requires sensor.read + camera.read",
        "navigation.slam" => "provides SlamAdapter/slam.*; requires sensor.read",
        _ => "stub adapter (orchestration hook only)",
    }
}
