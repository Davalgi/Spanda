# Operational Risk Model

Operational risk describes the **severity of harm or mission impact** if an autonomous system fails
or acts incorrectly.

## Risk tiers

| Tier | Description |
|------|-------------|
| `negligible` | No meaningful impact |
| `low` | Minor inconvenience |
| `medium` | Operational disruption |
| `high` | Significant harm or loss |
| `critical` | Major safety or business impact |
| `life_critical` | Potential loss of life |
| `mission_critical` | Mission failure unacceptable |

## Platform influence

| Domain | Influence |
|--------|-----------|
| Decision authority | High+ risk requires approval chain |
| Human approval | `high` and above require explicit approval |
| Recovery | Escalation contacts required at high tiers |
| Readiness | Live deployment blocked when readiness fails at high risk |
| Simulation | Medium+ risk requires simulation maturity |
| Assurance | Composes with mission assurance reports |

## Configuration

```toml
[governance]
risk_level = "high"
```

Deployment profiles provide `default_risk_level` baselines; entities inherit unless overridden.

## Commands

```bash
spanda risk report [--json]
spanda risk mission.sd          # mission deployment risk (spanda-risk)
```

## API

`GET /v1/risk` returns governed entities with risk metadata and health posture.

## Relationship to mission risk

- **Operational risk** (`spanda-governance`) — deployment context and entity governance
- **Mission risk** (`spanda-risk`) — program-level deployment scoring

Both compose in governance reports and executive analytics.
