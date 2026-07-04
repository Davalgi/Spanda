# Operational Governance Framework

Spanda's **Operational Governance Framework** extends the platform with abstractions for how autonomous systems are designed, validated, deployed, operated, audited, and maintained — without becoming a generic GRC product or a repository of regulatory text.

> **Disclaimer:** Spanda provides governance abstractions and validation mechanisms. It does **not** provide legal or regulatory advice.

## Platform service

The `spanda-governance` crate (`crates/spanda-governance/`) implements:

| Responsibility | Integration |
|----------------|-------------|
| Standards awareness | `StandardsProfileRef` via packages/plugins |
| Compliance validation | Composes with `spanda-compliance`, `spanda-readiness`, `spanda-trust` |
| Deployment governance | `DeploymentProfile` + entity metadata |
| Operational policies | `GovernancePolicyRef` (versioned, signed, auditable) |
| Certification tracking | Independent lifecycle from health |
| Risk assessment | `OperationalRisk` influencing decision/recovery |
| Audit support | `spanda-audit` + evidence export |
| Human accountability | `HumanAccountability` on missions/deployments |

## Architecture

```
spanda.governance.toml  →  ResolvedSystemConfig  →  EntityRegistry
                                                        ↓
                                              EntityGovernanceMeta
                                                        ↓
                    spanda-governance::evaluate_entity_governance
                                                        ↓
              CLI / REST / SDK / Control Center governance views
```

Governance does **not** duplicate readiness, assurance, trust, or health engines. It composes them through the unified entity model and validation orchestration.

## Entity extensions

Every [`EntityRecord`](../crates/spanda-config/src/entity.rs) may expose optional governance via `governance: Option<EntityGovernanceMeta>`:

- Autonomy level
- Deployment profile
- Operational maturity
- Certification status
- Risk level
- Governance policies
- Responsible owner
- Standards profiles
- Operational constraints

Configure system-wide defaults in `spanda.governance.toml` (auto-loaded when present, or via `[config] governance = "spanda.governance.toml"`). Override per-robot in device tree metadata.

## CLI

```bash
spanda compliance check [--strict] [--entity <id>] [--json]
spanda governance validate [--strict] [--json]
spanda governance report [--json]
spanda certification list|inspect|report [entity-id] [--json]
spanda deployment profile [name] [--json]
spanda deployment verify [--strict] [--json]
spanda risk report [--json]
```

## REST API

| Method | Path | Description |
|--------|------|-------------|
| GET | `/v1/governance` | Framework summary |
| GET | `/v1/compliance` | Compliance posture summary |
| POST | `/v1/compliance/check` | Run compliance check |
| POST | `/v1/governance/validate` | Validate governance configuration |
| GET | `/v1/certifications` | List entity certification records |
| GET | `/v1/deployment-profiles` | List or fetch deployment profile |
| GET | `/v1/risk` | Operational risk rollup |
| GET | `/v1/certifications/report` | Certification report |
| POST | `/v1/deployment/verify` | Deployment verification |
| GET | `/v1/governance/policies` | Policy assignments + audit |
| POST | `/v1/governance/policies/assign` | Assign/sign policy to entity |
| GET | `/v1/governance/audit` | Governance audit history |
| GET | `/v1/governance/accountability` | Responsible owners |

## gRPC (proto 1.0.12)

| RPC | REST parity |
|-----|-------------|
| `GetGovernance` | `GET /v1/governance` |
| `GetCompliance` | `GET /v1/compliance` |
| `CheckCompliance` | `POST /v1/compliance/check` |
| `ValidateGovernance` | `POST /v1/governance/validate` |
| `ListCertifications` | `GET /v1/certifications` |
| `ListDeploymentProfiles` | `GET /v1/deployment-profiles` |
| `GetDeploymentProfile` | `GET /v1/deployment-profiles?name=` |
| `GetOperationalRisk` | `GET /v1/risk` |

## SDK

Rust, Python, and TypeScript SDKs expose dedicated clients:

- `GovernanceClient` — framework summary, validate
- `ComplianceClient` — summary, check
- `CertificationClient` — list certifications
- `DeploymentProfileClient` — list/get profiles
- `RiskClient` — risk summary

## Control Center

The **Governance** tab (Governance & config group) shows deployment profiles, compliance validation, certification status, operational risk, and accountability fields.

## Related guides

- [Compliance framework](./compliance-framework.md)
- [Certification lifecycle](./certification-lifecycle.md)
- [Deployment profiles](./deployment-profiles.md)
- [Autonomy levels](./autonomy-levels.md)
- [Risk model](./risk-model.md)
- [Human accountability](./human-accountability.md)
- [Standards profiles](./standards-profiles.md)
- [Migration report](./governance-migration.md)
