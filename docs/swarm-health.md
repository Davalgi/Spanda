# Swarm Health

Swarm health checks verify quorum and mesh connectivity.

```spanda
health_check SwarmHealth for swarm DroneSwarm {
    require quorum >= 70%;
    require communication.mesh_connected == true;
}
```

See [Health Checks](./health-checks.md) and [Concurrency](./concurrency.md).
