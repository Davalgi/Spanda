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
    policy_version = "1.0.0";
    signature = "<hex-ed25519-signature>";
    expires_at = 1735689600000;
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

When `SPANDA_DECISION_REQUIRE_SIGNED_OFFLINE_POLICY=1`, runtime verifies each offline policy signature against `SPANDA_DECISION_POLICY_TRUST_KEY` before permitting tree actions.

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
spanda decision sign-policy mission.sd --key "$SPANDA_DECISION_POLICY_SIGNING_KEY" --write-cache
spanda decision cache show [--cache <path>] [--json]
spanda decision cache sync mission.sd [--sign] [--key <hex>] [--cache <path>]
spanda decision cache clear [--policy <name>] [--cache <path>]
```

Signed policies persist to `.spanda/decision-policy-cache.json` (override with `SPANDA_DECISION_POLICY_CACHE`). Runtime merges cached signatures when program source omits the `signature` field.

## API

`POST /v1/decisions/simulate` with `"offline": true`.

`GET /v1/decision-policy-cache` — inspect the persisted signed policy cache.

`POST /v1/programs/simulation` with `"execute": true`, `"decision_trace": true`, and `"record_trace": true` — run a sim that emits v3 decision frames (Control Center **Run sim with traces** uses this).
