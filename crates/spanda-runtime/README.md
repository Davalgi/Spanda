# spanda-runtime

Runtime kernel pieces extracted from `spanda-core` for the Phase 4 lean-core split:

- **scheduler** — sim vs wall-clock tick helpers
- **provider_types** — `ProviderId`, metadata types, capability sets
- **classification** — module ownership audit table
- **robotics** — `MissionRuntime`, `FleetRegistry`, zone registries
- **value** — `RuntimeValue`, `MotionCommand`, pose/velocity helpers
- **environment** — interpreter variable bindings
- **error** — `RuntimeError`
- **host** — `RuntimeHost` trait for domain hook extraction

The interpreter (`Interpreter`), `RobotBackend`, and full `ProviderRegistry` remain in `spanda-core` for now; `CoreRuntimeHost` wires SLAM/navigation import detection.
