# Known limitations

Honest constraints for **v0.6.3** evaluators. For capability tiers see [feature-status.md](./feature-status.md). For live-backend setup see [troubleshooting.md](./troubleshooting.md).

**Runtime notices:** `spanda run`, `spanda sim`, and live transport/AI calls print one-time `[spanda]` warnings when mock or in-memory backends are active. Set `SPANDA_QUIET=1` to hide them.

## Runtime and execution

- The **tree-walking interpreter** is the primary execution path. LLVM native codegen is experimental (`spanda compile-native`, `spanda llvm-ir`).
- Simulation is **physics-lite 2D** — suitable for logic and safety testing, not high-fidelity Gazebo-class physics.
- Multi-robot **fleet** examples default to **in-process** simulation with per-robot setup/execute. Distributed orchestration uses HTTP fleet agents and optional mesh coordinator — not a production fleet OS.
- **Mission continuity** checkpoints persist to `.spanda/mission-checkpoints.json` (override with `SPANDA_CONTINUITY_CHECKPOINTS`). Runtime takeover on fleet agents requires deployed programs and mesh/agent wiring — validate in staging before field trials.

## AI and providers

- AI models use **mock backends** by default. Live OpenAI, Anthropic, and ONNX require API keys or model paths (`SPANDA_LIVE_AI=0` forces mock).
- Provider packages wire through an in-process registry; there is no managed cloud inference service.
- Setup: [troubleshooting.md — Live AI and extern bridges](./troubleshooting.md#live-ai-and-extern-bridges) · [live-ai-provider.md](./live-ai-provider.md)

## Connectivity and IoT

- In-memory transport is the default. Live MQTT, WebSocket, DDS, Modbus, and OPC-UA require env flags and often `--features live-*` builds.
- DDS support is a **UDP JSON shim**, not full standards-compliant DDS middleware.
- Setup: [troubleshooting.md — Live IoT and transports](./troubleshooting.md#live-iot-and-transports) · [iot.md](./iot.md) · [ros2-golden-path.md](./ros2-golden-path.md)

## Packages and registry

- Monorepo installs prefer `registry/index.json` plus on-disk `packages/registry/`; compile-time `LOCAL_REGISTRY` stub is incomplete but no longer blocks install.
- `spanda publish` mirrors bundles to `registry/packages/` in-repo. Remote upload needs `SPANDA_REGISTRY_URL`.
- The hosted index lists **curated packages**; community expansion is ongoing.

## Verification and certification

- `certify ISO13849 { … }` is **verify-time metadata** — not a formal certification body sign-off.
- Capability traceability and minimum-hardware checks are **static analysis** plus runtime health hooks — not IEC 61508 tooling.

## Tooling

- **LSP** and **DAP** work with a built native CLI; VS Code extension builds in CI. **Marketplace publish** pending maintainer `VSCE_PAT`.
- **WASM playground** covers check/run/verify — smaller surface than native CLI.

## Security

- Encryption and signed messages are implemented for wire frames and audit records. **No production HSM or PKI integration** is bundled.
- `remote_signed` kill switch requires configured signature material — verify reports errors when missing.

## Replay and twins

- Mission traces are **local files** (`--record` → `spanda replay`). No managed trace cloud.
- OSS Twin Cloud (`/v1/twins/*`, `spanda twin cloud`) is **Stable** with file-backed storage. **Hosted managed product** (billing, SLA, multi-region) is separate — [hosted-twin-cloud-product.md](./hosted-twin-cloud-product.md).

## Platform

- ROS2 adapter requires **ROS Humble** and manual setup on Linux (`SPANDA_ROS2_LIVE=1` for live topics).
- Windows support is via MSI/prebuilt CLI; some golden paths are Linux/macOS only in CI.

## Bio-inspired resilient autonomy

- **Beta / Experimental tiers** — see [feature-status.md](./feature-status.md) and [bio-inspired-architecture.md](./bio-inspired-architecture.md).
- Sensory fusion validators are rule-based; no live multi-sensor fusion pipeline yet.
- Control Center **Cognitive & Resilience** tab uses live REST panels; gRPC parity ships for `/v1/autonomy/*`.
- Homeostasis merges entity health with interpreter scheduler telemetry when a recent `run`/`sim` completed.
- Reflex traces persist to `.spanda/autonomy-reflex-traces.json` (override with `SPANDA_AUTONOMY_TRACE_FILE`).
- Adaptive recovery learning is statistics-based (no ML).
- Maintenance/sleep mode scheduling is declarative; full OTA window integration is partial.
- Habituation/sensitization applies to CLI-reported alert analysis, not all telemetry backends.

---

## Organizational gates (not code blockers)

Enterprise operations and solution blueprints ship as **Stable** in code and CI, but full production claims still require:

- **30-day field soak** — `./scripts/enterprise_ops_field_soak_init.sh` then `./scripts/enterprise_ops_stable_promotion_gate.sh`
- **Third-party security audit** — `./scripts/security_audit_prep.sh` then external reviewer sign-off

Tracked as [RB-007 / #51](https://github.com/Davalgi/Spanda/issues/51). Full v1.0 checklist: [organizational-gates.md](./organizational-gates.md) · runbooks: [enterprise-ops-stable-promotion.md](./enterprise-ops-stable-promotion.md) · [release-blockers.md](./release-blockers.md).

## Not planned (by design)

Spanda intentionally does **not** target: blockchain production adapters, cryptocurrency integrations, advanced swarm intelligence research, self-hosting compiler as default, or custom database backends as core product scope.

## Reporting issues

If behavior differs from this document, file an issue with `spanda --version`, OS, and the smallest `.sd` reproducer. For setup and integration failures, start with [troubleshooting.md](./troubleshooting.md).
