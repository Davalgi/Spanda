# Offline Decisions

Entities maintain a **local policy cache** so they can operate safely when disconnected from the control center.

## Local policy cache

Cached on each edge entity:

- Safety rules
- Recovery playbooks
- Mission constraints
- Trust policy version
- Capability requirements
- Approval rules

## Offline policy syntax

```sd
offline_policy RoverOffline {
    max_duration = 30 min;
    allowed_actions [
        continue_current_safe_mission,
        return_home,
        pause_mission,
        enter_degraded_mode
    ];
    forbidden_actions [
        start_new_high_risk_mission,
        disable_safety,
        accept_unknown_device
    ];
}
```

## Offline rules

When offline, entities:

- Use **last-known-valid signed policy**
- **Expire** after `max_duration`
- **Forbid** high-risk actions (new missions, safety disable, unknown devices, firmware updates)
- **Require audit sync** when reconnected

## Simulation

```bash
spanda decision simulate mission.sd --offline
spanda decision inspect mission.sd --offline-minutes 20 --action return_home
```

## API

`POST /v1/decisions/simulate` with `"offline": true`.
