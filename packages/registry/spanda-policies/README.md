# spanda-policies

Official Spanda package for **cognitive policy scaffolds**: homeostasis and attention.

| Import | Role |
|--------|------|
| `std.policies.homeostasis` | Helpers for `@policy(kind: "homeostasis")` |
| `std.policies.attention` | Helpers for `@policy(kind: "attention")` |

## What this package is

Thin language/registry surface for the migration path in
[language-surface-inventory.md](../../../docs/language-surface-inventory.md). Prefer:

```spanda
import std.policies.homeostasis;

@policy(kind: "homeostasis")
PlatformStability {
    metric cpu_pct;
    metric battery_pct;
}
```

## What this package is not

Runtime evaluation stays in **`spanda-autonomy`** and platform CLI/API:

- `spanda homeostasis check|report`
- `GET /v1/autonomy/homeostasis`, `GET /v1/autonomy/attention`
- Control Center Cognitive & Resilience tab

Do not reimplement `evaluate_homeostasis` / `rank_events` here.

## Smoke

```bash
spanda test packages/registry/spanda-policies/tests/smoke.sd
```
