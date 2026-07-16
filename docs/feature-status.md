# Feature Status

Honest snapshot of **Spanda Platform** capabilities as of **v0.7.0**. The Spanda Language (`.sd`) is
one component; this matrix covers verification, simulation, fleet, packages, and tooling as well.
Use this document to understand what is production-ready, experimental, planned, or deprecated.

Platform overview: [platform-overview.md](./platform-overview.md) · Release hardening:
[scope-control.md](./scope-control.md) · Blockers: [release-blockers.md](./release-blockers.md)

## Stability labels (strict)

| Label | Meaning |
|-------|---------|
| **Stable** | Tested default path; not mock/demo/docs-only |
| **Beta** | Usable with known limitations; suitable for evaluation |
| **Experimental** | Works with caveats; optional live backends |
| **Preview** | Early API; may change without notice |
| **Stubbed** | Syntax or API exists without full external integration |
| **Mock-backed** | Default path uses mocks/simulators (not production-ready alone) |
| **Planned** | Not implemented |
| **Deprecated** | Replaced; do not use in new programs |

**Rules:** mock-backed, demo-only, docs-only, untested, and simulated-only features are **not**
Stable. Simulated-only paths must say so explicitly.

**Stubbed** = syntax or API exists without full external integration.  
**Broken** = known to fail or incomplete in current builds.

---

## v0.7.0 — Evaluation / beta (current)

| Area | Status |
|------|--------|
| **Differentiation NOW** | `spanda demo differentiation`, all five NEXT analytics pillars **Stable** |
| **Distributed decisions** | Decision trees, offline policy, signed trees, v3 signed traces, persistent escalation, conflict resolution — **Stable** |
| **Enterprise operations** | Control Center E1–E4, device pool, gRPC parity — Stable |
| **Cognitive & Resilience Architecture** | Eleven functional domains — **Stable** tier |
| | Reflex, immunity, homeostasis, attention, fusion, memory, adaptive recovery, maintenance |
| | CC Cognitive & Resilience tab **Stable**; live fusion **Stable-with-env-gate** (`SPANDA_LIVE_FUSION_SENSORS`) |
| | [architecture](./cognitive-resilience-architecture.md) · [maturity](./cognitive-resilience-maturity.md) · CI: `scripts/cognitive_resilience_smoke.sh` · gate: `scripts/cognitive_resilience_stable_promotion_gate.sh` |
| **Solution blueprints** | ADAS, Smart Spaces, Spatial Computing — Stable (organizational soak gates separate); Control Center **composite** domain tabs (SAR, Healthcare, Warehouse, Agriculture, Maritime) over shared fleet APIs — not dedicated domain backends |
| **Autonomous Entity Mesh** | `spanda mesh *`, `/v1/mesh/*`, gRPC mesh RPCs (proto **1.0.15+**), live transport discovery, SDK mesh REST + gRPC (**0.5.9+**), Control Center **Entity Mesh** tab, `entity_mesh_smoke.sh` — **Stable** (implementation); organizational field pilot **in progress** — [entity-mesh-stable-promotion.md](./entity-mesh-stable-promotion.md) |

## v0.4.0 — Deploy & tooling

| Area | Status |
|------|--------|
| **Native deploy** | `spanda deploy --target native`, `compile-native`, LLVM IR — Experimental (clang required) |
| **ROS 2 interop** | `spanda ros2 check`, rclpy bridge with `SPANDA_ROS2_LIVE=1` — Experimental |
| **Distributed fleet** | `fleet orchestrate --remote`, agent registry — Experimental |
| **CLI install** | `cargo install --path crates/spanda-cli` → binary `spanda` — Stable |
| **Bundled demos** | `spanda demo` without full clone — Stable |
| **Plugin system** | `spanda plugin search|install|enable|disable|list|inspect|trust`; `spanda.plugin.toml`; registry trust tiers; WASM sandbox (`wasm-loader`); namespaced CLI commands; Control Center `/v1/plugins` (+ search/install/enable/disable); sandboxed iframe panel host — **Stable** — [plugin-stable-promotion.md](./plugin-stable-promotion.md) |
| **Cascading configuration** | `spanda config`, `spanda drift`, `spanda device discover|inspect`, `spanda network scan`, `spanda device-tree`, `spanda map verify`; `DeviceRegistry` + network identity validation; config and agent drift (`--baseline`, `--agent`); `--config` on run/verify/readiness/replay/assurance — Experimental |

## v0.2.0 — Officially Supported

### Supported (stable for public evaluation)

