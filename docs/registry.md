# Spanda Package Registry

Spanda's package registry ships a **hosted index** in this repository (`registry/index.json`). The CLI defaults to:

`https://raw.githubusercontent.com/Davalgi/Spanda/main/registry`

Override with **`SPANDA_REGISTRY_URL`** (supports `https://` and `file://` bases). Entries merge with the local stub registry for search and `spanda install`.

## Searching packages

```bash
spanda registry search ros2
spanda registry search openai
```

## Curated packages (hosted)

| Package | Category | Import paths |
|---------|----------|--------------|
| `spanda-openai` | ai | `ai.openai` |
| `spanda-ros2` | ros2 | `robotics.ros2` |

Tarballs live at `registry/packages/<name>/<version>` in the repo. Rebuild with `./scripts/build-registry.sh`.

## Local stub packages

| Package | Category | Import paths |
|---------|----------|--------------|
| `spanda-vision` | vision | `vision.core` |
| `spanda-navigation` | navigation | `navigation.path_planning` |
| `spanda-mqtt` | mqtt | `communication.mqtt` |
| `spanda-lidar-rplidar` | sensors | `sensors.lidar.rplidar` |

## Planned framework packages

These are defined in the ecosystem metadata and will be published as the registry matures:

| Package | Description |
|---------|-------------|
| `spanda-ros2` | ROS 2 pub/sub, services, actions |
| `spanda-mqtt` | MQTT transport |
| `spanda-opencv` | OpenCV bindings |
| `spanda-yolo` | YOLO object detection |
| `spanda-slam` | SLAM algorithms |
| `spanda-nav` | Path planning |
| `spanda-manipulation` | Arm manipulation |
| `spanda-hri` | Human-robot interaction |
| `spanda-digital-twin` | Digital twin sync |
| `spanda-sim-gazebo` | Gazebo backend |
| `spanda-sim-webots` | Webots backend |

## Adding dependencies

From registry (local stub):

```bash
spanda add spanda-ros2 --version 0.1.0
spanda add spanda-openai --version 0.1.0
spanda install
```

From a local path:

```bash
spanda add my-lib --path ../my-lib
```

From Git:

```bash
spanda add spanda-nav --git https://github.com/spanda/spanda-nav
```

## Dependency resolution

Resolution order:

1. **Local path** — reads `spanda.toml` from the path, locks exact version
2. **Git** — locks URL + branch/tag/rev (no fetch in foundation; metadata only)
3. Registry — selects highest version from hosted index (default) or local stub

Run `spanda install` after changing dependencies to regenerate `spanda.lock`.

## Publishing (foundation)

```bash
spanda publish
```

Validates manifest, capabilities, hardware requirements, safety level, and license before marking the package publish-ready. Maintainers run `./scripts/build-registry.sh` and commit tarballs under `registry/packages/`.

## Version constraints

Supported semver operators: exact (`0.1.0`), caret (`^0.1.0`), comparisons (`>=0.1.0, <1.0.0`).

The lockfile pins exact resolved versions for reproducibility.
