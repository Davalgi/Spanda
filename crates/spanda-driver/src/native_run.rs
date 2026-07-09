//! Native LLVM execution path with interpreter LTS fallback.
//!
#[cfg(feature = "llvm")]
use spanda_interpreter::{RunOptions, RunResult};
#[cfg(feature = "llvm")]
use spanda_llvm::{compile_native, CompileNativeOptions};
#[cfg(feature = "llvm")]
use spanda_runtime::robot_state::{PoseState, RobotState, VelocityState};
#[cfg(feature = "llvm")]
use spanda_runtime::telemetry::RuntimeTelemetry;
#[cfg(feature = "llvm")]
use spanda_sir::{lower_program, sir_native_eligible};
#[cfg(feature = "llvm")]
use std::path::PathBuf;
#[cfg(feature = "llvm")]
use std::process::Command;

/// Outcome of attempting native execution before interpreter fallback.
#[cfg(feature = "llvm")]
pub enum NativeRunAttempt {
    Executed(RunResult),
    Fallback { reason: String },
}

#[cfg(feature = "llvm")]
pub fn try_run_native(source: &str, options: &RunOptions) -> NativeRunAttempt {
    if !options.execution_runtime.prefers_native() {
        return NativeRunAttempt::Fallback {
            reason: "runtime mode prefers interpreter".into(),
        };
    }

    let program = match crate::compile::compile(source) {
        Ok(result) => result.program,
        Err(error) => {
            return NativeRunAttempt::Fallback {
                reason: format!("compile failed: {error}"),
            };
        }
    };

    let sir = lower_program(&program);
    if !sir_native_eligible(&sir) {
        return NativeRunAttempt::Fallback {
            reason: "program contains statements unsupported by native codegen".into(),
        };
    }

    let workspace = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    let output = workspace.join("target/spanda-native/spanda-program-run");
    let compile = compile_native(
        &sir,
        &CompileNativeOptions {
            output: output.clone(),
            clang: None,
            workspace_root: workspace,
            target_triple: None,
            hal_profile: None,
        },
    );

    let compile = match compile {
        Ok(result) => result,
        Err(error) => {
            return NativeRunAttempt::Fallback {
                reason: format!("native compile failed: {error}"),
            };
        }
    };

    let output = Command::new(&compile.executable)
        .output()
        .map_err(|error| error.to_string());

    let output = match output {
        Ok(output) => output,
        Err(error) => {
            return NativeRunAttempt::Fallback {
                reason: format!("native execute failed: {error}"),
            };
        }
    };

    if !output.status.success() && options.execution_runtime.allows_interpreter_fallback() {
        return NativeRunAttempt::Fallback {
            reason: format!(
                "native binary exited with status {}",
                output.status.code().unwrap_or(-1)
            ),
        };
    }

    let mut logs = Vec::new();
    if !output.stdout.is_empty() {
        logs.extend(
            String::from_utf8_lossy(&output.stdout)
                .lines()
                .map(str::to_string),
        );
    }
    if !output.stderr.is_empty() {
        logs.extend(
            String::from_utf8_lossy(&output.stderr)
                .lines()
                .map(|line| format!("[native] {line}")),
        );
    }
    logs.insert(
        0,
        format!(
            "runtime: native ({})",
            compile.executable.display()
        ),
    );

    NativeRunAttempt::Executed(RunResult {
        state: RobotState {
            pose: PoseState {
                x: 0.0,
                y: 0.0,
                theta: 0.0,
                z: None,
            },
            velocity: VelocityState {
                linear: 0.0,
                angular: 0.0,
            },
            emergency_stop: false,
        },
        events: logs.clone(),
        logs,
        metrics: RuntimeTelemetry::default(),
        mission_trace: None,
        twin_replay: None,
    })
}

#[cfg(feature = "llvm")]
pub fn warn_native_fallback(reason: &str) {
    if matches!(
        std::env::var("SPANDA_QUIET").ok().as_deref(),
        Some("1") | Some("true") | Some("yes") | Some("on")
    ) {
        return;
    }
    eprintln!(
        "[spanda] native runtime unavailable ({reason}); falling back to interpreter LTS"
    );
}

#[cfg(not(feature = "llvm"))]
pub enum NativeRunAttempt {
    Fallback { reason: String },
}

#[cfg(not(feature = "llvm"))]
use spanda_interpreter::RunOptions;

#[cfg(not(feature = "llvm"))]
pub fn try_run_native(_source: &str, _options: &RunOptions) -> NativeRunAttempt {
    NativeRunAttempt::Fallback {
        reason: "CLI built without llvm feature".into(),
    }
}

#[cfg(not(feature = "llvm"))]
pub fn warn_native_fallback(reason: &str) {
    let _ = reason;
}
