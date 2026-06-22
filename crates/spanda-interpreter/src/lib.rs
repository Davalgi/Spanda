//! Spanda interpreter staging crate — re-exports the public run API from `spanda-core`.
//!
//! Phase 4 lean-core extraction target. The full `Interpreter` body (~9k lines) remains in
//! `spanda-core/src/runtime.rs` as the orchestration root until subsystems finish migrating to
//! [`spanda_runtime::RuntimeHost`].
//!

pub use spanda_core::replay::{MissionTrace, PlaybackReport, TraceVerification};
pub use spanda_core::runtime::{Interpreter, InterpreterOptions, RobotBackend};
pub use spanda_core::simulator::{
    create_default_simulator, Obstacle, Simulator, SimulatorConfig,
};
pub use spanda_core::telemetry::ExecutionMetrics;
pub use spanda_core::{
    playback_mission, replay_mission, run, run_program, run_tests, run_tests_with_registry,
    ObstacleConfig, PoseState, RobotState, RunOptions, RunResult, SpandaError, TestRunResult,
    VelocityState,
};
pub use spanda_runtime::RuntimeHost;

/// In-process simulator backend implementing [`RobotBackend`].
pub type SimRobotBackend = Simulator;
