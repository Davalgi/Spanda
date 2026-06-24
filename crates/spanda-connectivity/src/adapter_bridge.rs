//! Optional subprocess bridges for production Nav2/SLAM adapter backends.

use std::process::Command;

fn bridge_command(env_key: &str) -> Option<String> {
    // Description:
    //     Bridge command.
    //
    // Inputs:
    //     env_key: &str
    //         Caller-supplied env key.
    //
    // Outputs:
    //     result: Option<String>
    //         Return value from `bridge_command`.
    //
    // Example:

    //     let result = spanda_connectivity::adapter_bridge::bridge_command(env_key);

    std::env::var(env_key)
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
}

/// Invoke an external Nav2 bridge command when `SPANDA_NAV2_CMD` is configured.
pub fn invoke_nav2_bridge(goal: &str) -> Option<String> {
    // Description:
    //     Invoke nav2 bridge.
    //
    // Inputs:
    //     goal: &str
    //         Caller-supplied goal.
    //
    // Outputs:
    //     result: Option<String>
    //         Return value from `invoke_nav2_bridge`.
    //
    // Example:

    //     let result = spanda_connectivity::adapter_bridge::invoke_nav2_bridge(goal);

    let template = bridge_command("SPANDA_NAV2_CMD")?;
    let command_line = template.replace("{goal}", goal);
    let mut parts = command_line.split_whitespace();
    let program = parts.next()?;

    // Run the bridge executable and capture stdout when it exits successfully.
    let output = Command::new(program).args(parts).output().ok()?;
    if !output.status.success() {
        return None;
    }
    Some(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

/// Invoke an external SLAM bridge command when `SPANDA_SLAM_CMD` is configured.
pub fn invoke_slam_bridge(operation: &str) -> Option<String> {
    // Description:
    //     Invoke slam bridge.
    //
    // Inputs:
    //     operation: &str
    //         Caller-supplied operation.
    //
    // Outputs:
    //     result: Option<String>
    //         Return value from `invoke_slam_bridge`.
    //
    // Example:

    //     let result = spanda_connectivity::adapter_bridge::invoke_slam_bridge(operation);

    let template = bridge_command("SPANDA_SLAM_CMD")?;
    let command_line = template.replace("{op}", operation);
    let mut parts = command_line.split_whitespace();
    let program = parts.next()?;

    // Run the bridge executable and capture stdout when it exits successfully.
    let output = Command::new(program).args(parts).output().ok()?;
    if !output.status.success() {
        return None;
    }
    Some(String::from_utf8_lossy(&output.stdout).trim().to_string())
}
