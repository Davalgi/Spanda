# Spanda Type System

Spanda’s type system supports general-purpose programming and autonomous-systems domains: physical
units, spatial math, sensors, AI, agents, safety, digital twins, and distributed robotics.

## Foundation types

| Type | Example |
|------|---------|
| `Int` | `let n: Int = 42;` |
| `Float` | `let x: Float = 0.5;` |
| `Bool` | `let ok: Bool = true;` |
| `String` | `let s: String = "rover";` |
| `Char` | `let c: Char;` |
| `Bytes` | `let buf: Bytes;` |
| `Null` | `let empty: Null;` |

Type annotations are optional when an initializer is present; the checker infers types from
expressions.

```spanda
let count: Int = 3;
let label: String = "spanda";
```

## Generic types

Collections and distributed types use angle-bracket generics:

```spanda
let goals: Array<Goal>;
let map: Map<String, Int>;
let stream: Topic<LidarScan>;
let svc: Service<Command, Feedback>;
let nav: Action<Command, Feedback, Path>;
```

The compiler reports arity errors when generic parameters are missing or extra, e.g. `Array`
requires exactly one type argument.

### User-defined generics (Experimental)

Spanda also supports **type parameters** on module functions and structs:

```spanda
module std.collections;

export fn identity<T>(value: T) -> T {
  return value;
}

struct Box<T> {
  value: T;
}
```

**Current limits (honest):**

| Capability | Status |
|------------|--------|
| Module / export `fn` type params | Supported (inference from call args) |
| `struct Name<T>` + `Name<Int> { … }` literals | Supported |
| Trait / enum / agent type params | Not supported |
| `where` bounds / trait bounds | Not supported |
| Cross-module generic struct export parity | Limited — prefer same-program declarations |
| Full Hindley–Milner inference | Not supported — annotate when ambiguous |

Generics remain **Experimental** in [feature-status.md](./feature-status.md) until bounds and
broader declaration parity land with tests. Do not treat them as Stable language surface yet.

## Traits and `impl` (Stable API, compilation-unit scope)

```spanda
trait PathPlanner {
  fn plan(goal: Pose) -> Path;
}

robot R {
  agent Nav { … }
  impl PathPlanner for Nav {
    fn plan(goal: Pose) -> Path { … }
  }
}
```

Traits and `impl Trait for Agent` are resolved in the **same compilation unit** (one `.sd`
program after imports are expanded). There is no `export trait` yet — a trait used by an `impl`
must be declared in that program (or brought in via source that the checker sees as one unit).
`dyn Trait` objects are supported for typed trait-object values.

## Stringly seams (literal validation)

Some configuration still uses strings. The type checker **rejects unknown string literals** for:

| Seam | Accepted literals |
|------|-------------------|
| `ai_model { provider: "…" }` | `mock`, `openai`, `anthropic`, `onnx` |
| `serialize` / `deserialize` format | `json`, `yaml`, `binary` |

Non-literal expressions (variables) remain runtime-checked. CLI `spanda codegen --target` accepts
only `native`, `wasm`, or `esp32` (unknown targets exit with an error). Full typed enums for these
seams are a future hardening step; string shims stay for compatibility.

Unit-aware types prevent mixing incompatible dimensions:

```spanda
let speed: Velocity = 1.5 m/s;
let distance: Distance = 2.0 m;
let timeout: Duration = 500 ms;
```

Invalid operations are rejected at compile time:

```spanda
// ERROR: speed + distance — incompatible physical categories
let bad = speed + distance;
```

Supported unit types include `Distance`, `Velocity`, `Acceleration`, `Angle`, `AngularVelocity`,
`Mass`, `Force`, `Power`, `Voltage`, `Current`, `Temperature`, and `Pressure`.

### Unit literals

