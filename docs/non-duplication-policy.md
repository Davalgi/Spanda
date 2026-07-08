# Non-Duplication Policy

Spanda maintains a single coherent platform. **Duplicate capabilities are not allowed** unless
extension of an existing subsystem has been evaluated and rejected with documented justification.

**Parent:** [architecture-governance.md](./architecture-governance.md) ·
**Checklist:** [architecture-review-checklist.md](./architecture-review-checklist.md)

---

## Policy Statement

Before adding any of the following, contributors and reviewers must confirm that no existing
artifact
already provides the same or substantially overlapping capability:

- Workspace crate (`crates/spanda-*`)
- Official or community package (`packages/registry/`)
- Provider implementation or provider trait fork
- Plugin or governance extension
- Platform service (readiness, recovery, trust, telemetry, …)
- REST, gRPC, or CLI API endpoint
- SDK client method or parallel DTO
- Control Center feature or view
- Architecture concept (parallel registries, duplicate decision engines, …)

**Default action:** Extend the existing capability.  
**Exception:** Document why extension is insufficient; pass Architecture Review.

---

## What Counts as Duplication

Duplication is not limited to copy-paste code. Any of the following counts:

| Pattern | Example | Preferred approach |
|---------|---------|-------------------|
| **Parallel data model** | Second robot inventory beside Entity Registry | Extend `EntityRecord` / entity kinds |
| **Overlapping service** | New "health checker" crate beside `spanda-health` | Extend health service and APIs |
| **Duplicate API** | Second REST path for readiness scores | Extend `/v1/readiness/*` or entity APIs |
| **Forked provider** | Two MQTT packages with different traits | One package; versioned backends |
| **Shadow SDK** | Hand-rolled HTTP client beside official SDK | Add method to `spanda-sdk` / `@davalgi-spanda/sdk` |
| **Concept duplication** | Separate "mission status" model outside entities | Mission as entity kind |
| **Blueprint platform creep** | Blueprint adds workspace crate for one vertical | Implement in platform; blueprint composes |

Substantial overlap (>50% of the proposed user-facing behavior already exists elsewhere) triggers
redesign unless the proposal clearly scopes **additive** differentiation.

---

## Search Obligations

Proposers must search and cite existing artifacts:

### Crates and services

1. [crates/README.md](../crates/README.md) — workspace index
2. [platform-services.md](./platform-services.md) — service boundaries
3. [responsibility-matrix.md](./responsibility-matrix.md) — capability ownership
4. [module-ownership.md](./module-ownership.md) — owners

### Packages and providers

1. `packages/registry/` — official packages
2. [official-packages.md](./official-packages.md) — catalog
3. [how-providers-work.md](./how-providers-work.md) — dispatch and traits
4. [provider-interfaces.md](./provider-interfaces.md) — contracts

### Entity and operational domains

1. [entity-overview.md](./entity-overview.md) — documentation map
2. Recovery: [recovery-orchestrator.md](./recovery-orchestrator.md)
3. Readiness / health / trust: [readiness.md](./readiness.md),
   [entity-health.md](./entity-health.md), [entity-trust.md](./entity-trust.md)
4. Decisions: [distributed-decisions.md](./distributed-decisions.md)

### Interfaces

1. [control-center-api.md](./control-center-api.md) — REST v1
2. [entity-apis.md](./entity-apis.md) — entity REST/gRPC
3. [sdk.md](./sdk.md) — official SDKs
4. CLI: [spanda-reference.md](./spanda-reference.md)

### Automated checks

```bash
# Layer and dependency governance (CI)
python3 scripts/validate_architecture.py

# Documentation coverage
python3 scripts/validate_documentation.py --report
```

---

## Extension Hierarchy

When multiple placement options exist, prefer the **highest** applicable option in this list (lowest
platform expansion):

1. **Documentation / example** — clarify or compose existing capabilities
2. **Package** — provider-backed domain behavior
3. **Plugin** — optional governance or compliance extension
4. **Provider trait extension** — new backend, same contract
5. **Platform service extension** — new evaluation or API on existing service
6. **Core platform extension** — entity kinds, config, transport (justified)
7. **New workspace crate** — last resort; requires ADR
8. **New architectural layer** — requires ADR + maintainer approval; almost never

See [lean-core.md](./lean-core.md) and [design-principles.md](./design-principles.md).

---

## Justification Requirements

If duplication cannot be avoided, the proposal must include:

1. **Existing artifact cited** — name, path, and why it cannot be extended
2. **Migration plan** — deprecate or converge duplicate paths over time
3. **Ownership** — single owner for both paths until convergence
4. **ADR** — mandatory for new crate, service, or API parallel to existing
5. **Sunset criteria** — when the duplicate will be removed or merged

Without these five items, Architecture Review should **reject** the proposal.

---

## Rejection Criteria

Reject or require redesign when:

- An existing crate, package, or service covers the same operational question
- The proposal introduces a parallel entity or robot model
- A new REST or SDK surface duplicates an existing endpoint without versioning strategy
- A blueprint proposes platform features instead of composing them
- Duplication check section is empty or says "N/A" without search evidence
- Extension would be ≤200 lines in an existing module but a new crate is proposed

---

## Reviewer Responsibilities

Architecture reviewers must:

1. Independently verify the duplication check (do not rely on proposer search alone)
2. Recommend **extend** vs **create** with specific file/crate targets
3. Block merge if duplication is unjustified
4. Record outcome in the Architecture Scorecard
   ([architecture-review-checklist.md](./architecture-review-checklist.md))

---

## Convergence and Deprecation

Accepted exceptions must converge:

- Mark older paths **deprecated** in docs and CHANGELOG
- Add compatibility shims with clear removal version
- Track in [feature-status.md](./feature-status.md) if user-visible
- Remove duplicate after one minor release when safe

---

## Examples

### Accept — extend

**Proposal:** "Add fleet-wide trust rollup API."  
**Action:** Extend entity trust APIs and Control Center view; no new crate.

### Reject — duplicate

**Proposal:** "New `spanda-robot-registry` crate for robot inventory."  
**Action:** Reject — use Entity Registry ([entity-registry.md](./entity-registry.md)).

### Accept with ADR — justified parallel

**Proposal:** "Separate gRPC streaming for high-frequency telemetry."  
**Action:** Accept only if REST polling is insufficient, ADR documents tradeoffs, SDK adds one
client path, and DTOs reuse entity telemetry models.

---

## Related Policies

- [dependency-rules.md](./dependency-rules.md) — dependency direction and waivers
- [scope-control.md](./scope-control.md) — horizon phase allowed vs not allowed
- [design-principles.md](./design-principles.md) — entity-first, lean core, single responsibility
