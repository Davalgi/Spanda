//! Optional ONNX inference for learned anomaly `scan_learned` dispatch.
//!

use serde_json::json;
use std::io::{Read, Write};
use std::process::{Command, Stdio};

/// True when an anomaly or general ONNX model path is configured.
pub fn onnx_anomaly_enabled() -> bool {
    // Description:
    //     Onnx anomaly enabled.
    //
    // Inputs:
    //     None.
    //
    // Outputs:
    //     result: bool
    //         Return value from `onnx_anomaly_enabled`.
    //
    // Example:

    //     let result = spanda_providers::anomaly_onnx::onnx_anomaly_enabled();

    std::env::var("SPANDA_ANOMALY_ONNX_MODEL_PATH").is_ok()
        || std::env::var("SPANDA_ONNX_MODEL_PATH").is_ok()
}

/// Threshold + EMA volatility score used when ONNX is unavailable.
pub fn threshold_anomaly_score(observed: f64, volatility: f64) -> f64 {
    // Description:
    //     Threshold anomaly score.
    //
    // Inputs:
    //     observed: f64
    //         Caller-supplied observed.
    //     volatility: f64
    //         Caller-supplied volatility.
    //
    // Outputs:
    //     result: f64
    //         Return value from `threshold_anomaly_score`.
    //
    // Example:

    //     let result = spanda_providers::anomaly_onnx::threshold_anomaly_score(observed, volatility);

    if observed < 0.85 || volatility > 0.25 {
        1.0
    } else {
        0.0
    }
}

/// Score for `scan_learned`: ONNX when configured, otherwise lean thresholds.
pub fn scan_learned_score(detector: &str, observed: f64, volatility: f64) -> f64 {
    // Description:
    //     Scan learned score.
    //
    // Inputs:
    //     detector: &str
    //         Caller-supplied detector.
    //     observed: f64
    //         Caller-supplied observed.
    //     volatility: f64
    //         Caller-supplied volatility.
    //
    // Outputs:
    //     result: f64
    //         Return value from `scan_learned_score`.
    //
    // Example:

    //     let result = spanda_providers::anomaly_onnx::scan_learned_score(detector, observed, volatility);

    if onnx_anomaly_enabled() {
        if let Some(raw) = call_onnx_anomaly_infer(detector, observed, volatility) {
            return if raw > 0.5 { 1.0 } else { 0.0 };
        }
    }
    threshold_anomaly_score(observed, volatility)
}

fn call_onnx_anomaly_infer(detector: &str, observed: f64, volatility: f64) -> Option<f64> {
    // Description:
    //     Call onnx anomaly infer.
    //
    // Inputs:
    //     detector: &str
    //         Caller-supplied detector.
    //     observed: f64
    //         Caller-supplied observed.
    //     volatility: f64
    //         Caller-supplied volatility.
    //
    // Outputs:
    //     result: Option<f64>
    //         Return value from `call_onnx_anomaly_infer`.
    //
    // Example:

    //     let result = spanda_providers::anomaly_onnx::call_onnx_anomaly_infer(detector, observed, volatility);

    let features = json!({
        "detector": detector,
        "observed": observed,
        "volatility": volatility,
    });
    let response = call_python_bridge("onnx_anomaly_infer", vec![json!(features.to_string())])?;
    match response.get("result") {
        Some(serde_json::Value::Number(n)) => n.as_f64(),
        Some(serde_json::Value::String(s)) => s.parse().ok(),
        _ => None,
    }
}

fn call_python_bridge(fn_name: &str, args: Vec<serde_json::Value>) -> Option<serde_json::Value> {
    // Description:
    //     Call python bridge.
    //
    // Inputs:
    //     fn_name: &str
    //         Caller-supplied fn name.
    //     args: Vec<serde_json::Value>
    //         Caller-supplied args.
    //
    // Outputs:
    //     result: Option<serde_json::Value>
    //         Return value from `call_python_bridge`.
    //
    // Example:

    //     let result = spanda_providers::anomaly_onnx::call_python_bridge(fn_name, args);

    let script = bridge_script_path()?;
    let python = std::env::var("SPANDA_PYTHON").unwrap_or_else(|_| "python3".into());
    let request = json!({ "fn": fn_name, "args": args });
    let mut child = Command::new(python)
        .arg(script)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .ok()?;
    {
        let stdin = child.stdin.as_mut()?;
        let payload = serde_json::to_string(&request).ok()?;
        stdin.write_all(payload.as_bytes()).ok()?;
    }
    let mut stdout = String::new();
    child.stdout.as_mut()?.read_to_string(&mut stdout).ok()?;
    let _ = child.wait();
    let parsed: serde_json::Value = serde_json::from_str(stdout.trim()).ok()?;
    if parsed.get("ok") == Some(&json!(true)) {
        Some(parsed)
    } else {
        None
    }
}

fn bridge_script_path() -> Option<String> {
    // Description:
    //     Bridge script path.
    //
    // Inputs:
    //     None.
    //
    // Outputs:
    //     result: Option<String>
    //         Return value from `bridge_script_path`.
    //
    // Example:

    //     let result = spanda_providers::anomaly_onnx::bridge_script_path();

    if let Ok(path) = std::env::var("SPANDA_PYTHON_BRIDGE") {
        if std::path::Path::new(&path).is_file() {
            return Some(path);
        }
    }
    let candidates = [
        "scripts/spanda_python_bridge.py".to_string(),
        format!(
            "{}/../../scripts/spanda_python_bridge.py",
            env!("CARGO_MANIFEST_DIR")
        ),
    ];
    for candidate in candidates {
        if std::path::Path::new(&candidate).is_file() {
            return Some(candidate);
        }
    }
    std::env::current_dir().ok().and_then(|cwd| {
        let path = cwd.join("scripts/spanda_python_bridge.py");
        if path.is_file() {
            Some(path.to_string_lossy().into_owned())
        } else {
            None
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn threshold_flags_low_confidence_or_high_volatility() {
        // Description:
        //     Threshold flags low confidence or high volatility.
        //
        // Inputs:
        //     None.
        //
        // Outputs:
        //     None.
        //
        // Example:

        //     let result = spanda_providers::anomaly_onnx::threshold_flags_low_confidence_or_high_volatility();

        assert_eq!(threshold_anomaly_score(0.80, 0.0), 1.0);
        assert_eq!(threshold_anomaly_score(0.95, 0.30), 1.0);
        assert_eq!(threshold_anomaly_score(0.95, 0.10), 0.0);
    }
}
