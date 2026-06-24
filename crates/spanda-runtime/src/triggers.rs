//! Unified trigger-based execution model for Spanda autonomous systems.
//!
//! Triggers unify events, messages, timers, conditions, state transitions, safety,
//! hardware, AI, verification, and digital-twin reactive handlers under one registry.

use spanda_ast::foundations::{TaskPriority, TriggerHandlerDecl, TriggerKind};
use spanda_ast::nodes::{Span, SpandaType, Stmt};
use std::collections::{HashMap, HashSet};

/// Maximum trigger dispatches per scheduler tick (prevents trigger storms).
pub const MAX_TRIGGERS_PER_TICK: usize = 64;

/// Registered trigger handler with stable id for metrics.
#[derive(Debug, Clone)]
pub struct RegisteredTrigger {
    pub id: usize,
    pub name: String,
    pub kind: TriggerKind,
    pub priority: TaskPriority,
    pub body: Vec<Stmt>,

    /// Agent scope when declared inside an agent block.
    pub agent: Option<String>,
}

/// Unified registry for all trigger categories.
#[derive(Debug, Default)]
pub struct TriggerRegistry {
    handlers: Vec<RegisteredTrigger>,
    event_index: HashMap<String, usize>,
    next_id: usize,
}

impl TriggerRegistry {
    pub fn new() -> Self {
        // Description:
        //     Construct a new instance.
        //
        // Inputs:
        //     None.
        //
        // Outputs:
        //     result: Self
        //         Return value from `new`.
        //
        // Example:
        //     let value = spanda_runtime::triggers::new();

        // Build the result via default.
        Self::default()
    }

    pub fn register(&mut self, decl: &TriggerHandlerDecl, agent: Option<String>) {
        // Description:
        //     Register.
        //
        // Inputs:
        //     &mut self: input value
        //         Caller-supplied &mut self.
        //     decl: &TriggerHandlerDecl
        //         Caller-supplied decl.
        //     agen: Option<String>
        //         Caller-supplied agen.
        //
        // Outputs:
        //     None.
        //
        // Example:
        //     let result = spanda_runtime::triggers::register(&mut self, decl, agen);

        // Compute TriggerHandlerDecl for the following logic.
        let TriggerHandlerDecl::TriggerHandlerDecl {
            trigger_kind,
            priority,
            body,
            span,
            ..
        } = decl;
        let name = trigger_display_name(trigger_kind, agent.as_deref());
        let id = self.next_id;
        self.next_id += 1;

        // For event triggers, record the event name in the index.
        if let TriggerKind::Event { name: event_name } = trigger_kind {
            self.event_index.insert(event_name.clone(), id);
        }
        self.handlers.push(RegisteredTrigger {
            id,
            name,
            kind: trigger_kind.clone(),
            priority: *priority,
            body: body.clone(),
            agent,
        });
        let _ = span;
    }

    pub fn register_legacy_event(&mut self, event_name: String, body: Vec<Stmt>) {
        // Description:
        //     Register legacy event.
        //
        // Inputs:
        //     &mut self: input value
        //         Caller-supplied &mut self.
        //     event_name: String
        //         Caller-supplied event name.
        //     body: Vec<Stmt>
        //         Caller-supplied body.
        //
        // Outputs:
        //     None.
        //
        // Example:
        //     let result = spanda_runtime::triggers::register_legacy_event(&mut self, event_name, body);

        // Register the value handler.
        self.register(
            &TriggerHandlerDecl::TriggerHandlerDecl {
                trigger_kind: TriggerKind::Event {
                    name: event_name.clone(),
                },
                priority: TaskPriority::Normal,
                return_type: SpandaType::Void,
                body,
                span: Span::default(),
            },
            None,
        );
    }

    pub fn handler_count(&self) -> usize {
        // Description:
        //     Handler count.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //
        // Outputs:
        //     result: usize
        //         Return value from `handler_count`.
        //
        // Example:
        //     let result = spanda_runtime::triggers::handler_count(&self);

        // Call len on the current instance.
        self.handlers.len()
    }

    pub fn all(&self) -> &[RegisteredTrigger] {
        // Description:
        //     All.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //
        // Outputs:
        //     result: &[RegisteredTrigger]
        //         Return value from `all`.
        //
        // Example:
        //     let result = spanda_runtime::triggers::all(&self);

        // Return handlers from this handle.
        &self.handlers
    }

