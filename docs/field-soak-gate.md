# Field soak gate (30-day)

Enterprise operations promotion to **Stable** requires a **30-day field pilot** without data-loss
regressions. Use this gate before updating `docs/feature-status.md` or
`docs/stable-hardening-enterprise-ops.md`.

## Start the soak clock

```bash
mkdir -p .spanda
date -u +%Y-%m-%d > .spanda/field-soak-start.txt
```

Or use the init helper:

```bash
./scripts/enterprise_ops_field_soak_init.sh
```

Commit the start date to your pilot branch or store it in fleet configuration management.

## Run the gate

```bash
chmod +x scripts/field_soak_gate.sh
./scripts/field_soak_gate.sh
```

For the full **Stable promotion** workflow (soak + audit prep + smokes), see
[enterprise-ops-stable-promotion.md](./enterprise-ops-stable-promotion.md) and
`./scripts/enterprise_ops_stable_promotion_gate.sh`.

### Environment

| Variable | Default | Purpose |
|----------|---------|---------|
| `SPANDA_FIELD_SOAK_START_FILE` | `.spanda/field-soak-start.txt` | UTC start date (`YYYY-MM-DD`) |
| `SPANDA_FIELD_SOAK_MIN_DAYS` | `30` | Minimum elapsed days |

## What the gate checks

1. Soak start file exists and is at least 30 days old.
2. `scripts/enterprise_ops_smoke.sh` passes.
3. `scripts/failover_drill_smoke.sh` passes (when present).
4. `scripts/ota_fleet_soak.sh` quick mode passes (when present).

## CI integration

Add `./scripts/field_soak_gate.sh` to your fleet promotion pipeline after the soak period. Until the
clock elapses, the script exits non-zero by design.

For **Human Interaction** (separate 30-day clock), use `.spanda/hri-field-soak-start.txt` and
`./scripts/hri_stable_promotion_gate.sh` — see
[stable-hardening-human-interaction.md](./stable-hardening-human-interaction.md). Start the clock
with `./scripts/hri_field_soak_init.sh` (one-time).

For **Recovery Orchestrator** (separate clock), use `.spanda/recovery-field-soak-start.txt` and
`./scripts/recovery_orchestrator_stable_promotion_gate.sh` — see
[stable-hardening-recovery-orchestrator.md](./stable-hardening-recovery-orchestrator.md). Start with
`./scripts/recovery_orchestrator_field_soak_init.sh` (one-time).

For **Autonomous Entity Mesh** (separate clock), use `.spanda/entity-mesh-field-soak-start.txt` and
`./scripts/entity_mesh_stable_promotion_gate.sh` — see
[entity-mesh-stable-promotion.md](./entity-mesh-stable-promotion.md). Start the pilot with
`./scripts/entity_mesh_field_soak_init.sh` (one-time). Weekly smoke:
[entity-mesh-field-pilot.md](./entity-mesh-field-pilot.md).
