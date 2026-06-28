# Platform components

[← Overview](./README.md) · Structure: [platform-structure.md](./platform-structure.md)

One-line pointers — full platform guide: [platform-overview.md](../platform-overview.md).

| Component | Summary | Doc |
|-----------|---------|-----|
| **Spanda Language** | Safety-first `.sd` programs with robot, sensor, actuator, and safety primitives | [spanda-language.md](../spanda-language.md) |
| **Spanda Runtime** | Interpreter, scheduler, HAL, provider dispatch after compile-time gates | [architecture.md](../architecture.md) |
| **Spanda Verify** | Hardware fit, capability traceability, behavioral `verify { }` blocks | [hardware-compatibility.md](../hardware-compatibility.md) |
| **Spanda Config** | Cascading `spanda.toml`, device tree, device identity mapping, validation | [configuration.md](../configuration.md) |
| **Spanda Safety** | `SafeAction` type gate, safety zones, kill switch, emergency stop | [agentic-programming.md](../agentic-programming.md) |
| **Spanda Sim** | Simulation and digital twins without physical hardware | [killer-demo.md](../killer-demo.md) |
| **Spanda Replay** | Mission trace capture and deterministic playback | [replay.md](../replay.md) |
| **Persistent telemetry** | Device/sensor/heartbeat/health store (`--persist-telemetry`); JSONL or SQLite; OTLP export, `push --watch`, session auto-push (`SPANDA_OTLP_AUTO_PUSH`), fleet mesh aggregation (`telemetry fleet-push`); sessions + replay | [telemetry-store.md](../telemetry-store.md) |
| **Spanda Health** | Runtime health checks and fleet readiness requirements | [health-checks.md](../health-checks.md) |
| **Runtime fault detection** | Crashes, reboots, memory leaks, resource pressure, restart loops | [runtime-fault-detection.md](../runtime-fault-detection.md) |
| **Spanda Assurance** | Knowledge models, anomaly detection, prognostics, assurance cases | [mission-assurance.md](../mission-assurance.md) |
| **Mission continuity** | Takeover, delegation, succession, checkpoint resume | [mission-continuity.md](../mission-continuity.md) |
| **Spanda Diagnosis** | Root-cause analysis from mission traces and programs | [diagnostics.md](../diagnostics.md) |
| **Spanda Registry** | Package index, install, publish, signed tarballs | [registry.md](../registry.md) |
| **Spanda Providers** | Official package traits — ROS2, MQTT, vision, fleet, and more | [how-providers-work.md](../how-providers-work.md) |
| **Control Center** | Enterprise ops API (`spanda-api`), `spanda control-center serve`, web UI, Tauri desktop shell | [control-center.md](../control-center.md) · [enterprise-operations-roadmap.md](../enterprise-operations-roadmap.md) · [stable-hardening-enterprise-ops.md](../stable-hardening-enterprise-ops.md) |

Diagrams: [diagrams/README.md](../diagrams/README.md)
