# Twin Cloud SaaS

**Status:** Stable · **Package:** `spanda-twin-cloud` · **Horizon:** production pilots

Hosted mission twin snapshot registry for field fleets — push digital mission twin state from edge
Control Center or CLI, pull latest snapshots for ops dashboards and fleet analytics.

## Architecture

```text
Edge (robot / field laptop)                Twin Cloud SaaS
spanda twin cloud push patrol.sd    -->    Control Center /v1/twins/*
spanda twin mission (local)                File-backed store + history ring
GET /v1/analytics/mission-twin             GET /v1/twins/{id}/history
```

Production deployments point `SPANDA_TWIN_CLOUD_URL` at a hosted Control Center or dedicated
twin-cloud service. The open-source Control Center embeds the **Twin Cloud backend** for development
and field pilots. Snapshots persist to `.spanda/control-center-twins.json` (override with
`SPANDA_CONTROL_CENTER_STATE_DIR`). See [hosted-twin-cloud.md](./hosted-twin-cloud.md).

## Environment

| Variable | Purpose |
|----------|---------|
| `SPANDA_TWIN_CLOUD_URL` | Base URL (falls back to `SPANDA_CONTROL_CENTER_URL`) |
| `SPANDA_TWIN_CLOUD_API_KEY` | Bearer token (falls back to `SPANDA_API_KEY`) |
| `SPANDA_TWIN_CLOUD_TENANT` | Tenant id (defaults to `SPANDA_TENANT_ID` or `default`) |

Legacy replay upload via `SPANDA_CLOUD_UPLOAD_URL` remains for provider `cloud.upload` — migrate
with `spanda twin cloud import-replay` or `POST /v1/twins/import-replay`.

## CLI

```bash
export SPANDA_TWIN_CLOUD_URL=http://127.0.0.1:8080
export SPANDA_API_KEY=your-operator-key

spanda twin cloud push examples/showcase/mission_twin/patrol.sd
spanda twin cloud list
spanda twin cloud pull patrol --out patrol-twin.json
spanda twin cloud sync
spanda twin cloud import-replay replay.json --program patrol.sd
```

## REST API

| Method | Path | Auth | Description |
|--------|------|------|-------------|
| `GET` | `/v1/twins` | optional | List twin summaries for tenant |
| `GET` | `/v1/twins/usage` | optional | Per-tenant usage meters (store + API counters) |
| `GET` | `/v1/twins/{id}` | optional | Latest snapshot (403 on tenant mismatch) |
| `GET` | `/v1/twins/{id}/history` | optional | Snapshot history ring (403 on tenant mismatch) |
| `POST` | `/v1/twins/{id}/snapshots` | Bearer (Operate) | Push snapshot JSON (forces instance `tenant_id`) |
| `POST` | `/v1/twins/sync` | Bearer (Operate) | Evaluate + store twin for loaded program |
| `POST` | `/v1/twins/import-replay` | Bearer (Operate) | Import legacy replay JSON |

gRPC parity: `ListTwins`, `GetTwinUsage`, `GetTwin`, `GetTwinHistory`, `SyncTwin`, `PushTwinSnapshot`,
`ImportTwinReplay` (proto **1.0.17**).

## Crate

`crates/spanda-twin-cloud` — HTTP client, snapshot envelope, store with history.

## Registry package

`packages/registry/spanda-twin-cloud` — `import twin.cloud;` in Spanda programs.

## SDK (0.5.9)

Rust (`spanda-sdk`), Python (`spanda_sdk`), and TypeScript (`@davalgi/spanda-sdk`) expose:

- `list_twins` / `listTwins`
- `get_twin_usage` / `getTwinUsage`
- `get_twin` / `getTwin`
- `get_twin_history` / `getTwinHistory`
- `sync_twin` / `syncTwin` (auth required)
- `push_twin_snapshot` / `pushTwinSnapshot` (auth required)
- `import_twin_replay` / `importTwinReplay` (auth required)

Rust gRPC client (`GrpcClient`, `grpc` feature) mirrors all seven RPCs.

## Tests

- `cargo test -p spanda-twin-cloud`
- `cargo test -p spanda-api twin_cloud`
- `./scripts/twin_cloud_saas_smoke.sh`
- `./scripts/twin_cloud_unified_path.sh` (legacy + SaaS)
- `./scripts/twin_cloud_stable_promotion_gate.sh` (Stable promotion)

See [digital-mission-twin.md](./digital-mission-twin.md) · [replay.md](./replay.md) ·
[stable-hardening-twin-cloud-saas.md](./stable-hardening-twin-cloud-saas.md).
