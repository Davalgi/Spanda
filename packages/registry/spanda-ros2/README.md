# spanda-ros2

Official Spanda package: **ROS 2 integration**

## Import

```spanda
import robotics.ros2;
```

## Live backend

When `spanda-ros2` is installed, the runtime registers `crates/spanda-transport-ros2`
(native rclrs + rclpy daemon) and marks the ROS 2 comm-bus transport as registry-backed.

## Status

Spanda-language exports in `src/` are scaffold stubs. Live ROS 2 transport is implemented in
the `spanda-transport-ros2` workspace crate (core compatibility shim: `transport_rclrs.rs`).
