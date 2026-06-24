# Chapter 7 — Glossary

Back to [index](./README.md)

Plain-English definitions. Alphabetical.

---

| Term | In plain English |
|------|------------------|
| **ActionProposal** | “The AI thinks we should do this.” Not allowed to move motors directly. |
| **Actuator** | Something the robot moves: wheels, arm, gripper. |
| **Agent** | An AI worker with a goal, tools, and a `plan` block. |
| **Behavior** | Named robot logic — usually your main `patrol()` or `run()` entry point. |
| **check** | Compile/type-check without running. |
| **Deploy** | “This robot program targets this hardware board.” |
| **Digital twin** | A software shadow of the robot that mirrors pose/state for sim and replay. |
| **`.sd` file** | Spanda source file. |
| **Emergency stop** | Full stop — triggered by `stop_if` or safety rules. |
| **Event** | Named signal handlers can listen for (`on MyEvent`). |
| **Fusion** | Combining multiple sensor streams via `observe { }` + `fusion.read()`. |
| **Hardware profile** | Description of a board: CPU, memory, sensors, actuators. |
| **LLM** | Large language model — declared as `ai_model ... LLM`. |
| **Module** | Named file/unit for shared functions (`module navigation;`). |
| **Mission trace** | Recording of a sim run (`.trace` file) for replay and debugging. |
| **Option** | A value that might be missing (`Some` / `None`). |
| **Result** | Success or failure (`Ok` / `Err`) without exceptions. |
| **SafeAction** | Safety-approved motion command — only type actuators accept from AI. |
| **Safety block** | `safety { }` — max speed, stop conditions, zones. |
| **Sensor** | Input device: lidar, camera, IMU. |
| **Spanda** | The platform and language name; Sanskrit *the divine pulse*, pronounced **SPUN-duh** (/ˈspʌndə/). |
| **sim** | Run with verbose simulation output. |
| **spawn** | Queue a background function call. |
| **Task** | Periodic scheduled work (`task foo every 50ms`). |
| **Trait** | Interface an agent can implement (`trait Navigator`). |
| **Trigger** | “When X happens, run Y” — events, timers, conditions. |
| **Twin** | See digital twin. |
| **Units** | `m/s`, `rad`, `m`, `ms` — enforced by the compiler. |
| **verify** | Check program vs hardware profile before deploy. |

---

## Acronyms

| Acronym | Meaning |
|---------|---------|
| **CLI** | Command-line interface (`spanda` in the terminal) |
| **DAP** | Debug Adapter Protocol (IDE debugging) |
| **FFI** | Foreign function interface — call C++/Python from Spanda |
| **HAL** | Hardware abstraction layer — board-specific imports |
| **IMU** | Inertial measurement unit (accelerometer + gyro) |
| **Lidar** | Laser distance sensor |
| **LLM** | Large language model |
| **QoS** | Quality of service (topic delivery guarantees) |
| **ROS / ROS2** | Robot Operating System — messaging ecosystem |
| **WASM** | WebAssembly — run Spanda in the browser playground |

---

## “Which doc do I read?”

| I want to… | Read |
|------------|------|
| Learn casually | This guide |
| Learn systematically | [Spanda 101](../spanda-101/README.md) |
| Look up syntax | [spanda-language.md](../spanda-language.md) |
| Look up every function | [spanda-reference.md](../spanda-reference.md) |
| Install | [installation.md](../installation.md) |
| Impress my team | [killer-demo.md](../killer-demo.md) |

---

Back to [Spanda for Dummies index](./README.md)
