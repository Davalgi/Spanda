# Responsibility Matrix

Maps every major platform capability to its **functional domain**, **existing services**, **entity
integration**, **SDK/API**, and **Control Center** surface. Use this matrix to avoid duplicate
ownership and clarify contributor boundaries.

Architecture overview:
[cognitive-resilience-architecture.md](./cognitive-resilience-architecture.md) · Domain definitions:
[functional-domains.md](./functional-domains.md)

---

## Legend

| Column | Meaning |
|--------|---------|
| **Capability** | User-visible or contributor-facing feature |
| **Functional domain** | Cognitive & Resilience responsibility owner |
| **Platform services** | Existing crates / modules (not duplicated) |
| **Entity integration** | Fields on `EntityRecord` / `EntityAutonomyProfile` |
| **SDK / API** | REST, gRPC, CLI, SDK client |
| **Control Center** | Tab or panel |

---

## Strategic Planning

| Capability | Functional domain | Platform services | Entity integration | SDK / API | Control Center |
|------------|-------------------|-------------------|--------------------|-----------|----------------|
| Mission planning | Strategic Planning | `spanda-mission-planning`, assurance | Mission entities | `spanda mission plan`, REST programs | Mission tab |
| Policy evaluation | Strategic Planning | `spanda-policy`, governance | `Entity.metadata` | `GET /v1/governance`, `GovernanceClient` | Governance tab |
| Deployment planning | Strategic Planning | `spanda-deploy`, readiness | `Entity.readiness` | `POST /v1/programs/readiness` | Readiness tab |
| Simulation planning | Strategic Planning | `spanda-sim`, digital twin | Twin entities | `POST /v1/programs/sim` | Simulation tab |
| Assurance cases | Strategic Planning | `spanda-assurance` | Capability traceability | `POST /v1/programs/assure` | Assurance tab |
| Resource planning | Strategic Planning | Fleet, device pool | Fleet parent-child graph | `GET /v1/fleet` | Fleet tab |
| Governance validation | Strategic Planning | `spanda-governance` | All entities | `POST /v1/governance/validate` | Governance tab |
| Damage-informed abort | Strategic Planning + Damage Risk | Mission planner + `spanda-autonomy` | `Entity.damage_risk` | Entity autonomy API | Cognitive & Resilience |

---

## Operational Coordination

| Capability | Functional domain | Platform services | Entity integration | SDK / API | Control Center |
|------------|-------------------|-------------------|--------------------|-----------|----------------|
| Fleet coordination | Operational Coordination | `spanda-fleet`, mesh | Fleet/swarm entities | `GET /v1/fleet`, fleet CLI | Fleet tab |
| Task scheduling | Operational Coordination | `spanda-runtime`, scheduler | Robot entities | Runtime telemetry | Telemetry tab |
| Delegation / takeover | Operational Coordination | Mission continuity | Human ↔ robot edges | Continuity APIs | Continuity tab |
| Distributed decisions | Operational Coordination | Decision engine | `local_decision_authority` | `/v1/decisions*` | Decisions tab |
| Swarm coordination | Operational Coordination | Fleet mesh | Swarm entities | Fleet orchestrate CLI | Fleet map |
| Mission execution | Operational Coordination | Interpreter, triggers | Mission lifecycle | `spanda run`, `spanda sim` | Mission tab |
| Recovery orchestration | Operational Coordination + Recovery | Recovery orchestrator | Recovery graph | `/v1/recovery/*` | Recovery tab |

---

## Reflex & Safety

| Capability | Functional domain | Platform services | Entity integration | SDK / API | Control Center |
|------------|-------------------|-------------------|--------------------|-----------|----------------|
| Emergency stop | Reflex & Safety | Runtime safety, kill switch | `Entity.reflexes` | `spanda reflex`, `ReflexClient` | Reflex Events panel |
| Kill switch | Reflex & Safety | `spanda-runtime` safety engine | Safety metadata | Language `kill_switch` | Security tab |
| Reflex decision layer | Reflex & Safety | Distributed decisions L0 | Reflex summaries | `/v1/decisions` trace | Decisions tab |
| Reflex traces | Reflex & Safety | `spanda-autonomy` trace buffer | `Entity.reflexes.last_triggered_at` | `/v1/autonomy/reflex/traces` | Reflex Events panel |
| Runtime protection | Reflex & Safety | Safety types, `stop_if` | Health → reflex trigger | `spanda check` | Health tab |
| Protective shutdown | Reflex & Safety | Recovery + reflex | `Entity.damage_risk` | Recovery execute | Recovery tab |

---

## Homeostasis Engine

