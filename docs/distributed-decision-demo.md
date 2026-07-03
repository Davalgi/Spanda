# Distributed Decision Demo

Flagship end-to-end demo for hierarchical distributed autonomy: **GPS loss recovery**.

**Status: Stable** — end-to-end demo with signed trees, persistent escalation, and v3 signed traces.

## Scenario

1. Robot executes patrol mission
2. GPS fails (health fault injection)
3. **Reflex layer** keeps robot safe (obstacle reflex tree)
4. **Local layer** switches to visual odometry via decision tree
5. Robot reduces speed and enters degraded mode
6. **Fleet layer** notified via health policy
7. **Control Center** records escalation (via trace)
8. Readiness recalculated (assurance + diagnostics)
9. Mission continues in degraded mode
10. Replay shows full decision trace
11. Assurance report includes recovery evidence

## Demo path

```
examples/showcase/distributed_decisions/gps_loss_recovery/mission.sd
```

## Commands

```bash
# Inspect decision architecture
spanda decision list examples/showcase/distributed_decisions/gps_loss_recovery/mission.sd
spanda decision inspect examples/showcase/distributed_decisions/gps_loss_recovery/mission.sd \
  --entity Rover001 --action degraded_mode \
  --signal "gps.status == Failed=true,visual_odometry.available=true"

# Simulate offline GPS loss scenario
spanda decision simulate examples/showcase/distributed_decisions/gps_loss_recovery/mission.sd \
  --offline --entity Rover001

# Run mission with decision trace
export SPANDA_DECISION_TRACE=1
spanda sim examples/showcase/distributed_decisions/gps_loss_recovery/mission.sd \
  --record --inject-health-faults

# Replay and audit
spanda replay examples/showcase/distributed_decisions/gps_loss_recovery/mission.trace
spanda decision trace examples/showcase/distributed_decisions/gps_loss_recovery/mission.trace
spanda audit decisions examples/showcase/distributed_decisions/gps_loss_recovery/mission.trace

# Assurance
spanda assure examples/showcase/distributed_decisions/gps_loss_recovery/mission.sd
```

## Attack simulation proof

```bash
spanda decision simulate-attack policy-tamper
spanda decision simulate-attack replayed-decision
spanda decision simulate-attack fake-coordinator
spanda decision simulate-attack offline-abuse
```

Each command blocks the unsafe decision and prints JSON evidence.

## Control Center

Open the **Decisions** tab:

1. Click **Run sim with traces**
2. Enable **Live trace on** for 3-second polling
3. View decision timeline with layer, rejected alternatives, escalation, safety/trust results

## Bundled demo

```bash
spanda demo distributed-decisions
```

## CI verification

The GPS loss demo is exercised in `./scripts/distributed_decisions_smoke.sh` and the `distributed-decisions` CI job.

## Stable vs experimental

| Component | Status |
|-----------|--------|
| Decision tree evaluation | **Stable** |
| Offline signed policy | **Stable** |
| Decision tree Ed25519 signing | **Stable** |
| v3 signed trace emission | **Stable** |
| Persistent escalation store | **Stable** |
| Runtime conflict resolution | **Stable** |
| Attack simulations | **Stable** |

## Related examples

| Example | Layer |
|---------|-------|
| `obstacle_reflex_stop/` | Reflex (Layer 0) |
| `gps_loss_local_recovery/` | Local (Layer 1) |
| `offline_mission_continue/` | Offline policy |
| `fleet_takeover_decision/` | Fleet (Layer 2) |
| `control_center_escalation/` | Control Center (Layer 3) |

See [distributed-decisions.md](./distributed-decisions.md) for full architecture.
