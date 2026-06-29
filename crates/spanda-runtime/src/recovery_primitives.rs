//! AST-only recovery helpers shared between runtime and assurance layers.
//!
use crate::recovery_types::{
    FailureClassification, RecoveryKnowledgeBase, RecoveryKnowledgeEntry, RecoveryPolicySpec,
    RecoveryResult, RecoveryStatus,
};
use spanda_ast::assurance_decl::{MitigationDecl, RecoveryPolicyDecl};
use spanda_ast::nodes::Program;

/// Classify a failure description into a failure category.
pub fn classify_failure(issue: &str) -> FailureClassification {
    // Classify a failure description into a failure category.
    //
    // Parameters:
    // - `issue` — runtime fault or health event label
    //
    // Returns:
    // Failure classification used for recovery planning.
    //
    // Options:
    // None.
    //
    // Example:
    // let kind = classify_failure("gps sensor failed");

    let lower = issue.to_lowercase();
    if lower.contains("gps")
        || lower.contains("camera")
        || lower.contains("lidar")
        || lower.contains("sensor")
        || lower.contains("imu")
    {
        FailureClassification::SensorFailure
    } else if lower.contains("actuator") || lower.contains("motor") || lower.contains("drive") {
        FailureClassification::ActuatorFailure
    } else if lower.contains("lte")
        || lower.contains("wifi")
        || lower.contains("connect")
        || lower.contains("network")
    {
        FailureClassification::ConnectivityFailure
    } else if lower.contains("provider") {
        FailureClassification::ProviderFailure
    } else if lower.contains("package") {
        FailureClassification::PackageFailure
    } else if lower.contains("mission") {
        FailureClassification::MissionFailure
    } else if lower.contains("health") || lower.contains("degrad") {
        FailureClassification::HealthDegradation
    } else if lower.contains("fleet") {
        FailureClassification::FleetFailure
    } else if lower.contains("swarm") {
        FailureClassification::SwarmFailure
    } else if lower.contains("safety") || lower.contains("kill") {
        FailureClassification::SafetyFailure
    } else {
        FailureClassification::Unknown
    }
}

/// Extract recovery policies from program declarations.
pub fn extract_recovery_policies(program: &Program) -> Vec<RecoveryPolicySpec> {
    // Extract recovery policies from program declarations.
    //
    // Parameters:
    // - `program` — parsed `.sd` program
    //
    // Returns:
    // Recovery policy specs from `recovery_policy` and `mitigation` blocks.
    //
    // Options:
    // None.
    //
    // Example:
    // let policies = extract_recovery_policies(&program);

    let Program::Program {
        recovery_policies,
        mitigations,
        ..
    } = program;

    let mut specs: Vec<RecoveryPolicySpec> = recovery_policies
        .iter()
        .map(|decl| {
            let RecoveryPolicyDecl::RecoveryPolicyDecl { name, branches, .. } = decl;
            RecoveryPolicySpec {
                name: name.clone(),
                triggers: branches
                    .iter()
                    .map(|b| (b.condition.clone(), b.actions.clone()))
                    .collect(),
            }
        })
        .collect();

    for mit in mitigations {
        let MitigationDecl::MitigationDecl { name, branches, .. } = mit;
        specs.push(RecoveryPolicySpec {
            name: name.clone(),
            triggers: branches
                .iter()
                .map(|b| (b.condition.clone(), b.actions.clone()))
                .collect(),
        });
    }

    specs
}

/// Map a runtime fault or health event to a recovery policy issue key.
pub fn issue_to_recovery_issue(event: &str) -> Option<String> {
    // Normalize hardware and health events into recovery policy condition keys.
    //
    // Parameters:
    // - `event` — runtime fault, health label, or hardware event name
    //
    // Returns:
    // Issue key such as `gps.failed`, or None when unmappable.
    //
    // Options:
    // None.
    //
    // Example:
    // assert_eq!(issue_to_recovery_issue("GPSFailure").as_deref(), Some("gps.failed"));

    let lower = event.to_ascii_lowercase();
    if lower.contains("gps") {
        return Some("gps.failed".into());
    }
    if lower.contains("lidar") {
        return Some("lidar.failed".into());
    }
    if lower.contains("camera") {
        return Some("camera.failed".into());
    }
    if lower.contains("connectivity") || lower.contains("comm") || lower.contains("network") {
        return Some("connectivity.lost".into());
    }
    if lower.contains("battery") {
        return Some("battery.critical".into());
    }
    if lower.contains("robothealthcritical") || lower.contains("robot.failed") {
        return Some("robot.failed".into());
    }
    if lower.contains("degraded") {
        return Some("robot.degraded".into());
    }
    if lower.ends_with("failure") {
        let stem = lower.trim_end_matches("failure");
        if !stem.is_empty() {
            return Some(format!("{}.failed", stem));
        }
    }
    if lower.contains("failed") || lower.contains("fault") {
        return Some(lower.replace('_', "."));
    }
    None
}

