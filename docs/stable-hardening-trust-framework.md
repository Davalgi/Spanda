# Trust Framework — Stable Hardening Checklist

**Promoted 2026-07-02** — signature capability #5 (composite trust scoring).

**Related:** [trust-framework.md](./trust-framework.md) · [package-trust.md](./package-trust.md)

| Gate | Status |
|------|--------|
| `spanda trust <file>` | **Shipped** |
| REST `GET /v1/trust/program` | **Shipped** |
| gRPC `GetTrustProgram` | **Shipped** |
| Package / mission integrity | `trust_showcase_smoke.sh` |
| Crate tests | `spanda-trust` composite + entity trust |

```bash
SPANDA_TRUST_FRAMEWORK_SKIP_SOAK=1 SPANDA_TRUST_FRAMEWORK_SKIP_AUDIT=1 SPANDA_TRUST_FRAMEWORK_SKIP_SMOKE=1 ./scripts/trust_framework_stable_promotion_gate.sh
```

Field soak: `./scripts/trust_framework_field_soak_init.sh`  
Audit prep: `./scripts/trust_framework_security_audit_prep.sh`

**Note:** Trust **Graph** (NEXT pillar #9) is a separate capability — see [stable-hardening-trust-graph.md](./stable-hardening-trust-graph.md).
