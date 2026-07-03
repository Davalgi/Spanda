# Stable Hardening — Distributed Decisions

Promotion checklist for **Distributed Decisions** from Experimental to **Stable**.

## Stable capabilities

| Capability | Evidence |
|------------|----------|
| Runtime conflict resolution | `evaluate_live_decision_trees` collects competing decisions and calls `resolve_conflict`; safety reflex wins split-brain |
| Persistent escalation approvals | `.spanda/decision-escalations.json`; `POST /v1/decisions/escalate`; Control Center Approve button |
| Ed25519 decision tree signing | `spanda decision sign-tree`; `verify_decision_tree_signature`; cache merge via `PersistedPolicyCache.decision_trees` |
| Persisted nonce replay protection | `.spanda/decision-nonce-registry.json`; `register_persisted_nonce` on trace validation |
| v3 envelope crypto signatures | `decision_envelope_signing_payload` + Ed25519 via `SPANDA_DECISION_POLICY_SIGNING_KEY`; verified with trust key |
| Rule enforcement tests | `tests/rule_enforcement.rs`, `tests/attack_simulations.rs`, `tests/stable_gaps.rs` |
| CI gate | `distributed-decisions` job + `./scripts/distributed_decisions_smoke.sh` |

## Verification commands

```bash
cargo test -p spanda-decision
cargo test -p spanda-decision --test stable_gaps
cargo test -p spanda-interpreter --test decision_runtime
./scripts/distributed_decisions_smoke.sh
```

## Environment variables

| Variable | Purpose |
|----------|---------|
| `SPANDA_DECISION_POLICY_SIGNING_KEY` | Sign offline policies, decision trees, v3 envelopes |
| `SPANDA_DECISION_POLICY_TRUST_KEY` | Verify signed policies, trees, envelopes |
| `SPANDA_DECISION_REQUIRE_SIGNED_OFFLINE_POLICY` | Require signed offline policies |
| `SPANDA_DECISION_REQUIRE_SIGNED_TREES` | Require signed decision trees |
| `SPANDA_DECISION_POLICY_CACHE` | Signed policy/tree cache path |
| `SPANDA_DECISION_NONCE_CACHE` | Nonce replay registry path |
| `SPANDA_DECISION_ESCALATION_STORE` | Escalation approval store path |

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
