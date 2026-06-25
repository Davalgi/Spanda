//! Diagnosis analysis integrating mission traces and static fault models.
//!
use crate::types::{CausalGraph, Confidence, Diagnosis, FaultTree, RootCause};
use spanda_ast::nodes::Program;
use spanda_config::{diagnosis_policy, ResolvedSystemConfig};
use spanda_error::SpandaError;
use spanda_readiness::diagnose_trace;
use std::path::Path;

/// Diagnosis report combining static and trace-based analysis.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DiagnosisReport {
    pub static_diagnoses: Vec<Diagnosis>,
    pub trace_diagnosis: Option<spanda_readiness::RootCauseReport>,
    pub causal_graph: CausalGraph,
    pub passed: bool,
}

/// Diagnose from a mission trace file.
pub fn diagnose_from_trace(trace_path: &Path) -> Result<DiagnosisReport, SpandaError> {
    // Description:
    //     Diagnose from trace.
    //
    // Inputs:
    //     race_path: &Path
    //         Caller-supplied race path.
    //
    // Outputs:
    //     result: Result<DiagnosisReport, SpandaError>
    //         Return value from `diagnose_from_trace`.
    //
    // Example:

    //     let result = spanda_assurance::diagnosis::diagnose_from_trace(race_path);

    let trace = diagnose_trace(trace_path)?;
    Ok(DiagnosisReport {
        static_diagnoses: Vec::new(),
        trace_diagnosis: Some(trace),
        causal_graph: CausalGraph {
            nodes: Vec::new(),
            edges: Vec::new(),
        },
        passed: true,
    })
}

/// Static diagnosis from program declarations (mitigations, anomaly handlers).
pub fn diagnose_program(program: &Program) -> DiagnosisReport {
    diagnose_program_with_config(program, None)
}

/// Static diagnosis using optional configuration policy thresholds.
pub fn diagnose_program_with_config(
    program: &Program,
    config: Option<&ResolvedSystemConfig>,
) -> DiagnosisReport {
    // Description:
    //     Diagnose program.
    //
    // Inputs:
    //     progra: &Program
    //         Caller-supplied progra.
    //
    // Outputs:
    //     result: DiagnosisReport
    //         Return value from `diagnose_program`.
    //
    // Example:

    //     let result = spanda_assurance::diagnosis::diagnose_program(progra);

    let Program::Program {
        mitigations,
        anomaly_handlers,
        anomaly_detectors,
        ..
    } = program;

    let mut diagnoses = Vec::new();
    for mit in mitigations {
        let spanda_ast::assurance_decl::MitigationDecl::MitigationDecl { name, branches, .. } = mit;
        let root_causes: Vec<RootCause> = branches
            .iter()
            .map(|b| RootCause {
                description: b.condition.clone(),
                confidence: Confidence(0.7),
                contributing: b.actions.clone(),
            })
            .collect();
        diagnoses.push(Diagnosis {
            subject: name.clone(),
            root_causes,
            fault_tree: FaultTree {
                top_event: name.clone(),
                gates: branches.iter().flat_map(|b| b.actions.clone()).collect(),
            },
        });
    }

    let mut nodes = vec!["system".into()];
    let mut edges = Vec::new();
    for det in anomaly_detectors {
        let spanda_ast::assurance_decl::AnomalyDetectorDecl::AnomalyDetectorDecl { name, .. } = det;
        nodes.push(name.clone());
        edges.push(("system".into(), name.clone()));
    }
    for handler in anomaly_handlers {
        let spanda_ast::assurance_decl::AnomalyHandlerDecl::AnomalyHandlerDecl {
            detector,
            actions,
            ..
        } = handler;
        for action in actions {
            nodes.push(action.clone());
            edges.push((detector.clone(), action.clone()));
        }
    }

    let mut passed = !diagnoses.is_empty() || !anomaly_detectors.is_empty();

    if let Some(cfg) = config {
        let policy = diagnosis_policy(cfg);
        if policy.require_mitigations && mitigations.is_empty() {
            passed = false;
        }
        if policy.require_anomaly_handlers && anomaly_handlers.is_empty() {
            passed = false;
        }
    }

    DiagnosisReport {
        static_diagnoses: diagnoses,
        trace_diagnosis: None,
        causal_graph: CausalGraph { nodes, edges },
        passed,
    }
}
