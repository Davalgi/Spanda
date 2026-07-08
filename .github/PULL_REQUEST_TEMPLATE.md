## Summary

<!-- What changed and why (1–3 sentences). -->

## Architecture Review

<!-- Required for new crates, services, APIs, SDKs, Control Center features, or platform capabilities. -->
<!-- Link architecture issue or ADR. For bug fixes only, write "N/A — no architecture change." -->

**Architecture issue / ADR:** 

### What problem is solved?

<!-- User problem, production scenario, measurable value. -->

### What existing capability does this extend?

<!-- Entity Model, Recovery, Readiness, Trust, Provider Framework, etc. -->

### What duplication was considered?

<!-- List crates, packages, providers, APIs searched. Why extension was insufficient, or "none found." -->

### Why is a new component necessary?

<!-- If adding crate/service/API. If extending only, explain extension point. -->

### How does it integrate with the Entity Model?

<!-- Entity reads/writes/events, or justified N/A. -->

### How was security reviewed?

<!-- Trust, decision authority, attack surface, or N/A. -->

### How was compatibility reviewed?

<!-- APIs, SDKs, CLI, examples, migrations. -->

### What tests were added?

<!-- Unit, integration, golden, CI tier. -->

### What documentation was updated?

<!-- README, CHANGELOG, feature-status, topic docs. -->

## Quality Gate

- [ ] Architecture Review completed (issue/ADR linked above)
- [ ] Duplication Analysis completed
- [ ] Security Review completed (if applicable)
- [ ] Compatibility Review completed
- [ ] Entity Integration documented
- [ ] Tests added or planned
- [ ] Documentation updated
- [ ] ADR created (major architectural changes only)

## Test plan

<!-- How you verified the change locally. -->

- [ ] `./scripts/ci-fast.sh` (or equivalent)
- [ ] Additional: 

## Release impact

<!-- workspace / sdk / desktop / none — see docs/versioning.md -->
