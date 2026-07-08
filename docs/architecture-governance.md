# Architecture Governance

Permanent engineering governance for the Spanda Autonomous Systems Platform. Every architectural
change, platform capability, crate, package, provider, plugin, service, API, SDK, or roadmap
proposal must pass **Architecture Review** before it is accepted.

**Related:** [architecture-review-checklist.md](./architecture-review-checklist.md) ·
[non-duplication-policy.md](./non-duplication-policy.md) ·
[design-review-process.md](./design-review-process.md) ·
[platform-architecture.md](./platform-architecture.md) ·
[design-principles.md](./design-principles.md) ·
[adr/README.md](./adr/README.md)

---

## Purpose

Spanda has grown from a language and compiler into a full autonomous systems platform — entity model,
recovery, distributed decisions, Control Center, SDKs, packages, providers, and solution blueprints.
Without deliberate governance, platforms accumulate duplicate capabilities, inconsistent abstractions,
and unmaintainable surface area.

Architecture governance exists to:

- Keep Spanda **coherent** and **maintainable** as it grows
- **Prevent architectural duplication** across crates, services, APIs, and packages
- Ensure every capability is **deliberate**, **testable**, **demonstrable**, and **documented**
- Align contributions with the [Unified Entity Model](./entity-model.md) and [layered
  architecture](./platform-architecture.md)

This is a **permanent** rule — not a one-time initiative.

---

## Scope

Architecture Review applies to:

| Category | Examples |
|----------|----------|
| Platform architecture | New layers, services, cross-cutting frameworks |
| Roadmap | New pillars, blueprint capabilities, platform expansions |
| Design proposals | RFCs, design docs, spike outcomes |
| ADRs | Significant architectural decisions ([docs/adr/](./adr/)) |
| Pull requests | New crates, services, APIs, SDK methods, Control Center features |
| Crates | Workspace members under `crates/` |
| Packages | Official and community packages under `packages/registry/` |
| Providers | Provider traits and dispatch paths |
| Plugins | Governance, compliance, and extension plugins |
| SDKs | Rust, Python, TypeScript client surfaces |
| Control Center | REST v1, desktop shell, operator workflows |
| APIs | REST, gRPC, CLI JSON contracts |
| CLI | New commands, flags, and output formats |
| Documentation | Architecture diagrams, capability claims, roadmap items |

When in doubt, **open an architecture proposal** rather than skipping review.

---

## Architecture Review Gate

Every proposal must answer the twelve gate questions documented in
[architecture-review-checklist.md](./architecture-review-checklist.md):

1. **Purpose** — user problem, production scenario, measurable value
2. **Existing architecture** — which capability this extends
3. **Duplication check** — crates, packages, providers, APIs, concepts
4. **Architecture fit** — layer and component type
5. **Entity model** — integration with the Unified Entity Model
6. **Security** — Trust, messaging, decision authority, immunity, safety, recovery
7. **Distributed autonomy** — reflex, local/fleet decisions, mesh, continuity
8. **Non-regression** — APIs, SDKs, CLI, examples, tests
9. **Testability** — unit, integration, simulation, security, CI
10. **Demonstrability** — example, CLI, SDK, API, Control Center, demo
11. **Maintainability** — long-term cost and ownership
12. **Release impact** — complexity, footprint, learning curve

Proposals that cannot justify duplication or architecture fit should be **rejected** or **redesigned**
to extend existing capabilities.

---

## Architecture Scorecard

Reviewers score each proposal across dimensions such as problem clarity, architecture fit, entity
integration, security, maintainability, testability, documentation, backward compatibility,
performance, and complexity.

**Overall recommendations:**

| Score | Meaning |
|-------|---------|
| **Strongly Recommend** | Clear problem, strong fit, low duplication risk — proceed |
| **Recommend** | Acceptable with documented tradeoffs — proceed with conditions |
| **Recommend Later** | Valid idea, wrong timing or missing prerequisites — defer |
| **Needs Redesign** | Duplication or fit issues — extend existing capability instead |
| **Reject** | Insufficient justification or misaligned with platform vision |

See [design-review-process.md](./design-review-process.md) for reviewer roles and workflow.

---

## Architecture Principles

These principles are **permanent** and override local convenience:

