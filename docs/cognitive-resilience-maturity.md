# Cognitive & Resilience — Capability Maturity

Promotion criteria and current tier for each functional domain. Canonical architecture:
[cognitive-resilience-architecture.md](./cognitive-resilience-architecture.md).

## Current tiers (v1.0+)

| Domain | Tier | Rationale |
|--------|------|-----------|
| Reflex & Safety | **Stable** | Reflex catalog, runtime traces, kill-switch integration, REST+gRPC |
| Operational Coordination | **Stable** | Fleet, mission runtime, distributed decisions (separate architecture) |
| Strategic Planning | **Stable** | Mission planner, governance, deployment profiles (platform services) |
| Homeostasis Engine | **Stable** | Entity + scheduler telemetry, drift detection, corrections |
| Platform Immunity | **Stable** | Trust/tamper quarantine, registry overlay |
| Sensory Fusion | **Stable** | Rule-based multi-source fusion; conflicts feed readiness partial scoring; live sensors **Stable-with-env-gate** via `SPANDA_LIVE_FUSION_SENSORS` |
| Attention Engine | **Stable** | Health-mapped prioritization, habituation CLI |
| Operational Memory | **Stable** | Category refs, persistent episodic store ↔ replay index, browse-by-category UI |
| Adaptive Learning | **Stable** | Recovery confidence + strategy preference on mission abort/replan; ML **out of scope** |
| Maintenance & Optimization | **Stable** | OTA **Stable**; `spanda maintenance window` + Control Center schedule panel |
| Control Center tab | **Stable** | Live REST panels for all domains; promotion gate script |

## Promotion checklist

### Sensory Fusion → Stable

- [x] Rule-based `fuse_observations` with conflict detection
- [x] Entity-derived sensor bundle (`health_status`, `readiness_status`, `trust_status`)
- [x] REST `/v1/autonomy/fusion` + gRPC `GetAutonomyFusion`
- [x] Live multi-sensor pipeline — **Stable-with-env-gate** via `SPANDA_LIVE_FUSION_SENSORS=1` + registered supplier (CLI registers automotive/GPS/IMU/camera proxy); full hardware pipelines via packages remain optional
- [x] Readiness partial scoring wired from fusion conflicts in `evaluate_entity_readiness`

### Operational Memory → Stable

- [x] Five memory categories on `Entity.memory_refs`
- [x] Registry enrichment (reflex, procedural, episodic, working, semantic)
- [x] REST `/v1/autonomy/memory` returns refs + `OperationalMemoryModel` + `by_category`
- [x] Persistent episodic store linked to replay index (`.spanda/autonomy-episodic-memory.json`)
- [x] Control Center browse-by-category (not just JSON dump)

### Adaptive Learning → Stable

- [x] `RecoveryConfidence` from orchestrator history
- [x] `POST /v1/recovery/recommend` + recovery metrics
- [x] Strategy preference surfaced in mission planner abort/replan (`verify_mission_assurance_with_recovery`, `spanda mission verify`)
- [x] Documented accuracy thresholds (below); field soak remains organizational

#### Field accuracy thresholds

| Threshold | Value | Use |
|-----------|-------|-----|
| `min_attempts` | **3** | Prefer a strategy only after enough history |
| `escalate_below_rate` | **0.30** | Abort/replan when preferred rate or overall score is below this |
| Stable field success rate | **≥ 0.70** | Preferred strategy should hold ≥70% over soak before production claims |
| Field soak | **30 days** | Organizational gate — [organizational-gates.md](./organizational-gates.md) |

Defaults live in `AdaptiveRecoveryPolicy::platform_defaults()` and
`spanda_autonomy::adaptive_recovery::field_accuracy_thresholds`.

### Maintenance & Optimization → Stable

- [x] OTA deploy **Stable**
- [x] `MaintenanceWindow`, `SleepMode` types in `spanda-autonomy`
- [x] CLI `spanda maintenance window` list/set
- [x] Control Center maintenance schedule panel

### Control Center Cognitive & Resilience → Stable

- [x] All functional domain panels live
- [x] Strategic Planning summary (governance + deployment profiles)
- [x] Cross-interface REST/gRPC smoke
- [x] RBAC audit for autonomy routes — GETs are read-only (sensitive-read prefix); `POST /v1/autonomy/maintenance/windows` requires **Operate**
- [ ] Field soak sign-off per [organizational-gates.md](./organizational-gates.md) (**pending** — organizational)

## CI

```bash
./scripts/cognitive_resilience_smoke.sh
./scripts/cross_interface_consistency.sh   # includes fusion/memory + SDK probes
./scripts/cognitive_resilience_stable_promotion_gate.sh  # smoke + cross-interface + RBAC notes
```

Legacy alias: `./scripts/bio_inspired_autonomy_smoke.sh` (same checks as smoke).

## Related

- [feature-status.md](./feature-status.md) — platform-wide matrix
- [responsibility-matrix.md](./responsibility-matrix.md) — ownership
- [known-limitations.md](./known-limitations.md#cognitive--resilience-architecture) —
  fusion/recovery caveats
