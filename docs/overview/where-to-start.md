# Where should I start?

[← Overview](./README.md)

Audience-specific paths into Spanda. For a hands-on first project, see [getting
started](../getting-started.md).

## For developers

- Try the safety demo: `spanda demo safety`
- Read the language guide: [spanda-language.md](../spanda-language.md)

## For robotics engineers

- Try hardware verification: `spanda demo verify`
- Read capability verification: [capability-traceability.md](../capability-traceability.md) ·
  [hardware-compatibility.md](../hardware-compatibility.md)

## For safety / reliability engineers

- Try assurance and readiness: `spanda demo assurance` or the [flagship readiness
  demo](./flagship-demos.md#3-readiness--assurance--diagnosis)
- Try mission continuity: `spanda demo continuity` — takeover, delegation, and succession when
  robots fail mid-mission
- Read traceability and safety: [mission-assurance.md](../mission-assurance.md) ·
  [mission-continuity.md](../mission-continuity.md) ·
  [capability-traceability.md](../capability-traceability.md) ·
  [safety-reporting.md](../safety-reporting.md)

## For quality assurance or test engineers

- Try in-language tests and the rover demo: `spanda test examples/basics/07_in_language_tests.sd`
  and `spanda demo rover` (verify → sim → record/replay)
- Try fault injection and health checks: `spanda demo health`
- Read testing and verification: [testing.md](../testing.md) · [replay.md](../replay.md) ·
  [ci-verify.md](../ci-verify.md) · [mission-verification.md](../mission-verification.md)

## For fleet operators

- Try self-healing and continuity: `spanda demo self-healing` and `spanda demo continuity`
- Read fleet mesh APIs: [fleet-distributed.md](../fleet-distributed.md) ·
  [self-healing.md](../self-healing.md) · [continuity-policies.md](../continuity-policies.md)

## For contributors

- Read [CONTRIBUTING.md](../../CONTRIBUTING.md)
- Run tests: `cargo test --workspace && npm test`
- Pick a good first issue on [GitHub Issues](https://github.com/Davalgi/Spanda/issues)

## Learning tracks

| Track | Guide | Time |
|-------|-------|------|
| Plain English | [Spanda for Dummies](../spanda-for-dummies/README.md) | ~45 min |
| Hands-on course | [Spanda 101](../spanda-101/README.md) | ~3.5 hours |
| Quickstart | [Getting started](../getting-started.md) | ~10 min |
| All tutorials | [Tutorials index](../tutorials/README.md) | self-paced |