| Capability | Functional domain | Platform services | Entity integration | SDK / API | Control Center |
|------------|-------------------|-------------------|--------------------|-----------|----------------|
| Health monitoring | Homeostasis Engine | `spanda-readiness` entity health | `Entity.health_status` | `GET /v1/entities/{id}/health` | Health tab |
| Readiness scoring | Homeostasis Engine | `spanda-readiness` | `Entity.readiness_status` | `POST /v1/programs/readiness` | Readiness tab |
| Stability metrics | Homeostasis Engine | `spanda-autonomy::homeostasis` | `Entity.homeostasis` | `HomeostasisClient`, `/v1/autonomy/homeostasis` | Homeostasis panel |
| Drift detection | Homeostasis Engine | Homeostasis policy | `Entity.homeostasis.drift_signals` | `spanda homeostasis check` | Homeostasis panel |
| Auto-correction | Homeostasis Engine | Recovery hooks | Protective actions | Recovery recommend | Recovery tab |
| Scheduler telemetry | Homeostasis Engine | Runtime scheduler | Platform telemetry snapshot | Runtime context | Trends tab |
| Trust/readiness in stability | Homeostasis Engine | Entity projection | `Entity.trust`, readiness | Entity list filters | Entities tab |

---

## Platform Immunity

| Capability | Functional domain | Platform services | Entity integration | SDK / API | Control Center |
|------------|-------------------|-------------------|--------------------|-----------|----------------|
| Trust evaluation | Platform Immunity | `spanda-trust` | `Entity.trust_status` | Trust APIs | Security tab |
| Tamper detection | Platform Immunity | `spanda-tamper` | Metadata `tamper.detected` | Security audit CLI | Security tab |
| Device quarantine | Platform Immunity | `spanda-autonomy::immunity` | `Entity.immunity_status` | `ImmunityClient`, `/v1/autonomy/immunity` | Platform Immunity panel |
| Package trust | Platform Immunity | Package provenance | Package entities | Package verify | Security tab |
| Plugin isolation | Platform Immunity | `spanda-plugin` | Provider entities | Marketplace tab | Marketplace tab |
| Re-admission | Platform Immunity | Trust + verify | Lifecycle state | `POST /v1/entities/{id}/verify` | Provisioning tab |
| Policy violations | Platform Immunity | `spanda-policy` | Governance metadata | Governance validate | Governance tab |

---

## Sensory Fusion

| Capability | Functional domain | Platform services | Entity integration | SDK / API | Control Center |
|------------|-------------------|-------------------|--------------------|-----------|----------------|
| Multi-source fusion | Sensory Fusion | `spanda-autonomy::fusion`, `spanda-fusion` | `Entity.confidence` | `FusionClient`, `/v1/autonomy/fusion` | Entity autonomy |
| Conflict detection | Sensory Fusion | Confidence policy | `Entity.confidence.conflicts` | `spanda confidence report` | Diagnosis tab |
| Signal agreement | Sensory Fusion | Fusion types | `Entity.confidence.sources` | `spanda fusion check` | — |
| Readiness impact | Sensory Fusion + Homeostasis | Readiness engine | Readiness partial/degraded | Readiness API | Readiness tab |
| Diagnosis trigger | Sensory Fusion | `spanda-explain` | Incident entities | `POST /v1/programs/diagnose` | Diagnosis tab |

---

## Attention Engine

| Capability | Functional domain | Platform services | Entity integration | SDK / API | Control Center |
|------------|-------------------|-------------------|--------------------|-----------|----------------|
| Event prioritization | Attention Engine | `spanda-autonomy::attention` | `Entity.attention` | `AttentionClient`, `/v1/autonomy/attention` | Attention Queue panel |
| Alert suppression | Attention Engine | Habituation policy | `Entity.attention.suppressed_count` | `spanda alerts analyze` | Alerts tab |
| Critical-first routing | Attention Engine | Attention policy | Top priority on entity | gRPC `GetAutonomyAttention` | Attention Queue panel |
| Alert fatigue metrics | Attention Engine | `spanda-autonomy::habituation` | — | `spanda alerts fatigue-report` | Alerts tab |
| Mission event boost | Attention Engine | Attention + mission context | Mission entity link | Attention policy in `.sd` | Mission tab |

---

## Operational Memory

