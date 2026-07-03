# Twin Cloud SaaS — Stable Hardening Checklist

**Status:** Experimental (gate ready) · **Related:** [twin-cloud.md](./twin-cloud.md) · [hosted-twin-cloud.md](./hosted-twin-cloud.md)

Local digital mission twin evaluation (`spanda twin mission`) is **Stable**. Cloud snapshot sync (`spanda-twin-cloud`, `/v1/twins/*`) remains **Experimental** until field soak and this gate pass.

| Gate | Status |
|------|--------|
| REST `/v1/twins/*` + RBAC on mutations | **Shipped** |
| gRPC `ListTwins` / `GetTwin` / `SyncTwin` / `PushTwinSnapshot` / `ImportTwinReplay` (proto **1.0.10**) | **Shipped** |
| File-backed persistence + history ring | **Shipped** |
| CLI `spanda twin cloud push\|pull\|list\|sync\|import-replay` | **Shipped** |
| SDK REST + gRPC (0.5.5+) | **Shipped** |
| Registry `spanda-twin-cloud` + `import twin.cloud` | **Shipped** |
| Unified legacy + SaaS smoke | `scripts/twin_cloud_unified_path.sh` |
| Field soak (30 days) | `./scripts/twin_cloud_field_soak_init.sh` |

## Promotion gate

```bash
# CI / local (soak skipped)
SPANDA_TWIN_CLOUD_SKIP_SOAK=1 ./scripts/twin_cloud_stable_promotion_gate.sh

# Production promotion (requires soak file + elapsed days)
./scripts/twin_cloud_field_soak_init.sh   # once
SPANDA_TWIN_CLOUD_SKIP_SOAK=0 ./scripts/twin_cloud_stable_promotion_gate.sh
```

After promotion, update [feature-status.md](./feature-status.md) and [twin-cloud.md](./twin-cloud.md) to **Stable**.
