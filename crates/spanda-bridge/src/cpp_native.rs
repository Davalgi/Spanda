//! In-process C++ bridge (optional `cpp-native` feature).
//!
//! Calls the same handler dispatch as the subprocess bridge via a C ABI.

use spanda_ast::foundations::ExternFnDecl;
use spanda_error::SpandaError;
use spanda_runtime::value::RuntimeValue;
use std::ffi::CString;
use std::os::raw::c_char;

use super::protocol::{json_to_runtime_value, runtime_value_to_json};

extern "C" {
    fn spanda_cpp_bridge_call(
        fn_name: *const c_char,
        args_json: *const c_char,
        out_buf: *mut c_char,
        out_len: usize,
    ) -> i32;
}

pub fn native_available() -> bool {
    // Description:
    //     Reports whether the C++ native bridge was compiled into this binary.
    //
    // Inputs:
    //     None.
    //
    // Outputs:
    //     available: bool
    //         `true` when `SPANDA_CPP_NATIVE` was set at compile time.
    //
    // Example:
    //     if spanda_bridge::cpp_native::native_available() { /* use in-process bridge */ }

    option_env!("SPANDA_CPP_NATIVE").is_some()
}

pub fn call_extern(
    decl: &ExternFnDecl,
    args: &[RuntimeValue],
) -> Result<RuntimeValue, SpandaError> {
    // Description:
    //     Call extern.
    //
    // Inputs:
    //     decl: &ExternFnDecl
    //         Caller-supplied decl.
    //     args: &[RuntimeValue]
    //         Caller-supplied args.
    //
    // Outputs:
    //     result: Result<RuntimeValue, SpandaError>
    //         Return value from `call_extern`.
    //
    // Example:
    //     let result = spanda_bridge::cpp_native::call_extern(decl, args);

    // Requires `cpp-native` Cargo feature.
    let line = decl.span.start.line;
    let args_json = serde_json::json!({
        "args": args.iter().map(runtime_value_to_json).collect::<Vec<_>>()
    });
    let args_json = serde_json::to_string(&args_json).map_err(|e| SpandaError::Runtime {
        message: format!("Failed to encode native C++ bridge args: {e}"),
        line,
    })?;
    let fn_name = CString::new(decl.name.as_str()).map_err(|e| SpandaError::Runtime {
        message: format!("Invalid C++ extern name: {e}"),
        line,
    })?;
    let args_c = CString::new(args_json).map_err(|e| SpandaError::Runtime {
        message: format!("Invalid C++ bridge args: {e}"),
        line,
    })?;
    let mut out = vec![0i8; 4096];
    let ok = unsafe {
        spanda_cpp_bridge_call(
            fn_name.as_ptr(),
            args_c.as_ptr(),
            out.as_mut_ptr(),
            out.len(),
        )
    };

    // Take the branch when ok equals 0.
    if ok == 0 {
        return Err(SpandaError::Runtime {
            message: "C++ native bridge call failed".into(),
            line,
        });
    }
    let response = unsafe {
        std::ffi::CStr::from_ptr(out.as_ptr())
            .to_string_lossy()
            .into_owned()
    };
    let parsed: serde_json::Value =
        serde_json::from_str(&response).map_err(|e| SpandaError::Runtime {
            message: format!("Invalid C++ native bridge JSON: {e}"),
            line,
        })?;

    // Take this path when parsed.
    if parsed
        .get("ok")
        .and_then(|v| v.as_bool())
        .is_some_and(|ok| !ok)
    {
        let msg = parsed
            .get("error")
            .and_then(|v| v.as_str())
            .unwrap_or("C++ native bridge call failed");
        return Err(SpandaError::Runtime {
            message: msg.to_string(),
            line,
        });
    }
    Ok(json_to_runtime_value(
        parsed.get("result").unwrap_or(&serde_json::Value::Null),
        &decl.return_type,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use spanda_ast::foundations::BridgeKind;
    use spanda_ast::nodes::{SourceLocation, Span, SpandaType};

    fn test_decl(name: &str) -> ExternFnDecl {
        // Description:
        //     Test decl.
        //
        // Inputs:
        //     name: &str
        //         Caller-supplied name.
        //
        // Outputs:
        //     result: ExternFnDecl
        //         Return value from `test_decl`.
        //
        // Example:
        //     let result = spanda_bridge::cpp_native::test_decl(name);

        // Produce ExternFnDecl as the result.
        ExternFnDecl {
            name: name.into(),
            library: Some("cpp".into()),
            bridge: BridgeKind::Cpp,
            params: vec![],
            return_type: SpandaType::Int,
            span: Span {
                start: SourceLocation {
                    line: 1,
                    column: 1,
                    offset: 0,
                },
                end: SourceLocation {
                    line: 1,
                    column: 1,
                    offset: 0,
                },
            },
        }
    }

    #[test]
    fn native_cpp_add_when_available() {
        // Description:
        //     Native cpp add when available.
        //
        // Inputs:
        //     None.
        //
        // Outputs:
        //     None.
        //
        // Example:

        //     let result = spanda_bridge::cpp_native::native_cpp_add_when_available();

        if !native_available() {
            return;
        }
        let decl = test_decl("cpp_add");
        let result = call_extern(
            &decl,
            &[
                RuntimeValue::Number {
                    value: 2.0,
                    unit: spanda_ast::nodes::UnitKind::None,
                },
                RuntimeValue::Number {
                    value: 5.0,
                    unit: spanda_ast::nodes::UnitKind::None,
                },
            ],
        )
        .expect("cpp_add native");
        assert!(matches!(
            result,
            RuntimeValue::Number { value, .. } if (value - 7.0).abs() < f64::EPSILON
        ));
    }
}
