# Cognitive & Resilience Architecture — Migration Notes

This document describes the evolution from **Bio-Inspired Resilient Autonomy** naming to the **Cognitive & Resilience Architecture** functional view. **No breaking changes** — all existing APIs, CLI commands, crate names, and SDK methods remain valid.

## What changed

| Before | After |
|--------|-------|
| "Bio-inspired resilient autonomy" as primary framing | **Cognitive & Resilience Architecture** as canonical functional view |
| Concept list in [bio-inspired-architecture.md](./bio-inspired-architecture.md) | Eleven **functional domains** in [functional-domains.md](./functional-domains.md) |
| Implicit service overlap | Explicit [responsibility-matrix.md](./responsibility-matrix.md) |
| `AutonomyClient` only | Domain SDK clients (`HomeostasisClient`, `AttentionClient`, …) + `AutonomyClient` facade |
| Control Center "Resilient Autonomy" tab | **Cognitive & Resilience** tab (same `resilient-autonomy` route ID) |
| `EntityAutonomyProfile` without `attention` | Added `Entity.attention` snapshot |

## What did not change

- Crate name: `spanda-autonomy` (unchanged)
- REST prefix: `/v1/autonomy/*` (unchanged)
- Entity field: `Entity.autonomy` / `EntityAutonomyProfile` (additive `attention` field only)
- CLI commands: `spanda reflex`, `homeostasis`, `immunity`, `fusion`, `alerts`, `recovery confidence` (unchanged)
- gRPC RPC names: `GetAutonomyHomeostasis`, etc. (unchanged)
- [bio-inspired-architecture.md](./bio-inspired-architecture.md) — retained; links to cognitive-resilience doc

## New documentation

| Document | Role |
|----------|------|
| [cognitive-resilience-architecture.md](./cognitive-resilience-architecture.md) | Canonical architecture view |
| [functional-domains.md](./functional-domains.md) | Domain definitions |
| [responsibility-matrix.md](./responsibility-matrix.md) | Capability ownership |
| [platform-homeostasis.md](./platform-homeostasis.md) | Homeostasis domain guide |
| [attention-engine.md](./attention-engine.md) | Attention domain guide |
| [damage-risk.md](./damage-risk.md) | Damage risk domain guide |
| [adaptive-operations.md](./adaptive-operations.md) | Adaptive learning domain guide |

## SDK migration

### Rust

```rust
// Before (still supported)
client.autonomy().homeostasis_summary()?;

// After (preferred)
client.homeostasis().summary()?;
client.attention().queue()?;
client.reflex().list()?;
```

### TypeScript

```typescript
// Before (still supported)
await client.getAutonomyHomeostasis();

// After (preferred)
await client.homeostasis().summary();
await client.attention().queue();
```

### Python

```python
# Before (still supported)
client.get_autonomy_homeostasis()

# After (preferred)
client.homeostasis().summary()
client.attention().queue()
```

## Entity schema migration

`EntityAutonomyProfile` gains optional `attention: EntityAttentionSnapshot`. Existing JSON without `attention` deserializes with `None` — backward compatible.

## Contributor guidance

1. Assign new capabilities to a **functional domain** first ([functional-domains.md](./functional-domains.md)).
2. Check [responsibility-matrix.md](./responsibility-matrix.md) before creating new services.
3. Integrate via `Entity.autonomy` — do not introduce parallel object models.
4. Do **not** name modules after brain anatomy.

## Timeline

| Phase | Deliverable | Status |
|-------|-------------|--------|
| A | Documentation + responsibility matrix | Complete |
| B | Entity `attention` field + SDK clients | Complete |
| C | Control Center domain organization | Complete |
| D | Cross-domain integration tests | Complete |
| E | gRPC/OpenAPI for `/v1/autonomy/fusion` and `/memory` | Complete |
