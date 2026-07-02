//! What-if failure scenario analysis — predict mission outcomes under injected failures.
//!
mod scenario;

pub use scenario::{
    default_scenarios, format_what_if_report, normalize_scenario, run_what_if_analysis,
    WhatIfFormat, WhatIfOptions, WhatIfReport, WhatIfScenarioResult, WhatIfScenarioSummary,
};
