# Spanda Roadmap

Version plan organized by **platform area**. Tiers: **Stable** (CI-backed, documented), **Experimental** (usable with caveats), **Future** (planned, not shipped).

Current release line: **v0.4.0** (tagged 2026-06-22). **Next:** v0.5 beta (Q4 2026).

**Last audited:** 2026-06-25 — [roadmap-codebase-audit-2026-06.md](./roadmap-codebase-audit-2026-06.md)

Platform overview: [platform-overview.md](./platform-overview.md) · Feature truth table: [feature-status.md](./feature-status.md)

---

## Platform areas at a glance

| Area | Current focus (v0.4) | Next (v0.5+) |
|------|----------------------|--------------|
| [Language](#language) | Stable core; typed handler I/O | Generics polish; self-hosting subset (future) |
| [Runtime](#runtime) | Interpreter LTS; certify gate | Native codegen golden paths (experimental) |
| [Verification](#verification) | `spanda verify`, capability matrices | 5+ production hardware profiles (v1.0) |
| [Safety](#safety) | ActionProposal → SafeAction stable | Safety Coverage CLI; stricter certify workflows |
| [Simulation](#simulation) | `spanda sim`, twins, replay, telemetry store | OTLP/fleet aggregation polish; Gazebo/Webots scaffolds |
| [Health](#health) | health_check, readiness engine | Swarm quorum hardening |
| [Fleet](#fleet) | In-process + HTTP agents + mesh telemetry | Distributed orchestration polish |
| [Packages](#packages) | 38 official registry packages, publish mirror | Curated remote registry growth |
| [Tooling](#tooling) | CLI, 9 bundled demos, CI golden paths | VS Code Marketplace (blocked on `VSCE_PAT`) |
| [Mission assurance](#mission-assurance) | Static analysis + learned anomaly (experimental) | Package-backed ML anomaly backends |
| [Mission continuity](#mission-continuity) | Runtime takeover, checkpoints, fleet mesh (**Stable**) | Auto-trigger tuning; swarm hardening |
| [Self-healing](#self-healing--recovery) | Recovery planner + CLI (**Stable**); runtime dispatch experimental | Recovery Coverage CLI |
| [Platform maturity](#platform-maturity) | 16-area design specs + topic guides | Phase A: `spanda graph`, `explain`, gates, package trust |
| [Differentiation](#differentiation--signature-capabilities) | Topic guides + architecture specs (docs) | NOW engineering: contracts, explain, audit trail, coverage |
| [Enterprise operations](#enterprise-operations) | Design spec + partial foundations (config, telemetry, OTA) | NOW: Control Center, Device Pool, Provisioning, RBAC, Secrets |

---

## Platform maturity

**Adoption, trust, and operations** — not new unrelated features. Every item strengthens **Build · Verify · Simulate · Deploy · Operate · Recover**.

Full analysis: [platform-maturity-roadmap.md](./platform-maturity-roadmap.md)

| # | Area | Phase | Priority | Status |
|---|------|-------|----------|--------|
| 1 | AI-assisted development (`generate`, `explain`, `suggest`) | Build, Operate | P0.3 / P3.3 | **Future** |
| 2 | Dependency graph visualization | Build, Operate | P0.1 | **Experimental** |
| 3 | Threat modeling | Verify, Deploy | P1.2 | **Experimental** |
| 4 | Configuration drift detection | Deploy, Operate | P1.1 | **Experimental** |
| 5 | Policy engine | Verify, Operate | P1.5 | **Experimental** |
| 6 | Compliance profiles | Verify, Deploy | P2.4 | **Experimental** |
| 7 | Explainability reports | Operate, Recover | P0.3 / P3.2 | **Experimental** |
| 8 | Chaos engineering | Simulate, Recover | P2.1 | **Experimental** |
| 9 | Mission resource estimation | Simulate, Deploy | P2.3 | **Experimental** |
| 10 | Readiness trend analysis | Operate | P2.2 | **Experimental** |
| 11 | Package trust framework | Verify, Build | P0.4 | **Experimental** |
| 12 | Architecture decision records | Build | P2.5 | **Experimental** |
| 13 | Mission differencing | Build, Verify | P1.3 | **Experimental** |
| 14 | Deployment gates | Deploy | P0.2 | **Experimental** |
| 15 | Autonomous systems scorecard | Operate | P1.4 | **Experimental** |
| 16 | Hack / tamper detection | Verify, Operate, Recover | P3.1 | **Experimental** (verify-time) |

### Phased delivery

| Phase | Release | Theme | Key deliverables |
|-------|---------|-------|------------------|
| A | v0.5+ (Q3–Q4 2026) | Understand & trust | `spanda graph`, `explain`, `package trust`, deployment gates |
| B | v0.6 (Q1 2027) | Operate & compare | drift, threat model, mission diff, scorecard, policy (verify) |
| C | v0.7 (Q2 2027) | Resilience & planning | chaos, readiness trends, estimate, compliance profiles, ADR |
| D | v1.0 (2027) | Full trust platform | tamper/integrity, decision explain, runtime policy, AI generate/suggest (mock-first) |

Topic guides: [dependency-graphs.md](./dependency-graphs.md) · [deployment-gates.md](./deployment-gates.md) · [tamper-detection.md](./tamper-detection.md) · [security-assurance.md](./security-assurance.md)

---

## Enterprise operations

**Production-ready operations for enterprise, industrial, robotics, medical, warehouse, agricultural, research, and defense deployments** — composes Readiness, Assurance, Diagnosis, Recovery, Trust, Health, Device Registry, Configuration, Traceability, Audit, Security, and Packages without duplicating them.

Full analysis: [enterprise-operations-roadmap.md](./enterprise-operations-roadmap.md)

### Platform pillars (20 areas)

| # | Pillar | Priority | Status |
|---|--------|----------|--------|
| 1 | Control Center (web UI) | NOW | **Experimental** — `spanda control-center serve` |
| 2 | Device Pool (central inventory) | NOW | **Experimental** (extends `DeviceRegistry` + lifecycle) |
| 3 | Device Discovery (package transports) | NOW | **Experimental** (IP subnet, manual) / **Planned** (mDNS, BLE, CAN, …) |
| 4 | Provisioning (discover → ready workflow) | NOW | **Experimental** — `POST /v1/provision` |
| 5 | Configuration Management (versioned cascading TOML) | NOW | **Experimental** (resolve, diff, snapshots) / **Planned** (approval) |
| 6 | RBAC (roles + permissions) | NOW | **Experimental** — `SPANDA_API_KEY`, `/v1/rbac/matrix` |
| 7 | Secret Management (rotation, audit) | NOW | **Experimental** — `ManagedSecretVault` contract |
| 8 | Telemetry (time-series, trends) | NOW | **Experimental** — [telemetry-store.md](./telemetry-store.md) |
| 9 | Alerting (multi-channel) | NOW | **Experimental** — `spanda-ops`, webhook/email env |
| 10 | Configuration Drift (6 dimensions) | NEXT | **Experimental** (config, firmware) / **Planned** (package, provider, capability, policy, safety) |
| 11 | OTA & Rollback (canary, blue/green) | NEXT | **Experimental** (rollout, rollback) / **Planned** (canary, phased) |
| 12 | Package Trust (scoring) | NEXT | **Experimental** — `spanda trust` |
| 13 | SDKs (Python, REST, gRPC, WebSocket) | NEXT | **Planned** |
| 14 | Operator Workflows (approve, takeover, quarantine) | NEXT | **Experimental** (takeover, approval env) / **Planned** (UI workflows) |
| 15 | SRE (SLO, MTTR, incidents) | NEXT | **Planned** |
| 16 | Reporting (fleet, mission, compliance exports) | LATER | **Planned** |
| 17 | Compliance (evidence packs) | LATER | **Experimental** — `spanda compliance report` |
| 18 | APIs (REST + gRPC CLI parity) | NEXT | **Experimental** (REST v1) / **Planned** (gRPC) |
| 19 | Observability (OTel, traces, correlation) | NEXT | **Experimental** (OTLP push/serve) / **Planned** (distributed tracing) |
| 20 | Digital Thread (requirement → retirement) | LATER | **Future** |

### Priority horizons

| Horizon | Timeline | Pillars |
|---------|----------|---------|
| **NOW** | 0–6 months (v0.5–v0.6) | Control Center, Device Pool, Provisioning, Telemetry, Alerting, RBAC, Secrets |
| **NEXT** | 6–12 months (v0.6–v0.7) | SDKs, Configuration Drift (full), OTA strategies, Package Trust UI, Observability |
| **LATER** | 12–18 months (v0.8–v1.0) | Compliance Packs, Executive Dashboards, Digital Thread, Predictive Analytics |

### Phased delivery

| Phase | Release | Theme | Key deliverables |
|-------|---------|-------|------------------|
| E1 | v0.5+ (Q3–Q4 2026) | Control plane | `spanda-api` REST v1, Control Center shell (Dashboard, Fleet, Readiness), Device Pool lifecycle, RBAC v1, secret store contract, alerting core |
| E2 | v0.6 (Q1 2027) | Provision & observe | Provisioning workflow API, config snapshots, discovery API, Health/Assurance/Diagnosis modules, Slack alert formatting |
| E3 | v0.7 (Q2 2027) | Deploy & integrate | Python SDK, gRPC, full drift, OTA canary/phased, Package Trust UI, OpenTelemetry, operator workflows, SRE dashboard |
| E4 | v1.0 (2027) | Govern & trace | Compliance UI, executive dashboards, Digital Thread v1, predictive analytics, PDF reporting, Tauri desktop |

**Exit criteria (E1):** `spanda control-center serve` + `scripts/enterprise_ops_smoke.sh` — **shipped**

**Exit criteria (E2):** End-to-end provision demo + alert on readiness failure — **shipped** (`scripts/enterprise_ops_smoke.sh`)

**UI stack:** React + TypeScript (`@spanda/control-center`, extends `packages/web`); Rust backend (`spanda-api`); future Tauri desktop packaging.

**Lean-core:** Contracts in `spanda-api`, `spanda-config`, `spanda-security`, `spanda-ops`; vendor SDKs and alert channels in optional packages.

---

## Differentiation & signature capabilities

**Verifiable missions, explainable operations, predictive trust** — composes Readiness, Assurance, Diagnosis, Recovery, Trust, Health, Continuity, Simulation, Replay, and Traceability without duplicating them.

Full analysis: [differentiation-roadmap.md](./differentiation-roadmap.md)

### Signature capabilities

| Capability | Status |
|------------|--------|
| Safety-Typed AI | **Stable** |
| Readiness Engine | **Stable** |
| Continuity & Takeover | **Stable** |
| Mission Contracts | **Planned** (NOW) |
| Trust Framework | **Planned** (NEXT) |
| Explainability & Audit Trail | **Planned** (NOW) |

### Priority horizons

| Horizon | Timeline | Areas |
|---------|----------|-------|
| **NOW** | 0–3 months | Mission Contracts, Explainability, Decision Audit Trail, Safety Coverage, Recovery Coverage |
| **NEXT** | 3–6 months | What-If Analysis, Mission Risk Analysis, Readiness Forecasting, Trust Graph, Scorecards |
| **LATER** | 6–12 months | Digital Mission Twin, Certification Packs, Mission Time Travel, Human/Robot Teaming, Autonomous Governance |

### NOW deliverables (v0.5+)

Design specs and topic guides are **shipped**; CLI crates and commands are **implemented** (`spanda-contract`, `spanda-explain`, `spanda-decision` in the workspace).

| Item | CLI | Crate | Docs | Code |
|------|-----|-------|------|------|
| Mission Contracts | `spanda contract verify` | `spanda-contract` | [mission-contracts.md](./mission-contracts.md) | **Stable** (static analysis v1) |
| Explainability | `spanda explain` | `spanda-explain` | [explainability.md](./explainability.md) | **Stable** (static v1) |
| Decision Audit Trail | trace synthesis + `spanda audit decisions` | `spanda-decision` | [decision-audit-trail.md](./decision-audit-trail.md) | **Stable** (trace parse v1) |
| Safety Coverage | `spanda safety-coverage` | extends `spanda-readiness` | [safety-coverage.md](./safety-coverage.md) | **Stable** |
| Recovery Coverage | `spanda recovery-coverage` | extends `spanda-assurance` | [recovery-coverage.md](./recovery-coverage.md) | **Stable** |

**Exit criteria:** `spanda demo differentiation` + `scripts/differentiation_smoke.sh`.

Topic guides: [mission-contracts.md](./mission-contracts.md) · [explainability.md](./explainability.md) · [decision-audit-trail.md](./decision-audit-trail.md) · [safety-coverage.md](./safety-coverage.md) · [recovery-coverage.md](./recovery-coverage.md)

---

## Mission assurance

**Mission-grade autonomous operations** — knowledge models, state estimation, anomaly detection, prognostics, mitigation, resilience, assurance cases.

| Item | Status |
|------|--------|
| `spanda-assurance` crate (static analysis) | **Stable** |
| Language declarations (`knowledge_model`, `state_estimator`, `anomaly_detector`, …) | **Stable** |
| CLI (`assure`, `anomaly scan`, `state estimate`, `prognostics`, `mission verify`, `resilience check`, `mitigation plan`) | **Stable** |
| Runtime `state_estimator` → weighted fusion bindings | **Experimental** |
| Learned anomaly backends (`learned backend`, `spanda-anomaly`) | **Experimental** — runtime `scan_learned` + EMA volatility + optional ONNX (`SPANDA_ANOMALY_ONNX_MODEL_PATH`) |
| Weighted fusion package (`spanda-fusion`, `assurance.fusion`) | **Experimental** — provider dispatch for fusion weights |
| Full ML inference (custom ONNX architectures) | **Future** — beyond 2-feature anomaly scaffold |

See [mission-assurance.md](./mission-assurance.md), [state-estimation.md](./state-estimation.md).

---

## Self-healing & recovery

**Safety-first recovery** — `recovery_policy`, validation gates, knowledge store, runtime dispatch, fleet mesh relay.

| Item | Status |
|------|--------|
| `recovery_policy` syntax + `RecoveryPlanner` | **Stable** |
| CLI (`heal`, `recover`, `recovery-report`, `recovery knowledge`, `sim --inject-failure`) | **Stable** |
| Recovery diagnostics (`spanda check --readiness-json`) | **Stable** |
| Runtime dispatch (modes, speed caps, connectivity, mission pause) | **Experimental** |
| Operator approval (env, Approval topics, mission `requires approval`) | **Experimental** |
| Fleet mesh recovery (`POST /v1/fleet/recovery`, `SPANDA_FLEET_MESH_URL`) | **Experimental** |
| Recovery reassign → continuity mesh relay | **Stable** | Fleet recovery `reassign mission` relays continuity when mesh URL is set |
| Fleet agent assurance recovery (`POST /v1/recovery/execute`, deployed program) | **Experimental** |
| Fleet agent interpreter recovery (`execute_recovery_on_program`, `recovery_engine`) | **Experimental** |
| TypeScript recovery diagnostics (LSP fallback) | **Stable** |
| `spanda demo self-healing` | **Stable** |

See [self-healing.md](./self-healing.md), [recovery-policies.md](./recovery-policies.md).

---

## Mission continuity

**Mission continuity, delegation, takeover, and succession** — checkpoint resume, state transfer, successor ranking, safety-gated takeover.

| Item | Status |
|------|--------|
| Continuity framework (`MissionContinuityManager`, `TakeoverCoordinator`, `SuccessionPlanner`, …) | **Stable** |
| Takeover modes (resume, restart, partial, shadow, hot, cold, human) | **Stable** |
| State transfer (`MissionStateSnapshot`, `MissionContextTransfer`) | **Stable** |
| CLI (`continuity`, `takeover`, `delegate`, `succession`) | **Stable** |
| Continuity diagnostics (`spanda check --readiness-json`) | **Stable** |
| TypeScript continuity diagnostics (LSP fallback) | **Stable** |
| `spanda demo continuity` + showcase examples | **Stable** |
| Official package `spanda-mission-continuity` (`assurance.continuity`) | **Stable** |
| Language `continuity_policy` declarations | **Stable** |
| Durable checkpoint store (`.spanda/mission-checkpoints.json`) | **Stable** |
| Runtime takeover dispatch (interpreter + fleet agents) | **Stable** |
| Auto-trigger continuity during `run` / `sim` on health faults | **Stable** |
| Swarm member continuity (`spanda swarm coordinate --failed`) | **Stable** |
| TypeScript mission continuity mirror + checkpoint store | **Stable** |

See [mission-continuity.md](./mission-continuity.md) and [continuity-policies.md](./continuity-policies.md).

---

## Language

**Spanda Language (`.sd`)** — syntax, types, robot primitives, units, safety types.

| Item | Status |
|------|--------|
| Lexer, parser, AST, type checker | **Stable** |
| Physical units, `module`/`import`, structs/enums/traits | **Stable** |
| Robot primitives (`robot`, `sensor`, `actuator`, `task`, `agent`) | **Stable** |
| Trigger model (`on`, `every`, `when`, `while`) | **Stable** |
| Typed handler return types | **Stable** |
| Regex literals and validation rules | **Stable** |
| Self-hosting compiler subset | **Future** |
| LLVM as language execution path | **Experimental** — see [compiler-backend-roadmap.md](./compiler-backend-roadmap.md) |

Foundation: Phases 1–35 complete — [lean-core-roadmap.md](./lean-core-roadmap.md)

---

## Runtime

**Spanda Runtime** — interpreter, scheduler, HAL, concurrency, provider dispatch.

| Item | Status |
|------|--------|
| Tree-walking interpreter (primary path) | **Stable** |
| Cooperative concurrency (`spawn`, `join`, `select`) | **Stable** |
| Trigger-driven scheduler + telemetry flags | **Stable** |
| `spanda-certify` runtime gate | **Stable** |
| Real-time contracts (`deadline`, `jitter`, `priority`) | **Stable** |
| Reliability (watchdogs, modes, `recover from`) | **Stable** |
| World model / fusion belief hook | **Experimental** |
| Native binary via LLVM | **Experimental** — `spanda deploy --target native`, [native-deploy.md](./native-deploy.md) |

---

## Verification

**Spanda Verify** — hardware, capability, and behavioral verification.

| Item | Status |
|------|--------|
| `spanda verify` (profiles, `--simulate`, `--json`) | **Stable** |
| `deploy`, `requires_hardware`, hardware profiles | **Stable** |
| Behavioral `verify { }` assertions | **Stable** |
| Capability traceability matrices | **Stable** |
| `spanda check --verification-json` + LSP quick-fixes | **Stable** |
| CI integration guide | **Stable** — [ci-verify.md](./ci-verify.md) |
| Production verify on 5+ profiles | **Future** (v1.0) |
| Hardware adapter trait codegen | **Future** |

---

## Safety

**Spanda Safety** — type-level and runtime safety engine.

| Item | Status |
|------|--------|
| `ActionProposal` → `SafeAction` compile-time gate | **Stable** |
| `safety { }` zones, `max_speed`, `stop_if` | **Stable** |
| Kill switch + `remote_signed` handlers | **Stable** |
| Agent `can[]` capability clarity | **Stable** |
| Certification metadata (`certify`, `spanda certify prove`) | **Experimental** |
| Minimum-hardware safety analysis | **Stable** |

---

## Simulation

**Spanda Sim** and **Spanda Replay** — test and regress without hardware.

| Item | Status |
|------|--------|
| `spanda run` / `spanda sim` (physics-lite) | **Stable** |
| Digital twins (`twin`, mirror, replay buffer) | **Stable** |
| `simulate_compatibility` fault injection | **Stable** |
| Mission trace `--record` | **Stable** |
| `spanda replay` (`--deterministic`, `--playback`) | **Stable** |
| Persistent telemetry store (`--persist-telemetry`, `spanda telemetry`) | **Stable** — JSONL/SQLite, sessions, replay; OTLP `push`/`serve`, `fleet-push` mesh aggregation — [telemetry-store.md](./telemetry-store.md) |
| Wall-clock sim mode | **Stable** — [realtime.md](./realtime.md), [replay.md](./replay.md) |
| Twin cloud SaaS | **Future** |
| Full physics (Gazebo/Isaac class) | **Out of scope** — package bridges only |

---

## Health

**Spanda Health** — operational readiness and fleet policies.

| Item | Status |
|------|--------|
| `health_check`, `health_policy` | **Stable** |
| Fleet `require` clauses at runtime | **Stable** |
| `spanda demo health` showcase | **Stable** |
| Operational readiness engine (`spanda readiness`) | **Stable** — [readiness.md](./readiness.md) |
| Mission verification, failure analysis, safety reports | **Stable** — see readiness docs |
| Swarm quorum / mesh health | **Experimental** — [swarm-health.md](./swarm-health.md) |

---

## Fleet

**Spanda Fleet** — multi-robot simulation and distributed coordination.

| Item | Status |
|------|--------|
| `spanda fleet run` (in-process) | **Stable** |
| Fleet orchestrate (round-robin report) | **Stable** |
| HTTP fleet agents + `--remote` | **Experimental** — [fleet-distributed.md](./fleet-distributed.md) |
| Fleet mesh coordinator | **Experimental** |
| OTA deploy plan / rollout / rollback | **Stable** (local state) / remote **Experimental** |
| ROS2 rclpy golden path | **Experimental** — [ros2-golden-path.md](./ros2-golden-path.md) |
| `spanda ros2 check` | **Stable** |

---

## Packages

**Spanda Registry** and **Spanda Providers** — extensibility without bloating the core.

| Item | Status |
|------|--------|
| `spanda install` / `update` / `publish` | **Stable** |
| Hosted registry index (38 packages) | **Stable** — [registry.md](./registry.md) |
| Provider dispatch + `--trace-providers` | **Stable** |
| Official packages (ROS2, MQTT, GPS, vision, …) | **Stable** scaffolds / live **Experimental** |
| Live AI providers (OpenAI, Anthropic, ONNX) | **Experimental** — [live-ai-provider.md](./live-ai-provider.md) |
| Live IoT / MQTT bridges | **Experimental** |
| Blockchain / ledger adapters | **Future** (community packages only) |

---

## Tooling

CLI, LSP, debugger, docs site, and contributor ergonomics.

| Item | Status |
|------|--------|
| Native CLI (`check`, `verify`, `run`, `sim`, `fleet`, `fmt`, `lint`) | **Stable** |
| `cargo install spanda` | **Stable** |
| Bundled `spanda demo {rover,safety,verify,fleet,health,readiness,assurance,self-healing,continuity,differentiation}` | **Stable** |
| Operations dashboard (`packages/web` Operations view) | **Experimental** — local readiness scoring, live agent fetch, continuity panel, WASM telemetry panel |
| mdBook GitHub Pages | **Stable** |
| LSP hover + SafeAction quick-fix | **Stable** |
| VS Code snippets + VSIX CI | **Stable** |
| VS Code Marketplace listing | **Partial** — CI + release workflow ready; listing blocked on maintainer `VSCE_PAT` |
| DAP debugger | **Experimental** — [debugging.md](./debugging.md) |
| WASM web playground | **Experimental** — killer demo preset; Check/Run sim; Operations + telemetry when WASM built |

---

## Release milestones

### v0.4 — Deploy path (current tag)

**Theme:** Native binaries, ROS2 polish, distributed fleet docs.  
**Tagged:** 2026-06-22. Post-tag work on `main` (continuity runtime hardening, telemetry OTLP/fleet, differentiation docs) ships toward **v0.5**.

| Item | Status |
|------|--------|
| `spanda deploy --target native` | **Experimental** |
| `spanda compile-native` / LLVM golden paths | **Experimental** |
| `spanda ros2 check` | **Stable** |
| Distributed fleet guide | **Stable** |
| Mission continuity runtime (takeover, checkpoints, fleet mesh) | **Stable** (post-v0.4.0 on `main`) |
| Persistent telemetry + OTLP/fleet aggregation | **Stable** (post-v0.4.0 on `main`) |

### v0.5 — Beta credibility (next)

**Theme:** Close the last adoption blocker; implement differentiation NOW capabilities.  
**Target:** Q4 2026.

| Item | Status |
|------|--------|
| Killer demo + CI golden path | **Stable** |
| Live AI (OpenAI, Anthropic, ONNX) + CI | **Stable** |
| ROS2 rclpy golden path + CI | **Stable** |
| Hosted registry (38 packages) + `spanda publish` mirror | **Stable** |
| CI verify guide + adoption paths (P1 enablers) | **Stable** — [ci-verify.md](./ci-verify.md), [adoption-path.md](./adoption-path.md) |
| VS Code Marketplace listing | **Partial** — only open P0 blocker; needs maintainer `VSCE_PAT` |
| Mission Contracts (`spanda contract verify`) | **Stable** — static analysis over mission_plan + policies |
| Explainability (`spanda explain`) | **Stable** — static explain v1 |
| Decision Audit Trail (`spanda audit decisions`) | **Stable** — trace parse + synthesis v1 |
| Safety / Recovery Coverage CLIs | **Stable** |

**Exit criteria:** Marketplace publish + `spanda demo differentiation` + `scripts/differentiation_smoke.sh` — **differentiation smoke shipped**; Marketplace still pending `VSCE_PAT`.

See [product-strategy.md](./product-strategy.md) § v0.5 beta and [tier-3-priority-plan.md](./tier-3-priority-plan.md) § P0–P1.

### v0.3 — Tooling polish (complete)

**Theme:** IDE, diagnostics, registry, install ergonomics.

| Item | Status |
|------|--------|
| Crate rename → `spanda`, bundled demos | **Stable** |
| Hosted registry (38 packages) | **Stable** |
| LSP + showcase CI smoke | **Stable** |

### v0.2 — Credibility & onboarding (complete)

**Theme:** Trust table, showcase demos, docs site, one-command demos.

| Item | Status |
|------|--------|
| Feature status audit + `spanda demo` | **Stable** |
| mdBook GitHub Pages | **Stable** |

### v1.0 — Production positioning

**Theme:** Trust for field deployment and enterprise operations.

| Item | Tier |
|------|------|
| Interpreter + sim as supported LTS runtime | Stable |
| Safety + verify + replay as certified workflows | Stable |
| Native codegen for selected HAL profiles | Experimental → Stable |
| Control Center + `spanda-api` (REST/gRPC CLI parity) | Planned → Stable |
| Device Pool + Provisioning + RBAC | Planned → Stable |
| Self-hosting compiler subset | Future (not primary) |
| Blockchain / cryptocurrency adapters | **Out of scope** |
| Advanced swarm intelligence research | **Out of scope** |

---

## Related

- [enterprise-operations-roadmap.md](./enterprise-operations-roadmap.md) — Control Center, Device Pool, provisioning, RBAC, APIs, observability (20 pillars)
- [differentiation-roadmap.md](./differentiation-roadmap.md) — signature capabilities, mission contracts, explainability, coverage (15 areas)
- [platform-maturity-roadmap.md](./platform-maturity-roadmap.md) — adoption, trust, operations expansion (16 areas)
- [platform-overview.md](./platform-overview.md)
- [feature-status.md](./feature-status.md)
- [product-strategy.md](./product-strategy.md)
- [compiler-backend-roadmap.md](./compiler-backend-roadmap.md)
- [lean-core-roadmap.md](./lean-core-roadmap.md)
- [roadmap-codebase-audit-2026-06.md](./roadmap-codebase-audit-2026-06.md)
