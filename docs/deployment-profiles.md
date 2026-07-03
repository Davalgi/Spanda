# Deployment Profiles

Deployment profiles define the **operational context** for autonomous systems: safety policies, recovery policies, risk levels, required capabilities, hardware, certifications, decision authority, and environmental constraints.

## Built-in profiles

| Profile | Typical risk | Max autonomy |
|---------|--------------|--------------|
| `warehouse` | medium | partial_automation |
| `factory` | high | partial_automation |
| `hospital` | life_critical | assisted |
| `operating_room` | life_critical | assisted |
| `search_rescue` | mission_critical | conditional_autonomy |
| `agriculture` | medium | partial_automation |
| `mining` | high | partial_automation |
| `construction` | high | partial_automation |
| `maritime` | high | conditional_autonomy |
| `aviation` | mission_critical | conditional_autonomy |
| `space` | mission_critical | high_autonomy |
| `road_vehicle` | life_critical | conditional_autonomy |
| `campus` | low | partial_automation |
| `smart_building` | low | partial_automation |
| `home` | low | assisted |
| `retail` | low | partial_automation |
| `office` | low | partial_automation |
| `defense` | mission_critical | conditional_autonomy |
| `research` | medium | partial_automation |

Custom profiles use `Custom("name")` and can be supplied via packages.

## Profile structure

Each profile defines:

- Safety and recovery policy references
- Default risk level
- Required capabilities and hardware
- Required certification identifiers (references, not embedded standards)
- Decision authority rules (max autonomy, approval thresholds)
- Communication constraints (offline, latency, bandwidth)
- Environmental constraints (temperature, humidity, hazards)
- Operational constraint tags
- Applicable standards profile references

## Usage

```toml
[governance]
deployment_profile = "warehouse"
```

```bash
spanda deployment profile warehouse
spanda deployment profile --json
```

## API

- `GET /v1/deployment-profiles` — list all profiles
- `GET /v1/deployment-profiles?name=warehouse` — profile detail

## Package extension

Profiles can be extended or overridden via registry packages implementing the deployment profile schema — keeping the platform modular and package-driven.
