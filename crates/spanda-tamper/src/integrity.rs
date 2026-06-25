//! Verify-time integrity hashing and baseline comparison for program artifacts.

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use spanda_ast::foundations::{
    DeployDecl, HardwareDecl, HealthPolicyDecl, KillSwitchDecl, MissionDecl,
};
use spanda_ast::nodes::{Program, RobotDecl};
use spanda_ast::policy_decl::OperationalPolicyDecl;

/// Output format for integrity reports.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum IntegrityFormat {
    #[default]
    Text,
    Json,
}

/// Per-artifact integrity posture.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ArtifactIntegrityStatus {
    Trusted,
    Modified,
    Unknown,
}

/// One hashed program artifact with optional baseline comparison.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IntegrityArtifact {
    pub artifact_type: String,
    pub name: String,
    pub hash: String,
    pub status: ArtifactIntegrityStatus,
    pub baseline_hash: Option<String>,
}

/// Full integrity verification report for a program.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IntegrityReport {
    pub program: String,
    pub baseline: Option<String>,
    pub artifacts: Vec<IntegrityArtifact>,
    pub passed: bool,
}

/// Hash a serializable artifact into a SHA-256 hex digest.
fn hash_artifact<T: Serialize>(value: &T) -> String {
    let json = serde_json::to_string(value).unwrap_or_default();
    hex::encode(Sha256::digest(json.as_bytes()))
}

/// Collect integrity artifacts from a parsed program.
fn collect_artifacts(program: &Program) -> Vec<(String, String, String)> {
    let mut artifacts = Vec::new();
    let Program::Program {
        hardware_profiles,
        operational_policies,
        deployments,
        kill_switches,
        health_policies,
        imports,
        robots,
        ..
    } = program;

    for hardware in hardware_profiles {
        let HardwareDecl::HardwareDecl { name, .. } = hardware;
        artifacts.push((
            "hardware".into(),
            name.clone(),
            hash_artifact(hardware),
        ));
    }

    for policy in operational_policies {
        let OperationalPolicyDecl::OperationalPolicyDecl { name, .. } = policy;
        artifacts.push((
            "policy".into(),
            name.clone(),
            hash_artifact(policy),
        ));
    }

    for ks in kill_switches {
        let KillSwitchDecl::KillSwitchDecl { name, .. } = ks;
        artifacts.push((
            "kill_switch".into(),
            name.clone(),
            hash_artifact(ks),
        ));
    }

    for health in health_policies {
        let HealthPolicyDecl::HealthPolicyDecl { name, .. } = health;
        artifacts.push((
            "health_policy".into(),
            name.clone(),
            hash_artifact(health),
        ));
    }

    for deployment in deployments {
        let DeployDecl::DeployDecl { robot_name, .. } = deployment;
        artifacts.push((
            "deploy".into(),
            robot_name.clone(),
            hash_artifact(deployment),
        ));
    }

    for import in imports {
        let spanda_ast::nodes::ImportDecl::ImportDecl { path, .. } = import;
        artifacts.push(("package".into(), path.clone(), hash_artifact(import)));
    }

    for robot in robots {
        collect_robot_artifacts(robot, &mut artifacts);
    }

    artifacts
}

fn collect_robot_artifacts(robot: &RobotDecl, artifacts: &mut Vec<(String, String, String)>) {
    let RobotDecl::RobotDecl {
        name,
        mission,
        safety,
        exposes_capabilities,
        ..
    } = robot;

    if let Some(mission_decl) = mission {
        let MissionDecl::MissionDecl { name: mission_name, .. } = mission_decl;
        let mission_label = mission_name.as_deref().unwrap_or("default");
        artifacts.push((
            "mission".into(),
            format!("{name}/{mission_label}"),
            hash_artifact(mission_decl),
        ));
    }

    if let Some(safety_block) = safety {
        artifacts.push((
            "safety".into(),
            name.clone(),
            hash_artifact(safety_block),
        ));
    }

    if !exposes_capabilities.is_empty() {
        artifacts.push((
            "capabilities".into(),
            name.clone(),
            hash_artifact(exposes_capabilities),
        ));
    }
}

/// Generate an integrity report for a program with optional baseline comparison.
pub fn generate_integrity_report(
    program: &Program,
    source_label: &str,
    baseline_program: Option<&Program>,
    baseline_label: Option<&str>,
) -> IntegrityReport {
    // Hash declared artifacts and compare against an optional approved baseline program.
    //
    // Parameters:
    // - `program` — parsed candidate program
    // - `source_label` — file label for the candidate
    // - `baseline_program` — optional approved baseline AST
    // - `baseline_label` — optional baseline file label
    //
    // Returns:
    // Integrity report with per-artifact status and pass/fail rollup.
    //
    // Options:
    // None.
    //
    // Example:
    // let report = generate_integrity_report(&program, "rover.sd", None, None);

    let current = collect_artifacts(program);
    let baseline_map = baseline_program.map(|baseline| {
        collect_artifacts(baseline)
            .into_iter()
            .map(|(kind, name, hash)| ((kind, name), hash))
            .collect::<std::collections::BTreeMap<(String, String), String>>()
    });

    let artifacts = current
        .into_iter()
        .map(|(artifact_type, name, hash)| {
            let (status, baseline_hash) = match baseline_map.as_ref() {
                None => (ArtifactIntegrityStatus::Unknown, None),
                Some(map) => match map.get(&(artifact_type.clone(), name.clone())) {
                    None => (ArtifactIntegrityStatus::Modified, None),
                    Some(base_hash) if base_hash == &hash => {
                        (ArtifactIntegrityStatus::Trusted, Some(base_hash.clone()))
                    }
                    Some(base_hash) => (
                        ArtifactIntegrityStatus::Modified,
                        Some(base_hash.clone()),
                    ),
                },
            };
            IntegrityArtifact {
                artifact_type,
                name,
                hash,
                status,
                baseline_hash,
            }
        })
        .collect::<Vec<_>>();

    let passed = if baseline_map.is_none() {
        true
    } else {
        artifacts
            .iter()
            .all(|artifact| artifact.status == ArtifactIntegrityStatus::Trusted)
    };

    IntegrityReport {
        program: source_label.into(),
        baseline: baseline_label.map(str::to_string),
        artifacts,
        passed,
    }
}

/// Format an integrity report for CLI output.
pub fn format_integrity_report(report: &IntegrityReport, format: IntegrityFormat) -> String {
    // Render integrity report as JSON or human-readable text.
    //
    // Parameters:
    // - `report` — integrity verification report
    // - `format` — text or JSON output
    //
    // Returns:
    // Formatted report string.
    //
    // Options:
    // None.
    //
    // Example:
    // let text = format_integrity_report(&report, IntegrityFormat::Text);

    if format == IntegrityFormat::Json {
        return serde_json::to_string_pretty(report).unwrap_or_else(|e| e.to_string());
    }

    let mut lines = vec![
        format!("Integrity check: {}", report.program),
        if let Some(baseline) = &report.baseline {
            format!("Baseline: {baseline}")
        } else {
            "Baseline: none (hashes only)".into()
        },
        if report.passed {
            "Result: PASS".into()
        } else {
            "Result: FAIL".into()
        },
    ];
    if report.artifacts.is_empty() {
        lines.push("No artifacts found.".into());
    } else {
        lines.push("Artifacts:".into());
        for artifact in &report.artifacts {
            lines.push(format!(
                "  [{:?}] {}:{} — {}",
                artifact.status, artifact.artifact_type, artifact.name, artifact.hash
            ));
        }
    }
    lines.join("\n")
}
