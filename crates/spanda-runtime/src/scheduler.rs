//! Scheduler clock modes for cooperative sim-time vs wall-clock RTOS ticks.

use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};

/// How the runtime advances task scheduling time.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum SchedulerClock {
    /// Discrete-event simulation (default, deterministic).
    #[default]
    Sim,

    /// Wall-clock scheduling with real sleeps between ticks.
    Wall,
}

impl SchedulerClock {
    pub fn as_str(self) -> &'static str {
        // Description:
        //     As str.
        //
        // Inputs:
        //     None.
        //
        // Outputs:
        //     result: &'static str
        //         Return value from `as_str`.
        //
        // Example:

        //     let result = instance.as_str();

        match self {
            SchedulerClock::Sim => "sim",
            SchedulerClock::Wall => "wall",
        }
    }
}

/// Sleep until an absolute wall-clock deadline when in wall mode.
pub fn sleep_until(deadline: Instant) {
    // Description:
    //     Sleep until.
    //
    // Inputs:
    //     deadline: Instant
    //         Caller-supplied deadline.
    //
    // Outputs:
    //     None.
    //
    // Example:

    //     let result = spanda_runtime::scheduler::sleep_until(deadline);

    let now = Instant::now();
    if deadline > now {
        std::thread::sleep(deadline - now);
    }
}

/// Compute elapsed milliseconds between two monotonic instants.
pub fn elapsed_ms(start: Instant, end: Instant) -> f64 {
    // Description:
    //     Elapsed ms.
    //
    // Inputs:
    //     star: Instant
    //         Caller-supplied star.
    //     end: Instant
    //         Caller-supplied end.
    //
    // Outputs:
    //     result: f64
    //         Return value from `elapsed_ms`.
    //
    // Example:

    //     let result = spanda_runtime::scheduler::elapsed_ms(star, end);

    end.duration_since(start).as_secs_f64() * 1000.0
}

/// Advance a wall-clock tick anchor by the nominal period.
pub fn advance_wall_tick(anchor: &mut Instant, period_ms: f64) -> Instant {
    // Description:
    //     Advance wall tick.
    //
    // Inputs:
    //     anchor: &mut Instant
    //         Caller-supplied anchor.
    //     period_ms: f64
    //         Caller-supplied period ms.
    //
    // Outputs:
    //     result: Instant
    //         Return value from `advance_wall_tick`.
    //
    // Example:

    //     let result = spanda_runtime::scheduler::advance_wall_tick(anchor, period_ms);

    let deadline = *anchor;
    *anchor += Duration::from_secs_f64(period_ms / 1000.0);
    deadline
}
