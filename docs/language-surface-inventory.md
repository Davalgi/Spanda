# Language surface: primitives vs library-shaped constructs

**Status:** Design note (inventory + migration path)  
**Date:** 2026-07-10  
**Related:** [verification-vocabulary.md](./verification-vocabulary.md) · [architecture-governance.md](./architecture-governance.md) · [non-duplication-policy.md](./non-duplication-policy.md)

## Goal

Shrink the hard-coded grammar surface by classifying every top-level / robot-member
declaration keyword, then providing an extensibility path so **library-shaped** constructs
can move to packages without losing today’s syntax (deprecation-preserving).

## Classification

| Class | Meaning |
|-------|---------|
| **primitive** | Core language or safety/runtime contract — keep in grammar |
| **borderline** | Useful as syntax today; could become attrs/types later |
| **library-shaped** | Framework/policy/model concern — prefer packages + registration |

`†` = soft keyword (Ident-dispatched), not always in the lexer reserved set.

### Primitive (keep)

| Keyword | Scope | Reasoning |
|---------|-------|-----------|
| `module`, `import`, `fn`, `export`/`public`/`private`/`async`, `extern`, `test`† | program | Module / function system |
| `struct`, `enum`, `trait`, `impl` | program/robot | Type system |
| `hardware`, `deploy`, `requires_hardware` | program/both | Deploy / compatibility core |
| `fleet`, `swarm` | program | Multi-robot orchestration |
| `robot`, `sensor`, `actuator`, `soc`, `hal`, `node` | robot | Entity graph |
| `topic`, `service`, `action`, `message`, `bus`, `device` | robot/program | Comm / interconnect |
| `safety`, `ai_model`, `agent`, `behavior`, `task` | robot | Safety-typed AI + control |
| `state_machine`, `event`, `on`/`every`/`when`/`while` | robot | Reactivity |
| `verify` / `assert`, `observe` | robot | Runtime assertions / fusion |

### Borderline

| Keyword | Scope | Reasoning |
|---------|-------|-----------|
| `requires_network`, `requires_connectivity` | both | Platform gates; could be typed attrs |
| `geofence`, `safety_zone` | program | Safety data; could be typed under a generic safety DSL |
| `certify` | program | Declared metadata (see verification vocabulary) |
| `validate`†, `kill_switch`† | program/both | Safety-ish but pack-shaped |
| `pipeline`, `watchdog`, `retry`, `recover` | robot | Reliability — often library patterns elsewhere |
| `twin` / `twin sync`, `mission`, `mode`†, `operating_mode`† | robot/program | Product-core but policy-like |
| `secrets`/`secret`, `identity`†, `trust`, `permissions` | robot | Platform security vs config packs |
| `uses` (+ hardware) | robot | Binding; could be an attribute |

### Library-shaped (migrate candidates)

| Keyword | Scope | Reasoning |
|---------|-------|-----------|
| `homeostasis_policy`†, `attention_policy`† | program | Thin named rule/metric lists |
| `knowledge_model`†, `state_estimator`†, `anomaly_detector`†, `prognostics`† | program | “Register a named model” shape |
| `assurance_case`†, `record`†, `provenance`†, `audit`† | program/robot | Assurance / compliance artifacts |
| `policy`, `resilience_policy`†, `recovery_policy`†, `tamper_policy`†, `continuity_policy`†, `offline_policy`†, `health_policy`†, `restart_policy`†, `connectivity_policy` | program/robot | Repeated named-ruleset pattern |
| `decision_tree`†, `mitigation`†, `mission_plan`† | program | Plan / decision artifacts |
| `health_check`†, `heartbeat`†, `memory_watch`†, `resource_watch`† | both/robot | Probe / watch config |
| `world_model`†, `secure_comm`†, `trust_boundary`† | robot | Domain / security packs |
| `exposes`† capabilities, `local_decision_authority`†, `requires_central_approval`† | robot | Governance lists |
| `ble_service`, `bluetooth` | program/robot | Protocol-specific |

## Extensibility mechanism (proposed)

**Do not** invent a second parallel language. Prefer one of:

### Option A — Attributes on library types (preferred for thin policies)

```spanda
import std.policies.homeostasis;

@policy(kind: "homeostasis")
HomeostasisPolicy PatrolHomeostasis {
  metric battery_soc;
  metric thermal_margin;
}
```

- Parser accepts `@attr` / `#attr` annotations on `struct` / `type` / future `policy` forms.
- Registry maps `(kind, type)` → runtime hooks (same as today’s homeostasis monitor).
- Old `homeostasis_policy Name { … }` desugars to the annotated form in the parser (compat shim).

### Option B — Trait-based registration API

```spanda
trait PolicyHost {
  fn register_homeostasis(name: String, metrics: List<String>);
}
```

- Heavier; better for providers that need custom evaluation.
- Complements Option A for packages that ship evaluators.

### Migration path (deprecation-preserving)

1. **Inventory** (this doc) — done.
2. **Lint warnings** on library-shaped soft keywords (`homeostasis_policy`, `attention_policy`, …)
   pointing here — additive; no breakage.
3. **Desugar shim** — parser still accepts old syntax; emits annotated IR / AST flag `legacy_syntax`.
4. **Package APIs** — move evaluators into `packages/registry` / `std.policies.*`.
5. **Docs + examples** — new code uses attributes; old examples keep working with lint.
6. **Hard remove** — only after a major version and empty usage in official examples.

## Proof of concept

Shipped:

- Inventory + design note
- Lint rule `library-shaped-decl` on legacy `homeostasis_policy` / `attention_policy` only
- **`@policy(kind: "homeostasis")`** and **`@policy(kind: "attention")`** parse paths
  (`legacy_syntax = false`); legacy keywords still parse (`legacy_syntax = true`)
- Feature examples under `examples/features/` use the attribute form

Still deferred: hard-remove of legacy keywords (major version only).

Shipped package APIs: official **`spanda-policies`** → `std.policies.homeostasis` /
`std.policies.attention` (scaffolds; evaluation remains in `spanda-autonomy`). See
[ADR 0002](./adr/0002-std-policies-package.md).

AST → evaluator wiring (shipped): `spanda homeostasis check --program <file.sd>` and
`spanda attention check --program <file.sd>` apply declared metrics/rules via
`HomeostasisPolicy::from_declared_metrics` / `AttentionPolicy::from_declared_rules`.
Still optional: Control Center / REST using the same path.

## Non-goals

- Removing `safety` / `ai_model` / `agent` / ActionProposal gate from the grammar
- Hard-renaming `verify` / `certify` (see [verification-vocabulary.md](./verification-vocabulary.md))
- Expanding core with new `*_policy` keywords
