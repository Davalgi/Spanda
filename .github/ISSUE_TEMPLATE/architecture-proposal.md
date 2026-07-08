---
name: Architecture Proposal
about: Propose a new platform capability, crate, service, API, or architectural change
title: '[Architecture]: '
labels: architecture-proposal
assignees: ''
---

Use this template for platform architecture changes **before** large implementation work.
See [docs/architecture-governance.md](../../docs/architecture-governance.md) and
[docs/architecture-review-checklist.md](../../docs/architecture-review-checklist.md).

For language syntax changes, use the Language Proposal template instead.
For community packages, use the Package Proposal template (include duplication analysis below).

---

## 1. Purpose

**Why does this capability exist?**

- User problem:
- Production scenario:
- Measurable value:

---

## 2. Existing Architecture

**Which existing platform capability does this extend?**

<!-- Entity Model, Recovery, Readiness, Trust, Distributed Decisions, Provider Framework, Control Center, SDKs, etc. -->

- Extended capability(ies):
- Extension approach:

---

## 3. Duplication Check

**Does this duplicate an existing crate, package, provider, plugin, service, API, SDK, Control Center feature, or architecture concept?**

| Category | Duplicate? | Existing artifact | Why extension is insufficient |
|----------|------------|-------------------|-------------------------------|
| Crate | | | |
| Package | | | |
| Provider | | | |
| Plugin | | | |
| Runtime service | | | |
| API | | | |
| SDK | | | |
| Control Center | | | |
| Architecture concept | | | |

**Search performed:** <!-- List docs, crates, packages consulted -->

---

## 4. Architecture Fit

**Primary placement** (one):

- [ ] Core Runtime
- [ ] Platform Service
- [ ] Provider
- [ ] Plugin
- [ ] Package
- [ ] SDK
- [ ] CLI / API
- [ ] Control Center
- [ ] Example / Blueprint
- [ ] Documentation only

**New layer proposed?** Yes / No — if yes, justify:

---

## 5. Entity Model Integration

<!-- How this reads/writes entities or emits entity-linked events. Or justify N/A. -->

---

## 6. Security

<!-- Trust, secure messaging, decision authority, governance, platform immunity, safety, recovery. New attack surface. -->

---

## 7. Distributed Autonomy

<!-- Interactions with reflex, local/fleet decisions, Control Center, recovery, mission continuity, entity mesh. -->

---

## 8. Non-Regression

<!-- Risk to APIs, SDKs, CLI, examples, tests. Prevention plan. -->

---

## 9. Testability

<!-- Unit, integration, simulation, security, performance, golden, CI. -->

---

## 10. Demonstrability

<!-- Example, CLI, SDK, API, Control Center, demo plan. -->

---

## 11. Maintainability

| Cost type | Estimate (Low / Medium / High) | Notes |
|-----------|-------------------------------|-------|
| Maintenance | | |
| Documentation | | |
| Testing | | |
| Future compatibility | | |
| Migration | | |

**Proposed owner:**

---

## 12. Release Impact

<!-- Delays, complexity, learning curve, footprint, attack surface. Stream: workspace / sdk / desktop. -->

---

## Architecture Scorecard (reviewers)

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
| Complexity | | |

**Overall recommendation:**

- [ ] Strongly Recommend
- [ ] Recommend
- [ ] Recommend Later
- [ ] Needs Redesign
- [ ] Reject

---

## Additional context

<!-- Diagrams, links, prior art, related issues. -->
