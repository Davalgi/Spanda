# ADR 0002: Official `std.policies.*` package scaffolds

- **Status:** Accepted
- **Date:** 2026-07-10
- **Authors:** Spanda maintainers
- **Reviewers:** Architecture Review (PR)
- **Related:** [language-surface-inventory.md](../language-surface-inventory.md) · PR for `spanda-policies`

---

## Problem

Library-shaped `homeostasis_policy` / `attention_policy` keywords are migrating to
`@policy(kind: …)`. The inventory calls for package APIs under `std.policies.*` so programs can
`import` helpers without expanding core grammar. Without a package, the attribute form has no
registry home and evaluators stay undocumented relative to language surface.

---

## Decision

Ship official package **`spanda-policies`** exporting:

- `std.policies.homeostasis` — helpers aligned with `HomeostasisPolicy::platform_defaults`
- `std.policies.attention` — helpers aligned with feature-example attention rules

**Evaluation remains in `spanda-autonomy`** (CLI/API/Control Center). The package is a scaffold /
import surface only (same pattern as `spanda-resilience`).

Register import paths in `spanda-package` adapter + typecheck catalogs + `std_namespaces` type names.

---

## Alternatives

### Alternative A — Put helpers only in `assurance.*`

- **Pros:** Matches existing assurance package naming
- **Cons:** Inventory and docs already specify `std.policies.*`; std prefix signals language migration

### Alternative B — Reimplement evaluate/rank in `.sd`

- **Pros:** Self-contained package demos
- **Cons:** Duplicates `spanda-autonomy`; violates non-duplication policy

### Alternative C — Defer package until hard-remove of keywords

- **Pros:** Less surface now
- **Cons:** Blocks migration step 4; examples cannot `import` the preferred path

---

## Consequences

### Positive

- Completes inventory migration step 4 for homeostasis/attention
- Clear split: language/registry vs platform evaluation

### Negative / risks

- Package count +1; docs must track count via `ls packages/registry`
- `std.policies.*` is both std-namespace types and package modules — keep catalogs in sync

### Follow-ups

- Wire AST `@policy` metrics/rules into autonomy evaluators (optional)
- Hard-remove legacy keywords only after a major version

---

## Architecture Review (summary)

| Gate | Notes |
|------|-------|
| Purpose | Migration path for library-shaped policies |
| Existing | Extends package registry + autonomy (no new evaluator) |
| Duplication | Avoided by not reimplementing evaluate/rank |
| Entity | Policies remain program decls; entity snapshots unchanged |
| Security | No new trust/authority surface |
| Non-regression | Additive imports; legacy keywords kept |
| Testability | `tests/smoke.sd` |
| Demonstrability | Feature examples + package README |
| Release | Workspace docs/package count only |

---

## References

- [docs/language-surface-inventory.md](../language-surface-inventory.md)
- [docs/non-duplication-policy.md](../non-duplication-policy.md)
- `packages/registry/spanda-policies/`