| Area | Capabilities |
|------|----------------|
| **Language core** | Lexer, parser, AST, type checker, physical units, `module`/`import`, structs/enums/traits, `match`, `Result`/`Option`, `test` blocks |
| **AI agents** | `ai_model`, `agent`, `goal`, `memory`, **mock-backed** LLM/Vision providers by default, `ActionProposal` → `safety.validate()` → `SafeAction` |
| **Robotics primitives** | `robot`, `sensor`, `actuator`, `behavior`, `task every Nms`, state machines, events |
| **Hardware profiles** | `hardware`, `deploy`, `requires_hardware`, `requires_network`, SoC/HAL validation |
| **Compatibility verification** | `spanda verify` (alias `spanda compatibility`) — hardware fit checking, not formal verification; see [verification-vocabulary.md](./verification-vocabulary.md) |
| **Simulation** | `spanda run` / `spanda sim`, physics-lite 2D backend, lidar/arm/drone models |
| **Communication** | `message`, `topic`, `service`, `action`, `publish`/`call`/`send_goal`, in-memory transport |
| **Safety validation** | Safety zones, `max_speed`, optional `max_angular`, `stop_if`, emergency stop, compile-time `SafeAction` gate (including `drive`/`follow` AI bypass rejection) |
| **Trigger-driven execution** | Unified `on` / `every` / `when` / `while`; event, message, timer, condition, state, safety, hardware, AI, verification, twin |
| **Cooperative concurrency** | `spawn`, `join`, `parallel`, channels, `select`, per-task `budget { }`; TypeScript mirror parity |
| **Fleet simulation** | `spanda fleet run` — in-process multi-robot with deploy/peer wiring |
| **Swarm coordinator (experimental)** | `swarm { fleet; policy; }` + `spanda swarm coordinate` — round-robin cursors in `.spanda/swarm-state.json`; `--mesh-url` relays peer/leader-follow steps via fleet mesh |
| **Robotics platform** | `mission`, `fleet`, `safety_zone`, `certify`; navigation/fusion runtime; Nav2 adapter hook |
| **OTA deploy CLI** | `spanda deploy plan|rollout|rollback|status` — local rollout state (`.spanda/deploy-state.json`) |
| **Remote OTA agents** | `spanda deploy agent start|register|list` + `deploy rollout --remote` — HTTP agent on devices; `--require-certify` on agent and rollout |
| **Fleet orchestration** | `spanda fleet orchestrate` — round-robin mission coordination report; `--remote` relays peer steps via HTTP fleet agents |
| **Fleet peer agents** | `spanda fleet agent start|register|list` — on-device peer relay server (`.spanda/fleet-agents.json`) |
| **Fleet mesh coordinator** | `spanda fleet mesh start` + `fleet orchestrate --mesh-url` — centralized multi-host peer relay |
| **Adapter package verify** | `spanda verify-adapter` — validate `[adapter]` provides/requires against registry metadata |
| **Tooling** | Native CLI (`check`, `verify`, `run`, `sim`, `fleet`, `deploy`, `fmt`, `lint`, `doc`), package manager (`init`, `build`, `test`, `install`), **prebuilt installable packages** (Linux/macOS/Windows via GitHub Releases) |
| **Showcase demos** | `spanda demo {rover,safety,verify,fleet,health,readiness,assurance,self-healing,continuity,adas}`; `examples/showcase/*`; `examples/solutions/adas/` (**Stable** — [stable-hardening-adas.md](./stable-hardening-adas.md)) |
| **Platform policy examples** | Minimal + options: `examples/features/{decision_tree,recovery_policy,continuity_policy,*_options}.sd`; stitched workflows: `examples/workflows/`; guide: [platform-feature-examples.md](./platform-feature-examples.md) (**Stable** docs) |
| **Security / audit** | Capabilities, secrets, signed messages, audit records |
| **Secure communication** | `secure_comm`, encrypted buses, trusted-source publish/receive enforcement, AES-GCM wire frames, TLS session + rustls PEM validation, `spanda security check|audit`, TS runtime parity |
| **Digital twins** | `twin`, mirror fields, replay buffer, `twin sync` telemetry |
| **Real-time contracts** | `deadline`, `jitter <=`, `priority`, `critical isolated` on tasks; latency `pipeline` budgets |
| **Reliability runtime** | Watchdogs, operating `mode` blocks, `recover from`, retry/fallback; topic QoS deadline detection |
| **Runtime fault detection** | `heartbeat`, `memory_watch`, `resource_watch`, `restart_policy`, `on runtime crash`; CLI `spanda fault scan|report`, `spanda runtime health|diagnose`, `spanda replay --show-faults`; mission trace fault frames |
| **Mission trace replay** | `spanda sim --record`, `spanda replay`, `--deterministic`, `--playback`, `--wall-clock` |
| **Persistent telemetry** | `--persist-telemetry`, `SPANDA_TELEMETRY_STORE=1`, `spanda telemetry` — JSONL or SQLite; OTLP `push`/`serve`, `fleet-push` mesh aggregation, sessions + replay |
| **First-class regex** | Literals, `Regex` type, string methods, trigger/subscribe filters, `validate` rules |
| **Lean-core workspace** | 50+ focused Rust crates; `spanda-core` facade; CLI/bindings use workspace deps directly ([crates/README.md](../crates/README.md)) |
| **Verification & DX** | `spanda-capability` — traceability, minimum-hardware, health analysis; `spanda-readiness` — operational readiness, mission verification, safety reports; `spanda check --verification-json`; LSP verification diagnostics and quick-fixes |
| **Health & kill switch** | `health_check`, `health_policy`, fleet `require` runtime; `kill_switch`, `remote_signed`, `on kill_switch` handlers |
| **Self-healing & recovery** | `recovery_policy`, recovery planner, validation gates, audit/traceability; runtime dispatch (modes, speed caps, fleet mesh relay, reassign → continuity mesh); CLI `heal`, `recover`, `recovery-report`, `recovery knowledge`, `sim --inject-failure`; fleet agent interpreter + assurance recovery; mission operator approval gating |
| **Mission continuity** | Checkpoint resume, state transfer, succession ranking, takeover/delegation; CLI `continuity`, `takeover`, `delegate`, `succession`; `continuity_policy`; diagnostics in `spanda check --readiness-json`; `spanda demo continuity`; official `spanda-mission-continuity` package |
| **Distributed decisions** | Brain/spinal-cord/reflex layers; `decision_tree`, `offline_policy`, entity `local_decision_authority` |
| | Runtime policy gates; signed offline policy + signed decision trees + cache; live v3 signed trace emission |
| | Persistent escalation store; runtime conflict resolution; CLI `spanda decision *`; attack simulations |
| | Rule enforcement tests — **Stable** ([stable-hardening-distributed-decisions.md](./stable-hardening-distributed-decisions.md)) |
| **Differentiation NOW** | `spanda contract verify`, `spanda explain`, `spanda audit decisions`, `spanda safety-coverage`, `spanda recovery-coverage`; `spanda demo differentiation`; decision trail showcase (`differentiation/decision_trail/`) with `explain decision` on v3 trace |
| **What-If Analysis** | `spanda what-if`, `/v1/analytics/what-if`, gRPC `GetAnalyticsWhatIf` — **Stable** — [stable-hardening-what-if.md](./stable-hardening-what-if.md) |
| **Mission Risk Analysis** | `spanda risk`, `/v1/analytics/mission-risk` — **Stable** — [stable-hardening-mission-risk.md](./stable-hardening-mission-risk.md) |
| **Readiness Forecasting** | `spanda readiness forecast`, `/v1/analytics/readiness-forecast` — **Stable** — [stable-hardening-readiness-forecast.md](./stable-hardening-readiness-forecast.md) |
| **Trust Framework** | `spanda trust`, `/v1/trust/program`, gRPC `GetTrustProgram` — **Stable** — [stable-hardening-trust-framework.md](./stable-hardening-trust-framework.md) |
| **Trust Graph** | `spanda trust-graph`, `/v1/analytics/trust-graph` — **Stable** — [stable-hardening-trust-graph.md](./stable-hardening-trust-graph.md) |
| **Scorecards** | `spanda score`, `/v1/executive/scorecard` — **Stable** — [stable-hardening-scorecards.md](./stable-hardening-scorecards.md) |
| **Digital Mission Twin** | `spanda twin mission` — **Stable** — [stable-hardening-digital-mission-twin.md](./stable-hardening-digital-mission-twin.md) |
| **Certification Packs** | `spanda certify pack --bundle` — **Stable** — [stable-hardening-certification-packs.md](./stable-hardening-certification-packs.md) |
| **Mission Time Travel** | `spanda replay --at` / `--inspect` — **Stable** — [stable-hardening-mission-time-travel.md](./stable-hardening-mission-time-travel.md) |
| **Human/Robot Teaming** | `spanda team verify` — **Stable** — [stable-hardening-human-robot-teaming.md](./stable-hardening-human-robot-teaming.md) |
| **Autonomous Governance** | `spanda governance` (program policy blocks) — **Stable** — [stable-hardening-autonomous-governance.md](./stable-hardening-autonomous-governance.md) |
| **Operational Governance** | Autonomy levels, deployment profiles, certification lifecycle, risk, accountability, standards profiles |
| | Live decision enforcement; `spanda compliance check`, `governance validate|report`, `certification list|inspect|report` |
| | `deployment profile|verify`, `risk report`; REST/gRPC `/v1/governance/*`; Control Center Governance tab — **Stable** |
| | [governance.md](./governance.md) · [stable-hardening-operational-governance.md](./stable-hardening-operational-governance.md) |
| **Twin Cloud SaaS** | `spanda twin cloud`, `/v1/twins/*` (+ `/usage`), gRPC twins (proto **1.0.17**), tenant isolation, usage meters, file-backed store + history — **Stable** — [twin-cloud.md](./twin-cloud.md) · [stable-hardening-twin-cloud-saas.md](./stable-hardening-twin-cloud-saas.md) · [hosted-twin-cloud-product.md](./hosted-twin-cloud-product.md) |
| **LATER Control Center analytics** | `/v1/analytics/{mission-twin,certification-pack,time-travel,human-teaming,governance}` + gRPC `GetAnalytics*` (proto **1.0.7**); Analytics tab |
| **Mission assurance** | `knowledge_model`, `state_estimator`, `anomaly_detector`, `on anomaly`, `prognostics`, `mitigation`, `resilience_policy`, `assurance_case`; CLI `assure`, `anomaly scan`, `diagnose`, `state estimate`, `prognostics`, `mission verify`, `resilience check`, `mitigation plan`; `spanda demo assurance` |
| **Weighted sensor fusion** | `observe { }`, `state_estimator`, `fusion.read()` with type-weighted confidence; `spanda-fusion` package |
| **Learned anomaly runtime** | `learned backend assurance.anomaly`; EMA volatility; optional ONNX (`SPANDA_ANOMALY_ONNX_MODEL_PATH`) |
| **Typed handler I/O** | Return types on behavior, task, trigger, event, and agent plan handlers (Rust + TS mirror) |

