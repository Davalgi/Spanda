# Decision trail — differentiation NOW

Demonstrates **Decision Audit Trail** (differentiation pillar #6): sim emits v3 decision frames, then audit and explain commands reconstruct why the system acted.

```bash
export SPANDA_DECISION_TRACE=1
spanda sim main.sd --record --inject-health-faults
spanda audit decisions main.trace
spanda explain decision main.trace
spanda decision trace main.trace
```

`main.trace` is a checked-in golden fixture for replay time-travel smoke (`spanda replay main.trace --at T+00:01 --inspect decisions`).

Part of `spanda demo differentiation` and `scripts/differentiation_smoke.sh`.
