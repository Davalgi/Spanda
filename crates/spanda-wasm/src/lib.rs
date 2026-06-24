//! src crate public API and re-exports.
//!
use serde::{Deserialize, Serialize};
use spanda_driver::{check, lower_to_sir, run, verify_compatibility, RunOptions, RunResult};
use spanda_error::Diagnostic;
use spanda_format::format_source;
use spanda_hardware::{CompatItem, CompatSeverity, VerifyOptions};
use wasm_bindgen::prelude::*;

#[derive(Serialize, Deserialize)]
struct CheckResponse {
    ok: bool,
    diagnostics: Vec<Diagnostic>,
}

#[derive(Serialize, Deserialize)]
struct RunResponse {
    ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<RunResult>,
    #[serde(skip_serializing_if = "Option::is_none")]
    diagnostics: Option<Vec<Diagnostic>>,
}

fn to_js<T: Serialize>(value: &T) -> JsValue {
    // Description:
    //     To js.
    //
    // Inputs:
    //     value: &T
    //         Caller-supplied value.
    //
    // Outputs:
    //     result: JsValue
    //         Return value from `to_js`.
    //
    // Example:
    //     let result = spanda_wasm::to_js(value);

    // Produce NULL) as the result.
    serde_wasm_bindgen::to_value(value).unwrap_or(JsValue::NULL)
}

#[wasm_bindgen]
pub fn wasm_check(source: &str) -> JsValue {
    // Description:
    //     Wasm check.
    //
    // Inputs:
    //     source: &str
    //         Caller-supplied source.
    //
    // Outputs:
    //     result: JsValue
    //         Return value from `wasm_check`.
    //
    // Example:
    //     let result = spanda_wasm::wasm_check(source);

    // Compute resp for the following logic.
    let resp = match check(source) {
        Ok(()) => CheckResponse {
            ok: true,
            diagnostics: vec![],
        },
        Err(e) => CheckResponse {
            ok: false,
            diagnostics: e.diagnostics(),
        },
    };
    to_js(&resp)
}

#[wasm_bindgen]
pub fn wasm_run(source: &str, max_loop_iterations: u32) -> JsValue {
    // Description:
    //     Wasm run.
    //
    // Inputs:
    //     source: &str
    //         Caller-supplied source.
    //     ax_loop_iterations: u32
    //         Caller-supplied ax loop iterations.
    //
    // Outputs:
    //     result: JsValue
    //         Return value from `wasm_run`.
    //
    // Example:
    //     let result = spanda_wasm::wasm_run(source, ax_loop_iterations);

    // Compute resp for the following logic.
    let resp = match run(
        source,
        RunOptions {
            max_loop_iterations: max_loop_iterations as usize,
            ..Default::default()
        },
    ) {
        Ok(result) => RunResponse {
            ok: true,
            result: Some(result),
            diagnostics: None,
        },
        Err(e) => RunResponse {
            ok: false,
            result: None,
            diagnostics: Some(e.diagnostics()),
        },
    };
    to_js(&resp)
}

#[wasm_bindgen]
pub fn wasm_ir(source: &str) -> JsValue {
    // Description:
    //     Wasm ir.
    //
    // Inputs:
    //     source: &str
    //         Caller-supplied source.
    //
    // Outputs:
    //     result: JsValue
    //         Return value from `wasm_ir`.
    //
    // Example:
    //     let result = spanda_wasm::wasm_ir(source);

    // Match on lower to sir and handle each case.
    match lower_to_sir(source) {
        Ok(sir) => serde_wasm_bindgen::to_value(&sir).unwrap_or(JsValue::NULL),
        Err(e) => to_js(&CheckResponse {
            ok: false,
            diagnostics: e.diagnostics(),
        }),
    }
}

#[wasm_bindgen]
pub fn wasm_fmt(source: &str) -> String {
    // Description:
    //     Wasm fmt.
    //
    // Inputs:
    //     source: &str
    //         Caller-supplied source.
    //
    // Outputs:
    //     result: String
    //         Return value from `wasm_fmt`.
    //
    // Example:
    //     let result = spanda_wasm::wasm_fmt(source);

    // Produce format source as the result.
    format_source(source)
}

#[wasm_bindgen]
pub fn wasm_version() -> String {
    // Description:
    //     Wasm version.
    //
    // Inputs:
    //     None.
    //
    // Outputs:
    //     result: String
    //         Return value from `wasm_version`.
    //
    // Example:
    //     let result = spanda_wasm::wasm_version();

    // Produce to string as the result.
    env!("CARGO_PKG_VERSION").to_string()
}

#[derive(Serialize, Deserialize)]
struct VerifyResponse {
    ok: bool,
    compatible: bool,
    items: Vec<CompatItem>,
}

#[wasm_bindgen]
pub fn wasm_verify(source: &str) -> JsValue {
    // Description:
    //     Wasm verify.
    //
    // Inputs:
    //     source: &str
    //         Caller-supplied source.
    //
    // Outputs:
    //     result: JsValue
    //         Return value from `wasm_verify`.
    //
    // Example:
    //     let result = spanda_wasm::wasm_verify(source);

    // Compute resp for the following logic.
    let resp = match verify_compatibility(source, &VerifyOptions::default()) {
        Ok(report) => VerifyResponse {
            ok: report.compatible,
            compatible: report.compatible,
            items: report.items,
        },
        Err(e) => VerifyResponse {
            ok: false,
            compatible: false,
            items: e
                .diagnostics()
                .into_iter()
                .map(|d| CompatItem {
                    category: "error".into(),
                    message: d.message,
                    severity: CompatSeverity::Error,
                    line: d.line,
                    column: d.column,
                })
                .collect(),
        },
    };
    to_js(&resp)
}