### Experimental (usable with caveats)

| Area | Capabilities | Caveats |
|------|--------------|---------|
| **Digital twins (live sync)** | Twin mirror + replay; Twin Cloud SaaS (`spanda twin cloud`, `/v1/twins/*` + usage meters, tenant isolation, file-backed store + history, RBAC mutations) — **Stable**; legacy `SPANDA_CLOUD_UPLOAD_URL` + `import-replay` bridge | Hosted managed product (SLA / multi-region) — [hosted-twin-cloud-product.md](./hosted-twin-cloud-product.md) |
| **Replay** | `replay true`, frame buffer, mission traces | In-process only; v2 traces embed state snapshots for `--playback` |
| **Advanced verification** | Fault injection, compatibility matrix | Matrix may report stub targets |
| **Multi-agent systems** | Agent-to-agent comm, fleet peer messaging | In-process mesh + HTTP fleet agent relay (`fleet orchestrate --remote` / `--mesh-url`) |
| **OTA rollout** | Deploy plan/rollout/rollback/status | Local state file + HTTP deploy agents; `--require-certify` blocks uncertified rollouts |
| **Certification metadata** | `certify ISO13849 { level PLd; }` | **Declared metadata** for verify/CI — not a certification result; `--strict-certify` / `--enforce-certify`; `spanda certify prove`; deploy plan proof summary |
| **Nav2 / SLAM packages** | Registry adapter stubs + example packages | External Nav2/Gazebo/OpenCV not bundled; optional `SPANDA_NAV2_CMD` / `SPANDA_SLAM_CMD` bridges |
| **ROS2 adapter** | Native `rclrs` cdylib, rclpy daemon, CLI bridge | Requires ROS Humble; not default transport |
| **LLVM / native codegen** | `spanda run --runtime auto\|native\|interpreter`; `llvm-ir`, `compile-native`; `scripts/llvm_golden_path.sh` | **Primary** execution path when SIR eligible; interpreter LTS fallback |
| **GPS / IMU / camera pipelines** | `spanda-gps`, `spanda-imu`, `spanda-camera`; `SPANDA_LIVE_GPS/IMU/CAMERA`; `scripts/sensor_pipeline_golden_path.sh` | Hub stubs + env-gated CMD bridges; fusion via `SPANDA_LIVE_FUSION_SENSORS` |
| **FFI** | `extern python`/`extern cpp` subprocess bridges; optional `cpp-native` in-process | PyO3 path is Tier 2 adoption unlock |
| **World models** | `world_model { }` block parser; `fusion.read()` → belief hook; `world_model.update` / `belief` / `export`; Rust + TS typecheck parity | Minimal belief buffer; see [world_model_patrol.sd](../examples/showcase/world_model_patrol.sd) |
| **Ledger / provenance** | `spanda-ledger` provider → `MockLedgerBackend` | Mock chain only; no production blockchain adapters |
| **MQTT / DDS live** | `SPANDA_LIVE_MQTT=1`, `--features live-mqtt`; CI Nightly `mqtt-golden-path` | DDS is UDP JSON shim, not full DDS middleware |
| **Self-hosting bootstrap** | `examples/self_host/lexer_keywords.sd`; Rust parity tests | Rust compiler remains authoritative |
| **LSP** | Diagnostics, completion, hover, rename, verification quick-fixes | Requires built native CLI; VS Code extension with bundled LSP; continuity/recovery policy quick-fixes; CI builds VSIX |
| **DAP debugger** | Breakpoints, step over/in/out, `every` trigger entry | VS Code + `spanda-dap`; tested in `phase34_gaps.rs` / `phase35_gaps.rs` |
| **WASM / web playground** | Browser check/run/verify | Limited surface vs native CLI |
| **Live AI providers** | OpenAI, Anthropic, ONNX via Python bridge | Requires API keys or `SPANDA_ONNX_MODEL_PATH`; mock fallback by default |
| **Live IoT bridges** | Modbus TCP, OPC-UA, zigbee, lora, matter, canbus | Env-gated (`SPANDA_LIVE_*=1`); in-memory hub fallback |
| **Package publish** | `spanda publish`, registry search, mirror to `registry/packages/` | Remote upload via `SPANDA_REGISTRY_URL`; hosted index lists **92** packages after `build-registry.sh` |
| **Official package provenance** | Registry-only provider bootstrap; path/git name-squatting blocked | `OfficialProvenance` API; `official_provenance` install warning; production `deploy gate` hard-fail |
| **Registry signature policy** | `SPANDA_REGISTRY_REQUIRE_SIGNATURE=1` + lockfile signature audit | Required for production `deploy gate`; optional at install otherwise |

