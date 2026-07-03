# Stable Hardening — Distributed Decisions

Promotion checklist for **Distributed Decisions** — **Stable**.

## Stable capabilities

| Capability | Evidence |
|------------|----------|
| Runtime conflict resolution | `evaluate_live_decision_trees` collects competing decisions and calls `resolve_conflict`; safety reflex wins split-brain |
| Persistent escalation approvals | `.spanda/decision-escalations.json`; `POST /v1/decisions/escalate`; Control Center Approve button |
| Ed25519 decision tree signing | `spanda decision sign-tree`; `verify_decision_tree_signature`; cache merge via `PersistedPolicyCache.decision_trees` |
| Persisted nonce replay protection | `.spanda/decision-nonce-registry.json`; shared mesh registry via `POST /v1/fleet/decisions/nonce/register` |
| v3 envelope crypto signatures | `decision_envelope_signing_payload` + Ed25519 via `SPANDA_DECISION_POLICY_SIGNING_KEY`; pluggable HSM via `SPANDA_CRYPTO_BACKEND` |
| Fleet mesh conflict aggregation | `POST /v1/fleet/decisions/vote/ingest`, `GET /v1/fleet/decisions/conflicts`; per-robot vote posting at runtime |
| Rule enforcement tests | `tests/rule_enforcement.rs`, `tests/attack_simulations.rs`, `tests/stable_gaps.rs` |
| Mesh attack simulation | `spanda decision simulate-attack split-brain-mesh` |
| CI gate | `distributed-decisions` job + `./scripts/distributed_decisions_smoke.sh` |

## Verification commands

```bash
cargo test -p spanda-decision
cargo test -p spanda-decision --test stable_gaps
cargo test -p spanda-interpreter --test decision_runtime
cargo test -p spanda-fleet mesh_coordinator_resolves_decision_conflicts_and_shared_nonce
./scripts/distributed_decisions_smoke.sh
spanda decision simulate-attack split-brain-mesh
```

## Environment variables

| Variable | Purpose |
|----------|---------|
| `SPANDA_DECISION_POLICY_SIGNING_KEY` | Sign offline policies, decision trees, v3 envelopes |
| `SPANDA_DECISION_POLICY_TRUST_KEY` | Verify signed policies, trees, envelopes |
| `SPANDA_DECISION_REQUIRE_SIGNED_OFFLINE_POLICY` | Require signed offline policies |
| `SPANDA_DECISION_REQUIRE_SIGNED_TREES` | Require signed decision trees |
| `SPANDA_DECISION_POLICY_CACHE` | Signed policy/tree cache path |
| `SPANDA_DECISION_NONCE_CACHE` | Local nonce replay registry path |
| `SPANDA_DECISION_NONCE_MESH_URL` | Shared nonce mesh URL (falls back to `SPANDA_FLEET_MESH_URL`) |
| `SPANDA_DECISION_NONCE_MESH_REQUIRED` | Fail closed when mesh nonce register fails |
| `SPANDA_DECISION_ESCALATION_STORE` | Escalation approval store path |
| `SPANDA_FLEET_MESH_URL` | Fleet mesh coordinator for vote ingest and conflict resolution |
| `SPANDA_FLEET_MESH_TOKEN` | Bearer token for fleet mesh HTTP |
| `SPANDA_FLEET_SYNTHESIZE_MEMBER_VOTES` | Single-process demo: coordinator posts synthetic member votes |
| `SPANDA_FLEET_MEMBER_VOTE_<ROBOT>` | Override per-robot mesh vote action (tests) |
| `SPANDA_CRYPTO_BACKEND` | `software`, `mock_hsm`, `script`, `tpm2` |
| `SPANDA_DECISION_SIGNING_KEY_ID` | HSM key id for signing backend |
| `SPANDA_HSM_SIGN_SCRIPT` | External HSM sign command (stdin payload, stdout hex sig) |
| `SPANDA_TPM2_SIGN_SCRIPT` | TPM2 sign command alias for `tpm2` backend |

## Flagship demo

```bash
spanda decision simulate examples/showcase/distributed_decisions/gps_loss_recovery/mission.sd --offline
spanda decision sign-tree examples/showcase/distributed_decisions/gps_loss_recovery/mission.sd --tree GPSLossRecovery --write-cache
export SPANDA_DECISION_TRACE=1
spanda sim examples/showcase/distributed_decisions/gps_loss_recovery/mission.sd --record --inject-health-faults
spanda replay examples/showcase/distributed_decisions/gps_loss_recovery/mission.trace
spanda audit decisions examples/showcase/distributed_decisions/gps_loss_recovery/mission.trace
```

## Related

- [distributed-decisions.md](./distributed-decisions.md)
- [distributed-decision-security.md](./distributed-decision-security.md)
- [distributed-decision-demo.md](./distributed-decision-demo.md)
