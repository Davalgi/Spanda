//! AST accessor helpers, task scheduling metadata, and declaration extension traits.

use spanda_ast::foundations::{TaskDecl, TaskPriority, TriggerKind};
use spanda_ast::nodes::{BehaviorDecl, Expr, RobotDecl, SafetyRule, SafetyZoneDecl, Stmt};

type BehaviorContracts = (Vec<Stmt>, Option<Expr>, Option<Expr>, Option<Expr>);
type TaskContracts = (Vec<Stmt>, f64, Option<Expr>, Option<Expr>, Option<Expr>);

pub(super) struct TaskSchedule {
    pub(super) name: String,
    pub(super) priority: TaskPriority,
    pub(super) interval_ms: f64,
    pub(super) deadline_ms: Option<f64>,
    pub(super) jitter_ms_max: Option<f64>,
    pub(super) isolated: bool,
    pub(super) next_due_ms: f64,
    pub(super) last_start_ms: Option<f64>,
    pub(super) body: Vec<Stmt>,
    pub(super) requires: Option<Expr>,
    pub(super) ensures: Option<Expr>,
    pub(super) invariant: Option<Expr>,
    pub(super) budget: Option<spanda_ast::foundations::ResourceBudgetDecl>,
}

pub(super) const RUNTIME_TASK_COST_MS: f64 = 5.0;

pub(super) fn task_budget_violation_kind(
    budget: &spanda_ast::foundations::ResourceBudgetDecl,
    duration_ms: f64,
    interval_ms: f64,
) -> Option<&'static str> {
    // Description:
    //     Task budget violation kind.
    //
    // Inputs:
    //     budge: &spanda_ast::foundations::ResourceBudgetDecl
    //         Caller-supplied budge.
    //     duration_ms: f64
    //         Caller-supplied duration ms.
    //     interval_ms: f64
    //         Caller-supplied interval ms.
    //
    // Outputs:
    //     result: Option<&'static str>
    //         Return value from `task_budget_violation_kind`.
    //
    // Example:

    //     let result = spanda_interpreter::runtime_decl_extensions::task_budget_violation_kind(budge, duration_ms, interval_ms);

    let spanda_ast::foundations::ResourceBudgetDecl::ResourceBudgetDecl {
        cpu_pct_max,
        battery_pct_max,
        ..
    } = budget;
    let interval = interval_ms.max(1.0);
    let duty_pct = (duration_ms / interval) * 100.0;

    if let Some(cpu_max) = cpu_pct_max {
        if duty_pct > *cpu_max {
            return Some("cpu");
        }
    }

    if let Some(bat_max) = battery_pct_max {
        if duty_pct > *bat_max {
            return Some("battery");
        }
    }
    None
}

impl TaskSchedule {
    pub(super) fn priority_rank(&self) -> u8 {
        // Description:
        //     Priority rank.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //
        // Outputs:
        //     result: u8
        //         Return value from `priority_rank`.
        //
        // Example:

        //     let result = spanda_interpreter::runtime_decl_extensions::priority_rank(&self);

        let isolation_rank = if self.isolated { 0 } else { 1 };
        let priority_rank = match self.priority {
            TaskPriority::Critical => 0,
            TaskPriority::High => 1,
            TaskPriority::Normal => 2,
            TaskPriority::Low => 3,
        };
        isolation_rank * 10 + priority_rank
    }
}

pub(super) fn priority_label(priority: TaskPriority) -> &'static str {
    // Description:
    //     Priority label.
    //
    // Inputs:
    //     priority: TaskPriority
    //         Caller-supplied priority.
    //
    // Outputs:
    //     result: &'static str
    //         Return value from `priority_label`.
    //
    // Example:

    //     let result = spanda_interpreter::runtime_decl_extensions::priority_label(priority);

    match priority {
        TaskPriority::Critical => "critical",
        TaskPriority::High => "high",
        TaskPriority::Normal => "normal",
        TaskPriority::Low => "low",
    }
}

