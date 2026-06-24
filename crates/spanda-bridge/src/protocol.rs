//! Shared JSON stdin/stdout protocol for subprocess FFI bridges.
//!
//! Defines request/response envelopes and helpers to spawn bridge processes
//! for Python and C++ extern function calls.

use serde::{Deserialize, Serialize};
use spanda_ast::foundations::ExternFnDecl;
use spanda_ast::nodes::SpandaType;
use spanda_error::SpandaError;
use spanda_runtime::value::RuntimeValue;
use std::io::Write;
use std::path::Path;
use std::process::{Child, Command, Stdio};
use std::sync::mpsc;
use std::time::Duration;

/// JSON request envelope sent to a bridge subprocess on stdin.
#[derive(Serialize)]
pub struct BridgeRequest<'a> {
    /// Extern function name to invoke.
    #[serde(rename = "fn")]
    pub fn_name: &'a str,

    /// JSON-encoded argument values.
    pub args: Vec<serde_json::Value>,
}

/// JSON response envelope read from a bridge subprocess stdout.
#[derive(Deserialize)]
pub struct BridgeResponse {
    /// `true` when the handler succeeded.
    pub ok: bool,

    /// Handler return value when `ok` is true.
    pub result: Option<serde_json::Value>,

    /// Error message when `ok` is false.
    pub error: Option<String>,
}

pub fn runtime_value_to_json(value: &RuntimeValue) -> serde_json::Value {
    // Description:
    //     Runtime value to json.
    //
    // Inputs:
    //     value: &RuntimeValue
    //         Caller-supplied value.
    //
    // Outputs:
    //     result: serde_json::Value
    //         Return value from `runtime_value_to_json`.
    //
    // Example:
    //     let result = spanda_bridge::protocol::runtime_value_to_json(value);

    // assert_eq!(json, serde_json::json!(true));
    match value {
        RuntimeValue::Number { value, .. } => serde_json::Value::Number(
            serde_json::Number::from_f64(*value).unwrap_or_else(|| serde_json::Number::from(0)),
        ),
        RuntimeValue::Bool { value } => serde_json::Value::Bool(*value),
        RuntimeValue::String { value } => serde_json::Value::String(value.clone()),
        RuntimeValue::Void => serde_json::Value::Null,
        other => serde_json::Value::String(format!("{other:?}")),
    }
}

pub fn json_to_runtime_value(value: &serde_json::Value, return_type: &SpandaType) -> RuntimeValue {
    // Description:
    //     Json to runtime value.
    //
    // Inputs:
    //     value: &serde_json::Value
    //         Caller-supplied value.
    //     return_type: &SpandaType
    //         Caller-supplied return type.
    //
    // Outputs:
    //     result: RuntimeValue
    //         Return value from `json_to_runtime_value`.
    //
    // Example:
    //     let result = spanda_bridge::protocol::json_to_runtime_value(value, return_type);

    // Coerced [`RuntimeValue`] (defaults for missing fields).
    use spanda_ast::nodes::UnitKind;

    // Match on return type and handle each case.
    match return_type {
        SpandaType::Bool => RuntimeValue::Bool {
            value: value.as_bool().unwrap_or(false),
        },
        SpandaType::String => RuntimeValue::String {
            value: value.as_str().unwrap_or("").to_string(),
        },
        SpandaType::Int | SpandaType::Float | SpandaType::Number { .. } => RuntimeValue::Number {
            value: value.as_f64().unwrap_or(0.0),
            unit: UnitKind::None,
        },
        _ => match value {
            serde_json::Value::Number(n) => RuntimeValue::Number {
                value: n.as_f64().unwrap_or(0.0),
                unit: UnitKind::None,
            },
            serde_json::Value::Bool(b) => RuntimeValue::Bool { value: *b },
            serde_json::Value::String(s) => RuntimeValue::String { value: s.clone() },
            _ => RuntimeValue::Void,
        },
    }
}

