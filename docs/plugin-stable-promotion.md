# Plugin System — Stable Promotion

**Status:** **Stable** (implementation promotion **2026-07-11**)

Operational checklist to promote the **Plugin system** from **Experimental** to **Stable**
in `docs/feature-status.md` and `ROADMAP.md` Pillar 8.

Remote curated marketplace growth remains **Next** (separate from local plugin lifecycle).

---

## Prerequisites

| Step | Status |
|------|--------|
| `spanda-plugin` crate (manifest, registry, security, WASM loader, runtime) | ✅ |
| CLI `spanda plugin search\|install\|enable\|disable\|list\|inspect\|trust` | ✅ |
| REST `GET /v1/plugins` with `control_center_panels` | ✅ |
| REST search / install / enable / disable (CLI parity) | ✅ |
| Control Center Marketplace tab (search + lifecycle) | ✅ |
| Sandboxed iframe panel host (`sandbox` + postMessage) | ✅ |
| Install-time Ed25519 signature verification (official/`signed`) | ✅ Server-side |
| Example Control Center panel + `index.js` bundle | ✅ |
| Docs ([plugins.md](./plugins.md), [plugin-security.md](./plugin-security.md)) | ✅ |

---

## Automated gate

```bash
chmod +x scripts/plugin_stable_promotion_gate.sh

# Implementation checks (CI / local):
./scripts/plugin_stable_promotion_gate.sh
```

The gate runs:

1. `cargo test -p spanda-plugin`
2. `scripts/plugin_system_smoke.sh` — install example CC plugin, list `/v1/plugins`, fetch bundle

---

## Compatibility rules (must hold at Stable)

- Plugins **extend** Control Center / CLI / hooks — they do not replace packages or providers
- Signature verification for official signed plugins is **server-side at install** (not re-checked on every bundle GET)
- Control Center UI bundles run in a **sandboxed iframe** (`allow-scripts` only; no `allow-same-origin`)
- Remote curated marketplace index growth stays **Next**

---

## Related

- [plugins.md](./plugins.md)
- [plugin-security.md](./plugin-security.md)
- [control-center-plugins.md](./control-center-plugins.md)
- [feature-status.md](./feature-status.md)