| Category | Canonical | Also accepted |
|----------|-----------|---------------|
| Distance | `m` | `mm`, `cm`, `km`, `ft`, `in` |
| Duration | `s` | `ms`, `us`, `min`, `h` |
| Velocity | `m/s` | `km/h`, `mph` |
| Acceleration | `m/s²` | `g` (standard gravity) |
| Angle | `rad` | `deg` |
| Angular velocity | `rad/s` | `deg/s` |
| Mass | `kg` | `gram`, `lb` |
| Force | `N` | `kN` |
| Power | `W` | `kW`, `MW` |
| Voltage | `V` | `mV`, `kV` |
| Current | `A` | `mA` |
| Temperature | `celsius` | `fahrenheit`, `kelvin` |
| Pressure | `Pa` | `kPa`, `bar`, `mbar`, `psi` |
| Frequency | `Hz` | `kHz`, `MHz` |

### Sensor / environmental units

| Category | Type name | Canonical | Also accepted |
|----------|-----------|-----------|---------------|
| Humidity | `Humidity` | `rh` | `%RH` |
| Illuminance | `Illuminance` | `lux` | `lx` |
| Luminance | `Luminance` | `cd/m²` | `nit` |
| Gas concentration | `Concentration` | `ppm` | `ppb` |
| Sound level | `SoundLevel` | `dB` | `dBA` |
| Magnetic field | `MagneticField` | `uT` | `gauss` |
| Rotational speed | `RotationalSpeed` | `rpm` | — |
| Torque | `Torque` | `N·m` | `Nm` |
| Energy | `Energy` | `J` | `Wh`, `kWh` |
| UV index | `UvIndex` | `uvi` | — |
| Acidity | `Ph` | `pH` | — |
| Conductivity | `Conductivity` | `uS/cm` | `mS/cm`, `S/m` |
| Particulate matter | `ParticulateMatter` | `ug/m3` | `µg/m³` |
| Turbidity | `Turbidity` | `NTU` | `FNU` |
| Salinity | `Salinity` | `ppt` | `psu` |
| Radiation | `Radiation` | `uSv/h` | `mSv/h` |
| Soil moisture | `SoilMoisture` | `%VWC` | `vwc` |

```spanda
let humidity: Humidity = 65 %RH;
let ambient: Illuminance = 320 lux;
let co2: Concentration = 800 ppm;
let noise: SoundLevel = 42 dBA;
let uv: UvIndex = 6.5 uvi;
let acidity: Ph = 7.2 pH;
let ec: Conductivity = 850 uS/cm;
let pm25: ParticulateMatter = 12 ug/m3;
let turbidity: Turbidity = 4.5 NTU;
let salt: Salinity = 35 ppt;
let dose: Radiation = 0.12 uSv/h;
let soil: SoilMoisture = 42 %VWC;
```

Compatible units may be mixed in comparisons and addition (e.g. `500 ms + 0.5 s`, `100 cm + 1 m`).
Incompatible dimensions are rejected at compile time.

## Time types

```spanda
let timeout: Duration = 500 ms;
let started_at: Timestamp;
```

Namespace: `import std.time;`

## Spatial and robotics types

`Point2D`, `Point3D`, `Vector2D`, `Vector3D`, `Quaternion`, `Pose`, `Transform`, `Trajectory`,
`Path`, `Waypoint`, `MotionCommand`, `ControlSignal`, `PIDConfig`.

Namespace: `import std.spatial;`

## Sensor types

`CameraFrame`, `Image`, `DepthImage`, `PointCloud`, `LidarScan`, `GpsFix`, `ImuData`, `AudioFrame`.

Namespace: `import std.sensors;`

## AI types

`LLM`, `VisionModel`, `EmbeddingModel`, `Prompt`, `Completion`, `Embedding`, `Token`, `Context`,
`Memory`, `Plan`, `ReasoningTrace`, `Goal`.

Namespace: `import std.ai;`

### AI model hardware config

`ai_model` blocks accept config keys used by hardware verification:

```spanda
ai_model Vision: VisionModel {
  memory_required: 2 GB;
  gpu_required: true;
}
```

| Config key | Verification |
|------------|----------------|
| `memory_required` | Compared to target profile `memory` |
| `gpu_required` | Target must have GPU / `gpu_tops` |

## Agent and autonomy types

`Agent`, `Goal`, `Task`, `Skill`, `Capability`, `Intent`, `ActionProposal`, `SafeAction`.

### ActionProposal vs SafeAction