### Planned (v0.5 beta and beyond)

| Area | Description |
|------|-------------|
| **Human Interaction & Spatial Computing (H1–H6)** | **Stable** — H1–H6 platform APIs, Control Center Humans tab, `/v1/humans` + `/v1/hri/*`, promotion gate `hri_stable_promotion_gate.sh`; registry vendor packages remain **Experimental** — [stable-hardening-human-interaction.md](./stable-hardening-human-interaction.md) |
| **Smart Spaces & Ambient Intelligence** | **Stable** — blueprint #15 feature-complete: simulation matrix, Control Center panels (REST + gRPC 1.0.5), live env I/O bridges, weighted readiness, CI `smart_spaces_promotion_gate.sh`. Detail telemetry may be **simulated** (`source` badges); live BACnet via `SPANDA_LIVE_BACNET=1` — [stable-hardening-smart-spaces.md](./stable-hardening-smart-spaces.md) · [solutions/smart-spaces.md](./solutions/smart-spaces.md) |
| **Platform maturity (Phase A)** | **Stable** — `spanda graph`, `spanda deploy gate`, `spanda explain`, `spanda trust` — [platform-maturity-roadmap.md](./platform-maturity-roadmap.md) |
| **Platform maturity (Phase B)** | **Stable** — threat model, mission diff, scorecard (`spanda score`), policy engine (`spanda verify --policy`, `readiness --policy`, `deploy gate --operational-policy`, runtime `--enforce-policy`) |
| **Platform maturity (Phase C)** | **Stable** — chaos, readiness trends, resource estimation, compliance profiles, ADR (`spanda adr`) |
| **Platform maturity (Phase D)** | **Stable** — verify-time tamper/integrity, composite program trust, secure-boot attestation, compliance accreditation export, decision explain, runtime policy, AI generate/suggest, spoof-check, security assurance, tamper_policy runtime |
| **Enterprise operations (E1–E4)** | Control Center (`spanda control-center serve`, embedded UI, `ControlCenterPanel` in `@davalgi-spanda/web`, Tauri desktop **0.6.3**) |
| | REST v1 (`spanda-api`); Device Pool lifecycle; host-backed discovery + pool ingest; RBAC v1 (`SPANDA_API_KEY`) |
| | `ManagedSecretVault`, alerting core (`spanda-ops`); provisioning/snapshots/discovery (E2) |
| | Operational drift/OTA/trust/SRE/operator APIs + Python SDK + WebSocket telemetry + OTLP trace export (E3) |
| | Compliance export/digital thread/executive scorecard/PDF reports (E4); **Stable** tier |
| | [enterprise-operations-roadmap.md](./enterprise-operations-roadmap.md) · [control-center.md](./control-center.md) |
| **Enterprise operations (organizational gates)** | 30-day field soak completion (`enterprise_ops_field_soak_init.sh`); third-party security audit sign-off — tracked separately from tier promotion — [enterprise-ops-stable-promotion.md](./enterprise-ops-stable-promotion.md) |
| **LLVM backend (production primary)** | ✓ Auto-native dispatch on `spanda run`/`sim`; interpreter remains LTS fallback |
| **Self-hosting compiler (full)** | Complete Spanda-authored compiler pipeline |
| **ROS2 production adapter** | First-class, zero-config ROS2 deployment |
| **VS Code Marketplace publish** | **Partial** — `VSCE_PAT` + publisher `spanda-lang` configured; VSIX on GitHub releases; public listing blocked by Marketplace scanner pending Microsoft review — [vscode-marketplace-publish.md](./vscode-marketplace-publish.md) |
| **Production blockchain** | `spanda-ledger-ethereum` and related chain adapters |
| **Full world models** | Knowledge graphs, beliefs, policies beyond minimal runtime |
| **Twin cloud SaaS (hosted product)** | Managed multi-tenant service (billing, SLA, multi-region) — OSS `/v1/twins/*` is **Stable**; pilot at [deploy/twin-cloud-hosted/](../deploy/twin-cloud-hosted/) — [hosted-twin-cloud-product.md](./hosted-twin-cloud-product.md) |

