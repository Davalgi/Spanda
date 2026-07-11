# Twin Cloud SaaS — Stable Hardening Checklist

**Promoted 2026-07-02** — Twin Cloud OSS backend (`spanda-twin-cloud`, `/v1/twins/*`).

**Related:** [twin-cloud.md](./twin-cloud.md) · [hosted-twin-cloud.md](./hosted-twin-cloud.md)

Local digital mission twin evaluation (`spanda twin mission`) remains **Stable** separately.

| Gate | Status |
|------|--------|
| REST `/v1/twins/*` + RBAC on mutations | **Shipped** |
| gRPC `ListTwins` / `GetTwinUsage` / `GetTwin` / `SyncTwin` / `PushTwinSnapshot` / `ImportTwinReplay` (proto **1.0.17**) | **Shipped** |
| Tenant isolation (get/history 403; push forces `tenant_id`) + `GET /v1/twins/usage` | **Shipped** |
| File-backed persistence + history ring | **Shipped** |
| CLI `spanda twin cloud push\|pull\|list\|sync\|import-replay` | **Shipped** |
| SDK REST + gRPC (0.5.5+) | **Shipped** |
| Registry `spanda-twin-cloud` + `import twin.cloud` | **Shipped** |
| Unified legacy + SaaS smoke | `scripts/twin_cloud_unified_path.sh` |
| Hosted tenant smoke | `scripts/hosted_twin_cloud_smoke.sh` |
| Field soak (30 days) | `./scripts/twin_cloud_field_soak_init.sh` |

```bash
# CI / local (soak skipped)
SPANDA_TWIN_CLOUD_SKIP_SOAK=1 ./scripts/twin_cloud_stable_promotion_gate.sh

# Production re-verify after soak
SPANDA_TWIN_CLOUD_SKIP_SOAK=0 ./scripts/twin_cloud_stable_promotion_gate.sh
```

Hosted **managed product** (billing, multi-region SLA) remains on the product track —
[hosted-twin-cloud-product.md](./hosted-twin-cloud-product.md).
