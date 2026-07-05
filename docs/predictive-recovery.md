# Predictive Recovery

The Recovery Orchestrator integrates telemetry to detect early failure indicators and trigger
preventative recovery.

## Supported indicators

| Indicator | Severity | Recommended action |
|-----------|----------|-------------------|
| Memory leak | warning | `restart_component` |
| Increasing latency | warning | `graceful_degradation` |
| CPU spike | warning | `restart_component` |
| Battery degradation | warning | `graceful_degradation` |
| Temperature increase | critical | `safe_shutdown` |
| Connectivity instability | warning | `reconnect` |
| Packet loss | warning | `switch_network` |
| Sensor drift | warning | `reinitialize` |
| Repeated retries | warning | `restart_component` |
| Crash frequency | critical | `restart_robot` |
| Health degradation | warning | `graceful_degradation` |

## API

```rust
let (indicators, trigger) = orchestrator.check_predictive(&registry, telemetry.as_ref());
if trigger {
    // preventative recovery recommended
}
```

### REST

```bash
# Global indicators (no program context)
curl -s http://127.0.0.1:8080/v1/recovery/predictive

# With optional telemetry in POST body
curl -s -X POST http://127.0.0.1:8080/v1/recovery/predictive \
  -H 'Content-Type: application/json' \
  -d '{"file":"rover.sd","entity_id":"Rover"}'
```

Response envelope: `{ "version": "v1", "indicators": [...], "should_trigger_preventative": bool }`.

### Knowledge-base recommendation

```bash
curl -s -X POST http://127.0.0.1:8080/v1/recovery/recommend \
  -H 'Content-Type: application/json' \
  -d '{"failure":"gps_loss"}'
```

### SDK (0.5.6+)

```rust
client.get_recovery_predictive(None)?;
client.recommend_recovery(&json!({ "failure": "gps_loss" }))?;
```

```python
client.get_recovery_predictive()
client.recommend_recovery({"failure": "gps_loss"})
```

```typescript
await client.getRecoveryPredictive();
await client.recommendRecovery({ failure: "gps_loss" });
```

## Prognostics integration

Program-level `prognostics` declarations in Spanda source complement telemetry scanning. See
[self-healing.md](./self-healing.md) and assurance prognostics.

## Learning

Historical recovery outcomes feed the rule-based knowledge base (success rates, strategy
effectiveness). No machine learning in Phase 2 — statistics and rules only.

## See also

- [recovery-orchestrator.md](./recovery-orchestrator.md)
- [recovery-playbooks.md](./recovery-playbooks.md)
