# Scope Control — Next Horizon Phase

**Previous phase:** release hardening (v0.6.3) — **complete** (2026-07-04).

**Current phase:** **Next horizon** — platform hardening and adoption after the evaluation/beta
release.

v0.6.3 shipped with CI-backed quality gates and honest stability labels. Work now prioritizes
**v1.0-bound capabilities**, **Experimental → Stable promotion**, and **organizational gate
preparation** — not greenfield platform invention.

**Organizational gates (field soak, third-party audit):**
[organizational-gates.md](./organizational-gates.md) ·
[#51](https://github.com/Davalgi/Spanda/issues/51)

## Allowed

- Bug fixes that restore documented behavior
- Regression, golden-output, smoke, and security tests
- Documentation correctness and stability labeling
- CI hardening and quality gates
- Demo hardening (existing demos only)
- SDK / REST / CLI / gRPC consistency fixes
- Security regression fixes (trust, signatures, sandbox, tamper, privilege)
- **Experimental → Stable promotion** when tier rules in [feature-status.md](./feature-status.md)
  are met (tests, non-mock default where required)
- **Next-horizon items** listed in [ROADMAP.md § Next horizon
  priorities](../ROADMAP.md#next-horizon-priorities-post-v063)
- Field soak and audit prep automation (scripts, docs, CI probes — not bypassing human sign-off)

## Not allowed

- New platform services or major subsystems outside the Next horizon list
- New solution blueprints without roadmap approval
- New architecture layers or protocols unrelated to v1.0 exit criteria
- Labeling mock-default paths **Stable** for production marketing
- Claiming **v1.0 production readiness** before [organizational-gates.md](./organizational-gates.md)
  exit checklist is met

## Review checklist

Before merging:

1. Is the change on the Next horizon list or a justified fix/test/doc?
2. Are stability labels honest (no Stable for mock/demo/docs-only)?
3. Are release blockers updated in [release-blockers.md](./release-blockers.md) if user-visible?
4. Does public messaging stay **evaluation / beta** until v1.0 gates close?

## Related

- [organizational-gates.md](./organizational-gates.md)
- [release-readiness.md](./release-readiness.md)
- [release-blockers.md](./release-blockers.md)
- [feature-status.md](./feature-status.md)
- [../ROADMAP.md](../ROADMAP.md)
