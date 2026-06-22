# Lean-Core Roadmap

Phased plan to complete the package-first architecture after the initial scaffold.

## Phase 1 — Complete ✓

- Provider trait contracts in `spanda-core/src/providers/`
- `ProviderRegistry` and `bootstrap_default_providers()`
- 20 official package scaffolds under `packages/registry/`
- Compatibility shims documented on legacy core modules
- Architecture docs and migration guide
- TypeScript providers mirror and fleet CLI fix

## Phase 2 — Runtime wiring (complete ✓)

| Task | Status | Notes |
|------|--------|-------|
| Attach `ProviderRegistry` to `Interpreter` | Done | Auto-bootstrap when unset |
| Resolve official deps from `spanda.toml` | Done | `installed_official_packages()` |
| Load package providers from lockfile at `spanda run` | Done | CLI `run_options_for_file()` |
| Sync comm-bus transports for installed packages | Done | `sync_comm_bus_for_official_packages()` |
| Package-scoped provider bootstrap | Done | `bootstrap_providers_for_packages()` |
| Install reports official packages | Done | `spanda install` verbose output |

## Phase 3 — Crate extraction (complete ✓)

| Crate | Status | Notes |
|-------|--------|-------|
| `spanda-transport-mqtt` | Done | Live MQTT bridge extracted; core shim retained |
| `spanda-transport-ros2` | Done | Native rclrs + rclpy daemon extracted; core shims retained |
| `spanda-connectivity` | Done | Type catalogs and link classification extracted |
| `spanda-transport-dds` | Done | Live UDP multicast bridge extracted; core shim retained |
| `spanda-transport-websocket` | Done | Live tungstenite bridge extracted; core shim retained |
| `spanda-deploy-http` | Done | Shared HTTP/TLS helpers for agents and OTA |
| `spanda-fleet` | Done | Remote relay, agents, mesh extracted; orchestrator shim in core |
| `spanda-ota` | Done | Rollout runtime, agents, bundles extracted; AST plan shim in core |
| Comm-bus registry routing | Done | `RoutingCommBus` delegates to `ProviderRegistry` for official transports |

## Phase 4 — Compiler split (in progress)

Break circular `spanda-package` → `spanda-core` dependency:

| Crate | Status | Notes |
|-------|--------|-------|
| `spanda-hardware` | Done | Builtin profile catalog; `spanda-package` no longer depends on `spanda-core` |
| `spanda-ast` | Done | `nodes`, `foundations`, `comm_decl`, `robotics_decl`, `regex` — core shims |
| `spanda-lexer` | Done | Tokenization + `LexerError`; core `tokenize` shim |
| `spanda-typecheck` | Done | Full `TypeChecker` + `TypeCheckHost`; `CoreTypeCheckHost` wiring |
| `spanda-runtime` | In progress | Kernel: scheduler, provider types, robotics state, `RuntimeValue`, `Environment`, `RuntimeError`, `RuntimeHost`; interpreter body remains in core |

```
spanda-hardware          (profile catalog — done)
spanda-ast               (AST + foundation + comm decl types — done)
spanda-lexer             (tokenization — done)
spanda-typecheck         (program checker + host hooks — done)
spanda-runtime           (kernel primitives + RuntimeHost — in progress)
spanda-core              ← thin facade; interpreter migration via RuntimeHost next
```

## Phase 5 — Live package backends (in progress)

Replace scaffold `.sd` exports with full implementations where workspace crates already exist:

| Priority | Package | Live crate / shim | Status |
|----------|---------|-------------------|--------|
| 1 | `spanda-ros2` | `spanda-transport-ros2` | Transport registered; `.sd` scaffold |
| 2 | `spanda-mqtt` | `spanda-transport-mqtt` | Transport registered; `.sd` scaffold |
| **Capability grant + stub** | `spanda-gps`, `spanda-nav`, `spanda-slam` | `PositioningProvider` / `NavigationProvider` / `SlamProvider` stubs in bootstrap |
| 5 | `spanda-fleet` / `spanda-ota` | `spanda-fleet` / `spanda-ota` | CLI + workspace crates; `.sd` scaffold |

See [official-packages.md](./official-packages.md) for the live vs scaffold matrix.

## Phase 6 — TypeScript parity (in progress)

| Task | Status | Notes |
|------|--------|-------|
| `bootstrapProvidersForPackages()` | Done | Mirrors Rust `bootstrap.rs` |
| Registry-backed `RoutingCommBus` | Done | `attachProviderRegistry`, `syncCommBusForOfficialPackages` |
| Interpreter `officialPackages` / `providerRegistry` | Done | Parity with Rust `InterpreterOptions` |
| Full classification table | Done | Aligned with `spanda-runtime/src/classification.rs` |
| Comm-bus routing tests | Done | `tests/providers-comm.test.ts` |

## Known gaps

| Gap | Impact | Mitigation today |
|-----|--------|------------------|
| Interpreter body in core | Larger binary | `RuntimeHost` trait started; full move deferred |
| Package `.sd` scaffolds | No Spanda-language vendor I/O | Workspace crates + core shims |
| No dynamic `.so` loading | Packages are compile-time | Registry registration API ready |
| Clippy `-D warnings` failures | CI noise | Pre-existing; fix separately |
| `spanda-package` ↔ `spanda-core` cycle | Harder testing | **Broken:** package uses `spanda-hardware` only |

## Success criteria

- [x] `cargo test --workspace` green
- [x] `npm test` green (TS provider comm-bus parity added)
- [ ] All 164 examples run without regression
- [ ] Zero protocol-specific code in core except traits + wire types
- [ ] Every official package has live backend or documented stub status

See also: [lean-core.md](./lean-core.md), [migration.md](./migration.md#lean-core-package-first-refactor)
