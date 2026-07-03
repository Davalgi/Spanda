# Autonomy Levels

Autonomy level is a first-class entity attribute describing how much decision authority an autonomous system holds.

## Levels (SAE-inspired)

| Level | Name | Human role |
|-------|------|------------|
| 0 | `manual` | Full human control |
| 1 | `assisted` | Driver/operator assistance |
| 2 | `partial_automation` | Human monitors automation |
| 3 | `conditional_autonomy` | System drives within ODD; human fallback |
| 4 | `high_autonomy` | System drives within ODD; human not required |
| 5 | `full_autonomy` | No human intervention expected |

## Platform influence

Autonomy level affects governance validation for:

| Domain | Influence |
|--------|-----------|
| Decision authority | Max level capped by deployment profile |
| Recovery | Higher autonomy requires automated recovery policies |
| Human approval | Levels 0–3 require responsible person |
| Safety validation | Level 3+ requires trusted entity posture |
| Trust | Minimum trust tier escalates with level |
| Mission planning | Approval workflows scale with level |
| Readiness | High autonomy missions require readiness gates |

## Configuration

```toml
[governance]
autonomy_level = "conditional_autonomy"
```

Per-robot override via metadata:

```toml
# spanda.devices.toml metadata or device tree
[governance]
autonomy_level = "partial_automation"
```

## Parsing

CLI and API accept flexible aliases: `level_3`, `l3`, `3`, `conditional`, `conditional_autonomy`.

## Deployment profile interaction

Each deployment profile sets `max_autonomy_level`. Governance validation warns when entity autonomy exceeds the profile maximum.
