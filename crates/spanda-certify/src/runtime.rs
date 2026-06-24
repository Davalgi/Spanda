//! Runtime certification gate before executing deploy-target programs.

use crate::verify::verify_certification_proof;
use spanda_ast::nodes::Program;
use spanda_error::SpandaError;
use spanda_hardware::CompatSeverity;

/// Fail fast when deploy/certify metadata does not satisfy runtime enforcement.
pub fn enforce_certification_runtime(program: &Program, strict: bool) -> Result<(), SpandaError> {
    // Description:
    //     Enforce certification runtime.
    //
    // Inputs:
    //     progra: &Program
    //         Caller-supplied progra.
    //     stric: bool
    //         Caller-supplied stric.
    //
    // Outputs:
    //     result: Result<(), SpandaError>
    //         Return value from `enforce_certification_runtime`.
    //
    // Example:

    //     let result = spanda_certify::runtime::enforce_certification_runtime(progra, stric);

    if !strict {
        return Ok(());
    }

    let items = verify_certification_proof(program, true);
    let blocking = items
        .iter()
        .find(|item| item.severity == CompatSeverity::Error);
    if let Some(item) = blocking {
        return Err(SpandaError::Runtime {
            message: format!("certification runtime gate: {}", item.message),
            line: item.line,
        });
    }
    Ok(())
}

/// Return true when runtime certification enforcement is enabled via environment.
pub fn certification_runtime_enabled_from_env() -> bool {
    // Description:
    //     Certification runtime enabled from env.
    //
    // Inputs:
    //     None.
    //
    // Outputs:
    //     result: bool
    //         Return value from `certification_runtime_enabled_from_env`.
    //
    // Example:

    //     let result = spanda_certify::runtime::certification_runtime_enabled_from_env();

    matches!(
        std::env::var("SPANDA_ENFORCE_CERTIFY").ok().as_deref(),
        Some("1") | Some("true") | Some("yes")
    )
}
