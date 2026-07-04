# Operational Governance — Migration & Compatibility

## Summary

Operational governance is **additive**. Existing projects continue to work without `spanda.governance.toml`. Governance fields are optional on all entities.

## What changed

| Area | Change | Breaking? |
|------|--------|-----------|
| `EntityRecord` | New optional `governance` field | No — serde default |
| `spanda-config` | `apply_governance_metadata()` stamps fleet/robots | No |
| `ResolvedSystemConfig` | New `governance_config()` accessor | No |
| REST API | New `/v1/governance`, `/v1/compliance`, `/v1/certifications`, `/v1/deployment-profiles`, `/v1/risk` | No |
| gRPC | Proto **1.0.12** adds 8 operational governance RPCs | No (additive) |
| Config | `spanda.governance.toml` auto-load + `[config] governance` fragment | No |
| Runtime | Readiness, trust, deployment gates, decisions, recovery consume governance influence | No |
| Policies | Signed assignment store + audit trail (`/v1/governance/policies`) | No |
| Certification | Persistent record store + `certification report` | No |
| CLI | New subcommands under `compliance`, `governance`, `certification`, `deployment`, `risk` | No |
| SDK | New client types (additive methods) | No |
| Control Center | New **Governance** tab | No |
| Crate | New `spanda-governance` workspace member | No |

## Metadata compatibility

Governance uses `governance.*` metadata keys alongside existing `compliance.profile`, `assurance.profile`, etc. No key collisions.

| Legacy key | Governance equivalent |
|------------|----------------------|
| `compliance.profile` | Unchanged — program/industry compliance |
| `owner` | Maps to `governance.responsible_person` when governance section present |
| Device `compliance_profile` | Complements `governance.deployment_profile` |

## Existing services reused

| Service | Role in governance |
|---------|-------------------|
| Entity model | Canonical projection |
| Readiness | Readiness gates in validation |
| Assurance | Mission assurance composition |
| Trust | Autonomy/trust cross-checks |
| Health | Critical health findings |
| Compliance | Program profile evaluation |
| Policy | Operational policy blocks (`.sd`) |
| Certify | Runtime certification proofs |
| Audit | Mutation and evidence trail |
| Risk (`spanda-risk`) | Mission risk scoring (complementary) |

## Migration steps

1. Add `spanda.governance.toml` to project root (optional).
2. Set `deployment_profile`, `operational_maturity`, `risk_level`, `autonomy_level`.
3. Assign `responsible_person` and `deployment_owner`.
4. Run `spanda governance validate` before promoting maturity stage.
5. Run `spanda compliance check` before production deployment.
6. Wire SDK/CI: `POST /v1/governance/validate` in deploy pipelines.

## Example config fragment

```toml
[governance]
autonomy_level = "partial_automation"
deployment_profile = "warehouse"
operational_maturity = "pilot"
certification_status = "validated"
risk_level = "medium"
responsible_person = "ops@example.com"
deployment_owner = "fleet@example.com"
standards_profiles = "industrial_safety"
constraints = "indoor,connectivity"
```

## Version

Introduced in workspace **0.6.2** (operational governance framework milestone).
