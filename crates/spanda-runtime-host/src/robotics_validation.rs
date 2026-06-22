//! Compile-time robotics declaration validation helpers.
//!

/// Validate fleet member names against declared robots.
pub fn validate_fleet_members(
    fleet_name: &str,
    members: &[String],
    robot_names: &[String],
) -> Option<String> {
    for member in members {
        if !robot_names.iter().any(|r| r == member) {
            return Some(format!(
                "fleet '{fleet_name}' references unknown robot '{member}'"
            ));
        }
    }
    None
}

/// Validate swarm declarations reference declared fleet groups.
pub fn validate_swarm_fleet(
    swarm_name: &str,
    fleet_name: &str,
    fleet_names: &[String],
) -> Option<String> {
    if fleet_names.iter().any(|name| name == fleet_name) {
        return None;
    }
    Some(format!(
        "swarm '{swarm_name}' references unknown fleet '{fleet_name}'"
    ))
}

/// Validate mission declarations have either duration or steps.
pub fn validate_mission_decl(
    name: &Option<String>,
    duration_hours: Option<f64>,
    steps: &[String],
) -> Option<String> {
    if duration_hours.is_none() && steps.is_empty() {
        let label = name
            .as_deref()
            .map(|n| format!("mission '{n}'"))
            .unwrap_or_else(|| "mission".into());
        return Some(format!(
            "{label} requires at least one of duration or mission steps"
        ));
    }
    None
}
