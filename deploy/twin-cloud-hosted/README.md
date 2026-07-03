# Hosted Twin Cloud — deploy scaffold

Docker Compose bundle for a **single-tenant** Twin Cloud Control Center with persisted snapshot storage.

Full runbook: [docs/hosted-twin-cloud.md](../../docs/hosted-twin-cloud.md)

## Quick start

```bash
cd deploy/twin-cloud-hosted
cp .env.example .env
# Edit SPANDA_API_KEY and optional SPANDA_TENANT_ID
docker compose up --build
```

Edge clients:

```bash
export SPANDA_TWIN_CLOUD_URL=http://localhost:8080
export SPANDA_TWIN_CLOUD_API_KEY="$SPANDA_API_KEY"
spanda twin cloud push examples/showcase/mission_twin/patrol.sd
spanda twin cloud list
```

## Multi-tenant production

Run **one compose stack per tenant** (different `SPANDA_TENANT_ID`, API key, and volume), or operate a shared Control Center with per-key `tenant_id` scoping — see the hosted runbook.

## Verify

```bash
./scripts/hosted_twin_cloud_smoke.sh
```