pub fn call_subprocess_bridge(
    bridge_label: &str,
    executable: &Path,
    extra_args: &[&str],
    decl: &ExternFnDecl,
    args: &[RuntimeValue],
) -> Result<RuntimeValue, SpandaError> {
    // Description:
    //     Call subprocess bridge.
    //
    // Inputs:
    //     bridge_label: &str
    //         Caller-supplied bridge label.
    //     executable: &Path
    //         Caller-supplied executable.
    //     extra_args: &[&str]
    //         Caller-supplied extra args.
    //     decl: &ExternFnDecl
    //         Caller-supplied decl.
    //     args: &[RuntimeValue]
    //         Caller-supplied args.
    //
    // Outputs:
    //     result: Result<RuntimeValue, SpandaError>
    //         Return value from `call_subprocess_bridge`.
    //
    // Example:
    //     let result = spanda_bridge::protocol::call_subprocess_bridge(bridge_label, executable, extra_args, decl, args);

    // // Typically invoked via bridge::python::call_extern or bridge::cpp::call_extern.
    let line = decl.span.start.line;
    let request = BridgeRequest {
        fn_name: &decl.name,
        args: args.iter().map(runtime_value_to_json).collect(),
    };
    let request_json = serde_json::to_string(&request).map_err(|e| SpandaError::Runtime {
        message: format!("Failed to encode {bridge_label} bridge request: {e}"),
        line,
    })?;
    let mut command = Command::new(executable);
    command
        .args(extra_args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    let mut child = command.spawn().map_err(|e| SpandaError::Runtime {
        message: format!("Failed to spawn {bridge_label} bridge: {e}"),
        line,
    })?;

    // Take this path when let Some(mut stdin) = child.stdin.take().
    if let Some(mut stdin) = child.stdin.take() {
        stdin
            .write_all(request_json.as_bytes())
            .map_err(|e| SpandaError::Runtime {
                message: format!("Failed to write {bridge_label} bridge request: {e}"),
                line,
            })?;
        stdin.write_all(b"\n").ok();
    }
    let output = wait_child_with_timeout(child, bridge_label, line)?;

    // Handle output when the subprocess succeeds.
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(SpandaError::Runtime {
            message: format!(
                "{bridge_label} bridge exited with {}: {}",
                output.status,
                stderr.trim()
            ),
            line,
        });
    }
    let stdout = String::from_utf8_lossy(&output.stdout);
    let resp: BridgeResponse =
        serde_json::from_str(stdout.trim()).map_err(|e| SpandaError::Runtime {
            message: format!(
                "Invalid {bridge_label} bridge response: {e} (got: {})",
                stdout.trim()
            ),
            line,
        })?;

    // Take the branch when ok is false.
    if !resp.ok {
        return Err(SpandaError::Runtime {
            message: resp
                .error
                .unwrap_or_else(|| format!("{bridge_label} bridge call failed")),
            line,
        });
    }
    Ok(json_to_runtime_value(
        &resp.result.unwrap_or(serde_json::Value::Null),
        &decl.return_type,
    ))
}

fn bridge_timeout() -> Duration {
    // Description:
    //     Bridge timeout.
    //
    // Inputs:
    //     None.
    //
    // Outputs:
    //     result: Duration
    //         Return value from `bridge_timeout`.
    //
    // Example:

    //     let result = spanda_bridge::protocol::bridge_timeout();

    let secs = std::env::var("SPANDA_BRIDGE_TIMEOUT_SECS")
        .ok()
        .and_then(|value| value.parse::<u64>().ok())
        .unwrap_or(30)
        .min(3600);
    Duration::from_secs(secs)
}

fn wait_child_with_timeout(
    child: Child,
    bridge_label: &str,
    line: u32,
) -> Result<std::process::Output, SpandaError> {
    // Description:
    //     Wait child with timeout.
    //
    // Inputs:
    //     child: Child
    //         Caller-supplied child.
    //     bridge_label: &str
    //         Caller-supplied bridge label.
    //     line: u32
    //         Caller-supplied line.
    //
    // Outputs:
    //     result: Result<std::process::Output, SpandaError>
    //         Return value from `wait_child_with_timeout`.
    //
    // Example:

    //     let result = spanda_bridge::protocol::wait_child_with_timeout(child, bridge_label, line);

    let (tx, rx) = mpsc::channel();
    std::thread::spawn(move || {
        let _ = tx.send(child.wait_with_output());
    });
    match rx.recv_timeout(bridge_timeout()) {
        Ok(Ok(output)) => Ok(output),
        Ok(Err(err)) => Err(SpandaError::Runtime {
            message: format!("{bridge_label} bridge process failed: {err}"),
            line,
        }),
        Err(mpsc::RecvTimeoutError::Timeout) => Err(SpandaError::Runtime {
            message: format!(
                "{bridge_label} bridge timed out after {}s",
                bridge_timeout().as_secs()
            ),
            line,
        }),
        Err(mpsc::RecvTimeoutError::Disconnected) => Err(SpandaError::Runtime {
            message: format!("{bridge_label} bridge worker exited unexpectedly"),
            line,
        }),
    }
}
