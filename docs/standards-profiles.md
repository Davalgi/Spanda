# Standards Profiles

Spanda does **not** embed standards or regulatory text. Instead, **standards profiles** define what checks, evidence, reports, approvals, and documentation are required — implemented through packages and plugins.

## Built-in profile kinds

| Kind | Package reference |
|------|-------------------|
| `functional_safety` | `spanda-standards-functional_safety` |
| `industrial_safety` | `spanda-standards-industrial_safety` |
| `cybersecurity` | `spanda-standards-cybersecurity` |
| `medical_device` | `spanda-standards-medical_device` |
| `automotive` | `spanda-standards-automotive` |
| `aviation` | `spanda-standards-aviation` |
| `rail` | `spanda-standards-rail` |
| `maritime` | `spanda-standards-maritime` |
| `energy` | `spanda-standards-energy` |
| `space` | `spanda-standards-space` |
| `government` | `spanda-standards-government` |

## Profile structure

Each [`StandardsProfileRef`](../crates/spanda-governance/src/policy.rs) defines:

- `required_checks` — validation steps (e.g. hazard analysis)
- `required_evidence` — artifact types (e.g. FMEA report)
- `required_reports` — deliverables (e.g. safety assessment)
- `required_approvals` — roles (e.g. safety officer)
- `required_documentation` — documents (e.g. safety manual)

## Assignment

```toml
[governance]
standards_profiles = "functional_safety,cybersecurity"
```

Deployment profiles automatically associate relevant standards (e.g. `hospital` → `medical_device`, `functional_safety`).

## Program compliance integration

Use `spanda compliance report` with industry profiles from `spanda-compliance` for program-level accreditation. Standards profiles govern **what evidence categories** must exist; compliance profiles evaluate **program structure**.

## Extending via packages

Publish a package that registers custom standards profile requirements. Spanda validates presence and references — not legal interpretation.

## Disclaimer

Standards profiles are structural templates. Organizations must map them to applicable regulations and obtain qualified review.
