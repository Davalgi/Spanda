<p align="center">
  <img src="assets/image/banner.png" alt="Spanda — The Autonomous Systems Platform" width="640">
</p>

# Spanda

**The Autonomous Systems Platform** — *with a safety-first programming language at its core.*

*Build. Verify. Simulate. Deploy. Operate.*

Spanda is a **safety-first Autonomous Systems Platform** with a dedicated programming language at
its core. It orchestrates robots, devices, AI agents, vehicles, humans, and intelligent environments
using a unified Entity Model and built-in capabilities for readiness, assurance, recovery, trust,
health, distributed autonomy, and governance.

Spanda implements a [Cognitive & Resilience Architecture](docs/cognitive-resilience-architecture.md)
inspired by proven engineering principles observed in biological nervous systems — functional
concepts (reflexes, homeostasis, immunity, fusion, attention, memory) without biological anatomy in
code.

> Spanda is not a biologically inspired AI platform and does not attempt to model consciousness,
> emotions, or neural structures.

Spanda is an autonomous systems platform centered on the **Spanda Language** (`.sd` files): typed
robot programs, safety gates, hardware verification, cascading TOML configuration, simulation,
replay, fleet operations, mission assurance, mission continuity, and **38** official packages.

**Spanda focuses on Readiness, Assurance, and Diagnosis for safety-critical autonomous systems.**

