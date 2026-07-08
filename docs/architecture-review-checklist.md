# Architecture Review Checklist

Use this checklist for every architecture proposal, RFC, ADR, roadmap item, and pull request that
adds or materially changes platform capabilities.

**Parent:** [architecture-governance.md](./architecture-governance.md) ·
**Process:** [design-review-process.md](./design-review-process.md) ·
**Duplication:** [non-duplication-policy.md](./non-duplication-policy.md)

Copy the sections below into an issue, ADR, or PR description and complete every item before
requesting review.

---

## 1. Purpose

**Why does this capability exist?**

- [ ] User problem stated clearly
- [ ] Production scenario described (who, where, when)
- [ ] Measurable value identified (latency, safety, operability, adoption, …)

<details>
<summary>Prompts</summary>

- What breaks or fails today without this?
- Who is the primary operator or developer beneficiary?
- What metric or outcome improves?

</details>

---

## 2. Existing Architecture

**Which existing platform capability does this extend?**

Select all that apply and explain the extension path:

- [ ] Entity Model · Entity Graph · Entity Registry
- [ ] Recovery · Recovery Orchestrator · Recovery Graph
- [ ] Readiness · Health · Trust
- [ ] Distributed Decisions · Reflex · Local / Fleet / Control Center layers
- [ ] Mission Continuity · Continuity policies
- [ ] Provider Framework · Package System · Plugin Framework
- [ ] Control Center · SDKs · REST / gRPC / CLI
- [ ] Governance · Platform Immunity · Homeostasis
- [ ] Telemetry · Events · Replay
- [ ] Other: _______________

**Preference:** A proposal that extends an existing capability is preferred over introducing a new
subsystem. If proposing a new subsystem, justify in section 3.

---

## 3. Duplication Check

**Does this duplicate any existing artifact?**

Check each category. If **yes**, explain why extending the existing capability is insufficient. If
justification is weak, **reject or redesign**.

| Category | Duplicate? | Existing artifact | Why extension is insufficient |
|----------|------------|-------------------|-------------------------------|
| Crate | ☐ Yes ☐ No | | |
| Package | ☐ Yes ☐ No | | |
| Provider | ☐ Yes ☐ No | | |
| Plugin | ☐ Yes ☐ No | | |
| Runtime service | ☐ Yes ☐ No | | |
| API (REST/gRPC/CLI) | ☐ Yes ☐ No | | |
| SDK surface | ☐ Yes ☐ No | | |
| Control Center feature | ☐ Yes ☐ No | | |
| Architecture concept | ☐ Yes ☐ No | | |

**Search before proposing:**

- `crates/README.md` and workspace members
- `packages/registry/` official packages
- [platform-services.md](./platform-services.md) and [responsibility-matrix.md](./responsibility-matrix.md)
- [entity-overview.md](./entity-overview.md) documentation map
- Existing REST/gRPC paths in [control-center-api.md](./control-center-api.md)

See [non-duplication-policy.md](./non-duplication-policy.md) for rejection criteria.

---

## 4. Architecture Fit

**Where does this belong?**

Select **one primary** placement (secondary placements optional):

- [ ] Core Runtime (L2 — scheduler, interpreter, triggers, comm, safety)
- [ ] Platform Service (L4 — readiness, recovery, trust, telemetry, …)
- [ ] Provider (package-backed trait implementation)
- [ ] Plugin (optional governance/compliance extension)
- [ ] Package (`packages/registry/`)
- [ ] SDK (Rust / Python / TypeScript)
- [ ] CLI / API (L5 — interfaces)
- [ ] Control Center (UI + REST v1)
- [ ] Example / Blueprint (L6 — composes platform only)
- [ ] Documentation only

**Do not** introduce new architectural layers unless absolutely necessary. New layers require an ADR
and architecture maintainer approval.

Reference: [platform-architecture.md](./platform-architecture.md) layered model.

---

## 5. Entity Model

**How does this integrate with the Unified Entity Model?**

- [ ] Reads entity state from `EntityRegistry` / entity APIs
- [ ] Writes or updates entity metadata (document fields)
- [ ] Emits entity-linked events (see [event-model.md](./event-model.md))
- [ ] N/A — justified (e.g. pure compiler pass with no runtime entity)

Avoid disconnected object models (`RobotRecord`, parallel inventories, blueprint-local registries).

Reference: [entity-model.md](./entity-model.md), [entity-best-practices.md](./entity-best-practices.md).

---

## 6. Security

**Does it respect platform security contracts?**

- [ ] Trust framework ([entity-trust.md](./entity-trust.md), [trust-framework.md](./trust-framework.md))
- [ ] Secure messaging / decision traceability
- [ ] Decision authority ([decision-authority.md](./decision-authority.md))
- [ ] Governance ([governance.md](./governance.md))
- [ ] Platform immunity ([platform-immunity.md](./platform-immunity.md))
- [ ] Safety (structural types, capability traceability)
- [ ] Recovery (does not bypass recovery gates)