| Capability | Functional domain | Platform services | Entity integration | SDK / API | Control Center |
|------------|-------------------|-------------------|--------------------|-----------|----------------|
| Mission traces | Operational Memory | Replay store | `memory_refs.episodic` | `spanda replay` | Replay tab |
| Entity graph | Operational Memory | `spanda-config` graph | `memory_refs.semantic` | `GET /v1/entities/graph` | Entities tab |
| Recovery playbooks | Operational Memory | Recovery orchestrator | `memory_refs.procedural` | `/v1/recovery/playbooks` | Recovery tab |
| Decision history | Operational Memory | Decision traces | `memory_refs.episodic` | `/v1/decisions/traces` | Decisions tab |
| Knowledge graph | Operational Memory | `spanda-knowledge-model` | Semantic refs | Assurance APIs | Digital thread |
| Reflex rules | Operational Memory | Reflex catalog | `memory_refs.reflex` | `/v1/autonomy/reflex` | Reflex Events |
| Working context | Operational Memory | Mission runtime | `memory_refs.working` | `MemoryClient`, `/v1/autonomy/memory` | Mission tab |

---

## Adaptive Learning

| Capability | Functional domain | Platform services | Entity integration | SDK / API | Control Center |
|------------|-------------------|-------------------|--------------------|-----------|----------------|
| Recovery confidence | Adaptive Learning | `spanda-autonomy::adaptive_recovery` | `Entity.recovery_confidence` | `spanda recovery confidence` | Recovery Confidence panel |
| Strategy preference | Adaptive Learning | Recovery history stats | `preferred_strategy` | `/v1/recovery/recommend` | Recovery tab |
| Historical recommendations | Adaptive Learning | Recovery orchestrator | Recovery history | `/v1/recovery/history` | Recovery tab |
| Rule-based adaptation | Adaptive Learning | AdaptiveRecoveryPolicy | — | `spanda recovery learning-report` | — |
| Confidence updates | Adaptive Learning | Fusion + recovery bridge | `Entity.confidence` + recovery | Entity autonomy API | Cognitive & Resilience |

---

## Damage Risk Assessment

| Capability | Functional domain | Platform services | Entity integration | SDK / API | Control Center |
|------------|-------------------|-------------------|--------------------|-----------|----------------|
| Harm-risk index | Damage Risk Assessment | `spanda-autonomy::damage_risk` | `Entity.damage_risk` | `RiskClient`, entity autonomy | Damage Risk panel |
| Protective actions | Damage Risk Assessment | Recovery + reflex | `protective_action` field | Recovery plan | Recovery tab |
| Mission abort signal | Damage Risk Assessment | Mission planner input | Risk index threshold | Mission verify | Mission tab |
| Operator risk | Damage Risk Assessment | Human entity health | Human health fields | Humans tab | Humans tab |
| Degraded mode trigger | Damage Risk Assessment | Operating modes | Lifecycle + risk | `degraded-modes` runtime | Health tab |
| Preventative maintenance | Damage Risk Assessment | Prognostics | Metadata signals | `spanda prognostics` | Trends tab |

---

## Maintenance & Optimization

| Capability | Functional domain | Platform services | Entity integration | SDK / API | Control Center |
|------------|-------------------|-------------------|--------------------|-----------|----------------|
| OTA rollouts | Maintenance & Optimization | Fleet OTA | Firmware version fields | `/v1/ota` | OTA tab |
| Calibration windows | Maintenance & Optimization | `spanda-autonomy::maintenance` | Lifecycle scheduled | Maintenance mode types | — |
| Log rotation | Maintenance & Optimization | Telemetry store | — | Telemetry CLI | Telemetry tab |
| Backup | Maintenance & Optimization | Config snapshots | Config entities | Config tab | Config tab |
| Resource balancing | Maintenance & Optimization | Fleet scheduler | Fleet readiness | Fleet health | Fleet tab |
| Sleep / low-activity mode | Maintenance & Optimization | Maintenance module | Metadata | Preview CLI | — |

---

## Cross-domain interactions (integration tests)

| Interaction | Domains involved | Test location |
|-------------|------------------|---------------|
| Reflex triggered by homeostasis drift | Reflex & Safety + Homeostasis | `crates/spanda-autonomy/tests/cognitive_resilience_integration.rs` |
| Fusion lowers readiness on conflict | Sensory Fusion + Homeostasis | same |
| Tamper triggers immunity quarantine | Platform Immunity + Trust | same |
| Attention ranks recovery events | Attention + Recovery | same |
| Memory refs link to replay traces | Operational Memory + Replay | same |
| Damage risk informs mission posture | Damage Risk + Strategic Planning | same |
| Adaptive recovery feeds orchestrator | Adaptive Learning + Recovery | same |

---

## Anti-patterns (do not duplicate)

| Wrong | Right |
|-------|-------|
| New `RobotHealthRecord` parallel to Entity | Extend `EntityRecord` / autonomy profile |
| Separate immunity service crate | Use `spanda-autonomy::immunity` + trust/tamper |
| ML fusion pipeline in autonomy crate | Rule-based fusion now; ML via packages later |
| Brain-anatomy module names | Functional domain names only |
| Duplicate REST business logic in SDK | SDK clients wrap `/v1/*` endpoints |
