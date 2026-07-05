# Spanda Documentation

<p align="center">
  <img src="../../assets/image/low_res_logo.png" alt="Spanda logo" width="280">
</p>

**Spanda is an Autonomous Systems Platform with a safety-first programming language at its core.**
The `.sd` language, verification engine, simulation, replay, health framework, fleet tooling, and
package registry ship as one lean-core, package-first toolchain.

*Pronounced **SPUN-duh** (/ˈspʌndə/)* — Sanskrit for *the divine pulse*. Philosophy and etymology:
[Philosophy](./repo-docs/docs/overview/philosophy.md).

Platform overview: [Platform overview](./repo-docs/docs/platform-overview.md)

## Quick links

| Topic | Guide |
|-------|-------|
| Platform components | [Platform Overview](./repo-docs/docs/platform-overview.md) |
| Install & first program | [Getting Started](./repo-docs/docs/getting-started.md) |
| Language syntax | [Language Guide](./repo-docs/docs/spanda-language.md) |
| Safety & verification | [Architecture](./repo-docs/docs/architecture.md) |
| Hardware profiles | [Hardware Compatibility](./repo-docs/docs/hardware-compatibility.md) |
| Capabilities & traceability | [Capability Traceability](./repo-docs/docs/capability-traceability.md) |
| Health monitoring | [Health Checks](./repo-docs/docs/health-checks.md) |
| Mission continuity | [Mission Continuity](./repo-docs/docs/mission-continuity.md) |
| Tests | [Test Plan](./repo-docs/docs/test-plan.md) |

Build locally:

```bash
python3 scripts/sync_mdbook_sources.py
mdbook build docs-site
mdbook serve docs-site
```

Or from npm (when configured):

```bash
npm run docs:build
```