Repository: [github.com/Davalgi/Spanda](https://github.com/Davalgi/Spanda)

**Current release:** **v0.6.3** (2026-07-04) — **evaluation / beta**. Suitable for pilots, demos,
and integration testing. Not a full production claim: default AI and IoT paths are mock-backed
unless live env is configured. See [docs/known-limitations.md](docs/known-limitations.md) ·
[docs/release-readiness.md](docs/release-readiness.md) · path to v1.0:
[docs/organizational-gates.md](docs/organizational-gates.md).

<p align="center">

![Rust](https://img.shields.io/badge/Rust-000000?style=flat&logo=rust&logoColor=white)
![Python](https://img.shields.io/badge/Python-3776AB?style=flat&logo=python&logoColor=white)
![TypeScript](https://img.shields.io/badge/TypeScript-3178C6?style=flat&logo=typescript&logoColor=white)
![Tauri](https://img.shields.io/badge/Tauri-24C8DB?style=flat&logo=tauri&logoColor=white)
![Docker](https://img.shields.io/badge/Docker-2496ED?style=flat&logo=docker&logoColor=white)
![gRPC](https://img.shields.io/badge/gRPC-4285F4?style=flat&logo=grpc&logoColor=white)
![ROS](https://img.shields.io/badge/ROS-22314E?style=flat&logo=ros&logoColor=white)
![WebAssembly](https://img.shields.io/badge/WebAssembly-654FF0?style=flat&logo=webassembly&logoColor=white)
![MQTT](https://img.shields.io/badge/MQTT-660066?style=flat&logo=eclipsemosquitto&logoColor=white)
![VS Code](https://img.shields.io/badge/VS%20Code-007ACC?style=flat&logo=visualstudiocode&logoColor=white)
<br>
[![crates.io](https://img.shields.io/crates/v/spanda-sdk?logo=rust)](https://crates.io/crates/spanda-sdk)
[![PyPI](https://img.shields.io/pypi/v/spanda-sdk?logo=pypi)](https://pypi.org/project/spanda-sdk/)
[![npm](https://img.shields.io/npm/v/@davalgi-spanda/sdk?logo=npm)](https://www.npmjs.com/package/@davalgi-spanda/sdk)
[![docs](https://img.shields.io/badge/docs-mdBook-orange?logo=rust)](https://davalgi.github.io/Spanda/)
![status](https://img.shields.io/badge/status-evaluation%20%2F%20beta-yellow)
![safety](https://img.shields.io/badge/focus-safety--first-blue)
![release](https://img.shields.io/github/v/release/Davalgi/Spanda?label=release)
![license](https://img.shields.io/github/license/Davalgi/Spanda)
<br>
![CI Fast](https://github.com/Davalgi/Spanda/actions/workflows/ci-fast.yml/badge.svg)
![CI Integration](https://github.com/Davalgi/Spanda/actions/workflows/ci-integration.yml/badge.svg)
![CI Nightly](https://github.com/Davalgi/Spanda/actions/workflows/ci-nightly.yml/badge.svg)
![last commit](https://img.shields.io/github/last-commit/Davalgi/Spanda)
![commit activity](https://img.shields.io/github/commit-activity/y/Davalgi/Spanda)
![contributors](https://img.shields.io/github/contributors/Davalgi/Spanda)
![open issues](https://img.shields.io/github/issues/Davalgi/Spanda)
![open PRs](https://img.shields.io/github/issues-pr/Davalgi/Spanda)
![top language](https://img.shields.io/github/languages/top/Davalgi/Spanda)
<br>
![stars](https://img.shields.io/github/stars/Davalgi/Spanda?style=social)
![forks](https://img.shields.io/github/forks/Davalgi/Spanda?style=social)
![watchers](https://img.shields.io/github/watchers/Davalgi/Spanda?style=social)
</p>

---

## Philosophy

Hardware is the body.  
Sensors are the senses.  
AI models are the mind.  
Actuators are the muscles.  
Spanda is the intelligent pulse that transforms perception, intent, and safety into action.

Autonomy is layered — reflex, spinal cord, brain — with the mind proposing and the pulse enforcing.
[Nervous system →](docs/overview/philosophy.md#nervous-system)

Spanda uses a **Cognitive & Resilience Architecture** inspired by proven engineering principles from
biological nervous systems: local reflexes, distributed coordination, sensory fusion, homeostasis,
platform immunity, operational memory, adaptive recovery, and attention management. The goal is not
to imitate biology, but to apply measurable resilience patterns to safety-critical autonomous
systems. [Cognitive & Resilience Architecture →](docs/cognitive-resilience-architecture.md)

> Spanda is not a biologically inspired AI platform and does not attempt to model consciousness,
> emotions, or neural structures. Biological concepts are used only where they provide measurable
> engineering benefits.

**Spanda** (*Pronounced **SPUN-duh** (/ˈspʌndə/)*) is a Sanskrit term meaning *the divine pulse* —
the creative vibration of consciousness and energy that manifests as expansion and contraction in
all entities, bridging stillness and movement within consciousness; and the first stir of awareness
that creates and sustains the universe.

---

## What is Spanda?

Spanda is an **autonomous systems platform** built around the **Spanda Language** — a typed
programming language where sensors, AI models, actuators, safety rules, and deployment targets are
first-class concepts in source code.

You write a `robot` block with sensors, actuators, safety zones, and agents. The compiler enforces
physical units, validates AI proposals before they reach hardware, and checks that your program fits
the deployment target before you ship.

```spanda
robot SafePatrol {
  local_decision_authority [emergency_stop, degraded_mode];

  sensor lidar: Lidar;
  actuator wheels: DifferentialDrive;
  ai_model planner: LLM { provider: "mock"; model: "patrol"; }

  safety {
    max_speed = 0.5 m/s;
    stop_if lidar.nearest_distance < 0.5 m;
  }

  behavior patrol() {
    loop every 100ms {
      let proposal = planner.reason(prompt: "Plan motion", input: lidar.read());
      wheels.execute(safety.validate(proposal));  // SafeAction gate — AI cannot drive hardware directly
    }
  }
}
```

Policy blocks (`decision_tree`, `recovery_policy`, `continuity_policy`, `homeostasis_policy`) extend
this pattern — see [examples/features/](examples/features/) and [Spanda 101 lesson
11](docs/spanda-101/11-distributed-decisions.md).

What Spanda is / isn't: [docs/overview/what-spanda-is.md](docs/overview/what-spanda-is.md) · Why
Spanda (detail): [docs/overview/philosophy.md](docs/overview/philosophy.md)

---

## Platform navigation

Spanda is a **product ecosystem** — not only a language repository.

| | |
|---|---|
| **[Platform Pillars](ROADMAP.md#platform-pillars)** | Language · Compiler & Runtime · Verification · Device & Fleet · Security · Operations · Developer · Packages |
| **[Official Solution Blueprints](ROADMAP.md#official-solution-blueprints)** | Warehouse · Search & Rescue · Healthcare · ADAS · Smart Factory · Agriculture · Critical Infrastructure · Environmental · Maritime · Transportation · Space · Defense · Research · Spatial HRI |
| **[Architecture overview](docs/platform-overview.md)** | Components, workflow, product family |
| **[Platform Architecture v2.0](docs/platform-architecture.md)** | Layers, dependency rules, ownership, CI validation — **0 production upward waivers** |
| **[Full roadmap](ROADMAP.md)** | Ownership model, dependency maps, timeline (Now / Next / Later / Long Term / Research) |

**Product family:** Spanda Language → Runtime → Verify → Readiness → Assurance → Diagnosis →
Recovery → Trust → Control Center → Registry → SDKs

---

## Unified Entity Model

Everything managed by Spanda is represented as an **Entity** — robots, humans, devices, wearables,
packages, providers, missions, facilities, and cloud services share one common architecture for
identity, health, readiness, trust, relationships, and lifecycle.

```text
Entity
 ├── Health / Readiness / Trust
 ├── Capabilities & Relationships
 └── Verification (verify_entity)
         ↓
   Platform Services (Verify, Readiness, Device Pool, Fleet)
```

- **Browse:** `spanda entity list --config
  crates/spanda-config/tests/fixtures/warehouse/spanda.toml` · Control Center **Entities** tab ·
  `GET /v1/entities`
- **Verify:** `spanda entity verify rover-001 --config
  crates/spanda-config/tests/fixtures/warehouse/spanda.toml` · `POST /v1/entities/{id}/verify`
- **Graph:** `spanda entity graph --config
  crates/spanda-config/tests/fixtures/warehouse/spanda.toml` · `GET /v1/entities/graph`

Guide: [docs/entity-model.md](docs/entity-model.md) · APIs:
[docs/entity-apis.md](docs/entity-apis.md) · SDK: [docs/entity-sdk.md](docs/entity-sdk.md) ·
Examples: [examples/entity/](examples/entity/) — **Stable** tier; SDKs **0.4.2** on crates.io, PyPI,
npm

Entity CLI commands require a project `spanda.toml` or `--config` (warehouse fixture above).

---

## Quick start

```bash
# Install (from clone)
git clone https://github.com/Davalgi/Spanda.git
cd Spanda && ./scripts/install.sh
# Or: cargo install --path crates/spanda-cli --locked

spanda demo rover          # flagship platform demo
spanda demo assurance      # mission assurance CLI suite
spanda demo self-healing   # recovery policies, heal/recover/sim
spanda recovery plan examples/showcase/self_healing/rover.sd --failure gps  # orchestrator
spanda recovery explain examples/showcase/self_healing/rover.sd --failure gps
spanda demo continuity     # takeover, delegation, succession
spanda decision list examples/showcase/distributed_decisions/main.sd  # brain/spinal-cord/reflex autonomy
spanda reflex list --json              # Cognitive & Resilience reflex catalog
spanda homeostasis check --json        # platform stability snapshot

# Or step by step:
spanda check examples/showcase/killer_demo.sd      # type-check
spanda verify examples/showcase/hardware_compatibility.sd  # hardware fit
spanda sim examples/showcase/killer_demo.sd        # simulate
```

Install options: [docs/installation.md](docs/installation.md) · First project:
[docs/getting-started.md](docs/getting-started.md) · Troubleshooting:
[docs/troubleshooting.md](docs/troubleshooting.md)

**Official SDKs** (Control Center API clients):

```bash
cargo add spanda-sdk
pip install spanda-sdk
npm install @davalgi-spanda/sdk
```

Guide: [docs/sdk.md](docs/sdk.md) · Publish: [docs/sdk-publishing.md](docs/sdk-publishing.md) ·
Desktop: [docs/desktop-release-runbook.md](docs/desktop-release-runbook.md) · Versioning:
[docs/control-center-versioning.md](docs/control-center-versioning.md) (`desktop-v0.6.3`)

---

## Explore further

| Topic | Guide |
|-------|--------|
| **5-minute eval & flagship demos** | [docs/overview/flagship-demos.md](docs/overview/flagship-demos.md) |
| **Where should I start?** (by role) | [docs/overview/where-to-start.md](docs/overview/where-to-start.md) |
| **Signature capabilities** | [docs/overview/signature-capabilities.md](docs/overview/signature-capabilities.md) |
| **Platform components** | [docs/overview/platform-components.md](docs/overview/platform-components.md) |
| **Feature status** | [docs/overview/feature-snapshot.md](docs/overview/feature-snapshot.md) · [docs/feature-status.md](docs/feature-status.md) |
| **Known limitations** | [docs/known-limitations.md](docs/known-limitations.md) · mock/live backend setup in [docs/troubleshooting.md](docs/troubleshooting.md) |
| **v0.6.3 → v1.0 gates** | [docs/organizational-gates.md](docs/organizational-gates.md) · field soak + security audit |
| **Distributed decisions** | **Stable** — [docs/distributed-decisions.md](docs/distributed-decisions.md) · [docs/distributed-decision-demo.md](docs/distributed-decision-demo.md) · `spanda decision simulate-attack` |
| **Cognitive & Resilience Architecture** | **Beta** — [docs/cognitive-resilience-architecture.md](docs/cognitive-resilience-architecture.md) · [docs/cognitive-resilience-maturity.md](docs/cognitive-resilience-maturity.md) · domain SDK clients · Control Center **Cognitive & Resilience** tab |
| **Recovery Orchestrator** | **Stable** — [docs/recovery-orchestrator.md](docs/recovery-orchestrator.md) · REST `/v1/recovery/*` · Control Center **Recovery** tab |
| **Demos & examples** | [docs/overview/demos-and-examples.md](docs/overview/demos-and-examples.md) |
| **Code samples** | [docs/overview/code-samples.md](docs/overview/code-samples.md) |
| **Platform feature examples** | [docs/platform-feature-examples.md](docs/platform-feature-examples.md) · [examples/workflows/](examples/workflows/) |
| **Differentiators** | [docs/overview/differentiators.md](docs/overview/differentiators.md) |

**Full overview index:** [docs/overview/README.md](docs/overview/README.md)

---

## Documentation

| Start here | Description |
|------------|-------------|
| [ROADMAP.md](ROADMAP.md) | **Product roadmap** — pillars, blueprints, timeline |
| [docs/getting-started.md](docs/getting-started.md) | First robot in 10 minutes |
| [docs/control-center.md](docs/control-center.md) | **Control Center** — start API/UI, rebuild, `serve` with `--config` and `--program` |
| [docs/authentication.md](docs/authentication.md) | **Control Center auth** — hashed API keys, OIDC session JWTs, read-auth policy |
| [docs/control-center-versioning.md](docs/control-center-versioning.md) | **Control Center versioning** — UI/CLI/API semver, `desktop-v*` auto release |
| [docs/troubleshooting.md](docs/troubleshooting.md) | Symptom-first fixes — CLI, SDK install, verify, fleet, ROS2, Control Center |
| [docs/known-limitations.md](docs/known-limitations.md) | Honest scope limits, mock backends, organizational gates |
| [docs/organizational-gates.md](docs/organizational-gates.md) | v0.6.3 → v1.0 path — field soak, security audit, exit checklist |
| [docs/sdk.md](docs/sdk.md) | **Official SDKs** — `cargo add spanda-sdk`, `pip install spanda-sdk`, `@davalgi-spanda/sdk` |
| [docs/platform-overview.md](docs/platform-overview.md) | Platform components and workflow |
| [docs/spanda-language.md](docs/spanda-language.md) | Language guide |
| [docs/solutions/README.md](docs/solutions/README.md) | Official Solution Blueprints |
| [docs/tutorials/README.md](docs/tutorials/README.md) | Tutorials and learning paths |
| [examples/README.md](examples/README.md) | Runnable examples library |
| [docs/plugins.md](docs/plugins.md) | Plugin system — install, trust tiers, Control Center panels |
| [docs/README.md](docs/README.md) | Full documentation index |
| [docs/ci-architecture.md](docs/ci-architecture.md) | **Tiered CI** — Fast / Integration / Nightly, branch protection |

CLI reference: `spanda man <command>` · [docs/man/](docs/man/README.md) · Language API:
[docs/spanda-reference.md](docs/spanda-reference.md)

---

## Contributing

[CONTRIBUTING.md](CONTRIBUTING.md) · [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md) · CI tiers:
[docs/ci-architecture.md](docs/ci-architecture.md)

```bash
./scripts/ci-fast.sh
```

---

## License

Apache-2.0 — see [LICENSE](LICENSE).