| Principle | Guidance |
|-----------|----------|
| **Extend before creating** | Prefer extending an existing service, crate, or API over a new subsystem |
| **Reuse before duplicating** | Search crates, packages, providers, and docs before adding parallel paths |
| **Compose before coupling** | Wire capabilities through traits, events, and entity metadata — not hard imports |
| **Prefer providers over forks** | Domain integrations belong in packages implementing provider traits |
| **Prefer plugins over core changes** | Optional governance/compliance behavior via plugins when possible |
| **Prefer packages over platform expansion** | Keep the workspace lean; see [lean-core.md](./lean-core.md) |
| **Prefer Entity integration** | Operate on `EntityRecord` and entity kinds — avoid parallel object models |
| **Prefer additive changes** | Stable APIs evolve additively; breaking changes require ADR and migration |
| **Keep the core small** | Language, compiler, runtime, entity infrastructure stay in workspace |
| **Keep APIs stable** | CLI JSON, REST, gRPC, and SDKs share models — see [design-principles.md](./design-principles.md) |
| **Every feature must be demonstrable** | Example, CLI path, or Control Center view required |
| **Every capability must be testable** | CI-covered tests planned before merge |
| **Every architectural decision must be documented** | ADR for significant changes; user docs for visible behavior |

These complement — not replace — [design-principles.md](./design-principles.md) and
[dependency-rules.md](./dependency-rules.md).

---

## Quality Gate

No architecture proposal may merge unless **all** of the following are complete:

- [ ] **Architecture Review** completed (issue, ADR, or PR checklist)
- [ ] **Duplication Analysis** completed — see [non-duplication-policy.md](./non-duplication-policy.md)
- [ ] **Security Review** completed — Trust, authority, attack surface documented
- [ ] **Compatibility Review** completed — APIs, SDKs, CLI, examples unaffected or migrated
- [ ] **Entity Integration** documented — how entities are read, updated, or emitted
- [ ] **Tests planned or added** — CI coverage for new behavior
- [ ] **Documentation updated** — user-facing docs, feature status, CHANGELOG as applicable
- [ ] **ADR created** — for major or cross-cutting architectural changes

CI enforcement (layer validation, documentation audit) remains in place; this gate adds **human
review** for intent and duplication.

---

## Roadmap Rule

Every **new roadmap item** in [ROADMAP.md](../ROADMAP.md) must include:

- Problem
- Architecture fit (layer and component type)
- Entity integration
- Existing capability extended
- Duplication analysis
- Security review
- Test plan
- Demo plan
- Release impact

Items missing these sections should not be accepted into the canonical roadmap until completed.
Use the [architecture proposal template](../.github/ISSUE_TEMPLATE/architecture-proposal.md) as a
starting point.

---

## Architecture Decision Records (ADRs)

Significant architectural changes require an ADR under [docs/adr/](./adr/).

**Required when:**

- Adding a workspace crate or platform service
- Introducing a new API surface or breaking an existing one
- Changing layer boundaries, dependency rules, or entity model contracts
- Choosing between alternatives with long-term platform impact

**Template sections:** Problem, Decision, Alternatives, Tradeoffs, Consequences, Compatibility,
Migration, Rejected Alternatives.

---

## Contributor Entry Points

| Action | Entry point |
|--------|-------------|
| Propose new platform capability | [Architecture proposal issue](../.github/ISSUE_TEMPLATE/architecture-proposal.md) |
| Implement after approval | Pull request — [PR template](../.github/PULL_REQUEST_TEMPLATE.md) |
| Record a major decision | [docs/adr/](./adr/) |
| Check duplication | [non-duplication-policy.md](./non-duplication-policy.md) |
| Review a proposal | [architecture-review-checklist.md](./architecture-review-checklist.md) |

See [CONTRIBUTING.md](../CONTRIBUTING.md#architecture-review) for contributor requirements.

---

## Relationship to Existing Governance

Architecture governance **extends** existing enforcement — it does not replace it:

| Mechanism | Role |
|-----------|------|
| `scripts/validate_architecture.py` | Layer classification, dependency waivers (CI) |
| [dependency-rules.md](./dependency-rules.md) | Allowed dependency directions |
| [module-ownership.md](./module-ownership.md) | Crate and module ownership |
| [design-principles.md](./design-principles.md) | Day-to-day decision checklist |
| [governance.md](./governance.md) | Operational governance framework (runtime) |
| **Architecture governance (this doc)** | Intent, duplication, and cross-cutting review |

---

## Success Criteria

Spanda establishes a permanent architecture governance process that:

- Prevents uncontrolled platform growth
- Eliminates duplicate capabilities across crates, packages, and APIs
- Encourages reuse of existing subsystems (Entity Model, Recovery, Readiness, Trust, …)
- Ensures every architectural decision is deliberate, testable, maintainable, and aligned with the
  platform vision

The process is part of engineering culture, documentation, roadmap planning, and code review.
