# Hosted Twin Cloud — Product Track

**Status:** Pilot (managed service) · **OSS backend:** **Stable** at `/v1/twins/*`

Commercial hosted Twin Cloud builds on the open-source Control Center twin registry. This document
covers **product** concerns beyond the OSS pilot ([hosted-twin-cloud.md](./hosted-twin-cloud.md)).

## Deployment options

| Tier | Artifact | Use case |
|------|----------|----------|
| **Docker Compose** | [deploy/twin-cloud-hosted/](../deploy/twin-cloud-hosted/) | Single-tenant pilot |
| **Kubernetes (raw)** | [deploy/twin-cloud-hosted/k8s/](../deploy/twin-cloud-hosted/k8s/) | One tenant per namespace |
| **Helm** | [deploy/twin-cloud-hosted/helm/twin-cloud/](../deploy/twin-cloud-hosted/helm/twin-cloud/) | Repeatable fleet rollout |

```bash
# Helm install (single tenant)
helm upgrade --install acme-twin-cloud deploy/twin-cloud-hosted/helm/twin-cloud \
  --set tenantId=acme \
  --set apiKey="$SPANDA_API_KEY" \
  --set ingress.enabled=true \
  --set ingress.host=twins.acme.example.com
```

## Multi-tenant models

| Model | Isolation | Ops |
|-------|-----------|-----|
| **Stack per tenant** | Dedicated Deployment + PVC + `SPANDA_TENANT_ID` | Recommended for GA pilots |
| **Shared Control Center** | Per API key `tenant_id`; store filters by tenant; get/history enforce snapshot tenant match (403); push forces instance tenant | Internal platform teams |
| **Regional cells** | One Helm release per region; DNS geo-routing | Multi-region GA |

## Product checklist (GA)

| Capability | Pilot | GA target |
|------------|-------|-----------|
| Snapshot push/pull/sync/history | Shipped (OSS) | Same API |
| RBAC + tenant scoping | Shipped | Per-tenant key rotation |
| Snapshot tenant isolation (get/history/push) | **Shipped** | Same |
| Persistent volume / backup | K8s PVC + compose volume | Automated backup (S3-compatible) |
| Ingress + TLS | Helm optional ingress | Required |
| Billing meter (`GET /v1/twins/usage` + gRPC `GetTwinUsage`) | **Shipped** | Export to billing pipeline |
| SLA / status page | — | 99.9% target, public status |
| Multi-region | — | Active-passive or cell-per-region |
| Control Center tenant switcher | **Shipped** (profile stores API key + `tenant_id` label) | Per-tenant key rotation UX |
| Twin Cloud usage dashboard | **Shipped** (Twins + Administration meters) | Snapshot meters, sync status in Control Center |

## Billing integration

Meter these dimensions per `tenant_id` via `GET /v1/twins/usage` (and gRPC `GetTwinUsage`):

| Field | Source |
|-------|--------|
| `twin_count` | Store-backed distinct twins for the instance tenant |
| `snapshot_count` | Store-backed history ring size for the tenant |
| `push_count` | In-process counter (`POST …/snapshots`, import-replay) |
| `sync_count` | In-process counter (`POST /v1/twins/sync`) |
| `history_count` | In-process counter (`GET …/history`) |

Also export via Control Center audit (`GET /v1/audit/mutations`) or OTLP metrics to your billing
pipeline. Process counters reset on Control Center restart; store counts survive persistence.

## SLA operations

- **Health:** `GET /v1/health` (liveness/readiness probes in K8s)
- **Smoke:** `./scripts/hosted_twin_cloud_smoke.sh` per cell after deploy
- **Soak:** OSS Stable gate — `scripts/twin_cloud_field_soak_init.sh` + 30-day clock

## Related

- [twin-cloud.md](./twin-cloud.md) — OSS API
- [stable-hardening-twin-cloud-saas.md](./stable-hardening-twin-cloud-saas.md) — Stable promotion
- [sdk-publishing.md](./sdk-publishing.md) — client SDK releases