`ActionProposal` is **untrusted** output from AI planners. It must never reach actuators directly.

```spanda
let proposal: ActionProposal = planner.reason(prompt: "go");
let action: SafeAction = safety.validate(proposal);
wheels.execute(action);   // OK
```

```spanda
wheels.execute(proposal);   // COMPILE ERROR
wheels.drive(linear: proposal.linear, angular: proposal.angular);  // COMPILE ERROR
```

The type checker rejects `ActionProposal` passed to `actuator.execute()`, and rejects
`ActionProposal.linear` / `.angular` (opaque `UntrustedLinear` / `UntrustedAngular`) as arguments to
`drive()` / `follow()`. Only `SafeAction` from `safety.validate()` may reach `execute()`. Literal
non-AI `drive(linear: …, angular: …)` remains valid and is envelope-clamped at runtime.

### Safety motion guarantee (authoritative)

**Compile time:** AI output is typed as `ActionProposal`. Actuator `execute()` accepts only
`SafeAction` from `safety.validate(ActionProposal)`. `ActionProposal` motion components cannot feed
`DifferentialDrive.drive` / `follow` (including via `let` bindings). Non-AI literal `drive` /
`follow(path:)` remain available as low-level APIs. **Runtime (interpreter `run`/`sim`):**
`safety { max_speed = … }` clamps linear velocity on `drive`, `execute`, and `follow(path:)` cruise
speed (default 0.5 m/s, reduced by `max_speed` / zone caps at the call pose; also inside
`safety.validate`); optional `max_angular = … rad/s` clamps turn rate on `drive` / `execute`;
`stop_if`, zones, and emergency stop still gate motion via `before_motion`. **Not claimed:**
`follow(path:)` does not re-derive SafeAction per waypoint; hard real-time deadlines are
intent/monitoring on the interpreter path, not OS-level guarantees.

## Human interaction types

`Command`, `Conversation`, `Speech`, `Gesture`, `Emotion`, `Feedback`.

Namespace: `import std.hri;`

## Safety types

`Risk`, `Hazard`, `SafetyConstraint`, `EmergencyStop`, `SafeAction`.

Namespace: `import std.safety;`

## Digital twin types

`Twin`, `SimulationState`, `Telemetry`, `Replay`, `Fault`, `Scenario`.

Namespace: `import std.twin;`

Fault types for `simulate_compatibility` (verification, not runtime twin API): `CameraFailure`,
`LidarFailure`, `ImuFailure`, `BatteryDegradation`, `NetworkOutage`.

## Hardware compatibility types

Declared in programs, not runtime values:

| Construct | Role |
|-----------|------|
| `hardware Profile { }` | Platform capability declaration |
| `deploy Robot to Target` | Deployment binding |
| `requires_hardware { }` | Minimum platform requirements |
| `requires_network { }` | Connectivity requirements |
| `budget { }` | Per-task resource limits |
| `mission { duration }` | Mission length for power estimation |

Verification output types (Rust/JSON): `CompatibilityReport`, `CompatItem`, `CompatibilityMatrix`.

See [hardware-compatibility.md](./hardware-compatibility.md).

## Networking / distributed robotics

`Topic<T>`, `Message<T>`, `Service<Request, Response>`, `Action<Request, Feedback, Result>`,
`Endpoint`.

## Advanced autonomous intelligence

`KnowledgeGraph`, `Belief`, `Observation`, `WorldModel`, `Policy`, `Reward`, `StateEstimate`.

## Standard library namespaces

| Module | Domain |
|--------|--------|
| `std.time` | Time and duration |
| `std.units` | Physical units |
| `std.spatial` | Pose, path, transforms |
| `std.ai` | Models and reasoning |
| `std.robotics` | Agents, motion, capabilities |
| `std.sensors` | Perception data |
| `std.safety` | Constraints and safe actions |
| `std.twin` | Simulation and replay |
| `std.hri` | Human–robot interaction |

Import with `import std.units;` then annotate types normally (`Distance`, `Velocity`, …).

## Examples

See `examples/types/` for annotated programs covering each category.

```bash
spanda check examples/types/units.sd
spanda run examples/types/safety.sd
```