See [tier-3-experimental.md](./tier-3-experimental.md) and
[tier-3-golden-paths.md](./tier-3-golden-paths.md).

### Deprecated

| Feature | Replacement | Notes |
|---------|-------------|-------|
| Legacy inference-only AI paths | `ai_model` + `agent` | Import-based ONNX/TFLite remain for classical workflows |
| TypeScript-only verification | Native `spanda verify` | TS mirror validates deploy syntax; Rust CLI is authoritative |
| `spanda_core::transport_live` | `spanda_transport_routing::transport_live` | Removed Phase 17 |
| `spanda_core::transport_mqtt` / `transport_dds` / `transport_websocket` / `transport_live` | `spanda-transport-*` or `spanda_transport_routing::live_bridges` | Removed Phase 17 |
| `spanda_core::transport` / `transport_wire` / `transport_security` / `transport_rclrs` | `spanda-transport-routing`, `spanda-transport`, `spanda-transport-ros2` | Removed Phase 19 |

---

## Feature matrix

### Language core

| Feature | Status | Notes |
|---------|--------|-------|
| Lexer / parser / AST | **Stable** | Rust authoritative; TS mirror |
| Type checker + units | **Stable** | Physical unit algebra enforced |
| modules / imports | **Stable** | `spanda install` vendor support |
| structs / enums / traits | **Stable** | Generic struct literals; enum payloads |
| generics | **Stable** | Module fn + struct type params; `T: Bound` / `where` / trait·enum generics still unsupported — [spanda-type-system.md](./spanda-type-system.md#user-defined-generics-stable-subset) |
| trait objects (`dyn Trait`) | **Stable** | `export trait` + import; same-program `impl` |
| match / Result / Option | **Stable** | |
| async / await | **Stable** | Cooperative single-threaded |
| spawn / select / channels | **Stable** | Cooperative concurrency with TS mirror |
| triggers (`on` / `every` / `when`) | **Stable** | Unified `TriggerRegistry`; see `docs/triggers.md` |
| test blocks | **Stable** | Rust runtime + TS `runTests()` |
| `extern fn` / FFI | **Experimental** | Subprocess bridges; optional in-process |
| Spanda IR (SIR) | **Stable** | JSON export via `spanda ir` |
| Codegen / LLVM | **Experimental** | HAL profiles; conditional codegen; `--target native\|wasm\|esp32` |

### Autonomous systems

| Feature | Status | Notes |
|---------|--------|-------|
| robot / sensor / actuator | **Stable** | |
| agent / goal / task / skill | **Mock-backed** (Stable API) | Default mock AI backend; live providers optional |
| ActionProposal → SafeAction | **Stable** | Compile + runtime; `drive`/`follow` reject AI motion components; `max_speed` / optional `max_angular` clamped on interpreter `drive`/`execute`/`follow` cruise (follow re-clamped each tick through zones) |
| safety zones / emergency stop | **Stable** | |
| deterministic scheduler | **Stable** | `task every Nms` |
| deadline / jitter / priority | **Stable** | Compile-time validation + runtime telemetry; **not** OS hard-RT on interpreter — [realtime.md](./realtime.md) |
| pipelines / watchdogs / modes | **Stable** | See `docs/reliability.md`, `docs/watchdogs.md`, `docs/degraded-modes.md` |
| mission trace replay | **Stable** | `--record`, `spanda replay --deterministic` / `--playback` |
| persistent telemetry store | **Stable** | `--persist-telemetry`, `spanda telemetry`; JSONL (default) or SQLite; OTLP export/push/serve, `fleet-push`, sessions — [telemetry-store.md](./telemetry-store.md) |
| regex literals / filters | **Stable** | See `docs/regex.md` |
| state machine / events | **Stable** | |
| twin / replay | **Experimental** | Replay buffer; live sync simulated |
| observe / fusion | **Stable** | Weighted fusion by sensor type; `state_estimator` runtime bindings |
| mission assurance (static + CLI) | **Stable** | `spanda-assurance` crate; 9 official packages (includes `spanda-mission-continuity`) |
| self-healing & recovery (static + CLI) | **Stable** | Recovery planner, validation gates, audit, knowledge store |
| **Recovery Orchestrator** | **Stable** | `spanda-recovery` crate; escalation levels 0–8, graph, playbooks, policies, simulation |
| | | Predictive indicators, persisted history; CLI `spanda recovery *`; REST `/v1/recovery/*` (14 routes) |
| | | gRPC (proto **1.0.15** via `GET /v1/version`); plugin `[recovery.extensions]`; SDK; Control Center Recovery tab |
| | | `scripts/recovery_orchestrator_smoke.sh`, stable promotion gate — [recovery-orchestrator.md](./recovery-orchestrator.md) |
| mission continuity (static + CLI + diagnostics) | **Stable** | `spanda-assurance` continuity module; CLI `continuity`, `takeover`, `delegate`, `succession`; `continuity:*` diagnostics in check JSON and LSP |
| mission continuity runtime dispatch | **Stable** | Interpreter mode-specific takeover, durable checkpoints, auto-trigger on health faults, fleet agent `/v1/continuity/execute`, mesh relay, swarm `--failed` handoff |
| self-healing runtime dispatch | **Stable** | Auto-trigger on health faults, approval polling/retry, fleet mesh relay with failure events, mission approval gating; `scripts/fleet_field_validation.sh` |
| fleet agent interpreter recovery | **Stable** | `POST /v1/recovery/execute` with `recovery_engine: interpreter`; `scripts/fleet_agent_recovery_smoke.sh` |
| recovery diagnostics (CLI + LSP) | **Stable** | `spanda check --readiness-json` merges `recovery:*` categories; TS mirror in `scripts/lsp-readiness.mts` |
| continuity diagnostics (CLI + LSP) | **Stable** | `spanda check --readiness-json` merges `continuity:*` categories including `continuity:mission`; TS mirror in `src/continuity-diagnostics.ts` |
| learned anomaly backends | **Experimental** | Runtime `scan_learned`; ONNX optional |
| verify { } / assert { } runtime assertions | **Stable** | Not formal verification; `assert { }` preferred alias |
| requires / ensures / invariant | **Stable** | Runtime contracts; `ensures` checked after body (not static proof) |
| hardware / deploy | **Stable** | Rust verify CLI (`spanda compatibility` alias) |

### Tooling

| Feature | Status | Notes |
|---------|--------|-------|
| Native CLI (full) | **Stable** | check, verify, run, sim, replay, fleet, fmt, lint, doc, man, reference, package |
| Prebuilt packages | **Stable** | Linux/macOS/Windows archives, shell/PowerShell installers, Windows MSI, Homebrew formula; see [installation.md](./installation.md) |
| TypeScript CLI | **Stable** | Delegates to Rust when built |
| Formatter / linter / docgen | **Stable** | `///` doc comments in `.sd`; `spanda doc` (markdown/HTML/JSON); `spanda man`; [man pages](./man/README.md) |
| LSP | **Experimental** | VS Code extension scaffold; CI builds VSIX on push |
| DAP debugger | **Experimental** | VS Code + `spanda-dap`; `every` trigger entry (Phase 35) |
| N-API | **Experimental** | check, run, verify, sir, fmt |
| WASM | **Experimental** | check, run, verify, sir, fmt |

### Ecosystem / FFI

| Feature | Status | Notes |
|---------|--------|-------|
| python.* / cpp.* imports | **Experimental** | Subprocess bridges |
| ROS2 adapter | **Experimental** | Native rclrs cdylib; CI Nightly `ros2-rclrs-native` on Ubuntu 22.04 + Humble |
| Transport adapters | **Experimental** | In-memory + optional rclrs/rclpy |
| Package manager | **Stable** | Hosted index + local mirror; `spanda publish` copies to `registry/packages/` |
| LLVM / native codegen | **Experimental** | `compile-native` early stage |

### Enterprise operations (20 pillars)

| Pillar | Status | Key surfaces |
|--------|--------|--------------|
| **Control Center** | **Stable** | `spanda control-center serve`, embedded HTML + `@davalgi-spanda/web` panel; RBAC tabs, OIDC SSO, API keys, admin config, mission/sim/replay, drift/trust/compliance/SRE; domain tabs (ADAS, Humans, Smart Spaces, SAR, Healthcare, Warehouse, Agriculture, Maritime); Tauri **0.6.3** (`desktop-v0.6.3`) — [authentication.md](./authentication.md) · [control-center-versioning.md](./control-center-versioning.md) |
| **Device Pool** | **Stable** | Lifecycle states, assign/trust/quarantine/retire, failover chains; multi-tenant API key scoping |
| **Device Discovery** | **Stable** | Subnet, mDNS, BLE, USB, CAN, MQTT, ROS2 host probes; production TLS policy (`SPANDA_DISCOVERY_REQUIRE_TLS`, `spanda-discovery-tls`) |
| **Provisioning** | **Stable** | `POST /v1/provision`, discover → ready workflow |
| **Configuration Management** | **Stable** | Snapshots, diff, resolve; approval queue + publish-on-approve (`/v1/config/approvals`) |
| **RBAC** | **Stable** | 7 roles; hashed file-backed API keys; session JWTs after OIDC; `/v1/auth/*`, `/v1/rbac/matrix` — [authentication.md](./authentication.md) |
| **Secret Management** | **Stable** | `ManagedSecretVault`, rotation metadata |
| **Telemetry** | **Stable** | Health/readiness/mission signals; trend analysis; forecasting **Planned** |
| **Alerting** | **Stable** | Webhook, email, PagerDuty (bi-directional sync), Teams; per-severity dedup; registry alert packages |
| **Configuration Drift** | **Stable** | Full operational drift API; scheduled scans (`SPANDA_DRIFT_SCAN_INTERVAL_SECS`); seven dimensions via `GET /v1/drift` |
| **OTA & Rollback** | **Stable** | Canary, blue/green, phased dry-run; production `--require-certify` via `SPANDA_OTA_REQUIRE_CERTIFY` |
| **Package Trust** | **Stable** | `spanda trust`, `/v1/trust/package`, trust score |
| **SDKs** | **Stable** | Official Rust/Python/TypeScript clients **published** at **0.5.9** (`cargo add spanda-sdk`, `pip install spanda-sdk`, `npm install @davalgi-spanda/sdk`) — entity read/eval/mutation helpers (`entityReadiness`, `entityRelationships`, gRPC `entity_health`/`entity_trust`); Twin Cloud, Recovery Orchestrator, and Entity Mesh clients in **0.5.5+** (mesh REST **0.5.7+**, gRPC **0.5.8+** TS/Rust, **0.5.9+** Python `spanda-sdk[grpc]`); `@davalgi-spanda/web` Control Center panel; program-level REST + gRPC; legacy `packages/sdk-python` |
| **Operator Workflows** | **Stable** | Mission approve, takeover, quarantine, recovery approval |
| **SRE** | **Stable** | `/v1/sre/summary` with `slo`, `burn_rate`, MTTR/MTBF hints; incident workflow; PagerDuty webhook sync; fast-burn background monitor |
| **Reporting** | **Stable** | Markdown, JSON, PDF exports; scheduled webhook delivery (`GET/POST /v1/reports/schedules`) |
| **Compliance** | **Stable** | Evidence packs, `GET /v1/compliance/export`, signed profile catalog (`GET /v1/compliance/profiles`) |
| **APIs** | **Stable** | REST v1 + OpenAPI; unified entity routes (`/v1/entities/*`); program-level SDK routes (`/v1/programs/*`) |
| | | JSON-RPC gateway (Control Center methods); native gRPC (tonic) — **174** RPCs, proto **1.0.15** via `GET /v1/version` |
| | | Rate limits (`SPANDA_API_RATE_LIMIT_PER_MINUTE`); versioning (`GET /v1/version`, `X-Spanda-Api-Version`) |
| **Unified Entity Model** | **Stable** | `EntityRegistry` projects fleet, devices, humans, providers, packages into entity graph |
| | | Verification, readiness, health, and trust via `verify_entity` / `evaluate_entity_*` |
| | | Control Center Entities tab with read/write mutations; CI `entity_model_smoke.sh` (REST + TS + Python + Rust SDK) |
| | | SDKs **0.5.9** on crates.io, PyPI, npm — [entity-model.md](./entity-model.md), [entity-apis.md](./entity-apis.md), [entity-sdk.md](./entity-sdk.md) |
| **Observability** | **Stable** | OTLP trace/metrics export, correlation IDs, WebSocket telemetry; `spanda-otel-collector`; Grafana templates (`spanda-grafana-dashboards`); HA collector guide |
| **Digital Thread** | **Stable** | Full lifecycle graph (requirement → retirement); `lifecycle_edges` + optional `phase_path` on `GET /v1/digital-thread/query`; Control Center pan/zoom/search/export |

See [enterprise-operations-roadmap.md](./enterprise-operations-roadmap.md) ·
[control-center.md](./control-center.md) ·
[stable-hardening-enterprise-ops.md](./stable-hardening-enterprise-ops.md)

### Solution blueprints & platform maturity

| Area | Status | Key surfaces |
|------|--------|--------------|
| **ADAS & Autonomous Driving** | **Stable** | `spanda demo adas`, ISO 26262 profile, golden traces, Control Center ADAS tab — [stable-hardening-adas.md](./stable-hardening-adas.md) |
| **Human Interaction & Spatial Computing** | **Stable** | H1–H6 APIs, Humans tab, `/v1/humans`, `/v1/hri/*`, `spatial_computing_smoke.sh` — [stable-hardening-human-interaction.md](./stable-hardening-human-interaction.md) |
| **Smart Spaces & Ambient Intelligence** | **Stable** | Six blueprint apps, Smart Spaces tab, BACnet/KNX/HA bridges, `smart_spaces_promotion_gate.sh` — [stable-hardening-smart-spaces.md](./stable-hardening-smart-spaces.md) |
| **Platform maturity (Phases A–D)** | **Stable** | Graph, deploy gate, explain, trust, threat model, policy engine, chaos, scorecard, tamper, compliance export, ADR — [platform-maturity-roadmap.md](./platform-maturity-roadmap.md) |

**Stable promotion (2026-07-02):** Human Interaction & Spatial Computing (H1–H6), Smart Spaces
blueprint, ADAS blueprint, and Platform maturity Phases A–D promoted to **Stable** after
implementation promotion gates (soak/audit skipped in CI). Organizational gates — per-blueprint
field soak and third-party security audit sign-off — remain tracked in
[stable-hardening-human-interaction.md](./stable-hardening-human-interaction.md),
[stable-hardening-smart-spaces.md](./stable-hardening-smart-spaces.md),
[stable-hardening-adas.md](./stable-hardening-adas.md), and
[enterprise-ops-stable-promotion.md](./enterprise-ops-stable-promotion.md).

**Enterprise ops (2026-06-28):** All 20 enterprise operations pillars promoted to **Stable** after
`enterprise_ops_stable_promotion_gate.sh` (implementation checks). SDK **0.5.9** and desktop
**0.6.3** (`desktop-v0.6.3`) published. Ongoing organizational gates — 30-day field soak
([field-soak-gate.md](./field-soak-gate.md)) and third-party security audit sign-off — tracked in
[enterprise-ops-stable-promotion.md](./enterprise-ops-stable-promotion.md).

---

## Honesty audit (2026-07-04)

Release-hardening pass against the strict labels above
([#50](https://github.com/Davalgi/Spanda/issues/50)):

| Area | Label | Rationale |
|------|-------|-----------|
| AI agents / LLM providers | **Mock-backed** (Stable API) | Default path is mock; live keys optional |
| Live IoT / MQTT / ROS2 / LLVM | **Experimental** | Env-gated or optional toolchains |
| Ledger / blockchain | **Stubbed** / **Mock-backed** | `MockLedgerBackend` only |
| Solution blueprints (ADAS, Smart Spaces, HRI) | **Stable** implementation; organizational soak **open** | CI promotion gates skip field soak / third-party audit ([#51](https://github.com/Davalgi/Spanda/issues/51)) |
| Enterprise ops pillars | **Stable** implementation; organizational soak **open** | Same |

Remaining review: any row still marked **Stable** whose *default* path is simulated-only or
docs-only should be demoted in a follow-up PR.

## Known limitations (v0.6.3)

- AI providers use **mock backends** by default; set `OPENAI_API_KEY`, `ANTHROPIC_API_KEY`, or
  `SPANDA_ONNX_MODEL_PATH` for live calls (`SPANDA_LIVE_AI=0` forces mock). Label: **Mock-backed**.
- ROS2 integration requires manual ROS Humble setup and is not the default simulator transport.
- Native compilation via LLVM is **experimental**; the tree-walking interpreter is the primary
  runtime.
- `spanda publish` mirrors bundles to `registry/packages/` in-repo; remote upload requires
  `SPANDA_REGISTRY_URL`. Run `./scripts/build-registry.sh` to refresh the hosted index after adding
  scaffolds under `packages/registry/`.
- VS Code extension builds in CI Integration + path-filtered `vscode-extension-ci.yml`;
  **Marketplace listing** partial — `VSCE_PAT` configured; automated upload blocked pending Microsoft review ([vscode-marketplace-publish.md](./vscode-marketplace-publish.md)).
- Multi-robot fleet examples run in a single process by default; distributed orchestration uses HTTP
  fleet agents and an optional fleet mesh coordinator (`spanda fleet mesh start`, `--mesh-url` on
  orchestrate/swarm).
- Organizational field soak and third-party security audit remain open for enterprise ops and
  solution blueprints ([#51](https://github.com/Davalgi/Spanda/issues/51)).

---

## Broken / stubbed (honest audit)

| Item | Category | Detail |
|------|----------|--------|
| Global package registry | Hosted + mirror | Default `SPANDA_REGISTRY_URL` points at repo index; `spanda publish` mirrors to `registry/packages/` |
| Live OpenAI / Anthropic / ONNX | Optional live path | `OPENAI_API_KEY`, `ANTHROPIC_API_KEY`, or `SPANDA_ONNX_MODEL_PATH`; Python bridge; mock fallback |
| Live Modbus / OPC-UA IoT | Optional live hardware | `SPANDA_LIVE_MODBUS=1` / `SPANDA_LIVE_OPCUA=1`; `--features live-iot` for native Modbus TCP |
| IoT protocol bridges (zigbee/lora/matter/canbus/bacnet/knx) | Live + hub fallback | `SPANDA_LIVE_ZIGBEE=1` etc.; BACnet/KNX via bacpypes3/xknx or `SPANDA_*_CMD`; `./scripts/live_iot_golden_path.sh`, `./scripts/smart_spaces_live_iot_smoke.sh` |
| Kill switch remote_signed | Runtime + verify enforced | Requires `kill_switch_signature` JSON when `remote_signed` is set; verify reports **error** without signed comm |
| MQTT / DDS / WebSocket live transport | Production wire + optional live brokers | AES-256-GCM wire frames; live MQTT/WebSocket/DDS via `--features live-transport` + `SPANDA_LIVE_MQTT=1` / `SPANDA_LIVE_WEBSOCKET=1` / `SPANDA_LIVE_DDS=1`; TypeScript mirrors the same env flags |
| Secure comm live crypto | Production wire | AES-256-GCM for transport frames and `EncryptedMessage` payloads; session material from robot secrets |
| Full native binary deploy | Experimental | `spanda deploy --target native`, `compile-native` (clang + llvm feature) |
| Blockchain / ledger cloud | Stubbed | Audit records local; see `future-blockchain-support.md` |

Nothing in the **Supported** list above is known broken in CI Fast (`./scripts/ci-fast.sh`) or CI
Integration on `main`. ROS2 rclrs native and `cargo audit` run in **CI Nightly**. See
[ci-architecture.md](./ci-architecture.md).

---

## Architecture summary

```
.sd source
  → lexer → parser → AST
  → type checker (+ units, safety, capabilities)
  → [optional] hardware verifier (deploy targets)
  → interpreter + simulator
  → [optional] SIR → LLVM (experimental)
```

| Crate | Role |
|-------|------|
| `spanda-core` | Language implementation (authoritative) |
| `spanda-cli` | Native `spanda` binary |
| `spanda-package` | Package manager |
| `spanda-audit` / `spanda-security` | Audit and security |
| `spanda-llvm` / `spanda-rt` | Experimental native codegen |
| `spanda-node` / `spanda-wasm` | Bindings |
| `spanda-dap` | Debug adapter |
| `@spanda/lsp` / `@davalgi-spanda/web` | LSP and web playground |

See [architecture.md](./architecture.md) for diagrams.

---

## Related docs

- [README.md](../README.md) — project overview
- [getting-started.md](./getting-started.md) — first robot in 10 minutes
- [installation.md](./installation.md) — prebuilt packages and source install
- [triggers.md](./triggers.md) — trigger-driven execution
- [concurrency.md](./concurrency.md) — tasks, spawn, channels, fleet CLI
- [realtime.md](./realtime.md) — deadline-aware tasks and wall-clock scheduling
- [reliability.md](./reliability.md) — pipelines, watchdogs, recovery
- [replay.md](./replay.md) — mission trace record and replay
- [telemetry-store.md](./telemetry-store.md) — persistent device/sensor/heartbeat storage
- [regex.md](./regex.md) — first-class regex
- [vision.md](./vision.md) — long-term positioning
- [product-strategy.md](./product-strategy.md) — v0.5 beta priorities
- [ffi-and-ecosystem.md](./ffi-and-ecosystem.md) — Python/C++/ROS2 interop
- [compiler-backend-roadmap.md](./compiler-backend-roadmap.md) — LLVM evolution
- [health-checks.md](./health-checks.md) — health checks and fleet requirements
- [kill-switch.md](./kill-switch.md) — kill switch syntax and handlers
- [capability-traceability.md](./capability-traceability.md) — traceability matrices
- [verification-diagnostics.md](./verification-diagnostics.md) — `--verification-json` and LSP
  quick-fixes
- [typed-handler-io.md](./typed-handler-io.md) — handler return type annotations
- [testing.md](./testing.md) — `expect_compile_error` and test CLI
- [iot.md](./iot.md) — IoT packages and live bridges
- [live-ai-provider.md](./live-ai-provider.md) — OpenAI, Anthropic, ONNX
- [debugging.md](./debugging.md) — VS Code DAP workflow
- [registry.md](./registry.md) — hosted registry and publish mirror
- [packages.md](./packages.md) — package manager
