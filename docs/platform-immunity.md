# Platform Immunity

**Status: Beta** — built on trust, tamper detection, security, and device quarantine.

## Purpose

Protect against unknown, untrusted, tampered, or compromised entities.

## Types

`ImmunePolicy`, `QuarantineAction`, `ThreatResponse`, `TrustBoundaryViolation`, `ImmuneEvent`, `IsolationDecision`

## Examples

- Unknown device discovered → quarantine
- Unsigned plugin installed → block
- Package tampered → isolate
- Entity compromised → remove from mission
- Sensor spoofing detected → reject signal

## CLI

```bash
spanda immunity scan
spanda immunity quarantine [entity_id]
spanda immunity report
```

## API

`GET /v1/autonomy/immunity`

See [device-quarantine.md](./device-quarantine.md), [trust framework](./trust-framework.md).
