# Warehouse AMR Governance Example

Demonstrates operational governance for warehouse autonomous mobile robots.

## Attributes

| Field | Value |
|-------|-------|
| Deployment profile | `warehouse` |
| Risk level | `medium` |
| Certification | `validated` |
| Standards | `industrial_safety`, `cybersecurity` |

## Validation

```bash
cd examples/governance/warehouse
spanda governance validate
spanda compliance check
spanda governance report --json
spanda deployment profile warehouse
```

## Expected governance checks

- Indoor + connectivity constraints
- ISO 3691-4 certification reference
- Partial automation within warehouse profile max
