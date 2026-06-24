//! Compile-time robotics declaration validation helpers.
//!
use spanda_ast::robotics_decl::CertificationStandard;

/// Validate fleet member names against declared robots.
pub fn validate_fleet_members(
    fleet_name: &str,
    members: &[String],
    robot_names: &[String],
) -> Option<String> {
    // Description:
    //     Validate fleet members.
    //
    // Inputs:
    //     fleet_name: &str
    //         Caller-supplied fleet name.
    //     embers: &[String]
    //         Caller-supplied embers.
    //     robot_names: &[String]
    //         Caller-supplied robot names.
    //
    // Outputs:
    //     result: Option<String>
    //         Return value from `validate_fleet_members`.
    //
    // Example:

    //     let result = spanda_runtime_host::robotics_validation::validate_fleet_members(fleet_name, embers, robot_names);

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
    // Description:
    //     Validate swarm fleet.
    //
    // Inputs:
    //     swarm_name: &str
    //         Caller-supplied swarm name.
    //     fleet_name: &str
    //         Caller-supplied fleet name.
    //     fleet_names: &[String]
    //         Caller-supplied fleet names.
    //
    // Outputs:
    //     result: Option<String>
    //         Return value from `validate_swarm_fleet`.
    //
    // Example:

    //     let result = spanda_runtime_host::robotics_validation::validate_swarm_fleet(swarm_name, fleet_name, fleet_names);

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
    // Description:
    //     Validate mission decl.
    //
    // Inputs:
    //     name: &Option<String>
    //         Caller-supplied name.
    //     duration_hours: Option<f64>
    //         Caller-supplied duration hours.
    //     steps: &[String]
    //         Caller-supplied steps.
    //
    // Outputs:
    //     result: Option<String>
    //         Return value from `validate_mission_decl`.
    //
    // Example:

    //     let result = spanda_runtime_host::robotics_validation::validate_mission_decl(name, duration_hours, steps);

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

/// Validate certification standard identifiers at parse/type-check time.
pub fn validate_certification_standard(name: &str) -> Option<String> {
    // Description:
    //     Validate certification standard.
    //
    // Inputs:
    //     name: &str
    //         Caller-supplied name.
    //
    // Outputs:
    //     result: Option<String>
    //         Return value from `validate_certification_standard`.
    //
    // Example:

    //     let result = spanda_runtime_host::robotics_validation::validate_certification_standard(name);

    if CertificationStandard::parse_ident(name).is_some() {
        return None;
    }
    Some(format!(
        "unknown certification standard '{name}' (expected ISO13849, IEC61508, or ISO26262)"
    ))
}
