//! Execution runtime selection tests.
//!
use spanda_runtime::ExecutionRuntime;

#[test]
fn execution_runtime_defaults_to_auto() {
    std::env::remove_var("SPANDA_RUNTIME");
    assert_eq!(ExecutionRuntime::resolve(None), ExecutionRuntime::Auto);
}

#[test]
fn execution_runtime_parses_cli_flag() {
    assert_eq!(
        ExecutionRuntime::resolve(Some("native")),
        ExecutionRuntime::Native
    );
    assert_eq!(
        ExecutionRuntime::resolve(Some("interpreter")),
        ExecutionRuntime::Interpreter
    );
    assert_eq!(
        ExecutionRuntime::resolve(Some("llvm")),
        ExecutionRuntime::Native
    );
}

#[test]
fn execution_runtime_respects_env_when_no_flag() {
    std::env::set_var("SPANDA_RUNTIME", "interpreter");
    assert_eq!(ExecutionRuntime::resolve(None), ExecutionRuntime::Interpreter);
    std::env::remove_var("SPANDA_RUNTIME");
}

#[test]
fn execution_runtime_flag_overrides_env() {
    std::env::set_var("SPANDA_RUNTIME", "interpreter");
    assert_eq!(
        ExecutionRuntime::resolve(Some("native")),
        ExecutionRuntime::Native
    );
    std::env::remove_var("SPANDA_RUNTIME");
}
