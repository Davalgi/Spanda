//! Mission differencing and change-impact analysis between Spanda programs.
//!
pub mod diff;

pub use diff::{
    diff_programs, diff_programs_with_capabilities, format_mission_diff, DiffChangeKind,
    MissionDiffChange, MissionDiffDimension, MissionDiffReport,
};
