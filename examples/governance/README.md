# Governance Examples

Operational governance examples demonstrating deployment profiles, risk, certification, standards, and validation.

| Example | Profile | Risk | Maturity |
|---------|---------|------|----------|
| [warehouse](./warehouse/) | warehouse | medium | pilot |
| [hospital](./hospital/) | hospital | life_critical | pre_production |
| [search-rescue](./search-rescue/) | search_rescue | mission_critical | pilot |
| [industrial-robot](./industrial-robot/) | factory | high | production |
| [adas](./adas/) | road_vehicle | life_critical | simulation |
| [connected-healthcare](./connected-healthcare/) | hospital | life_critical | pilot |
| [smart-building](./smart-building/) | smart_building | low | production |

Each example includes `spanda.governance.toml`. From an example directory:

```bash
spanda governance validate
spanda compliance check
spanda governance report --json
```

See [docs/governance.md](../../docs/governance.md) for the full framework.