**New attack surface:**

| Surface | Description | Mitigation |
|---------|-------------|------------|
| | | |

Reference: [security-architecture.md](./security-architecture.md),
[distributed-decision-security.md](./distributed-decision-security.md).

---

## 7. Distributed Autonomy

**Does it interfere with distributed decision layers?**

Document interactions with:

- [ ] Reflex layer ([reflex-architecture.md](./reflex-architecture.md))
- [ ] Local decisions ([local-decision-trees.md](./local-decision-trees.md))
- [ ] Fleet decisions ([distributed-decisions.md](./distributed-decisions.md))
- [ ] Control Center approval paths
- [ ] Recovery and Recovery Orchestrator
- [ ] Mission Continuity
- [ ] Entity Mesh / fleet mesh

Explain precedence, offline behavior, and conflict resolution if applicable.

---

## 8. Non-Regression

**Can this break existing surfaces?**

| Surface | At risk? | Prevention |
|---------|----------|------------|
| REST / gRPC APIs | ☐ | |
| SDKs (Rust, Python, TS) | ☐ | |
| CLI commands / JSON output | ☐ | |
| Examples / golden tests | ☐ | |
| Entity Model contracts | ☐ | |
| Recovery / Readiness / Trust | ☐ | |
| Control Center | ☐ | |

- [ ] Cross-surface protocol followed if APIs changed ([ci-architecture.md](./ci-architecture.md))
- [ ] CHANGELOG entry planned for user-visible changes

---

## 9. Testability

**How will this be tested?**

- [ ] Unit tests (crate / module level)
- [ ] Integration tests (service + entity + API)
- [ ] Simulation / replay ([replay.md](./replay.md))
- [ ] Security / attack simulation
- [ ] Performance / soak (if latency- or resource-sensitive)
- [ ] Golden tests / fixture updates
- [ ] CI job identified (Fast / Integration / Nightly)

List concrete test files or scenarios: _______________

---

## 10. Demonstrability

**Can this be demonstrated?**

Provide at least one:

- [ ] Runnable example (`.sd` under `examples/`)
- [ ] CLI command path
- [ ] SDK usage snippet
- [ ] REST/gRPC call
- [ ] Control Center view or workflow
- [ ] Recorded demo or walkthrough doc

If it cannot be demonstrated, reconsider whether it belongs in the platform.

---

## 11. Maintainability

**Long-term cost estimate**

| Cost type | Estimate (Low / Medium / High) | Notes |
|-----------|-------------------------------|-------|
| Maintenance | | |
| Documentation | | |
| Testing | | |
| Future compatibility | | |
| Migration | | |

- [ ] Named owner or module assignment ([module-ownership.md](./module-ownership.md))
- [ ] No new permanent architecture waivers (target: zero — see [architecture-waiver-burn-down.md](./architecture-waiver-burn-down.md))

---

## 12. Release Impact

**Does this affect release posture?**

- [ ] Delays current release milestone
- [ ] Increases platform complexity
- [ ] Increases contributor learning curve
- [ ] Increases runtime footprint (binary size, memory, deps)
- [ ] Increases attack surface

If any checked, justify why value outweighs cost.

**Stream impact:** workspace / sdk / desktop / proto — see [versioning.md](./versioning.md).

---

## Architecture Scorecard

Reviewers complete after reading the proposal. Use one row per dimension.

| Dimension | Score (1–5) | Notes |
|-----------|-------------|-------|
| Problem clarity | | |
| Architecture fit | | |
| Entity integration | | |
| Security | | |
| Maintainability | | |
| Testability | | |
| Documentation | | |
| Backward compatibility | | |
| Performance | | |
| Complexity (5 = simple) | | |

**Overall recommendation** (select one):

- [ ] **Strongly Recommend** — proceed
- [ ] **Recommend** — proceed with documented conditions
- [ ] **Recommend Later** — defer; list prerequisites
- [ ] **Needs Redesign** — extend existing capability instead
- [ ] **Reject** — does not meet governance bar

**Reviewer(s):** _______________  
**Date:** _______________  
**Linked ADR / issue / PR:** _______________

---

## Quick Reference — Gate Summary

| # | Gate | Fail if |
|---|------|---------|
| 1 | Purpose | Problem vague or non-production |
| 2 | Existing architecture | New subsystem without extension path |
| 3 | Duplication | Duplicate with no justification |
| 4 | Architecture fit | Wrong layer or unnecessary new layer |
| 5 | Entity model | Parallel object model without justification |
| 6 | Security | Bypasses trust, authority, or safety |
| 7 | Distributed autonomy | Breaks reflex/local/fleet precedence |
| 8 | Non-regression | Breaks APIs without migration plan |
| 9 | Testability | No CI test plan |
| 10 | Demonstrability | No example or demo path |
| 11 | Maintainability | Unowned or high cost unjustified |
| 12 | Release impact | Cost exceeds value |
