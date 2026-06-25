//! Top-level mission assurance composition.
//!
use spanda_ast::nodes::Program;
use spanda_config::{assurance_policy, assurance_score_from_flags, ResolvedSystemConfig};

use crate::anomaly::{scan_anomalies, AnomalyReport};
use crate::diagnosis::{diagnose_program_with_config, DiagnosisReport};
use crate::evidence::{build_assurance_report_with_config, AssuranceReport};
use crate::knowledge::validate_knowledge_models;
use crate::mission::{verify_mission_assurance_with_config, MissionAssuranceReport};
use crate::mitigation::extract_mitigations;
use crate::modes::validate_modes;
use crate::prognostics::{evaluate_prognostics, PrognosticsReport};
use crate::recovery::{evaluate_recovery, RecoveryReport};
use crate::resilience::{check_resilience, ResilienceReport};
use crate::state::{evaluate_state_assurance, StateAssuranceReport};

/// Composite mission assurance summary.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MissionAssuranceSummary {
    pub assurance: AssuranceReport,
    pub anomalies: AnomalyReport,
    pub prognostics: PrognosticsReport,
    pub resilience: ResilienceReport,
    pub mission: MissionAssuranceReport,
    pub state: StateAssuranceReport,
    pub recovery: RecoveryReport,
    pub issues: Vec<String>,
    pub passed: bool,
}

/// Run full mission assurance analysis on a program.
pub fn assure_program(program: &Program, source_label: &str) -> MissionAssuranceSummary {
    assure_program_with_config(program, source_label, None)
}

/// Run mission assurance using optional resolved system configuration thresholds.
pub fn assure_program_with_config(
    program: &Program,
    source_label: &str,
    config: Option<&ResolvedSystemConfig>,
) -> MissionAssuranceSummary {
    // Description:
    //     Assure program.
    //
    // Inputs:
    //     progra: &Program
    //         Caller-supplied progra.
    //     source_label: &str
    //         Caller-supplied source label.
    //
    // Outputs:
    //     result: MissionAssuranceSummary
    //         Return value from `assure_program`.
    //
    // Example:

    //     let result = spanda_assurance::analyze::assure_program(progra, source_label);

    let assurance = build_assurance_report_with_config(program, source_label, config);
    let anomalies = scan_anomalies(program);
    let prognostics = evaluate_prognostics(program);
    let resilience = check_resilience(program);
    let mission = verify_mission_assurance_with_config(program, config);
    let state = evaluate_state_assurance(program);
    let recovery = evaluate_recovery(program, None);

    let mut issues = Vec::new();
    issues.extend(validate_knowledge_models(program));
    issues.extend(state.issues.clone());
    issues.extend(validate_modes(program));

    let passed_flags = [
        assurance.passed,
        anomalies.passed,
        prognostics.passed,
        resilience.passed,
        mission.passed,
        state.passed,
        recovery.passed,
    ];
    let mut passed = passed_flags.iter().all(|p| *p) && issues.is_empty();

    if let Some(cfg) = config {
        let policy = assurance_policy(cfg);
        let score = assurance_score_from_flags(&passed_flags);
        if score < policy.minimum_score {
            issues.push(format!(
                "Assurance score {score} below configured minimum {}",
                policy.minimum_score
            ));
            passed = false;
        }
        if policy.require_recovery && !recovery.passed {
            issues.push("Recovery assurance required by configuration but failed".into());
            passed = false;
        }
        if policy.require_resilience && !resilience.passed {
            issues.push("Resilience assurance required by configuration but failed".into());
            passed = false;
        }
    }

    MissionAssuranceSummary {
        assurance,
        anomalies,
        prognostics,
        resilience,
        mission,
        state,
        recovery,
        issues,
        passed,
    }
}

/// Re-export mitigation report builder for CLI.
pub fn mitigation_report(program: &Program) -> crate::mitigation::MitigationReport {
    // Description:
    //     Mitigation report.
    //
    // Inputs:
    //     progra: &Program
    //         Caller-supplied progra.
    //
    // Outputs:
    //     result: crate::mitigation::MitigationReport
    //         Return value from `mitigation_report`.
    //
    // Example:

    //     let result = spanda_assurance::analyze::mitigation_report(progra);

    extract_mitigations(program)
}

/// Re-export diagnosis for program-only path.
pub fn diagnosis_report(program: &Program) -> DiagnosisReport {
    diagnosis_report_with_config(program, None)
}

/// Diagnosis report with optional configuration policy.
pub fn diagnosis_report_with_config(
    program: &Program,
    config: Option<&ResolvedSystemConfig>,
) -> DiagnosisReport {
    // Description:
    //     Diagnosis report.
    //
    // Inputs:
    //     progra: &Program
    //         Caller-supplied progra.
    //
    // Outputs:
    //     result: DiagnosisReport
    //         Return value from `diagnosis_report`.
    //
    // Example:

    //     let result = spanda_assurance::analyze::diagnosis_report(progra);

    diagnose_program_with_config(program, config)
}
