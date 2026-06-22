# Fleet Health

Fleet-level health checks aggregate robot health and coordinator status.

```spanda
health_check FleetHealth for fleet WarehouseFleet {
    require at_least 80% robots Healthy;
    require no robot Unsafe;
    require coordinator.status == Healthy;
}

on health fleet becomes Critical {
    pause_new_missions();
}
```

CLI: `spanda health robot fleet.sd --json`

See [Health Checks](./health-checks.md).
