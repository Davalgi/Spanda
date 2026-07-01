# spanda-knx

KNX building automation bridge for Smart Spaces blueprints.

## Runtime

Provider dispatch (`iot.knx.read_group_address`) routes through `spanda-providers` → `iot_hub` → `iot_live` when:

- `SPANDA_LIVE_KNX=1`
- `SPANDA_KNX_CMD` shell template (`{address}`), or
- Python bridge handler `knx_read_group` (mock without hardware)

## Smoke

```bash
spanda check packages/registry/spanda-knx/tests/smoke.sd
./scripts/smart_spaces_live_iot_smoke.sh
```

## Example

```bash
export SPANDA_LIVE_KNX=1
export SPANDA_KNX_CMD='echo live-knx:{address}'
spanda control-center smart-spaces environment --zone-id room-lobby
```

## Native integration

For production KNX/IP, point `SPANDA_KNX_CMD` at an `xknx` or vendor CLI wrapper, or register a Python `knx_read_group` handler in your provider bootstrap.
