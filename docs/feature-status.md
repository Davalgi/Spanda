# Feature Status

Honest snapshot of Spanda capabilities. **Stubbed** = syntax or API exists without real external integration.

## Language core

| Feature | Status | Notes |
|---------|--------|-------|
| Lexer / parser / AST | Implemented | Rust authoritative; TS mirror includes hardware/deploy |
| Type checker + units | Implemented | Physical unit algebra enforced |
| modules / imports | Implemented | Rust + TS `ModuleRegistry`; project vendor via `spanda install` |
| structs / enums / traits | Implemented | Enum variants with associated data; generic struct params (declaration) |
| generics | Partially implemented | Module fn type params; struct `<T>` declarations (literals planned) |
| match / Result / Option | Implemented | |
| async / await | Implemented | Cooperative single-threaded |
| spawn / select / channels | Partially implemented | Cooperative concurrency |
| test blocks | Implemented | Rust runtime + TS `runTests()` |
| `extern fn` / FFI | Partially implemented | `extern python`/`extern cpp` subprocess bridges; optional PyO3 in-process |
| Spanda IR (SIR) | Partially implemented | `spanda ir`; stmt lowering incl. loop/publish/if/subscribe |
| Codegen / LLVM | Partially implemented | `spanda llvm-ir` + `spanda compile-native` via clang; `--target-triple` |

## Autonomous systems

| Feature | Status | Notes |
|---------|--------|-------|
| robot / sensor / actuator | Implemented | |
| agent / goal / task / skill | Implemented | Mock AI |
| ActionProposal → SafeAction | Implemented | Compile + runtime |
| safety zones / emergency stop | Implemented | |
| deterministic scheduler | Implemented | `task every Nms` |
| state machine / events | Implemented | |
| twin / replay | Implemented | Replay buffer; **`twin sync`** telemetry/replay wired |
| observe / fusion | Implemented | |
| verify { } behavioral assertions | Implemented | |
| hardware / deploy | Implemented | Rust verify CLI; TS parse + deploy validation |

## Tooling

| Feature | Status | Notes |
|---------|--------|-------|
| Native CLI (full) | Implemented | check, verify, run, fmt, lint, doc, package |
| TypeScript CLI | Implemented | Delegates to Rust when built; includes `llvm-ir` / `compile-native` |
| Formatter / linter / docgen | Implemented | Rust |
| LSP | Partially implemented | Symbols include hardware/deploy |
| DAP debugger | Partially implemented | Breakpoints + continue/step commands |
| N-API | Partially implemented | check, run, verify, sir, fmt |
| WASM | Partially implemented | check, run, verify, sir, fmt |

## Ecosystem / FFI

| Feature | Status | Notes |
|---------|--------|-------|
| python.* / cpp.* imports | Partially implemented | Subprocess bridges; optional `python-native` / `cpp-native` in-process |
| ROS2 adapter | Partially implemented | Log stub; live publish via Python bridge when `SPANDA_ROS2_LIVE=1` |
| Transport adapters | Partially implemented | In-memory + log stubs; optional live ROS2/MQTT via Python bridge |
| Package manager | Partially implemented | spanda.toml, lockfile, git vendor, local + optional remote registry |
| LLVM / native codegen | Partially implemented | `spanda compile-native` links IR + `crates/spanda-rt`; cross-target via `--target-triple` |

See also [README.md](../README.md), [ffi-and-ecosystem.md](./ffi-and-ecosystem.md), [compiler-backend-roadmap.md](./compiler-backend-roadmap.md).
