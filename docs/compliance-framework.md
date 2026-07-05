# Compliance Framework

Spanda's compliance framework validates autonomous systems against **industry profiles** and
**operational governance** requirements — not embedded regulatory text.

## Layers

1. **Program compliance** (`spanda-compliance`) — evaluate `.sd` programs against signed industry
   profiles (defense, medical, automotive, …).
2. **Operational governance** (`spanda-governance`) — validate deployment context: autonomy,
   maturity, certification, risk, accountability.
3. **Readiness & assurance** — existing engines provide health/readiness/trust inputs to governance
   validation.

## Validation outcomes

Compliance and governance checks produce:

| Outcome | Meaning |
|---------|---------|
| **Pass** | Requirement satisfied |
| **Warning** | Gap that may be acceptable depending on maturity |
| **Missing** | Required attribute or evidence absent |
| **Recommended action** | Suggested remediation |

## Commands

```bash
spanda compliance list
spanda compliance check [--strict] [--entity robot:alpha]
spanda compliance report mission.sd --profile defense
```

## API

```http
POST /v1/compliance/check
Content-Type: application/json

{"strict": true, "entity_id": "robot:alpha"}
```

## Configuration

```toml
# spanda.governance.toml
[governance]
deployment_profile = "warehouse"
operational_maturity = "pilot"
risk_level = "medium"
certification_status = "validated"
responsible_person = "ops.lead@example.com"
deployment_owner = "fleet.ops@example.com"
standards_profiles = "industrial_safety,cybersecurity"
constraints = "indoor,connectivity"
```

## Disclaimer

Spanda validates structural governance requirements. Organizations remain responsible for regulatory
interpretation and legal compliance.