    pub fn get(&self, id: usize) -> Option<&RegisteredTrigger> {
        // Description:
        //     Get.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //     id: usize
        //         Caller-supplied id.
        //
        // Outputs:
        //     result: Option<&RegisteredTrigger>
        //         Return value from `get`.
        //
        // Example:
        //     let result = spanda_runtime::triggers::get(&self, id);

        // Iterate over handlers.
        self.handlers.iter().find(|h| h.id == id)
    }

    pub fn event_handler_body(&self, event_name: &str) -> Option<&[Stmt]> {
        // Description:
        //     Event handler body.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //     event_name: &str
        //         Caller-supplied event name.
        //
        // Outputs:
        //     result: Option<&[Stmt]>
        //         Return value from `event_handler_body`.
        //
        // Example:
        //     let result = spanda_runtime::triggers::event_handler_body(&self, event_name);

        // Call event index on the current instance.
        self.event_index
            .get(event_name)
            .and_then(|id| self.get(*id))
            .map(|h| h.body.as_slice())
    }

    pub fn handlers_for_event(&self, event_name: &str) -> Vec<&RegisteredTrigger> {
        // Description:
        //     Handlers for event.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //     event_name: &str
        //         Caller-supplied event name.
        //
        // Outputs:
        //     result: Vec<&RegisteredTrigger>
        //         Return value from `handlers_for_event`.
        //
        // Example:
        //     let result = spanda_runtime::triggers::handlers_for_event(&self, event_name);

        // Call handlers on the current instance.
        self.handlers
            .iter()
            .filter(|h| matches!(&h.kind, TriggerKind::Event { name } if name == event_name))
            .collect()
    }

    pub fn handlers_for_message(
        &self,
        topic_name: &str,
        topic_path: &str,
    ) -> Vec<&RegisteredTrigger> {
        // Description:
        //     Handlers for message.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //     opic_name: &str
        //         Caller-supplied opic name.
        //     opic_path: &str
        //         Caller-supplied opic path.
        //
        // Outputs:
        //     result: Vec<&RegisteredTrigger>
        //         Return value from `handlers_for_message`.
        //
        // Example:
        //     let result = spanda_runtime::triggers::handlers_for_message(&self, opic_name, opic_path);

        // Call handlers on the current instance.
        self.handlers
            .iter()
            .filter(|h| match &h.kind {
                TriggerKind::Message { topic } => {
                    topic == topic_name
                        || topic == topic_path
                        || topic_path.ends_with(&format!("/{topic}"))
                        || format!("/{topic}") == topic_path
                }
                _ => false,
            })
            .collect()
    }

    pub fn timer_handlers(&self) -> Vec<&RegisteredTrigger> {
        // Description:
        //     Timer handlers.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //
        // Outputs:
        //     result: Vec<&RegisteredTrigger>
        //         Return value from `timer_handlers`.
        //
        // Example:
        //     let result = spanda_runtime::triggers::timer_handlers(&self);

        // Call handlers on the current instance.
        self.handlers
            .iter()
            .filter(|h| matches!(h.kind, TriggerKind::Timer { .. }))
            .collect()
    }

    pub fn condition_handlers(&self) -> Vec<&RegisteredTrigger> {
        // Description:
        //     Condition handlers.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //
        // Outputs:
        //     result: Vec<&RegisteredTrigger>
        //         Return value from `condition_handlers`.
        //
        // Example:
        //     let result = spanda_runtime::triggers::condition_handlers(&self);

        // Call handlers on the current instance.
        self.handlers
            .iter()
            .filter(|h| matches!(h.kind, TriggerKind::Condition { .. }))
            .collect()
    }

    pub fn handlers_for_state_entered(&self, state: &str) -> Vec<&RegisteredTrigger> {
        // Description:
        //     Handlers for state entered.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //     state: &str
        //         Caller-supplied state.
        //
        // Outputs:
        //     result: Vec<&RegisteredTrigger>
        //         Return value from `handlers_for_state_entered`.
        //
        // Example:
        //     let result = spanda_runtime::triggers::handlers_for_state_entered(&self, state);

        // Call handlers on the current instance.
        self.handlers
            .iter()
            .filter(|h| {
                matches!(
                    &h.kind,
                    TriggerKind::StateEntered { state: s } if s == state
                )
            })
            .collect()
    }

