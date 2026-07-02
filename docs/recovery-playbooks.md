# Recovery Playbooks

Recovery playbooks are versioned, reusable multi-step recovery workflows managed by the Recovery Orchestrator.

## Built-in playbooks

| Playbook | Trigger | Steps |
|----------|---------|-------|
| `battery_low` | battery | Navigate to charger → assign backup → transfer mission → resume |
| `sensor_failure` | sensor | Retry → reinitialize → switch redundant sensor → degraded mode |
| `connectivity_loss` | connectivity | Retry → reconnect → switch gateway |
| `fleet_member_loss` | fleet | Detect → redistribute tasks → reassign missions |
| `mission_transfer` | mission | Pause → select backup → takeover with state transfer |

## TOML configuration

Add custom playbooks in `spanda.toml`:

```toml
[recovery.playbooks]
[[recovery.playbooks]]
name = "custom_overheat"
version = "1.0.0"
description = "Thermal emergency playbook"
trigger = "temperature"
entity_kinds = ["robot"]

[[recovery.playbooks.steps]]
order = 1
description = "Reduce workload"
strategy = "graceful_degradation"
escalation_level = 1
timeout_secs = 30

[[recovery.playbooks.steps]]
order = 2
description = "Safe shutdown if temperature critical"
strategy = "safe_shutdown"
escalation_level = 8
timeout_secs = 15
requires_validation = true
```

## CLI

```bash
spanda recovery playbooks --config examples/packages/basic_project
spanda recovery plan rover.sd --failure battery --playbook battery_low
```

## REST API

`GET /v1/recovery/playbooks`

## SDK

```rust
client.list_recovery_playbooks()?;
```

```python
client.list_recovery_playbooks()
```

```typescript
await client.listRecoveryPlaybooks();
```

## Plugin playbooks

Plugins can register playbooks via `RecoveryPluginRegistry` with extension kind `playbook`. See [plugin-api.md](./plugin-api.md).

## See also

- [recovery-orchestrator.md](./recovery-orchestrator.md)
- [recovery-simulation.md](./recovery-simulation.md)
