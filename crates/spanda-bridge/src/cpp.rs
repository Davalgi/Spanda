//! Subprocess C++ bridge for `extern cpp fn` declarations.
//!
//! Invokes a small C++ helper binary (built via `build.rs`, or `SPANDA_CPP_BRIDGE`)
//! with the same JSON stdin/stdout protocol as the Python bridge.

use spanda_ast::foundations::ExternFnDecl;
use spanda_error::SpandaError;
use spanda_runtime::value::RuntimeValue;
use std::path::PathBuf;

use super::protocol::call_subprocess_bridge;

/// Resolve the C++ bridge executable path.
pub fn bridge_binary_path() -> Option<PathBuf> {
    // Description:
    //     Bridge binary path.
    //
    // Inputs:
    //     None.
    //
    // Outputs:
    //     result: Option<PathBuf>
    //         Return value from `bridge_binary_path`.
    //
    // Example:
    //     let result = spanda_bridge::cpp::bridge_binary_path();

    // handle the success value from var.
    if let Ok(path) = std::env::var("SPANDA_CPP_BRIDGE") {
        let p = PathBuf::from(path);

        // Continue only when the path is a regular file.
        if p.is_file() {
            return Some(p);
        }
    }
    candidate_binary_paths()
        .into_iter()
        .find(|candidate| candidate.is_file())
}

fn candidate_binary_paths() -> Vec<PathBuf> {
    // Description:
    //     Candidate binary paths.
    //
    // Inputs:
    //     None.
    //
    // Outputs:
    //     result: Vec<PathBuf>
    //         Return value from `candidate_binary_paths`.
    //
    // Example:
    //     let result = spanda_bridge::cpp::candidate_binary_paths();

    // Create mutable paths for accumulating results.
    let mut paths = Vec::new();

    // Emit output when option env! provides a path.
    if let Some(path) = option_env!("SPANDA_CPP_BRIDGE_BIN") {
        paths.push(PathBuf::from(path));
    }
    paths.push(PathBuf::from("scripts/spanda_cpp_bridge"));
    paths.push(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../scripts/spanda_cpp_bridge"));

    // Handle the success value from current dir.
    if let Ok(cwd) = std::env::current_dir() {
        paths.push(cwd.join("scripts/spanda_cpp_bridge"));
    }
    paths
}

pub fn bridge_available() -> bool {
    // Description:
    //     Bridge available.
    //
    // Inputs:
    //     None.
    //
    // Outputs:
    //     result: bool
    //         Return value from `bridge_available`.
    //
    // Example:
    //     let result = spanda_bridge::cpp::bridge_available();

    // Produce is some as the result.
    bridge_binary_path().is_some()
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
    //     let result = spanda_bridge::cpp::call_extern(decl, args);

    // Produce #[cfg as the result.
    #[cfg(feature = "cpp-native")]
    // Take this path when std::env::var("SPANDA CPP SUBPROCESS").is err().
    if std::env::var("SPANDA_CPP_SUBPROCESS").is_err() {
        // Take this path when super::cpp native::native available().
        if super::cpp_native::native_available() {
            return super::cpp_native::call_extern(decl, args);
        }
    }
    let line = decl.span.start.line;
    let binary = bridge_binary_path().ok_or_else(|| SpandaError::Runtime {
        message:
            "C++ bridge binary not found — set SPANDA_CPP_BRIDGE or rebuild with a C++ compiler"
                .into(),
        line,
    })?;
    call_subprocess_bridge("C++", &binary, &[], decl, args)
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
        //     let result = spanda_bridge::cpp::test_decl(name);

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
    fn subprocess_cpp_add_when_binary_available() {
        // Description:
        //     Subprocess cpp add when binary available.
        //
        // Inputs:
        //     None.
        //
        // Outputs:
        //     None.
        //
        // Example:

        //     let result = spanda_bridge::cpp::subprocess_cpp_add_when_binary_available();

        if !bridge_available() {
            return;
        }
        let decl = test_decl("cpp_add");
        let result = call_extern(
            &decl,
            &[
                RuntimeValue::Number {
                    value: 4.0,
                    unit: spanda_ast::nodes::UnitKind::None,
                },
                RuntimeValue::Number {
                    value: 5.0,
                    unit: spanda_ast::nodes::UnitKind::None,
                },
            ],
        )
        .expect("cpp_add");
        assert!(matches!(
            result,
            RuntimeValue::Number { value, .. } if (value - 9.0).abs() < f64::EPSILON
        ));
    }

    #[test]
    fn subprocess_unknown_fn_errors() {
        // Description:
        //     Subprocess unknown fn errors.
        //
        // Inputs:
        //     None.
        //
        // Outputs:
        //     None.
        //
        // Example:

        //     let result = spanda_bridge::cpp::subprocess_unknown_fn_errors();

        if !bridge_available() {
            return;
        }
        let decl = test_decl("cpp_missing");
        let err = call_extern(&decl, &[]).unwrap_err();
        assert!(err.to_string().contains("Unknown cpp extern"));
    }
}
