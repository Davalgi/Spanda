# Hosted Twin Cloud SaaS

**Status:** Experimental (product pilot) · **OSS backend:** Experimental · **Deploy:**
[deploy/twin-cloud-hosted/](../deploy/twin-cloud-hosted/)

Multi-tenant Twin Cloud runs as a dedicated Control Center deployment (or twin-cloud-only service)
that stores mission twin snapshots per tenant.

## Architecture

```text
                    ┌─────────────────────────────────────┐
  Edge fleet        │  Hosted Twin Cloud (Control Center) │
  ─────────         │  SPANDA_TENANT_ID + API key store   │
  twin cloud push ──┼─► POST /v1/twins/{id}/snapshots     │
  twin cloud sync ──┼─► POST /v1/twins/sync (Bearer)      │
  SDK / dashboard ◄─┼── GET /v1/twins, /history            │
                    │  Volume: control-center-twins.json  │
                    └─────────────────────────────────────┘
```

The open-source Control Center embeds the same `/v1/twins/*` backend for development. Production
pilots use the [Docker Compose scaffold](../deploy/twin-cloud-hosted/README.md) or your own
orchestration (Kubernetes, systemd, bare metal).

## Deployment options

| Model | When to use | Isolation |
|-------|-------------|-----------|
| **Single-tenant stack** | One customer per host | `SPANDA_TENANT_ID` + dedicated volume + API key |
| **Shared Control Center** | Internal ops platform | Per-key `tenant_id`; snapshots scoped in store |
| **Edge-only (OSS)** | Dev / field laptop | Embedded Control Center, no hosted infra |

### Docker Compose (pilot)

```bash
cd deploy/twin-cloud-hosted
cp .env.example .env
# Set SPANDA_API_KEY and SPANDA_TENANT_ID
docker compose up --build
```

Clients point at the host:

```bash
export SPANDA_TWIN_CLOUD_URL=https://twins.example.com
export SPANDA_TWIN_CLOUD_API_KEY="<operator-key>"
spanda twin cloud push patrol.sd
```

## Environment

| Variable | Purpose |
|----------|---------|
| `SPANDA_CONTROL_CENTER_STATE_DIR` | Persistent twin store (`control-center-twins.json`) |
| `SPANDA_API_KEY` / `SPANDA_API_KEYS_FILE` | Bearer auth for mutations |
| `SPANDA_TENANT_ID` | Default tenant for this instance |
| `SPANDA_TWIN_CLOUD_URL` | Edge/client base URL (client-side) |
| `SPANDA_TWIN_CLOUD_API_KEY` | Edge/client Bearer token |

Each API key may carry a `tenant_id`; keys must match the instance tenant or requests receive **403
tenant mismatch** (see `scripts/hosted_twin_cloud_smoke.sh`).

## Tenant onboarding runbook

1. **Provision stack** — deploy compose/K8s with unique `SPANDA_TENANT_ID` and persistent volume.
2. **Issue operator key** — `spanda control-center api-key generate --export` on the host, or admin
   API `POST /v1/admin/api-keys` with matching `tenant_id`.
3. **Smoke** — `./scripts/hosted_twin_cloud_smoke.sh` (local) or `SPANDA_TWIN_CLOUD_URL=…
   ./scripts/twin_cloud_saas_smoke.sh`.
4. **Edge config** — fleet agents set `SPANDA_TWIN_CLOUD_URL` + `SPANDA_TWIN_CLOUD_API_KEY`.
5. **Monitor** — Control Center `/v1/twins`, Administration twin registry tab, optional OTLP export.

## Legacy bridge

Programs using `SPANDA_CLOUD_UPLOAD_URL` (`cloud.upload`) migrate via:

```bash
spanda twin cloud import-replay /path/to/replay.json --program patrol.sd
```

REST: `POST /v1/twins/import-replay` with `{ "program": "path/to/file.sd", "twin_id": "optional" }`.

Unified validation: `./scripts/twin_cloud_unified_path.sh`.

## SDK clients (0.5.5+)

```python
from spanda_sdk import SpandaClient

client = SpandaClient("https://twins.example.com", api_key=os.environ["SPANDA_API_KEY"])
twins = client.list_twins()
client.sync_twin()
client.get_twin_history("patrol")
```

Publish new SDK builds: `./scripts/publish_sdk_release.sh` (after
[sdk-publishing.md](./sdk-publishing.md) secrets are configured).

## Product checklist (hosted pilot → GA)

| Item | OSS | Hosted product |
|------|-----|----------------|
| REST + gRPC twins API | Shipped | Same binary |
| RBAC + tenant scoping | Shipped | Per-tenant keys |
| Persistence + history | Shipped | Backed volume / object store |
| Docker deploy scaffold | Shipped | [deploy/twin-cloud-hosted/](../deploy/twin-cloud-hosted/) |
| Field soak → Stable | Gate ready | Run after pilot soak |
| Billing / SLA / multi-region | — | Product track |

## Related

- [twin-cloud.md](./twin-cloud.md) — OSS API and CLI
- [stable-hardening-twin-cloud-saas.md](./stable-hardening-twin-cloud-saas.md) — Stable promotion gate
- [sdk-publishing.md](./sdk-publishing.md) — release tags and registries
