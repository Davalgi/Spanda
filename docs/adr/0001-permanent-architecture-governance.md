# ADR 0001: Permanent Architecture Governance

- **Status:** Accepted
- **Date:** 2026-07-08
- **Authors:** Sujay Davalgi
- **Reviewers:** Architecture governance adoption (post-merge)
- **Related:** [architecture-governance.md](../architecture-governance.md) · commit `ce22a1d8`

---

## Problem

Spanda has grown from a language and compiler into a full autonomous systems platform — entity
model,
recovery, distributed decisions, Control Center, SDKs, packages, providers, and solution blueprints.
Automated CI enforces layer rules and dependency direction (`validate_architecture.py`), but there
was
no permanent **human review gate** for:

- Intent and user problem clarity before new crates or services land
- Duplication across crates, packages, providers, APIs, and SDKs
- Entity Model integration for new capabilities
- Security, compatibility, test, and demo requirements before merge
- Documented decisions (ADRs) for significant architectural choices

Without a formal process, the platform risked uncontrolled growth, parallel object models, and
duplicate operational engines — undermining coherence and maintainability.

---

## Decision

Establish **permanent Architecture Governance** as a required engineering process:

1. **Documentation** — `architecture-governance.md`, twelve-gate checklist, non-duplication policy,
   design review process, and ADR template under `docs/adr/`.
2. **Contributor workflow** — `CONTRIBUTING.md` requires Architecture Review for new platform
   services, crates, APIs, SDKs, Control Center features, and roadmap items with platform scope.
3. **GitHub templates** — architecture proposal issue template and PR template with quality gate
   checkboxes.
4. **Roadmap rule** — new roadmap items must include Problem, Architecture Fit, Entity Integration,
   Duplication Analysis, Security Review, Test Plan, Demo Plan, and Release Impact.
5. **Scorecard** — proposals receive structured review outcomes: Strongly Recommend through Reject.

Automated CI validation remains unchanged; this ADR adds **process and culture** on top of
mechanical
enforcement.

---

## Alternatives

### Alternative A — CI-only enforcement

- **Description:** Extend `validate_architecture.py` to block merges without checklist metadata.
- **Pros:** Fully automated; no reviewer dependency.
- **Cons:** Cannot judge duplication intent, user problem fit, or security tradeoffs; high false
  positive risk; contributors would game checkbox fields.

### Alternative B — Informal code review only

- **Description:** Rely on PR reviewers to catch duplication ad hoc.
- **Pros:** Zero process overhead.
- **Cons:** Inconsistent; knowledge not recorded; roadmap items lack standard sections; no ADR trail.

### Alternative C — RFC-only process (no PR gate)

- **Description:** Require design docs but not PR template or roadmap rules.
- **Pros:** Lighter weight.
- **Cons:** Implementation often starts before review; decisions not indexed; roadmap stays
  unstructured.

---

## Tradeoffs

| Gain | Cost |
|------|------|
| Coherent platform growth; extend-before-create culture | Extra issue/PR steps for architectural work |
| Duplication caught at proposal stage | Reviewer time for scorecard |
| ADR trail for major decisions | Contributors must learn twelve-gate checklist |
| Roadmap items become actionable | Existing roadmap rows may lack new sections until backfilled |

---

## Consequences

**Positive:**

- New crates and services require explicit justification against existing capabilities.
- Entity Model integration is documented before merge.
- Major decisions have a durable record in `docs/adr/`.
- PR and issue templates make expectations visible to all contributors.

**Negative:**

- Small architectural PRs carry template overhead (mitigated: bug fixes marked N/A).
- Backfill needed for legacy roadmap items without governance sections.

**Neutral:**

- No runtime or API behavior change.
- No workspace version bump required (contributor/docs stream only).

---

## Compatibility

| Surface | Breaking? | Mitigation |
|---------|-----------|------------|
| REST / gRPC | No | — |
| SDKs | No | — |
| CLI | No | — |
| Entity Model | No | — |
| Contributor workflow | Additive | `CONTRIBUTING.md` documents new requirements |

---

## Migration

1. **Immediate:** All new platform proposals use the architecture proposal template.
2. **PRs in flight:** Add architecture section from PR template when touching platform scope.
3. **Roadmap:** New items use required sections; existing items backfilled opportunistically when
   edited — not a blocking bulk rewrite.
4. **ADRs:** Significant past decisions may be recorded retroactively when touched; ADR 0001 records
   this governance decision itself.

---

## Rejected Alternatives

- **CI-only gate** — rejected; insufficient for intent and duplication judgment (Alternative A).
- **Informal review only** — rejected; does not scale with platform growth (Alternative B).
- **RFC-only without PR/roadmap integration** — rejected; weak enforcement at merge time
  (Alternative C).

---

## Architecture Review

- **Duplication analysis:** Extends existing `platform-architecture.md`, `design-principles.md`, and
  `validate_architecture.py` — does not duplicate operational governance in `spanda-governance`.
- **Entity integration:** All new capabilities must document Entity Model usage per checklist §5.
- **Security review:** Checklist §6 references Trust, decision authority, platform immunity.
- **Test plan:** Checklist §9; no new CI job for human gate (process-based).
- **Demo plan:** Checklist §10; demonstrability required for platform features.

---

## References

- [architecture-governance.md](../architecture-governance.md)
- [architecture-review-checklist.md](../architecture-review-checklist.md)
- [non-duplication-policy.md](../non-duplication-policy.md)
- [design-review-process.md](../design-review-process.md)
- [platform-architecture.md](../platform-architecture.md)
- [CONTRIBUTING.md](../../CONTRIBUTING.md#architecture-review)
