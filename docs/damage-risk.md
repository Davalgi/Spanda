# Damage Risk Assessment

**Functional domain:** [Damage Risk Assessment](./functional-domains.md#damage-risk-assessment)  
**Status: Beta** — harm-risk index on entity autonomy profiles from health, trust, and metadata
signals.

> Canonical architecture:
> [cognitive-resilience-architecture.md](./cognitive-resilience-architecture.md)
> Implementation detail: [damage-risk-model.md](./damage-risk-model.md)

## Purpose

Evaluate **potential harm** — not just errors. Damage risk models consequences: asset damage,
operator injury, mission failure, and environmental harm. This drives preventative action, mission
abort, degraded mode, and maintenance scheduling.

Spanda does not model pain or biological harm — `SafetyPainIndex` is an engineering severity scalar.

## Risk dimensions

| Dimension | Examples |
|-----------|----------|
| Asset risk | Motor overheating, brake degradation, battery swelling |
| Operator risk | Fatigue, fall detection, HRI proximity |
| Mission risk | Abort thresholds, continuity failure |
| Environmental risk | Hazard zone breach, geofence violation |

## Entity integration

| Field | Description |
|-------|-------------|
| `Entity.damage_risk` | `EntityDamageRisk` — `index`, `risk_signals`, `protective_action` |

## Types (`spanda-autonomy`)

`DamageRisk`, `HarmPotential`, `ProtectiveAction`, `RiskSignal`, `SafetyPainIndex`

## Actions on elevated risk

| Risk level | Typical action |
|------------|----------------|
| Elevated | Operator alert, attention boost |
| High | Degraded mode, reduced autonomy |
| Critical | Mission abort, emergency reflex, recovery plan |

## CLI

Evaluated via entity autonomy enrichment and dedicated risk APIs:

```bash
spanda entity list --json   # includes autonomy.damage_risk when enriched
```

Platform risk summary: SDK `RiskClient::summary()` → `GET /v1/risk`

## API

| Surface | Endpoint |
|---------|----------|
| Entity autonomy | `GET /v1/entities/{id}/autonomy` → `damage_risk` |
| Governance risk | `GET /v1/risk` |
| SDK | `RiskClient` |

## Control Center

**Damage Risk** section in the Cognitive & Resilience tab (entity autonomy profile).

## Integrations

- **Strategic Planning:** mission planner considers risk index for abort/replan
- **Reflex & Safety:** critical risk triggers protective reflex selection
- **Recovery:** protective actions map to recovery playbooks

See [responsibility-matrix.md](./responsibility-matrix.md#damage-risk-assessment).
