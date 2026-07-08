# Stable hardening — Recovery Orchestrator

Checklist for **Stable** tier promotion of the Recovery Orchestrator (`spanda-recovery`, REST
`/v1/recovery/*` — 14 routes, gRPC proto semver from `GET /v1/version` — currently **1.0.15**).

## Gate script

```bash
./scripts/recovery_orchestrator_stable_promotion_gate.sh
```

Runs:

1. `scripts/recovery_orchestrator_smoke.sh` — crate, REST API, gRPC, and CLI (including `recovery
   explain`)
2. Control Center probe — `GET /v1/recovery/playbooks`, `GET /v1/recovery/history`, `POST
   /v1/recovery/plan`, `GET /v1/recovery/predictive`, `GET /v1/recovery/recoverable-entities`, `POST
   /v1/recovery/recommend`

Skip smoke only: `SPANDA_RECOVERY_SKIP_SMOKE=1
./scripts/recovery_orchestrator_stable_promotion_gate.sh`

Skip field soak timer in CI: `SPANDA_RECOVERY_SKIP_SOAK=1` (default in
`.github/workflows/ci-nightly.yml` job `recovery-orchestrator-stable-promotion-gate`).

### Field soak (organizational gate)

Start the 30-day clock once per deployment:

```bash
./scripts/recovery_orchestrator_field_soak_init.sh
```

Soak file default: `.spanda/recovery-field-soak-start.txt`
(`SPANDA_RECOVERY_FIELD_SOAK_START_FILE`). See [field-soak-gate.md](./field-soak-gate.md).

## Smoke (CI)

```bash
./scripts/recovery_orchestrator_smoke.sh
```

## Test locations

| Layer | Location |
|-------|----------|
| Orchestrator unit tests | `crates/spanda-recovery/tests/orchestrator_tests.rs` |
| REST contract tests | `crates/spanda-api/tests/recovery_api_tests.rs` (plan, simulate, explain, predictive, recoverable-entities, recommend) |
| History persistence | `crates/spanda-api/tests/tenant_persistence_tests.rs` (`recovery_history_persists_across_restart`) |
| OpenAPI parity | `crates/spanda-api/tests/openapi_parity_tests.rs` |
| gRPC parity | `crates/spanda-api/tests/grpc_tests.rs` (`grpc_recovery_endpoints_with_self_healing_program`) |
| Legacy self-healing | `scripts/self_healing_smoke.sh` |
| Plugin example | `examples/plugins/recovery-plugin/` |

## Backward compatibility

- `spanda heal`, `spanda recover`, `POST /v1/programs/recovery/heal` unchanged
- Orchestrator wraps assurance — no API breaks

## Persistence

Recovery evidence history is written to `control-center-recovery.json` under
`SPANDA_CONTROL_CENTER_STATE_DIR` after orchestrator execute. See
[recovery-validation-report.md](./recovery-validation-report.md).

## See also

- [recovery-orchestrator.md](./recovery-orchestrator.md)
- [recovery-api.md](./recovery-api.md)
- [recovery-validation-report.md](./recovery-validation-report.md)
- [test-plan.md](./test-plan.md)