pub(super) fn trigger_category_label(kind: &TriggerKind) -> &'static str {
    // Description:
    //     Trigger category label.
    //
    // Inputs:
    //     kind: &TriggerKind
    //         Caller-supplied kind.
    //
    // Outputs:
    //     result: &'static str
    //         Return value from `trigger_category_label`.
    //
    // Example:

    //     let result = spanda_interpreter::runtime_decl_extensions::trigger_category_label(kind);

    match kind {
        TriggerKind::Event { .. } => "event",
        TriggerKind::Message { .. } => "message",
        TriggerKind::Timer { .. } => "timer",
        TriggerKind::Condition { .. } => "condition",
        TriggerKind::StateEntered { .. } => "state_entered",
        TriggerKind::StateExited { .. } => "state_exited",
        TriggerKind::Safety { .. } => "safety",
        TriggerKind::Hardware { .. } => "hardware",
        TriggerKind::Ai { .. } => "ai",
        TriggerKind::Verification { .. } => "verification",
        TriggerKind::Twin { .. } => "twin",
        TriggerKind::LogMatch { .. } => "log_match",
        TriggerKind::MessageMatch { .. } => "message_match",
        TriggerKind::Connectivity { .. } => "connectivity",
        TriggerKind::Geofence { .. } => "geofence",
        TriggerKind::SensorEvent { .. } => "sensor_event",
        TriggerKind::KillSwitch { .. } => "kill_switch",
    }
}

pub(super) trait RobotDeclExt {
    fn first_behavior_name(&self) -> Option<String>;
    fn behavior_with_contracts(&self, name: &str) -> Option<BehaviorContracts>;
    fn task_with_contracts(&self, name: &str) -> Option<TaskContracts>;
    fn all_task_schedules(&self) -> Vec<TaskSchedule>;
}

impl RobotDeclExt for RobotDecl {
    fn first_behavior_name(&self) -> Option<String> {
        // Description:
        //     First behavior name.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //
        // Outputs:
        //     result: Option<String>
        //         Return value from `first_behavior_name`.
        //
        // Example:

        //     let result = spanda_interpreter::runtime_decl_extensions::first_behavior_name(&self);

        let RobotDecl::RobotDecl {
            behaviors, tasks, ..
        } = self;

        if let Some(b) = behaviors.first() {
            return match b {
                BehaviorDecl::BehaviorDecl { name, .. } => Some(name.clone()),
            };
        }
        tasks.first().map(|t| match t {
            TaskDecl::TaskDecl { name, .. } => name.clone(),
        })
    }

    fn behavior_with_contracts(&self, name: &str) -> Option<BehaviorContracts> {
        // Description:
        //     Behavior with contracts.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //     name: &str
        //         Caller-supplied name.
        //
        // Outputs:
        //     result: Option<BehaviorContracts>
        //         Return value from `behavior_with_contracts`.
        //
        // Example:

        //     let result = spanda_interpreter::runtime_decl_extensions::behavior_with_contracts(&self, name);

        let RobotDecl::RobotDecl { behaviors, .. } = self;
        behaviors.iter().find_map(|b| match b {
            BehaviorDecl::BehaviorDecl {
                name: n,
                requires,
                ensures,
                invariant,
                body,
                ..
            } if n == name => Some((
                body.clone(),
                requires.clone(),
                ensures.clone(),
                invariant.clone(),
            )),
            _ => None,
        })
    }

    fn task_with_contracts(&self, name: &str) -> Option<TaskContracts> {
        // Description:
        //     Task with contracts.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //     name: &str
        //         Caller-supplied name.
        //
        // Outputs:
        //     result: Option<TaskContracts>
        //         Return value from `task_with_contracts`.
        //
        // Example:

        //     let result = spanda_interpreter::runtime_decl_extensions::task_with_contracts(&self, name);

        let RobotDecl::RobotDecl { tasks, .. } = self;
        tasks.iter().find_map(|t| match t {
            TaskDecl::TaskDecl {
                name: n,
                priority: _priority,
                interval_ms,
                requires,
                ensures,
                invariant,
                body,
                ..
            } if n == name => Some((
                body.clone(),
                *interval_ms,
                requires.clone(),
                ensures.clone(),
                invariant.clone(),
            )),
            _ => None,
        })
    }

