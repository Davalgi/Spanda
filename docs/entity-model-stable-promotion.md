# Unified Entity Model — Stable Promotion

**Status:** **Promoted to Stable** (2026-06-29)

Operational checklist used to promote **Unified Entity Model** from **Experimental** to **Stable** in `docs/feature-status.md`.

---

## Completed

| Step | Status |
|------|--------|
| Phases 1–7 implementation + stabilization | ✅ Shipped on `main` |
| `entity_model_smoke.sh` (REST + TS + Python + Rust SDK) | ✅ CI |
| `entity-model-promotion-gate` (implementation checks with soak/audit skip in CI) | ✅ CI |
| SDK **0.4.1** published | ✅ See [SDK publish](#sdk-publish) |
| `docs/feature-status.md` → **Stable** | ✅ |

---

## SDK publish

| Package | Version | Tag | Registry |
|---------|---------|-----|----------|
| `spanda-sdk` (Rust) | `0.4.1` | `crates-sdk-v0.4.1` | [crates.io](https://crates.io/crates/spanda-sdk) |
| `spanda-sdk` (Python) | `0.4.1` | `sdk-python-v0.4.1` | [PyPI](https://pypi.org/project/spanda-sdk/) |
| `@davalgi-spanda/sdk` (npm) | `0.4.1` | `npm-sdk-v0.4.1` | [npm](https://www.npmjs.com/package/@davalgi-spanda/sdk) |

Publish workflows: `.github/workflows/publish-sdk-{rust,python,typescript}.yml` — triggered by tag push.

To republish a patch with new entity helpers, bump version in `crates/spanda-sdk/Cargo.toml`, `sdk/python/pyproject.toml`, and `sdk/typescript/package.json`, then:

```bash
git tag sdk-python-v0.4.2 && git push origin sdk-python-v0.4.2
git tag crates-sdk-v0.4.2 && git push origin crates-sdk-v0.4.2
git tag npm-sdk-v0.4.2 && git push origin npm-sdk-v0.4.2
```

See [sdk-publishing.md](./sdk-publishing.md).

---

## Automated gate (for re-validation)

```bash
chmod +x scripts/entity_model_stable_promotion_gate.sh

# CI / local implementation checks only:
SPANDA_ENTITY_MODEL_SKIP_SOAK=1 SPANDA_ENTITY_MODEL_SKIP_AUDIT=1 \
  ./scripts/entity_model_stable_promotion_gate.sh

# Full gate (requires elapsed field soak + audit prep artifact):
./scripts/entity_model_stable_promotion_gate.sh
```

### Environment variables

| Variable | Default | Purpose |
|----------|---------|---------|
| `SPANDA_FIELD_SOAK_START_FILE` | `.spanda/field-soak-start.txt` | Shared soak clock with enterprise ops |
| `SPANDA_FIELD_SOAK_MIN_DAYS` | `30` | Minimum elapsed days |
| `SPANDA_SECURITY_AUDIT_PREP_FILE` | `.spanda/security-audit-prep.json` | Audit prep artifact |
| `SPANDA_ENTITY_MODEL_SKIP_SOAK` | `0` | Skip soak elapsed check |
| `SPANDA_ENTITY_MODEL_SKIP_AUDIT` | `0` | Skip audit prep file check |

---

## Enterprise platform gates (separate)

Shared **30-day field soak** and **third-party security audit** sign-off still apply to broader enterprise-ops Stable promotion — [enterprise-ops-stable-promotion.md](./enterprise-ops-stable-promotion.md), [field-soak-gate.md](./field-soak-gate.md).

---

## Related

- [entity-model.md](./entity-model.md) — architecture and phase checklist
- [entity-integration-report.md](./entity-integration-report.md) — integration phase status
- [feature-status.md](./feature-status.md) — capability matrix
