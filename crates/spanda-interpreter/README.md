# spanda-interpreter

Phase 4 lean-core **staging crate** for the Spanda interpreter and public run API.

Today this crate depends on `spanda-core` and re-exports the interpreter surface (`Interpreter`, `InterpreterOptions`, `RobotBackend`, `SimRobotBackend`, `RunOptions`, `RunResult`, `run`, `run_program`, and related helpers). Callers can depend on `spanda-interpreter` instead of the full core facade while the extraction proceeds.

## What stays in core (for now)

The full ~9k-line `Interpreter` implementation remains in `spanda-core/src/runtime.rs`. It is intentionally the **orchestration root**: it wires HAL, safety, transport, providers, scheduling, and mission replay until those subsystems expose narrower host traits on [`spanda-runtime::RuntimeHost`](../../spanda-runtime/README.md).

## Migration direction

1. Subsystems move behind `RuntimeHost` and smaller traits in `spanda-runtime` and domain crates.
2. `Interpreter` methods shrink and delegate to extracted hosts.
3. The `Interpreter` body moves from `spanda-core/src/runtime.rs` into this crate.
4. `spanda-core` keeps a thin compatibility re-export shim during the transition.

See [lean-core-roadmap.md](../../docs/lean-core-roadmap.md) Phase 4.
