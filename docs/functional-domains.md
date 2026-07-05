# Functional Domains

The Spanda **Cognitive & Resilience Architecture** organizes platform capabilities into eleven **functional responsibility domains**. Each domain has a clear engineering purpose, integrates with existing platform services, and operates on the unified [Entity model](./entity-model.md).

> Spanda does **not** model biological anatomy. Domain names describe **engineering responsibilities**, not brain regions.

Overview: [cognitive-resilience-architecture.md](./cognitive-resilience-architecture.md) · Ownership matrix: [responsibility-matrix.md](./responsibility-matrix.md)

---

## Strategic Planning

**Responsibilities:** mission planning, long-term optimization, policy evaluation, deployment planning, resource planning, governance.

**Integrates with:** Decision Engine, Mission Planner (`spanda-mission-planning`), Governance, Control Center, Simulation, Assurance.

**Entity touchpoints:** `Entity.capabilities`, mission entities, governance metadata, `Entity.damageRisk` (abort inputs).

**Status:** Mission planner and governance **Stable**; long-term optimization **Planned**.

---

## Operational Coordination

**Responsibilities:** mission execution, task scheduling, fleet coordination, swarm coordination, delegation, takeover, synchronization.

**Integrates with:** Fleet (`spanda-fleet`), Mission Runtime, [Distributed Decisions](./distributed-decisions.md), Recovery, Mission Continuity.

**Entity touchpoints:** fleet/swarm parent-child relationships, `Entity.readiness`, mission lifecycle state.

**Status:** **Stable**.

---

## Reflex & Safety

**Responsibilities:** emergency stop, immediate hazard response, local safety actions, kill switch, runtime protection, protective shutdown.

**Characteristics:** deterministic, ultra-low latency, no cloud dependency, safety bounded.

**Integrates with:** [Reflex Architecture](./reflex-architecture.md), Distributed Decisions (layer 0), Runtime, Recovery.

**Entity touchpoints:** `Entity.reflexes`, kill-switch handlers in `.sd` programs.

**Status:** **Beta** — see [reflex-architecture.md](./reflex-architecture.md).

---

## Homeostasis Engine

**Responsibilities:** maintain stable operating conditions — monitor CPU, memory, battery, storage, temperature, latency, network, sensor quality, trust, readiness; automatic correction, preventative action, drift detection, stability maintenance.

**Integrates with:** Health, Telemetry, Recovery, Readiness.

**Entity touchpoints:** `Entity.health`, `Entity.homeostasis`, `Entity.readiness`.

**Status:** **Beta** — see [platform-homeostasis.md](./platform-homeostasis.md).

---

## Platform Immunity

**Responsibilities:** detect and isolate compromised devices, entities, malicious plugins/packages, spoofed sensors, policy violations, trust violations; quarantine, isolation, re-verification, re-admission.

**Integrates with:** Security, Trust, Tamper Detection, Plugins, Packages.

**Entity touchpoints:** `Entity.trust`, `Entity.immunity` (`immunity_status`).

**Status:** **Beta** — see [platform-immunity.md](./platform-immunity.md).

---

## Sensory Fusion

**Responsibilities:** combine multiple observations; compute confidence, agreement, disagreement, signal quality, sensor reliability.

**Used by:** Readiness, Decision Engine, Recovery, Diagnosis.

**Entity touchpoints:** `Entity.confidence`.

**Status:** **Beta** — see [sensory-fusion.md](./sensory-fusion.md).

---

## Attention Engine

**Responsibilities:** prioritize critical events, mission events, operator events, telemetry, alerts; avoid alert fatigue via priority, suppression, aggregation, focus.

**Integrates with:** Telemetry, Alerts, Control Center, Diagnosis.

**Entity touchpoints:** `Entity.attention`.

**Status:** **Beta** — see [attention-engine.md](./attention-engine.md).

---

## Operational Memory

**Responsibilities:** maintain current mission state, historical missions, knowledge graph, entity graph, replay, recovery history, decision history, playbooks.

**Memory categories:**

| Category | Engineering mapping |
|----------|---------------------|
| **Working** | Current mission context |
| **Episodic** | Mission traces, incidents, replays |
| **Semantic** | Entity graph, knowledge graph |
| **Procedural** | Recovery playbooks, decision policies |
| **Reflex** | Fast local safety rules |

**Entity touchpoints:** `Entity.operationalMemory` (`memory_refs`).

**Status:** **Beta** — see [operational-memory.md](./operational-memory.md).

---

## Adaptive Learning

**Purpose:** improve operational decisions using historical outcomes.

**Initially:** rule-based adaptation, statistics, confidence updates, historical recommendations. **No ML dependency required.**

**Integrates with:** Recovery Orchestrator, Adaptive Recovery, Decision traceability.

**Entity touchpoints:** `Entity.recoveryConfidence`.

**Status:** **Experimental** — see [adaptive-operations.md](./adaptive-operations.md).

---

## Damage Risk Assessment

**Purpose:** evaluate potential harm — different from errors. Model damage, harm, operator risk, asset risk, mission risk.

**Actions:** preventative action, mission abort, degraded mode, maintenance scheduling.

**Entity touchpoints:** `Entity.damageRisk` (`damage_risk`).

**Status:** **Beta** — see [damage-risk.md](./damage-risk.md).

---

## Maintenance & Optimization

**Responsibilities:** maintenance windows, OTA updates, cleanup, calibration, log rotation, backup, optimization, resource balancing.

**Integrates with:** OTA, Device Pool, Telemetry store, Maintenance mode (`spanda-autonomy::maintenance`).

**Entity touchpoints:** `Entity.lifecycle_state`, firmware/software version fields.

**Status:** OTA **Stable**; maintenance/sleep mode **Beta** — see [platform-maintenance.md](./platform-maintenance.md).