/// Return true when the program declares `recovery_policy` for the issue.
pub fn program_has_recovery_for_issue(program: &Program, issue: &str) -> bool {
    // Check recovery policy branches against a normalized issue key.
    //
    // Parameters:
    // - `program` — parsed `.sd` program
    // - `issue` — recovery issue key (for example `gps.failed`)
    //
    // Returns:
    // True when a policy branch matches the issue.
    //
    // Options:
    // None.
    //
    // Example:
    // let covered = program_has_recovery_for_issue(&program, "gps.failed");

    let lower = issue.to_ascii_lowercase();
    extract_recovery_policies(program).iter().any(|policy| {
        policy
            .triggers
            .iter()
            .any(|(condition, _)| condition_matches(&lower, &condition.to_ascii_lowercase()))
    })
}

/// Default on-disk path for the recovery knowledge store.
pub fn default_knowledge_store_path() -> std::path::PathBuf {
    // Default on-disk path for the recovery knowledge store.
    //
    // Parameters:
    // None.
    //
    // Returns:
    // Path under `.spanda/recovery_knowledge.json`.
    //
    // Options:
    // None.
    //
    // Example:
    // let path = default_knowledge_store_path();

    std::path::PathBuf::from(".spanda/recovery_knowledge.json")
}

/// Load persisted recovery knowledge from disk.
pub fn load_recovery_knowledge_store(path: &std::path::Path) -> RecoveryKnowledgeBase {
    // Load persisted recovery knowledge from disk.
    //
    // Parameters:
    // - `path` — JSON knowledge store path
    //
    // Returns:
    // Parsed knowledge base, or empty when missing or invalid.
    //
    // Options:
    // None.
    //
    // Example:
    // let kb = load_recovery_knowledge_store(&path);

    std::fs::read_to_string(path)
        .ok()
        .and_then(|content| serde_json::from_str(&content).ok())
        .unwrap_or_default()
}

/// Persist recovery knowledge to disk.
pub fn save_recovery_knowledge_store(
    path: &std::path::Path,
    kb: &RecoveryKnowledgeBase,
) -> std::io::Result<()> {
    // Persist recovery knowledge to disk.
    //
    // Parameters:
    // - `path` — destination JSON path
    // - `kb` — knowledge base to serialize
    //
    // Returns:
    // I/O error when the write fails.
    //
    // Options:
    // None.
    //
    // Example:
    // save_recovery_knowledge_store(&path, &kb)?;

    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(path, serde_json::to_string_pretty(kb).unwrap_or_default())
}

/// Update knowledge base from a recovery outcome (recommendations only).
pub fn record_recovery_outcome(kb: &mut RecoveryKnowledgeBase, result: &RecoveryResult) {
    // Update knowledge base from a recovery outcome (recommendations only).
    //
    // Parameters:
    // - `kb` — mutable knowledge store
    // - `result` — executed recovery outcome
    //
    // Returns:
    // Nothing; updates `kb` in place.
    //
    // Options:
    // None.
    //
    // Example:
    // record_recovery_outcome(&mut kb, &result);

    let success = matches!(
        result.status,
        RecoveryStatus::Success | RecoveryStatus::PartialSuccess
    );
    let pattern = result.evidence.failure.clone();
    for action in &result.executed_actions {
        if let Some(entry) = kb.entries.iter_mut().find(|e| e.failure_pattern == pattern) {
            entry.success_rate = if success {
                (entry.success_rate * 0.85) + 0.15
            } else {
                entry.success_rate * 0.85
            };
            entry.recovery_pattern.clone_from(action);
            entry.recommendation = format!("On {pattern} failure: {action}");
        } else {
            kb.entries.push(RecoveryKnowledgeEntry {
                failure_pattern: pattern.clone(),
                recovery_pattern: action.clone(),
                success_rate: if success { 1.0 } else { 0.0 },
                recommendation: format!("On {pattern} failure: {action}"),
            });
        }
    }
}

/// Merge persisted recovery knowledge (runtime layer does not run static analysis).
pub fn merge_recovery_knowledge(
    _program: &Program,
    persisted: &RecoveryKnowledgeBase,
) -> RecoveryKnowledgeBase {
    // Merge persisted recovery knowledge for runtime use.
    //
    // Parameters:
    // - `_program` — parsed program (reserved for assurance-layer enrichment)
    // - `persisted` — on-disk knowledge entries
    //
    // Returns:
    // Persisted knowledge clone for interpreter runtime.
    //
    // Options:
    // None.
    //
    // Example:
    // let kb = merge_recovery_knowledge(&program, &persisted);

    persisted.clone()
}

fn condition_matches(issue: &str, condition: &str) -> bool {
    let parts: Vec<&str> = condition.split('.').collect();
    parts.iter().all(|p| issue.contains(p))
}