    pub fn handlers_for_state_exited(&self, state: &str) -> Vec<&RegisteredTrigger> {
        // Description:
        //     Handlers for state exited.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //     state: &str
        //         Caller-supplied state.
        //
        // Outputs:
        //     result: Vec<&RegisteredTrigger>
        //         Return value from `handlers_for_state_exited`.
        //
        // Example:
        //     let result = spanda_runtime::triggers::handlers_for_state_exited(&self, state);

        // Call handlers on the current instance.
        self.handlers
            .iter()
            .filter(|h| {
                matches!(
                    &h.kind,
                    TriggerKind::StateExited { state: s } if s == state
                )
            })
            .collect()
    }

    pub fn handlers_for_category(
        &self,
        category: SystemTriggerCategory,
        event: &str,
    ) -> Vec<&RegisteredTrigger> {
        // Description:
        //     Handlers for category.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //     category: SystemTriggerCategory
        //         Caller-supplied category.
        //     even: &str
        //         Caller-supplied even.
        //
        // Outputs:
        //     result: Vec<&RegisteredTrigger>
        //         Return value from `handlers_for_category`.
        //
        // Example:
        //     let result = spanda_runtime::triggers::handlers_for_category(&self, category, even);

        // Call handlers on the current instance.
        self.handlers
            .iter()
            .filter(|h| match (&h.kind, category) {
                (TriggerKind::Safety { event: e }, SystemTriggerCategory::Safety) => e == event,
                (TriggerKind::Hardware { event: e }, SystemTriggerCategory::Hardware) => e == event,
                (TriggerKind::Ai { event: e }, SystemTriggerCategory::Ai) => e == event,
                (TriggerKind::Verification { event: e }, SystemTriggerCategory::Verification) => {
                    e == event
                }
                (TriggerKind::Twin { event: e }, SystemTriggerCategory::Twin) => e == event,
                _ => false,
            })
            .collect()
    }

    pub fn handlers_for_connectivity(&self, domain: &str, event: &str) -> Vec<&RegisteredTrigger> {
        // Description:
        //     Handlers for connectivity.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //     domain: &str
        //         Caller-supplied domain.
        //     even: &str
        //         Caller-supplied even.
        //
        // Outputs:
        //     result: Vec<&RegisteredTrigger>
        //         Return value from `handlers_for_connectivity`.
        //
        // Example:

        //     let result = spanda_runtime::triggers::handlers_for_connectivity(&self, domain, even);

        self.handlers
            .iter()
            .filter(|h| {
                matches!(
                    &h.kind,
                    TriggerKind::Connectivity { domain: d, event: e }
                        if d == domain && e == event
                )
            })
            .collect()
    }

    pub fn handlers_for_geofence(&self, name: &str, phase: &str) -> Vec<&RegisteredTrigger> {
        // Description:
        //     Handlers for geofence.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //     name: &str
        //         Caller-supplied name.
        //     phase: &str
        //         Caller-supplied phase.
        //
        // Outputs:
        //     result: Vec<&RegisteredTrigger>
        //         Return value from `handlers_for_geofence`.
        //
        // Example:

        //     let result = spanda_runtime::triggers::handlers_for_geofence(&self, name, phase);

        self.handlers
            .iter()
            .filter(|h| {
                matches!(
                    &h.kind,
                    TriggerKind::Geofence { name: n, phase: p }
                        if n == name && p == phase
                )
            })
            .collect()
    }

    pub fn handlers_for_kill_switch(&self, name: &str) -> Vec<&RegisteredTrigger> {
        // Description:
        //     Handlers for kill switch.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //     name: &str
        //         Caller-supplied name.
        //
        // Outputs:
        //     result: Vec<&RegisteredTrigger>
        //         Return value from `handlers_for_kill_switch`.
        //
        // Example:

        //     let result = spanda_runtime::triggers::handlers_for_kill_switch(&self, name);

        self.handlers
            .iter()
            .filter(|h| {
                matches!(
                    &h.kind,
                    TriggerKind::KillSwitch { name: n } if n == name
                )
            })
            .collect()
    }

