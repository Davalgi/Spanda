# Mission Time Travel

**Status:** Stable · **Horizon:** LATER (v0.7) · **Priority:** P2

Replay mission state at any point in time for incident investigation.

## CLI

```bash
spanda replay examples/showcase/differentiation/decision_trail/main.trace --at T+00:01 --inspect decisions
spanda replay mission.trace --at 2026-06-24T14:32:00Z
spanda replay mission.trace --at T+01:30 --inspect health|readiness|safety|all --json
```

Golden fixture: `examples/showcase/differentiation/decision_trail/main.trace`.

## Core types

`MissionTimeTravel`, `HistoricalMissionState`, `TimelineExplorer`.

## Capabilities

- Inspect robot/mission state at timestamp
- Inspect decisions (requires Decision Audit Trail v3 traces)
- Inspect health, readiness, and safety posture at point in time

Extends [replay.md](./replay.md) with state snapshots and decision records embedded in trace v3.

See [differentiation-roadmap.md](./differentiation-roadmap.md) ·
[decision-audit-trail.md](./decision-audit-trail.md).
