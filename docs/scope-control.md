# Scope Control — Release Hardening Phase

**Current phase:** release hardening and defect discovery.

Spanda is treated as feature-complete for the current milestone. Work prioritizes
credibility over new capability.

## Allowed

- Bug fixes that restore documented behavior
- Regression, golden-output, smoke, and security tests
- Documentation correctness and stability labeling
- CI hardening and quality gates
- Demo hardening (existing demos only)
- SDK / REST / CLI consistency fixes
- Security regression fixes (trust, signatures, sandbox, tamper, privilege)

## Not allowed

- New platform services or major subsystems
- New solution blueprints
- New architecture layers or protocols
- New flagship product surfaces marketed as Stable without tests
- Expanding mock-backed paths and labeling them production-ready

## Review checklist

Before merging:

1. Does this fix a bug, test, doc, or CI gap?
2. Does it avoid introducing a new user-facing capability?
3. Are stability labels honest (no Stable for mock/demo/docs-only)?
4. Are release blockers updated in [release-blockers.md](./release-blockers.md)?

## Related

- [release-readiness.md](./release-readiness.md)
- [release-blockers.md](./release-blockers.md)
- [feature-status.md](./feature-status.md)
- [../ROADMAP.md](../ROADMAP.md)
