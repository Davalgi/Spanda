//! Build deployment plans from parsed Spanda programs.
//!
use crate::service::hash_program_artifact;
use crate::types::{DeployAssignment, DeployPlan};
use spanda_ast::foundations::DeployDecl;
use spanda_ast::nodes::Program;
use spanda_ast::robotics_decl::CertifyDecl;

/// Build a deployment plan from a parsed program.
pub fn build_deploy_plan_from_program(
    program: &Program,
    program_path: &str,
    version: &str,
) -> DeployPlan {
    // Extract deploy targets and certification metadata from the program AST.
    //
    // Parameters:
    // - `program` — parsed Spanda program
    // - `program_path` — source file path for reporting
    // - `version` — release version label
    //
    // Returns:
    // Deployment plan with robot/hardware assignments.
    //
    // Options:
    // None.
    //
    // Example:
    // let plan = build_deploy_plan_from_program(&program, "rover.sd", "1.2.0");

    let Program::Program {
        deployments,
        certifications,
        ..
    } = program;
    let mut assignments = Vec::new();
    for deploy in deployments {
        let DeployDecl::DeployDecl {
            robot_name,
            targets,
            ..
        } = deploy;
        for hardware in targets {
            assignments.push(DeployAssignment {
                robot_name: robot_name.clone(),
                hardware: hardware.clone(),
            });
        }
    }
    assignments.sort_by(|a, b| {
        a.robot_name
            .cmp(&b.robot_name)
            .then(a.hardware.cmp(&b.hardware))
    });
    let certs = certifications
        .iter()
        .map(|c| {
            let CertifyDecl::CertifyDecl {
                standard, level, ..
            } = c;
            match level {
                Some(l) => format!("{}:{}", standard.as_str(), l),
                None => standard.as_str().to_string(),
            }
        })
        .collect();
    DeployPlan {
        program: program_path.to_string(),
        version: version.to_string(),
        program_hash: hash_program_artifact(program_path),
        assignments,
        certifications: certs,
        certification_proof: None,
    }
}
