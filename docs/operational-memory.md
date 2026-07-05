# Operational Memory Model

**Functional domain:** [Operational Memory](./functional-domains.md#operational-memory)  
**Status: Beta**

> Canonical architecture: [cognitive-resilience-architecture.md](./cognitive-resilience-architecture.md)

## Purpose

Organize platform memory into useful engineering categories — not biological memory simulation.

| Category | Engineering mapping |
|----------|---------------------|
| **Reflex Memory** | Fast local rules and safety reflexes |
| **Working Memory** | Current mission context |
| **Episodic Memory** | Mission traces, incidents, replays |
| **Semantic Memory** | Entity graph, knowledge graph, relationships |
| **Procedural Memory** | Recovery playbooks, decision policies, procedures |

## Integration

Replay, diagnosis, recovery, assurance, entity graph, decision traceability.

Entity field: `Entity.memory_refs` (alias: `Entity.operationalMemory`)

REST: `GET /v1/autonomy/memory` · SDK: `MemoryClient`

See [operational-memory.md](./operational-memory.md), [cognitive-resilience-architecture.md](./cognitive-resilience-architecture.md).
