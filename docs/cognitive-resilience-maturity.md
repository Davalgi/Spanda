# Cognitive & Resilience — Capability Maturity

Promotion criteria and current tier for each functional domain. Canonical architecture:
[cognitive-resilience-architecture.md](./cognitive-resilience-architecture.md).

## Current tiers (v0.6.3+)

| Domain | Tier | Rationale |
|--------|------|-----------|
| Reflex & Safety | **Beta** | Reflex catalog, runtime traces, kill-switch integration, REST+gRPC |
| Operational Coordination | **Stable** | Fleet, mission runtime, distributed decisions (separate architecture) |
| Strategic Planning | **Stable** | Mission planner, governance, deployment profiles (platform services) |
| Homeostasis Engine | **Beta** | Entity + scheduler telemetry, drift detection, corrections |
| Platform Immunity | **Beta** | Trust/tamper quarantine, registry overlay |
| Sensory Fusion | **Beta** | Rule-based multi-source fusion from entity health/readiness/trust; live sensor pipeline **Planned** |
| Attention Engine | **Beta** | Health-mapped prioritization, habituation CLI |
| Operational Memory | **Beta** | Category refs on entities, enriched API, replay/playbook linkage |
| Adaptive Learning | **Experimental** | Recovery confidence statistics; ML **out of scope** |
| Maintenance & Optimization | **Beta** | OTA **Stable**; maintenance window types + OTA integration |
| Control Center tab | **Beta** | Live REST panels for all domains |

## Promotion checklist

### Sensory Fusion → Stable

- [x] Rule-based `fuse_observations` with conflict detection
- [x] Entity-derived sensor bundle (`health_status`, `readiness_status`, `trust_status`)
- [x] REST `/v1/autonomy/fusion` + gRPC `GetAutonomyFusion`
- [ ] Live multi-sensor pipeline (GPS/IMU/camera) — **Beta** via `SPANDA_LIVE_FUSION_SENSORS=1` + automotive proxy supplier; full hardware pipelines via packages remain **Planned**
- [ ] Readiness partial scoring wired from fusion conflicts in mission planner

### Operational Memory → Stable

- [x] Five memory categories on `Entity.memory_refs`
- [x] Registry enrichment (reflex, procedural, episodic, working, semantic)
- [x] REST `/v1/autonomy/memory` returns refs + `OperationalMemoryModel`
- [ ] Persistent episodic store linked to replay index — **Planned**
- [ ] Control Center browse-by-category (not just JSON dump)

### Adaptive Learning → Beta

- [x] `RecoveryConfidence` from orchestrator history
- [x] `POST /v1/recovery/recommend` + recovery metrics
- [ ] Strategy preference surfaced in mission planner abort/replan
- [ ] Documented accuracy thresholds over field soak

### Maintenance & Optimization → Stable

- [x] OTA deploy **Stable**
- [x] `MaintenanceWindow`, `SleepMode` types in `spanda-autonomy`
- [ ] CLI `spanda maintenance window` — **Planned**
- [ ] Control Center maintenance schedule panel — **Planned**

### Control Center Cognitive & Resilience → Stable

- [x] All functional domain panels live
- [x] Strategic Planning summary (governance + deployment profiles)
- [x] Cross-interface REST/gRPC smoke
- [ ] RBAC audit for autonomy routes
- [ ] Field soak sign-off per [organizational-gates.md](./organizational-gates.md)

## CI

```bash
./scripts/cognitive_resilience_smoke.sh
./scripts/cross_interface_consistency.sh   # includes fusion/memory + SDK probes
```

Legacy alias: `./scripts/bio_inspired_autonomy_smoke.sh` (same checks).

## Related

- [feature-status.md](./feature-status.md) — platform-wide matrix
- [responsibility-matrix.md](./responsibility-matrix.md) — ownership
- [known-limitations.md](./known-limitations.md#cognitive--resilience-architecture) —
  fusion/recovery caveats
