# Recovery validation report

Formal validation envelope produced by the Recovery Orchestrator dry-run and execute paths.

## Promotion gate

```bash
./scripts/recovery_orchestrator_stable_promotion_gate.sh
```

CI job: `recovery-orchestrator-stable-promotion-gate` in `.github/workflows/ci.yml`.

Field soak (30-day organizational gate):

```bash
./scripts/recovery_orchestrator_field_soak_init.sh
```

## Sources

| Layer | Validation |
|-------|------------|
| Assurance gates | `validate_recovery_plan`, readiness evaluation |
| Orchestrator | Health, capability, hardware, readiness, trust, security, mission gates |
| Plugin validators | Registered `[recovery.extensions]` kind `validator` require safe plans |
| Evidence | Immutable `OrchestratorRecoveryEvidence` records in history |

## CLI

```bash
spanda recovery validate rover.sd --failure gps
spanda recovery dry-run rover.sd --entity Rover --failure gps
```

## REST

```bash
curl -s -X POST http://127.0.0.1:8787/v1/recovery/validate \
  -H 'Content-Type: application/json' \
  -d '{"failure":"gps"}'
```

Response includes `report` and `validation` evidence arrays.

## Persistence

Evidence history is stored in `control-center-recovery.json` under `SPANDA_CONTROL_CENTER_STATE_DIR` and survives Control Center restart. Verified by `recovery_history_persists_across_restart` in `tenant_persistence_tests.rs`.

## Predictive and recommend endpoints

```bash
curl -s http://127.0.0.1:8787/v1/recovery/predictive
curl -s http://127.0.0.1:8787/v1/recovery/recoverable-entities
curl -s -X POST http://127.0.0.1:8787/v1/recovery/recommend \
  -H 'Content-Type: application/json' \
  -d '{"failure":"gps_loss"}'
```

## See also

- [recovery-orchestrator.md](./recovery-orchestrator.md)
- [recovery-assurance.md](./recovery-assurance.md)
- [stable-hardening-recovery-orchestrator.md](./stable-hardening-recovery-orchestrator.md)
