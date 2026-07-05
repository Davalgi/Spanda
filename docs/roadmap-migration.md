# Roadmap migration notes

**Date:** 2026-06-28

## Summary

The Spanda roadmap was restructured into a **product ecosystem** presentation. Scope is unchanged —
every feature, blueprint, and milestone from the prior `docs/roadmap.md` is preserved in
**[ROADMAP.md](../ROADMAP.md)**.

## What changed

| Before | After |
|--------|-------|
| Single `docs/roadmap.md` mixing language, ops, enterprise, blueprints | **`ROADMAP.md`** at repo root with 8 Platform Pillars |
| Solution blueprints as a subsection | Dedicated [Official Solution Blueprints](../ROADMAP.md#official-solution-blueprints) with 14 industries |
| Implicit product naming | Explicit product family (Spanda Language, Runtime, Verify, Readiness, …) |
| Flat feature lists | [Feature ownership model](../ROADMAP.md#feature-ownership-model) (Core / Package / Provider / Blueprint / …) |
| Release tables only | [Timeline by maturity](../ROADMAP.md#roadmap-timeline) (Now / Next / Later / Long Term / Research) |
| Scattered diagrams | [Dependency maps](../ROADMAP.md#dependency-map) (pillars → packages → providers → blueprints) |

## What did not change

- No features removed or descoped
- `docs/feature-status.md` remains the tier truth table
- Topic guides (`docs/mission-continuity.md`, `docs/control-center.md`, etc.) unchanged
- Deep-dive roadmaps (`enterprise-operations-roadmap.md`, `differentiation-roadmap.md`, …) unchanged
  — linked from pillars
- Examples, packages, and crates layout unchanged (reorganization is **recommended**, not executed)

## Link updates

| Old anchor / habit | New location |
|--------------------|--------------|
| `docs/roadmap.md#language` | [ROADMAP.md § Pillar 1](../ROADMAP.md#pillar-1--spanda-language) |
| `docs/roadmap.md#runtime` | [ROADMAP.md § Pillar 2](../ROADMAP.md#pillar-2--compiler--runtime) |
| `docs/roadmap.md#verification` | [ROADMAP.md § Pillar 3](../ROADMAP.md#pillar-3--verification-platform) |
| `docs/roadmap.md#fleet` | [ROADMAP.md § Pillar 4](../ROADMAP.md#pillar-4--device--fleet-platform) |
| `docs/roadmap.md#packages` | [ROADMAP.md § Pillar 8](../ROADMAP.md#pillar-8--packages--ecosystem) |
| `docs/roadmap.md#tooling` | [ROADMAP.md § Pillar 7](../ROADMAP.md#pillar-7--developer-platform) |
| `docs/roadmap.md#enterprise-operations` | [ROADMAP.md § Pillar 6](../ROADMAP.md#pillar-6--operations-platform) |
| `docs/roadmap.md#official-solution-blueprints` | [ROADMAP.md § Blueprints](../ROADMAP.md#official-solution-blueprints) |

## For contributors

1. Edit **`ROADMAP.md`** when adding or reclassifying roadmap items
2. Update **`docs/feature-status.md`** when tier changes (Stable / Experimental / Future)
3. Add industry-specific detail under `docs/solutions/{industry}.md` for new blueprints
4. Keep `docs/roadmap.md` as a redirect — do not restore the monolithic document

## For website / README

- README links to Platform Pillars and Solution Blueprints sections
- Website nav: Platform, Language, Packages, Solutions, Architecture, Roadmap, Control Center,
  Documentation, Examples, Community