    pub fn handlers_for_sensor_event(&self, sensor: &str, event: &str) -> Vec<&RegisteredTrigger> {
        // Description:
        //     Handlers for sensor event.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //     sensor: &str
        //         Caller-supplied sensor.
        //     even: &str
        //         Caller-supplied even.
        //
        // Outputs:
        //     result: Vec<&RegisteredTrigger>
        //         Return value from `handlers_for_sensor_event`.
        //
        // Example:

        //     let result = spanda_runtime::triggers::handlers_for_sensor_event(&self, sensor, even);

        self.handlers
            .iter()
            .filter(|h| {
                matches!(
                    &h.kind,
                    TriggerKind::SensorEvent { sensor: s, event: e }
                        if s == sensor && e == event
                )
            })
            .collect()
    }

    pub fn sorted_by_priority(handlers: Vec<&RegisteredTrigger>) -> Vec<&RegisteredTrigger> {
        // Description:
        //     Sorted by priority.
        //
        // Inputs:
        //     handlers: Vec<&RegisteredTrigger>
        //         Caller-supplied handlers.
        //
        // Outputs:
        //     result: Vec<&RegisteredTrigger>
        //         Return value from `sorted_by_priority`.
        //
        // Example:
        //     let result = spanda_runtime::triggers::sorted_by_priority(handlers);

        // Create mutable sorted for accumulating results.
        let mut sorted = handlers;
        sorted.sort_by_key(|h| priority_rank(h.priority));
        sorted
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SystemTriggerCategory {
    Safety,
    Hardware,
    Verification,
    Ai,
    Twin,
}

pub fn priority_rank(priority: TaskPriority) -> u8 {
    // Description:
    //     Priority rank.
    //
    // Inputs:
    //     priority: TaskPriority
    //         Caller-supplied priority.
    //
    // Outputs:
    //     result: u8
    //         Return value from `priority_rank`.
    //
    // Example:
    //     let result = spanda_runtime::triggers::priority_rank(priority);

    // Match on priority and handle each case.
    match priority {
        TaskPriority::Critical => 0,
        TaskPriority::High => 1,
        TaskPriority::Normal => 2,
        TaskPriority::Low => 3,
    }
}

pub fn trigger_display_name(kind: &TriggerKind, agent: Option<&str>) -> String {
    // Description:
    //     Trigger display name.
    //
    // Inputs:
    //     kind: &TriggerKind
    //         Caller-supplied kind.
    //     agen: Option<&str>
    //         Caller-supplied agen.
    //
    // Outputs:
    //     result: String
    //         Return value from `trigger_display_name`.
    //
    // Example:
    //     let result = spanda_runtime::triggers::trigger_display_name(kind, agen);

    // Compute base for the following logic.
    let base = match kind {
        TriggerKind::Event { name } => format!("event:{name}"),
        TriggerKind::Message { topic } => format!("message:{topic}"),
        TriggerKind::Timer { interval_ms } => format!("timer:{interval_ms}ms"),
        TriggerKind::Condition { .. } => "condition".into(),
        TriggerKind::StateEntered { state } => format!("state_entered:{state}"),
        TriggerKind::StateExited { state } => format!("state_exited:{state}"),
        TriggerKind::Safety { event } => format!("safety:{event}"),
        TriggerKind::Hardware { event } => format!("hardware:{event}"),
        TriggerKind::Ai { event } => format!("ai:{event}"),
        TriggerKind::Verification { event } => format!("verification:{event}"),
        TriggerKind::Twin { event } => format!("twin:{event}"),
        TriggerKind::LogMatch { pattern } => format!("log_match:/{}/", pattern.source),
        TriggerKind::MessageMatch { field, pattern } => {
            format!("message_match:{field}:/{}/", pattern.source)
        }
        TriggerKind::Connectivity { domain, event } => format!("connectivity:{domain}.{event}"),
        TriggerKind::Geofence { name, phase } => format!("geofence:{name}:{phase}"),
        TriggerKind::SensorEvent { sensor, event } => format!("sensor:{sensor}.{event}"),
        TriggerKind::KillSwitch { name } => format!("kill_switch:{name}"),
    };

    // Emit output when agent provides a agent.
    if let Some(agent) = agent {
        format!("{agent}/{base}")
    } else {
        base
    }
}

/// Per-trigger runtime schedule state for timer triggers.
#[derive(Debug, Clone)]
pub struct TriggerTimerSchedule {
    pub trigger_id: usize,
    pub interval_ms: f64,
    pub next_due_ms: f64,
}

impl TriggerTimerSchedule {
    pub fn from_handler(handler: &RegisteredTrigger) -> Option<Self> {
        // Description:
        //     From handler.
        //
        // Inputs:
        //     handler: &RegisteredTrigger
        //         Caller-supplied handler.
        //
        // Outputs:
        //     result: Option<Self>
        //         Return value from `from_handler`.
        //
        // Example:
        //     let result = spanda_runtime::triggers::from_handler(handler);

        // take this path when let TriggerKind::Timer { interval ms } = handler.kind.
        if let TriggerKind::Timer { interval_ms } = handler.kind {
            Some(Self {
                trigger_id: handler.id,
                interval_ms,
                next_due_ms: 0.0,
            })
        } else {
            None
        }
    }
}

/// Tracks edge state for condition triggers (fire on transition to true).
#[derive(Debug, Default)]
pub struct ConditionTriggerState {
    was_active: HashSet<usize>,
}

impl ConditionTriggerState {
    pub fn should_fire(&mut self, trigger_id: usize, active: bool) -> bool {
        // Description:
        //     Should fire.
        //
        // Inputs:
        //     &mut self: input value
        //         Caller-supplied &mut self.
        //     rigger_id: usize
        //         Caller-supplied rigger id.
        //     active: bool
        //         Caller-supplied active.
        //
        // Outputs:
        //     result: bool
        //         Return value from `should_fire`.
        //
        // Example:
        //     let result = spanda_runtime::triggers::should_fire(&mut self, rigger_id, active);

        // Compute was for the following logic.
        let was = self.was_active.contains(&trigger_id);

        // Take this path when active.
        if active {
            self.was_active.insert(trigger_id);
            !was
        } else {
            self.was_active.remove(&trigger_id);
            false
        }
    }

    pub fn is_level_active(&self, trigger_id: usize) -> bool {
        // Description:
        //     Is level active.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //     rigger_id: usize
        //         Caller-supplied rigger id.
        //
        // Outputs:
        //     result: bool
        //         Return value from `is_level_active`.
        //
        // Example:
        //     let result = spanda_runtime::triggers::is_level_active(&self, rigger_id);

        // Call contains on the current instance.
        self.was_active.contains(&trigger_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use spanda_ast::foundations::TriggerHandlerDecl;

    #[test]
    fn registers_and_sorts_by_priority() {
        // Description:
        //     Registers and sorts by priority.
        //
        // Inputs:
        //     None.
        //
        // Outputs:
        //     None.
        //
        // Example:

        //     let result = spanda_runtime::triggers::registers_and_sorts_by_priority();

        let mut registry = TriggerRegistry::new();
        registry.register(
            &TriggerHandlerDecl::TriggerHandlerDecl {
                trigger_kind: TriggerKind::Safety {
                    event: "EmergencyStop".into(),
                },
                priority: TaskPriority::Normal,
                return_type: SpandaType::Void,
                body: vec![],
                span: Span::default(),
            },
            None,
        );
        registry.register(
            &TriggerHandlerDecl::TriggerHandlerDecl {
                trigger_kind: TriggerKind::Safety {
                    event: "EmergencyStop".into(),
                },
                priority: TaskPriority::Critical,
                return_type: SpandaType::Void,
                body: vec![],
                span: Span::default(),
            },
            None,
        );
        let handlers =
            registry.handlers_for_category(SystemTriggerCategory::Safety, "EmergencyStop");
        let sorted = TriggerRegistry::sorted_by_priority(handlers);
        assert_eq!(sorted.len(), 2);
        assert_eq!(sorted[0].priority, TaskPriority::Critical);
    }

    #[test]
    fn condition_edge_detection() {
        // Description:
        //     Condition edge detection.
        //
        // Inputs:
        //     None.
        //
        // Outputs:
        //     None.
        //
        // Example:

        //     let result = spanda_runtime::triggers::condition_edge_detection();

        let mut state = ConditionTriggerState::default();
        assert!(state.should_fire(1, true));
        assert!(!state.should_fire(1, true));
        assert!(!state.should_fire(1, false));
        assert!(state.should_fire(1, true));
    }
}
