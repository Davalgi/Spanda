//! Deploy plan builder with embedded certification proof summary.
//!
use spanda_ast::nodes::Program;
use spanda_certify::build_certification_proof_summary;
use spanda_ota::{build_deploy_plan_from_program, CertificationProofSummary, DeployPlan};

/// Build a deployment plan with certification proof metadata attached.
pub fn build_deploy_plan(program: &Program, program_path: &str, version: &str) -> DeployPlan {
    // Description:
    //     Build deploy plan.
    //
    // Inputs:
    //     progra: &Program
    //         Caller-supplied progra.
    //     program_path: &str
    //         Caller-supplied program path.
    //     version: &str
    //         Caller-supplied version.
    //
    // Outputs:
    //     result: DeployPlan
    //         Return value from `build_deploy_plan`.
    //
    // Example:

    //     let result = spanda_driver::deploy_plan::build_deploy_plan(progra, program_path, version);

    let mut plan = build_deploy_plan_from_program(program, program_path, version);
    let proof = build_certification_proof_summary(program, program_path);
    plan.certification_proof = Some(CertificationProofSummary {
        passed: proof.passed,
        passed_strict: proof.passed_strict,
        summary: proof.summary,
        error_count: proof.error_count,
        warning_count: proof.warning_count,
    });
    plan
}
