# Decision Authority

**Status: Stable** — authority extraction and central-approval gates tested at runtime.

Each entity declares what it can decide locally versus what requires central approval.

## Types

| Type | Description |
|------|-------------|
| `DecisionAuthority` | Per-entity local vs central action lists |
| `DecisionScope` | Entity, fleet, site, and layer binding |
| `DecisionBoundary` | Hard limits no local layer may cross |
| `DecisionPolicy` | Versioned, signed policy with allowed/forbidden actions |
| `DecisionDelegation` | Temporary authority transfer between entities |
| `DecisionEscalation` | Step-up path when local resolution fails |

## Entity declaration

```sd
robot Rover001 {
    local_decision_authority [
        emergency_stop,
        obstacle_avoidance,
        degraded_mode,
        return_home,
        sensor_failover
    ];
    requires_central_approval [
        resume_high_risk_mission,
        override_safety_policy,
        disable_kill_switch,
        update_firmware
    ];
}
```

## Validation

The engine checks:

1. Action is in `local_decision_authority`
2. Action is **not** in `requires_central_approval`
3. Action passes `DecisionBoundary` checks (kill switch, firmware, unknown devices)
4. Action passes active `DecisionPolicy` rules

High-risk actions require **human approval**, **fleet quorum**, or **central approval**.

## CLI

```bash
spanda decision list mission.sd        # show all authorities
spanda decision inspect mission.sd --entity Rover001 --action emergency_stop
```

## API

`GET /v1/entities/{id}/decisions` returns authority evaluation for the entity.
