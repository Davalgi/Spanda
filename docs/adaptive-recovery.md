# Adaptive Recovery

**Status: Experimental** — rule-based statistics, not ML.

## Purpose

Improve recovery strategy selection using historical outcomes.

## Types

`RecoveryConfidence`, `RecoveryHistory`, `StrategySuccessRate`, `StrategyPreference`, `AdaptiveRecoveryPolicy`

## Examples

- Camera reconnect usually succeeds → prefer reconnect
- Provider restart repeatedly fails → escalate sooner
- Robot replacement succeeds faster than retry → prefer takeover

## CLI

```bash
spanda recovery confidence
spanda recovery learning-report
```

Integrates with [recovery orchestrator](./recovery-orchestrator.md).
