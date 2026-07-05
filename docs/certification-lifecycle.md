# Certification Lifecycle

Certification in Spanda is modeled **independently from health**. An entity can be healthy but
uncertified, or certified with degraded health requiring review.

## States

| State | Description |
|-------|-------------|
| `draft` | Initial record, not submitted |
| `under_review` | Awaiting reviewer |
| `testing` | Validation in progress |
| `validated` | Evidence reviewed, approved for scoped use |
| `certified` | Formal certification active |
| `expired` | Certification past validity period |
| `revoked` | Certification withdrawn |
| `suspended` | Temporarily inactive |
| `deprecated` | Superseded by newer certification |
| `archived` | Historical record only |

## Tracked metadata

Each [`CertificationRecord`](../crates/spanda-governance/src/certification.rs) tracks:

- Who certified (`certified_by`)
- When (`certified_at`, `expires_at`)
- Why (`reason`)
- Evidence references (`evidence[]`)
- Version (`version`)
- Applicable scope (`applicable_scope[]`)

## Entity integration

Set on entities via governance config:

```toml
[governance]
certification_status = "certified"
certification_id = "cert-warehouse-amr-2026-001"
```

## CLI

```bash
spanda certification list
spanda certification inspect robot:amr-01
```

## API

`GET /v1/certifications` returns certification summaries for all governed entities.

## Runtime relationship

- **Health** — live operational posture (sensors, connectivity, faults)
- **Certification** — approved authority to operate at a given maturity/profile
- **Readiness** — mission-time gate combining both

Governance validation requires `validated` or `certified` status before live deployment maturity
stages.