    fn all_task_schedules(&self) -> Vec<TaskSchedule> {
        // Description:
        //     All task schedules.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //
        // Outputs:
        //     result: Vec<TaskSchedule>
        //         Return value from `all_task_schedules`.
        //
        // Example:

        //     let result = spanda_interpreter::runtime_decl_extensions::all_task_schedules(&self);

        let RobotDecl::RobotDecl { tasks, .. } = self;
        tasks
            .iter()
            .map(|t| match t {
                TaskDecl::TaskDecl {
                    name,
                    priority,
                    interval_ms,
                    deadline_ms,
                    jitter_ms_max,
                    isolated,
                    requires,
                    ensures,
                    invariant,
                    budget,
                    body,
                    ..
                } => TaskSchedule {
                    name: name.clone(),
                    priority: *priority,
                    interval_ms: *interval_ms,
                    deadline_ms: *deadline_ms,
                    jitter_ms_max: *jitter_ms_max,
                    isolated: *isolated,
                    next_due_ms: 0.0,
                    last_start_ms: None,
                    body: body.clone(),
                    requires: requires.clone(),
                    ensures: ensures.clone(),
                    invariant: invariant.clone(),
                    budget: budget.clone(),
                },
            })
            .collect()
    }
}

pub(super) trait SocDeclExt {
    fn profile(&self) -> &str;
}

impl SocDeclExt for spanda_ast::nodes::SocDecl {
    fn profile(&self) -> &str {
        // Description:
        //     Profile.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //
        // Outputs:
        //     result: &str
        //         Return value from `profile`.
        //
        // Example:

        //     let result = spanda_interpreter::runtime_decl_extensions::profile(&self);

        match self {
            spanda_ast::nodes::SocDecl::SocDecl { profile, .. } => profile,
        }
    }
}

pub(super) trait HalBlockExt {
    fn members(&self) -> &[spanda_ast::nodes::HalMemberDecl];
}

impl HalBlockExt for spanda_ast::nodes::HalBlock {
    fn members(&self) -> &[spanda_ast::nodes::HalMemberDecl] {
        // Description:
        //     Members.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //
        // Outputs:
        //     result: &[spanda_ast::nodes::HalMemberDecl]
        //         Return value from `members`.
        //
        // Example:

        //     let result = spanda_interpreter::runtime_decl_extensions::members(&self);

        match self {
            spanda_ast::nodes::HalBlock::HalBlock { members, .. } => members,
        }
    }
}

pub(super) trait SafetyBlockExt {
    fn rules(&self) -> &[SafetyRule];
    fn zones(&self) -> &[SafetyZoneDecl];
}

impl SafetyBlockExt for spanda_ast::nodes::SafetyBlock {
    fn rules(&self) -> &[SafetyRule] {
        // Description:
        //     Rules.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //
        // Outputs:
        //     result: &[SafetyRule]
        //         Return value from `rules`.
        //
        // Example:

        //     let result = spanda_interpreter::runtime_decl_extensions::rules(&self);

        match self {
            spanda_ast::nodes::SafetyBlock::SafetyBlock { rules, .. } => rules,
        }
    }

    fn zones(&self) -> &[SafetyZoneDecl] {
        // Description:
        //     Zones.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //
        // Outputs:
        //     result: &[SafetyZoneDecl]
        //         Return value from `zones`.
        //
        // Example:

        //     let result = spanda_interpreter::runtime_decl_extensions::zones(&self);

        match self {
            spanda_ast::nodes::SafetyBlock::SafetyBlock { zones, .. } => zones,
        }
    }
}
