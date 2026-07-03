# Hosted Twin Cloud — Product Track

**Status:** Pilot (managed service) · **OSS backend:** **Stable** at `/v1/twins/*`

Commercial hosted Twin Cloud builds on the open-source Control Center twin registry. This document covers **product** concerns beyond the OSS pilot ([hosted-twin-cloud.md](./hosted-twin-cloud.md)).

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
| **Shared Control Center** | Per API key `tenant_id`; store filters by tenant | Internal platform teams |
| **Regional cells** | One Helm release per region; DNS geo-routing | Multi-region GA |

## Product checklist (GA)

| Capability | Pilot | GA target |
|------------|-------|-----------|
| Snapshot push/pull/sync/history | Shipped (OSS) | Same API |
| RBAC + tenant scoping | Shipped | Per-tenant key rotation |
| Persistent volume / backup | K8s PVC + compose volume | Automated backup (S3-compatible) |
| Ingress + TLS | Helm optional ingress | Required |
| Billing meter | — | API calls / stored snapshots per tenant |
| SLA / status page | — | 99.9% target, public status |
| Multi-region | — | Active-passive or cell-per-region |

## Billing integration (stub)

Meter these dimensions per `tenant_id`:

- `POST /v1/twins/{id}/snapshots` (push)
- `POST /v1/twins/sync`
- `GET /v1/twins/{id}/history` (egress-heavy)
- Stored snapshot count (PVC/object store size)

Export via Control Center audit (`GET /v1/audit/mutations`) or OTLP metrics to your billing pipeline.

## SLA operations

- **Health:** `GET /v1/health` (liveness/readiness probes in K8s)
- **Smoke:** `./scripts/hosted_twin_cloud_smoke.sh` per cell after deploy
- **Soak:** OSS Stable gate — `scripts/twin_cloud_field_soak_init.sh` + 30-day clock

## Related

- [twin-cloud.md](./twin-cloud.md) — OSS API
- [stable-hardening-twin-cloud-saas.md](./stable-hardening-twin-cloud-saas.md) — Stable promotion
- [sdk-publishing.md](./sdk-publishing.md) — client SDK releases
