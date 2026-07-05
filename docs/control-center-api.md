# Control Center API Reference (v1)

Stable Control Center API served by `spanda control-center serve` and implemented in
`crates/spanda-api`.

For the Rust/TypeScript **compiler crate index**, see [api-reference.md](./api-reference.md).

## Transports

| Transport | Entry |
|-----------|--------|
| REST | `http://host:8080/v1/*` |
| OpenAPI | `GET /v1/openapi.json` |
| gRPC | `ControlCenter` service — proto semver from `GET /v1/version` → `grpc.proto_semver` (currently **1.0.14**); see `proto/spanda/v1/control_center.proto` |
| WebSocket | `WS /v1/stream/telemetry` |
| JSON-RPC gateway | `POST /v1/rpc` |

## SDK program operations (CLI parity)

These endpoints delegate to the same Rust crates as CLI commands:

| Endpoint | CLI equivalent |
|----------|----------------|
| `POST /v1/programs/readiness` | `spanda readiness <file.sd>` |
| `POST /v1/programs/assure` | `spanda assure <file.sd>` |
| `POST /v1/programs/diagnose` | `spanda diagnose <file.sd\|.trace>` |
| `POST /v1/programs/recovery/heal` | `spanda heal` (legacy assurance) |

## Recovery Orchestrator (`/v1/recovery/*`)

Platform-wide recovery intelligence — 14 REST routes mirroring `spanda recovery *` CLI. Full
reference: [recovery-api.md](./recovery-api.md) · SDK: [recovery-sdk.md](./recovery-sdk.md).

| Endpoint | CLI equivalent |
|----------|----------------|
| `GET /v1/recovery/plans` | Active recovery plans |
| `GET /v1/recovery/history` | `spanda recovery history` (persisted in `control-center-recovery.json`) |
| `POST /v1/recovery/plan` | `spanda recovery plan` |
| `POST /v1/recovery/simulate` | `spanda recovery simulate` |
| `POST /v1/recovery/execute` | `spanda recovery execute` |
| `POST /v1/recovery/validate` | `spanda recovery validate` / `dry-run` |
| `GET /v1/recovery/playbooks` | `spanda recovery playbooks` |
| `GET /v1/recovery/metrics` | `spanda recovery metrics` |
| `GET /v1/recovery/graph` | `spanda recovery graph` |
| `GET /v1/recovery/policies` | Entity recovery policies |
| `POST /v1/recovery/explain` | `spanda recovery explain` |
| `GET/POST /v1/recovery/predictive` | Telemetry-driven indicators |
| `GET /v1/recovery/recoverable-entities` | Recoverable entity registry |
| `POST /v1/recovery/recommend` | Knowledge-base strategy recommendation |

gRPC RPCs: `ListRecoveryPlans`, `PlanRecovery`, `GetRecoveryPredictive`, `ListRecoverableEntities`,
`RecommendRecovery`, … (proto semver from `GET /v1/version` — currently **1.0.14**).

| `POST /v1/programs/verify/hardware` | `spanda verify` |
| `POST /v1/programs/verify/capabilities` | `spanda verify --capabilities` |
| `POST /v1/programs/verify/mission` | `spanda verify mission` |
| `POST /v1/programs/simulation` | `spanda sim` (set `"execute": true` to run) |
| `POST /v1/programs/replay` | `spanda replay` (set `"deterministic"` or `"playback"`) |
| `GET /v1/trust/program` | `spanda trust <file.sd>` |
| `GET /v1/instance` | `spanda control-center status` |

### `GET /v1/instance`

Returns runtime metadata for the running Control Center process: bind address, optional `--config` /
`--program` paths, tenant, device pool summary, fleet agent count, and alert count. No
authentication required.

```json
{
  "ok": true,
  "service": "spanda-control-center",
  "bind": "127.0.0.1:8080",
  "config_path": "spanda.toml",
  "program_path": "examples/robotics/fleet_field_trial.sd",
  "config_loaded": true,
  "overall_status": "healthy",
  "device_pool": { "total": 0, "healthy": 0, "degraded": 0, "failed": 0 },
  "fleet_agent_count": 0,
  "alert_count": 0
}
```

