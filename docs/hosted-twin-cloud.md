# Hosted Twin Cloud SaaS

**Status:** Experimental · **Horizon:** production pilots

Multi-tenant Twin Cloud runs as a dedicated Control Center deployment (or twin-cloud-only service) that stores mission twin snapshots per tenant.

## Deployment model

```text
Edge fleet                          Hosted Twin Cloud
──────────                          ─────────────────
spanda twin cloud push patrol.sd →  HTTPS /v1/twins/{id}/snapshots
spanda twin cloud sync             →  POST /v1/twins/sync (Bearer)
Ops dashboard / SDK                ←  GET /v1/twins, /history
```

The open-source Control Center embeds the same backend for development. Production hosts set:

| Variable | Purpose |
|----------|---------|
| `SPANDA_CONTROL_CENTER_STATE_DIR` | Persistent twin store directory |
| `SPANDA_API_KEY` / key store file | Bearer auth for mutations |
| `SPANDA_TENANT_ID` | Default tenant on single-tenant hosts |

Each API key may carry a `tenant_id`; snapshots are scoped per tenant in the store.

## Legacy bridge

Programs using `SPANDA_CLOUD_UPLOAD_URL` (provider `cloud.upload`) can migrate via:

```bash
spanda twin cloud import-replay /path/to/replay.json --program patrol.sd
```

REST: `POST /v1/twins/import-replay` with `{ "program": "path/to/file.sd", "twin_id": "optional" }`.

## SDK clients

Point `SPANDA_TWIN_CLOUD_URL` (or SDK `base_url`) at the hosted endpoint and set `SPANDA_API_KEY` for push/sync/import.

See [twin-cloud.md](./twin-cloud.md) · [sdk-publishing.md](./sdk-publishing.md).
