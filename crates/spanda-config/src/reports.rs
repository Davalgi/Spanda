//! Human-readable and JSON configuration reports.
//!
use crate::resolved::ResolvedSystemConfig;
use serde::Serialize;

/// Bundle of inspectable configuration reports.
#[derive(Debug, Clone, Serialize)]
pub struct ConfigReportBundle {
    pub resolved: ResolvedSummary,
    pub device_hierarchy: Vec<String>,
    pub logical_physical: LogicalPhysicalSummary,
    pub capabilities: CapabilitySummary,
    pub health: HealthSummary,
    pub trust_security: TrustSecuritySummary,
}

#[derive(Debug, Clone, Serialize)]
pub struct ResolvedSummary {
    pub project: String,
    pub fleet_id: Option<String>,
    pub layers_applied: Vec<String>,
    pub fragments_loaded: Vec<String>,
    pub validation_passed: bool,
    pub error_count: usize,
    pub warning_count: usize,
}

#[derive(Debug, Clone, Serialize)]
pub struct LogicalPhysicalSummary {
    pub robots: usize,
    pub sensors: usize,
    pub actuators: usize,
}

#[derive(Debug, Clone, Serialize)]
pub struct CapabilitySummary {
    pub entries: Vec<CapabilityEntry>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CapabilityEntry {
    pub device_id: String,
    pub robot_id: String,
    pub capabilities: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct HealthSummary {
    pub robots_with_policy: Vec<String>,
    pub robots_missing_policy: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct TrustSecuritySummary {
    pub identities: Vec<IdentityEntry>,
    pub untrusted_devices: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct IdentityEntry {
    pub device_id: String,
    pub identity: String,
}

pub fn generate_report_bundle(resolved: &ResolvedSystemConfig) -> ConfigReportBundle {
    // Build the full configuration report bundle.
    //
    // Parameters:
    // - `resolved` — fully resolved system configuration
    //
    // Returns:
    // Structured report sections for CLI and JSON output.
    //
    // Options:
    // None.
    //
    // Example:
    // let bundle = generate_report_bundle(&resolved);

    let mut capability_entries = Vec::new();
    let mut untrusted = Vec::new();
    let mut identities = Vec::new();
    for (robot, _compute, device) in resolved.device_tree.all_devices() {
        capability_entries.push(CapabilityEntry {
            device_id: device.id.clone(),
            robot_id: robot.id.clone(),
            capabilities: device.capabilities.clone(),
        });
        if device.trusted == Some(false) {
            untrusted.push(device.id.clone());
        }
        if let Some(ref id) = device.identity {
            identities.push(IdentityEntry {
                device_id: device.id.clone(),
                identity: id.clone(),
            });
        }
    }

    let robot_ids = resolved.robot_ids();
    let mut with_policy = Vec::new();
    let mut missing_policy = Vec::new();
    for rid in &robot_ids {
        if resolved.health_policy_for(rid).is_some() {
            with_policy.push((*rid).to_string());
        } else {
            missing_policy.push((*rid).to_string());
        }
    }

    ConfigReportBundle {
        resolved: ResolvedSummary {
            project: resolved.project_name().into(),
            fleet_id: resolved.fleet_id().map(str::to_owned),
            layers_applied: resolved.layers_applied.clone(),
            fragments_loaded: resolved.fragments_loaded.clone(),
            validation_passed: resolved.validation.passed,
            error_count: resolved.validation.error_count(),
            warning_count: resolved.validation.warning_count(),
        },
        device_hierarchy: resolved.device_tree.hierarchy_lines(),
        logical_physical: LogicalPhysicalSummary {
            robots: resolved.logical_map.robots.len(),
            sensors: resolved.logical_map.sensors.len(),
            actuators: resolved.logical_map.actuators.len(),
        },
        capabilities: CapabilitySummary {
            entries: capability_entries,
        },
        health: HealthSummary {
            robots_with_policy: with_policy,
            robots_missing_policy: missing_policy,
        },
        trust_security: TrustSecuritySummary {
            identities,
            untrusted_devices: untrusted,
        },
    }
}

pub fn format_report_text(bundle: &ConfigReportBundle) -> String {
    // Render the report bundle as plain text for terminal output.
    //
    // Parameters:
    // - `bundle` — generated report bundle
    //
    // Returns:
    // Multi-section text report.
    //
    // Options:
    // None.
    //
    // Example:
    // println!("{}", format_report_text(&bundle));

    let mut out = String::new();
    out.push_str("=== Resolved Configuration ===\n");
    out.push_str(&format!("Project: {}\n", bundle.resolved.project));
    if let Some(ref fleet) = bundle.resolved.fleet_id {
        out.push_str(&format!("Fleet: {fleet}\n"));
    }
    out.push_str(&format!(
        "Validation: {} ({} errors, {} warnings)\n",
        if bundle.resolved.validation_passed {
            "PASSED"
        } else {
            "FAILED"
        },
        bundle.resolved.error_count,
        bundle.resolved.warning_count
    ));
    if !bundle.resolved.layers_applied.is_empty() {
        out.push_str("\nLayers:\n");
        for layer in &bundle.resolved.layers_applied {
            out.push_str(&format!("  - {layer}\n"));
        }
    }
    if !bundle.resolved.fragments_loaded.is_empty() {
        out.push_str("\nFragments:\n");
        for frag in &bundle.resolved.fragments_loaded {
            out.push_str(&format!("  - {frag}\n"));
        }
    }
    out.push_str("\n=== Device Hierarchy ===\n");
    for line in &bundle.device_hierarchy {
        out.push_str(line);
        out.push('\n');
    }
    out.push_str("\n=== Logical / Physical Mapping ===\n");
    out.push_str(&format!(
        "Robots: {}, Sensors: {}, Actuators: {}\n",
        bundle.logical_physical.robots,
        bundle.logical_physical.sensors,
        bundle.logical_physical.actuators
    ));
    out.push_str("\n=== Capabilities ===\n");
    for entry in &bundle.capabilities.entries {
        out.push_str(&format!(
            "  {} @ {}: [{}]\n",
            entry.device_id,
            entry.robot_id,
            entry.capabilities.join(", ")
        ));
    }
    out.push_str("\n=== Health Policies ===\n");
    if bundle.health.robots_with_policy.is_empty() && bundle.health.robots_missing_policy.is_empty()
    {
        out.push_str("  (no robots configured)\n");
    } else {
        for r in &bundle.health.robots_with_policy {
            out.push_str(&format!("  [ok] {r}\n"));
        }
        for r in &bundle.health.robots_missing_policy {
            out.push_str(&format!("  [missing] {r}\n"));
        }
    }
    out.push_str("\n=== Trust / Security ===\n");
    for id in &bundle.trust_security.identities {
        out.push_str(&format!("  {} -> {}\n", id.device_id, id.identity));
    }
    for d in &bundle.trust_security.untrusted_devices {
        out.push_str(&format!("  [untrusted] {d}\n"));
    }
    out
}

pub fn config_drift_report(
    baseline: &ResolvedSystemConfig,
    current: &ResolvedSystemConfig,
) -> Vec<String> {
    // Compare two resolved configs and list drift.
    //
    // Parameters:
    // - `baseline` — reference configuration
    // - `current` — configuration under inspection
    //
    // Returns:
    // Drift description lines.
    //
    // Options:
    // None.
    //
    // Example:
    // let drift = config_drift_report(&base, &current);

    let mut lines = Vec::new();
    lines.extend(crate::resolver::diff_configs(&baseline.raw, &current.raw));
    if baseline.fleet_id() != current.fleet_id() {
        lines.push(format!(
            "~ fleet.id: {:?} -> {:?}",
            baseline.fleet_id(),
            current.fleet_id()
        ));
    }
    lines
}
