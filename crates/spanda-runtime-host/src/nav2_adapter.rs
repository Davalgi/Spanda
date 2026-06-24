//! Nav2 package adapter hooks for ROS 2 navigation integration.

use spanda_ast::nodes::ImportDecl;
use spanda_runtime::value::{runtime_velocity, RuntimeValue};
use std::collections::HashMap;

/// Import paths that enable Nav2 adapter behavior.
pub fn nav2_import_paths() -> &'static [&'static str] {
    // Description:
    //     Nav2 import paths.
    //
    // Inputs:
    //     None.
    //
    // Outputs:
    //     result: &'static [&'static str]
    //         Return value from `nav2_import_paths`.
    //
    // Example:

    //     let result = spanda_runtime_host::nav2_adapter::nav2_import_paths();

    &["navigation.nav2", "std.navigation"]
}

/// Return true when the program imports a Nav2-related module path.
pub fn program_uses_nav2(imports: &[ImportDecl]) -> bool {
    // Description:
    //     Program uses nav2.
    //
    // Inputs:
    //     imports: &[ImportDecl]
    //         Caller-supplied imports.
    //
    // Outputs:
    //     result: bool
    //         Return value from `program_uses_nav2`.
    //
    // Example:

    //     let result = spanda_runtime_host::nav2_adapter::program_uses_nav2(imports);

    imports.iter().any(|imp| {
        let ImportDecl::ImportDecl { path, .. } = imp;
        nav2_import_paths().contains(&path.as_str())
    })
}

/// Publish a stub `/cmd_vel` message when Nav2 bridge topics are declared.
pub fn try_publish_nav2_cmd_vel(
    topic_path_to_message_type: &HashMap<String, String>,
    publish: &mut dyn FnMut(&str, &str, RuntimeValue),
    goal: Option<&str>,
    log: &mut dyn FnMut(String),
) -> bool {
    // Description:
    //     Try publish nav2 cmd vel.
    //
    // Inputs:
    //     opic_path_to_message_type: &HashMap<String, String>
    //         Caller-supplied opic path to message type.
    //     publish: &mut dyn FnMut(&str, &str, RuntimeValue)
    //         Caller-supplied publish.
    //     goal: Option<&str>
    //         Caller-supplied goal.
    //     log: &mut dyn FnMut(String)
    //         Caller-supplied log.
    //
    // Outputs:
    //     result: bool
    //         Return value from `try_publish_nav2_cmd_vel`.
    //
    // Example:

    //     let result = spanda_runtime_host::nav2_adapter::try_publish_nav2_cmd_vel(opic_path_to_message_type, publish, goal, log);

    const CMD_VEL: &str = "/cmd_vel";
    let Some(message_type) = topic_path_to_message_type.get(CMD_VEL) else {
        return false;
    };
    let velocity = runtime_velocity(0.2, 0.0);
    publish(CMD_VEL, message_type, velocity);
    log(format!(
        "navigation: Nav2Adapter publish {CMD_VEL} goal='{}'",
        goal.unwrap_or("none")
    ));
    true
}
