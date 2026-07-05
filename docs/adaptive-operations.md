# Adaptive Operations

**Functional domain:** [Adaptive Learning](./functional-domains.md#adaptive-learning)  
**Status: Experimental** — rule-based statistics and confidence updates; **no ML dependency**.

> Canonical architecture: [cognitive-resilience-architecture.md](./cognitive-resilience-architecture.md)  
> Recovery learning detail: [adaptive-recovery.md](./adaptive-recovery.md)

## Purpose

Improve operational decisions using **historical outcomes**. Adaptive operations cover recovery strategy selection, confidence updates, and historical recommendations — implemented as transparent rules and statistics, not opaque models.

## Capabilities (initial)

| Capability | Implementation |
|------------|----------------|
| Rule-based adaptation | `AdaptiveRecoveryPolicy` success-rate thresholds |
| Statistics | `StrategySuccessRate`, rolling attempt counts |
| Confidence updates | `RecoveryConfidence.score` from history |
| Historical recommendations | `POST /v1/recovery/recommend` |

## Entity integration

| Field | Description |
|-------|-------------|
| `Entity.recovery_confidence` | `EntityRecoveryConfidence` — `score`, `preferred_strategy`, `attempts` |

## Types (`spanda-autonomy`)

`RecoveryConfidence`, `RecoveryHistory`, `StrategySuccessRate`, `StrategyPreference`, `AdaptiveRecoveryPolicy`

## Examples

- Camera reconnect succeeds 3/3 times → prefer reconnect strategy
- Provider restart fails repeatedly → escalate sooner in recommend
- Robot replacement faster than retry → prefer takeover in continuity
- Fusion confidence low after recovery → lower recovery confidence score

## CLI

```bash
spanda recovery confidence
spanda recovery learning-report
```

## API

| Surface | Endpoint |
|---------|----------|
| Recovery metrics | `GET /v1/recovery/metrics` → `recovery_confidence` |
| Recovery recommend | `POST /v1/recovery/recommend` |
| Entity autonomy | `GET /v1/entities/{id}/autonomy` → `recovery_confidence` |

## Control Center

**Recovery Confidence** metric in the Cognitive & Resilience tab.

## Integrations

- **Recovery Orchestrator:** history feeds `compute_recovery_confidence()`
- **Attention Engine:** failed recovery patterns may escalate attention
- **Operational Memory:** episodic recovery history in playbooks/traces

## Future (not required now)

ML-backed strategy selection may arrive via official packages — the functional domain boundary remains the same; only the recommendation backend would change.

See [responsibility-matrix.md](./responsibility-matrix.md#adaptive-learning).
