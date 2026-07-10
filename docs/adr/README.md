# Architecture Decision Records (ADRs)

Index of significant architectural decisions for the Spanda platform.

**Governance:** [architecture-governance.md](../architecture-governance.md) ·
**Process:** [design-review-process.md](../design-review-process.md) ·
**Template:** [template.md](./template.md)

---

## When to Write an ADR

Create an ADR when a change:

- Adds a workspace crate or platform service
- Introduces or breaks a public API (REST, gRPC, CLI JSON, SDK)
- Changes layer boundaries or dependency rules
- Alters Unified Entity Model contracts
- Chooses between alternatives with long-term platform impact

Minor bug fixes and internal refactors without contract changes do not require ADRs.

---

## How to Add an ADR

1. Copy [template.md](./template.md) to `NNNN-short-title.md` (next sequential number).
2. Set **Status** to `Proposed`.
3. Open a PR or link from an [architecture proposal
   issue](../../.github/ISSUE_TEMPLATE/architecture-proposal.md).
4. After Architecture Review, set status to `Accepted`, `Rejected`, or `Superseded`.
5. Add a row to the index table below.

---

## Index

| ADR | Title | Status | Date |
|-----|-------|--------|------|
| [0001](./0001-permanent-architecture-governance.md) | Permanent Architecture Governance | Accepted | 2026-07-08 |
| [0002](./0002-std-policies-package.md) | Official `std.policies.*` package scaffolds | Accepted | 2026-07-10 |

---

## Status Definitions

| Status | Meaning |
|--------|---------|
| **Proposed** | Under review; not yet binding |
| **Accepted** | Approved; implement per ADR |
| **Rejected** | Not adopted; rationale preserved |
| **Superseded** | Replaced by a newer ADR (link successor) |
| **Deprecated** | Still in codebase but scheduled for removal |

---

## Relationship to Other Docs

- **Architecture proposals (issues)** — discussion and scorecard before implementation
- **ADRs** — durable record of the decision and tradeoffs
- **Platform architecture docs** — current state of the system
  ([platform-architecture.md](../platform-architecture.md))
- **RFCs** (optional) — exploratory designs; accepted outcomes become ADRs