Use `spanda control-center status [--discover]` to query from the CLI (see
[control-center.md](./control-center.md)).

### Request body (program ops)

```json
{
  "file": "rover.sd",
  "target": "jetson-orin",
  "include_runtime": false,
  "traceability": true
}
```

`file` is optional when Control Center was started with `--program`.

## Cognitive & Resilience (`/v1/autonomy/*`)

Functional domain summaries — mirror `spanda reflex`, `homeostasis`, `immunity`, `fusion`, `alerts`
CLI. Guide: [cognitive-resilience-architecture.md](./cognitive-resilience-architecture.md).

| Endpoint | CLI / domain | Description |
|----------|--------------|-------------|
| `GET /v1/autonomy/reflex` | `spanda reflex list` | Reflex & Safety — platform reflex catalog |
| `GET /v1/autonomy/reflex/traces` | `spanda reflex trace` | Runtime/file-backed reflex traces |
| `GET /v1/autonomy/homeostasis` | `spanda homeostasis check` | Homeostasis Engine — stability reports |
| `GET /v1/autonomy/immunity` | `spanda immunity scan` | Platform Immunity — quarantine scan |
| `GET /v1/autonomy/attention` | `spanda alerts analyze` | Attention Engine — prioritized event window |
| `GET /v1/autonomy/fusion` | `spanda fusion check` | Sensory Fusion — entity-derived multi-source confidence |
| `GET /v1/autonomy/memory` | — | Operational Memory — category refs + model |
| `GET /v1/entities/{id}/autonomy` | — | Enriched `Entity.autonomy` profile |

gRPC parity (proto **1.0.14+**): `ListAutonomyReflexes`, `ListAutonomyReflexTraces`,
`GetAutonomyHomeostasis`, `GetAutonomyImmunity`, `GetAutonomyAttention`, `GetAutonomyFusion`,
`GetAutonomyMemory`, `GetEntityAutonomy`.

