# Readiness Trend Analysis

**Status:** Experimental · **Phase:** Operate · **Priority:** P2.2

Predict readiness degradation from historical evaluations stored locally.

## Types

- `ReadinessHistory` — time-series of readiness snapshots
- `ReadinessTrend` — slope and volatility per factor
- `ReadinessForecast` — predicted score and risk window

## Storage

Local history file: `.spanda/readiness-history.json` (append on each `spanda readiness --record`).

Override path with `--history <path>` on record and trends commands.

## CLI

```bash
spanda readiness examples/showcase/readiness/rover.sd --record
spanda readiness trends examples/showcase/readiness/rover.sd
spanda readiness trends rover.sd --forecast 7d --json
spanda readiness trends rover.sd --history .spanda/readiness-history.json
```

## Output

- Overall and per-factor slope (score change per day)
- Volatility across recorded samples
- Forecasted score at horizon with policy risk warnings

## Integration

Extends `spanda-readiness` engine; feeds scorecard and deployment gate trend signals.

Smoke: `scripts/readiness_trends_smoke.sh`

See [readiness.md](./readiness.md) · [platform-maturity-roadmap.md](./platform-maturity-roadmap.md).
