# Defense — Solution Blueprint

**Status:** Experimental · **Timeline:** Next · **Path:** `examples/showcase/secure_boot/`,
`examples/security/`

Official Solution Blueprint for secure autonomous systems with tamper resistance, attestation, RBAC,
and defense compliance evidence.

**Full roadmap entry:** [ROADMAP.md § Defense](../../ROADMAP.md#defense)

---

## Purpose

Deploy field systems where trust, tamper detection, secure operator commands, and audit trails are
mandatory — composing Security and Verification pillars without defense-specific language keywords.

## Platform pillars used

| Pillar | Capabilities |
|--------|--------------|
| Security | Encryption, RBAC, tamper detection, attestation, package trust |
| Verification | Trust scoring, decision audit trail, explainability |
| Operations | Incident workflow, compliance export, Control Center RBAC |
| Device & Fleet | Quarantine, device pool failover, fleet mesh recovery |

## Reference architecture

```text
Defense Stack
├── Secure-boot attestation chain
├── Signed operator commands (remote_signed)
├── Tamper verify-time + runtime checks
├── Program trust + package trust scoring
├── Mission approval queue
├── Quarantine on integrity failure
└── Compliance evidence bundle (defense profile)
```

## Device tree

| Node | Role |
|------|------|
| `platform` | Field autonomous unit |
| `c2_node` | Command and control |
| `operator` | Certified human operator |

Guides: [trust-boundaries.md](../trust-boundaries.md) ·
[operator-capabilities.md](../operator-capabilities.md)

## Packages & providers

| Package | Role |
|---------|------|
| `spanda-security-audit` | Audit rollup |
| `spanda-tamper` | Tamper detection hooks |
| `spanda-trust-pi` / `spanda-trust-jetson` | Hardware trust backends |
| `spanda-ledger` | Evidence anchoring (community/experimental) |
| `spanda-discovery-tls` | TLS discovery policy |

## Mission examples

| Example | Path |
|---------|------|
| Secure operator command | [examples/security/secure_operator_command.sd](../../examples/security/secure_operator_command.sd) |
| Invalid signature (expect fail) | [examples/security/invalid_signature.sd](../../examples/security/invalid_signature.sd) |
| Secure boot showcase | [examples/showcase/secure_boot/](../../examples/showcase/secure_boot/) |
| Mission tampering | [examples/showcase/mission_tampering/](../../examples/showcase/mission_tampering/) |
| Fleet tamper | [examples/showcase/fleet_tamper/](../../examples/showcase/fleet_tamper/) |
| GPS spoofing | [examples/showcase/gps_spoofing/](../../examples/showcase/gps_spoofing/) |

## Health policies & readiness

- Attestation require before deploy — [hardware-attestation.md](../hardware-attestation.md)
- Trust gates — [trust-framework.md](../trust-framework.md)
- Mission approval — Control Center operator workflows

## Assurance & recovery

- Security assurance rollup — [security-assurance.md](../security-assurance.md)
- Tamper diagnosis — [tamper-detection.md](../tamper-detection.md)
- Fleet mesh recovery — [self-healing.md](../self-healing.md)
- Decision audit — [decision-audit-trail.md](../decision-audit-trail.md)

## Compliance profile

Defense template: `crates/spanda-compliance/templates/defense.json`

Guide: [compliance-profiles.md](../compliance-profiles.md)

## Control Center

RBAC matrix, audit export, quarantine workflows — [control-center.md](../control-center.md)

## Simulation & replay

```bash
spanda sim examples/showcase/mission_tampering/modified.sd --inject-failure
spanda audit decisions --trace mission.trace
spanda explain examples/security/secure_operator_command.sd
```

## Quick start

```bash
spanda check examples/security/secure_operator_command.sd
spanda verify examples/showcase/secure_boot/rover.sd --json
./scripts/secure_boot_smoke.sh
./scripts/tamper_smoke.sh
```

## Smoke gates

| Script | Focus |
|--------|-------|
| `secure_boot_smoke.sh` | Attestation chain |
| `tamper_smoke.sh` | Tamper framework |
| `security_assurance_smoke.sh` | Assurance rollup |
| `fleet_mesh_tamper_smoke.sh` | Fleet tamper relay |
| `trust_program_smoke.sh` | Program trust |

Index: [scripts/gates/README.md](../../scripts/gates/README.md)

## Stable promotion

Third-party audit prep: [security-audit-third-party.md](../security-audit-third-party.md)

---

**Related blueprints:** [Critical Infrastructure](../ROADMAP.md#critical-infrastructure) · [Search &
Rescue](./spatial-computing.md)
