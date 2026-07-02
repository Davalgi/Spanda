# Digital Mission Twin

**Status:** Stable · **Horizon:** LATER (v0.7) · **Priority:** P2

Maintain a digital representation of mission feasibility — readiness, assurance, and mission contract alignment.

## CLI

```bash
spanda twin mission examples/showcase/mission_twin/patrol.sd
spanda twin mission patrol.sd --json
```

## Core types

| Type | Purpose |
|------|---------|
| `MissionTwin` | Live mission state mirror |
| `MissionStateModel` | Progress, checkpoints, objectives |
| `MissionRiskModel` | Active risks and forecasts |
| `MissionForecast` | Projected completion and degradation |

## Integration

Extends existing `twin` blocks and `spanda-readiness` twin module. Local twin in core; cloud sync via **`spanda-twin-cloud`** crate and Twin Cloud SaaS REST (`/v1/twins/*`) — see [twin-cloud.md](./twin-cloud.md).

Showcase: `examples/showcase/mission_twin/patrol.sd` · `spanda demo mission-twin` · `scripts/later_differentiation_smoke.sh`.

Feeds What-If (NEXT) and Mission Time Travel (LATER).

See [differentiation-roadmap.md](./differentiation-roadmap.md) · [replay.md](./replay.md).
