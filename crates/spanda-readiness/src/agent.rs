//! Agent and service readiness evaluation from deployed program source.

use crate::engine::evaluate_readiness_with_runtime;
use crate::runtime::build_runtime_context_with_config;
use crate::target::readiness_options_from_flags;
use crate::types::ReadinessReport;
use spanda_lexer::tokenize;
use spanda_parser::parse;

/// Evaluate readiness for an on-device agent from program source text.
pub fn evaluate_agent_readiness(
    source: &str,
    target: Option<&str>,
    include_runtime: bool,
    inject_health_faults: bool,
) -> Result<ReadinessReport, String> {
    // Description:
    //     Evaluate agent readiness.
    //
    // Inputs:
    //     source: &str
    //         Caller-supplied source.
    //     arge: Option<&str>
    //         Caller-supplied arge.
    //     include_runtime: bool
    //         Caller-supplied include runtime.
    //     inject_health_faults: bool
    //         Caller-supplied inject health faults.
    //
    // Outputs:
    //     result: Result<ReadinessReport, String>
    //         Return value from `evaluate_agent_readiness`.
    //
    // Example:

    //     let result = spanda_readiness::agent::evaluate_agent_readiness(source, arge, include_runtime, inject_health_faults);

    let tokens = tokenize(source).map_err(|e| e.to_string())?;
    let program = parse(tokens).map_err(|e| e.to_string())?;
    let options = readiness_options_from_flags(
        &program,
        target.map(String::from),
        include_runtime,
        inject_health_faults,
        false,
        false,
    );
    let runtime = options.include_runtime.then(|| {
        build_runtime_context_with_config(
            &program,
            options.inject_health_faults,
            options.system_config.as_deref(),
        )
    });
    Ok(evaluate_readiness_with_runtime(
        &program,
        &options,
        runtime.as_ref(),
    ))
}

/// JSON payload for `GET /v1/readiness` agent endpoints.
pub fn evaluate_agent_readiness_json(
    source: &str,
    target: Option<&str>,
    include_runtime: bool,
    inject_health_faults: bool,
) -> Result<String, String> {
    // Description:
    //     Evaluate agent readiness json.
    //
    // Inputs:
    //     source: &str
    //         Caller-supplied source.
    //     arge: Option<&str>
    //         Caller-supplied arge.
    //     include_runtime: bool
    //         Caller-supplied include runtime.
    //     inject_health_faults: bool
    //         Caller-supplied inject health faults.
    //
    // Outputs:
    //     result: Result<String, String>
    //         Return value from `evaluate_agent_readiness_json`.
    //
    // Example:

    //     let result = spanda_readiness::agent::evaluate_agent_readiness_json(source, arge, include_runtime, inject_health_faults);

    let report = evaluate_agent_readiness(source, target, include_runtime, inject_health_faults)?;
    serde_json::to_string(&serde_json::json!({
        "ok": true,
        "mission_ready": report.mission_ready,
        "readiness": report,
    }))
    .map_err(|e| e.to_string())
}