SDK: [entity-sdk.md](./entity-sdk.md#cognitive--resilience-domain-clients) · Maturity:
[cognitive-resilience-maturity.md](./cognitive-resilience-maturity.md).

## Entity registry

Full entity REST/gRPC reference: [entity-apis.md](./entity-apis.md). SDK methods:
[entity-sdk.md](./entity-sdk.md).

Read endpoints are unauthenticated by default; mutations require Bearer `SPANDA_API_KEY`.

| Endpoint | Auth | Description |
|----------|------|-------------|
| `GET /v1/entities` | — | Unified entity inventory (optional query filters) |
| `GET /v1/entities/graph` | — | Full entity graph |
| `GET /v1/entities/traceability` | — | Unified traceability (entity + program graph) |
| `POST /v1/entities/query` | — | Structured query body |
| `GET /v1/entities/{id}` | — | Entity by id |
| `GET /v1/entities/{id}/relationships` | — | Relationship edges and impact analysis |
| `GET /v1/entities/{id}/health` | — | Health snapshot |
| `GET /v1/entities/{id}/readiness` | — | Readiness snapshot |
| `GET /v1/entities/{id}/trust` | — | Trust and security metadata |
| `POST /v1/entities/{id}/verify` | — | Unified entity verification |
| `POST /v1/entities/register` | Bearer | Register or update entity overlay |
| `POST /v1/entities/{id}/tags` | Bearer | Add or remove tags |
| `POST /v1/entities/relationships` | Bearer | Relate two entities |
| `POST /v1/entities/sync` | Bearer | Sync overlay to TOML fragments |

### gRPC parity (`--grpc-bind`)

| gRPC RPC | REST equivalent |
|----------|-----------------|
| `ListEntities` | `GET /v1/entities` |
| `GetEntity` | `GET /v1/entities/{id}` |
| `GetEntityHealth` | `GET /v1/entities/{id}/health` |
| `GetEntityTrust` | `GET /v1/entities/{id}/trust` |
| `GetEntityGraph` | `GET /v1/entities/graph` |
| `GetEntityTraceability` | `GET /v1/entities/traceability` |
| `QueryEntities` | `POST /v1/entities/query` |
| `GetEntityRelationships` | `GET /v1/entities/{id}/relationships` |
| `GetEntityReadiness` | `GET /v1/entities/{id}/readiness` |
| `VerifyEntity` | `POST /v1/entities/{id}/verify` |
| `RegisterEntity` | `POST /v1/entities/register` |
| `TagEntity` | `POST /v1/entities/{id}/tags` |
| `RelateEntities` | `POST /v1/entities/relationships` |
| `SyncEntities` | `POST /v1/entities/sync` |

Rust `GrpcClient` (`spanda-sdk` `grpc` feature) mirrors these; mutations send Bearer from
`SPANDA_API_KEY`. See [entity-model.md](./entity-model.md) and [sdk-rust.md](./sdk-rust.md).

## Device registry

| Endpoint | Description |
|----------|-------------|
| `GET /v1/devices` | Device pool |
| `POST /v1/devices/discover` | Discovery scan |
| `POST /v1/devices/{id}/provision` | Provision workflow |

## Trust & assurance summaries

| Endpoint | Description |
|----------|-------------|
| `GET /v1/trust/package` | Package trust score |
| `GET /v1/assurance/summary` | Config assurance policy |
| `GET /v1/diagnosis/summary` | Config diagnosis policy |
| `POST /v1/readiness/run` | **Device pool** readiness impact (not program scoring) |

## Shared schemas

Domain types are defined in Rust with `serde` and documented in OpenAPI:

- `ReadinessReport` — `spanda_readiness::types`
- `AssuranceReport` / `MissionAssuranceSummary` — `spanda_assurance`
- `DiagnosisReport` — `spanda_assurance::diagnosis`
- `RecoveryReport` — `spanda_assurance::recovery`
- `HealthReport` — `spanda_capability::health`
- `TrustScoreReport` — `spanda_package::trust`
- `CompatibilityReport` — `spanda_hardware`

SDK wrappers mirror these in each language (`spanda-sdk` types modules).

## Versioning

- API version prefix: `/v1/`
- Policy: `GET /v1/version` — `control_center_ui_version` (Control Center UI semver),
  `spanda_version` (platform build), `grpc.proto_semver`, `grpc.rpc_count`
- Runtime status: `GET /v1/instance` — includes `control_center_ui_version` for `spanda
  control-center status`
- Operator CLI: `spanda control-center --version`
- Full release streams and auto bump: [control-center-versioning.md](./control-center-versioning.md)
- OpenAPI parity enforced by `crates/spanda-api/tests/openapi_parity_tests.rs`

## Authentication

- Bearer token: `Authorization: Bearer $SPANDA_API_KEY`
- RBAC enforced on mutations (provision, OTA, config approvals, entity overlay writes)
- Correlation: `X-Correlation-ID` header (optional, echoed in responses)

## JSON-RPC gateway (`POST /v1/rpc`)

gRPC-compatible JSON gateway for clients without tonic. Example:

```json
{
  "method": "spanda.v1.ControlCenter/EvaluateProgramReadiness",
  "params": { "body_json": "{\"file\":\"rover.sd\"}" }
}
```

Supported SDK methods include program ops (`EvaluateProgramReadiness`, `EvaluateProgramAssure`,
`EvaluateProgramDiagnose`, `EvaluateProgramHeal`, `VerifyProgramHardware`,
`VerifyProgramCapabilities`, `VerifyProgramMission`, `RunProgramSimulation`, `ReplayProgram`,
`GetTrustProgram`) and entity reads (`ListEntities`, `GetEntity`, `GetEntityHealth`,
`GetEntityTrust`, `GetEntityGraph`, `GetEntityTraceability`, `QueryEntities`,
`GetEntityRelationships`, `GetEntityReadiness`, `VerifyEntity`). Entity mutations are **gRPC-only**
(not exposed on the JSON-RPC gateway).

## Event types (WebSocket)

- `health_changed`
- `readiness_changed`
- `mission_started` / `mission_paused`
- `recovery_triggered`
- `device_offline`
- `tamper_detected`
- `kill_switch_triggered`

See [SDK overview](sdk.md) for client usage.
