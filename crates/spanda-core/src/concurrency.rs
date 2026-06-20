//! Cooperative channels, spawn handles, and select for concurrent Spanda tasks.

use crate::error::SpandaError;
use crate::runtime::{RuntimeError, RuntimeValue};
use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};
use std::rc::Rc;

pub type ChannelHandle = Rc<RefCell<VecDeque<RuntimeValue>>>;

#[derive(Debug, Clone)]
pub struct SpawnHandle {
    pub func_name: String,
    pub args: Vec<RuntimeValue>,
    pub result: Option<RuntimeValue>,
}

#[derive(Debug, Clone)]
pub struct AgentRoute {
    pub from: String,
    pub to: String,
    pub message_type: String,
}

#[derive(Debug, Clone)]
pub struct ConcurrencyRuntime {
    next_channel_id: u64,
    channels: HashMap<u64, ChannelHandle>,
    channel_type_tags: HashMap<u64, String>,
    next_handle_id: u64,
    handles: HashMap<u64, SpawnHandle>,
    fire_and_forget_queue: Vec<u64>,
    agent_inboxes: HashMap<String, VecDeque<RuntimeValue>>,
    agent_routes: Vec<AgentRoute>,
}

impl Default for ConcurrencyRuntime {
    fn default() -> Self {
        // Return the default value.
        //
        // Parameters:
        // None.
        //
        // Returns:
        // A new instance of this type.
        //
        // Options:
        // None.
        //
        // Example:
        // let value = spanda_core::concurrency::default();

        Self::new()
    }
}

impl ConcurrencyRuntime {
    pub fn new() -> Self {
        // Create a new instance.
        //
        // Parameters:
        // None.
        //
        // Returns:
        // A new instance of this type.
        //
        // Options:
        // None.
        //
        // Example:
        // let value = spanda_core::concurrency::new();

        Self {
            next_channel_id: 1,
            channels: HashMap::new(),
            channel_type_tags: HashMap::new(),
            next_handle_id: 1,
            handles: HashMap::new(),
            fire_and_forget_queue: Vec::new(),
            agent_inboxes: HashMap::new(),
            agent_routes: Vec::new(),
        }
    }

    pub fn register_agent_route(&mut self, from: &str, to: &str, message_type: &str) {
        // Register agent route.
        //
        // Parameters:
        // - `self` — method receiver
        // - `from` — input value
        // - `to` — input value
        // - `message_type` — input value
        //
        // Returns:
        // Nothing.
        //
        // Options:
        // None.
        //
        // Example:
        // let result = instance.register_agent_route(from, to, message_type);

        self.agent_routes.push(AgentRoute {
            from: from.to_string(),
            to: to.to_string(),
            message_type: message_type.to_string(),
        });
    }

    pub fn send_agent(
        &mut self,
        from: &str,
        to: &str,
        value: RuntimeValue,
        line: u32,
    ) -> Result<(), SpandaError> {
        // Send agent.
        //
        // Parameters:
        // - `self` — method receiver
        // - `from` — input value
        // - `to` — input value
        // - `value` — input value
        // - `line` — input value
        //
        // Returns:
        // Success value on completion, or an error.
        //
        // Options:
        // None.
        //
        // Example:
        // let result = instance.send_agent(from, to, value, line);

        let allowed = self
            .agent_routes
            .iter()
            .any(|route| route.from == from && route.to == to);
        if !allowed {
            return Err(RuntimeError::new(
                format!("No agent channel from '{from}' to '{to}'"),
                line,
            )
            .into_spanda());
        }
        if let Some(route) = self
            .agent_routes
            .iter()
            .find(|route| route.from == from && route.to == to)
        {
            if !route.message_type.is_empty() {
                let actual = runtime_type_tag(&value);
                let expected = format!("object:{}", route.message_type);
                if actual != expected && actual != route.message_type {
                    return Err(RuntimeError::new(
                        format!(
                            "Agent message type mismatch: expected {}, got {actual}",
                            route.message_type
                        ),
                        line,
                    )
                    .into_spanda());
                }
            }
        }
        self.agent_inboxes
            .entry(to.to_string())
            .or_default()
            .push_back(value);
        Ok(())
    }

    pub fn try_recv_agent(&mut self, agent: &str, _line: u32) -> Option<RuntimeValue> {
        // Try recv agent.
        //
        // Parameters:
        // - `self` — method receiver
        // - `agent` — input value
        // - `_line` — input value
        //
        // Returns:
        // Some value on success, otherwise none.
        //
        // Options:
        // None.
        //
        // Example:
        // let result = instance.try_recv_agent(agent, _line);

        self.agent_inboxes
            .get_mut(agent)
            .and_then(|inbox| inbox.pop_front())
    }

    pub fn agent_inbox_len(&self, agent: &str) -> usize {
        // Agent inbox len.
        //
        // Parameters:
        // - `self` — method receiver
        // - `agent` — input value
        //
        // Returns:
        // Numeric result.
        //
        // Options:
        // None.
        //
        // Example:
        // let result = instance.agent_inbox_len(agent);

        self.agent_inboxes
            .get(agent)
            .map(|inbox| inbox.len())
            .unwrap_or(0)
    }

    pub fn create_channel(&mut self) -> RuntimeValue {
        // Create channel.
        //
        // Parameters:
        // - `self` — method receiver
        //
        // Returns:
        // RuntimeValue.
        //
        // Options:
        // None.
        //
        // Example:
        // let result = instance.create_channel();

        let id = self.next_channel_id;
        self.next_channel_id += 1;
        let handle = Rc::new(RefCell::new(VecDeque::new()));
        self.channels.insert(id, handle);
        RuntimeValue::Channel { id }
    }

    pub fn send(
        &self,
        channel: &RuntimeValue,
        value: RuntimeValue,
        line: u32,
    ) -> Result<(), SpandaError> {
        // Send.
        //
        // Parameters:
        // - `self` — method receiver
        // - `channel` — input value
        // - `value` — input value
        // - `line` — input value
        //
        // Returns:
        // Success value on completion, or an error.
        //
        // Options:
        // None.
        //
        // Example:
        // let result = instance.send(channel, value, line);

        let RuntimeValue::Channel { id } = channel else {
            return Err(RuntimeError::new("send requires a channel", line).into_spanda());
        };
        let handle = self.channels.get(id).ok_or_else(|| {
            RuntimeError::new(format!("Unknown channel id {id}"), line).into_spanda()
        })?;
        if let Some(expected) = self.channel_type_tags.get(id) {
            let actual = runtime_type_tag(&value);
            if expected != &actual {
                return Err(RuntimeError::new(
                    format!("Channel type mismatch: expected {expected}, got {actual}"),
                    line,
                )
                .into_spanda());
            }
        }
        handle.borrow_mut().push_back(value);
        Ok(())
    }

    pub fn try_recv(
        &self,
        channel: &RuntimeValue,
        line: u32,
    ) -> Result<Option<RuntimeValue>, SpandaError> {
        // Try recv.
        //
        // Parameters:
        // - `self` — method receiver
        // - `channel` — input value
        // - `line` — input value
        //
        // Returns:
        // Success value on completion, or an error.
        //
        // Options:
        // None.
        //
        // Example:
        // let result = instance.try_recv(channel, line);

        let RuntimeValue::Channel { id } = channel else {
            return Err(RuntimeError::new("recv requires a channel", line).into_spanda());
        };
        let handle = self.channels.get(id).ok_or_else(|| {
            RuntimeError::new(format!("Unknown channel id {id}"), line).into_spanda()
        })?;
        Ok(handle.borrow_mut().pop_front())
    }

    pub fn create_task_handle(
        &mut self,
        func_name: String,
        args: Vec<RuntimeValue>,
    ) -> RuntimeValue {
        // Create task handle.
        //
        // Parameters:
        // - `self` — method receiver
        // - `func_name` — input value
        // - `args` — input value
        //
        // Returns:
        // RuntimeValue.
        //
        // Options:
        // None.
        //
        // Example:
        // let result = instance.create_task_handle(func_name, args);

        let id = self.next_handle_id;
        self.next_handle_id += 1;
        self.handles.insert(
            id,
            SpawnHandle {
                func_name,
                args,
                result: None,
            },
        );
        RuntimeValue::TaskHandle { id }
    }

    pub fn queue_fire_and_forget(&mut self, func_name: String, args: Vec<RuntimeValue>) {
        // Queue fire and forget.
        //
        // Parameters:
        // - `self` — method receiver
        // - `func_name` — input value
        // - `args` — input value
        //
        // Returns:
        // Nothing.
        //
        // Options:
        // None.
        //
        // Example:
        // let result = instance.queue_fire_and_forget(func_name, args);

        let handle = self.create_task_handle(func_name, args);
        if let RuntimeValue::TaskHandle { id } = handle {
            self.fire_and_forget_queue.push(id);
        }
    }

    pub fn handle(&self, id: u64) -> Option<&SpawnHandle> {
        // Handle.
        //
        // Parameters:
        // - `self` — method receiver
        // - `id` — input value
        //
        // Returns:
        // Some value on success, otherwise none.
        //
        // Options:
        // None.
        //
        // Example:
        // let result = instance.handle(id);

        self.handles.get(&id)
    }

    pub fn handle_mut(&mut self, id: u64) -> Option<&mut SpawnHandle> {
        // Handle mut.
        //
        // Parameters:
        // - `self` — method receiver
        // - `id` — input value
        //
        // Returns:
        // Some value on success, otherwise none.
        //
        // Options:
        // None.
        //
        // Example:
        // let result = instance.handle_mut(id);

        self.handles.get_mut(&id)
    }

    pub fn set_handle_result(&mut self, id: u64, result: RuntimeValue) {
        // Set handle result.
        //
        // Parameters:
        // - `self` — method receiver
        // - `id` — input value
        // - `result` — input value
        //
        // Returns:
        // Nothing.
        //
        // Options:
        // None.
        //
        // Example:
        // let result = instance.set_handle_result(id, result);

        if let Some(handle) = self.handles.get_mut(&id) {
            handle.result = Some(result);
        }
    }

    pub fn drain_fire_and_forget_queue(&mut self) -> Vec<u64> {
        // Drain fire and forget queue.
        //
        // Parameters:
        // - `self` — method receiver
        //
        // Returns:
        // Vec<u64>.
        //
        // Options:
        // None.
        //
        // Example:
        // let result = instance.drain_fire_and_forget_queue();

        std::mem::take(&mut self.fire_and_forget_queue)
    }

    pub fn bind_channel_type(
        &mut self,
        channel: &RuntimeValue,
        value: &RuntimeValue,
        line: u32,
    ) -> Result<(), SpandaError> {
        // Bind channel type.
        //
        // Parameters:
        // - `self` — method receiver
        // - `channel` — input value
        // - `value` — input value
        // - `line` — input value
        //
        // Returns:
        // Success value on completion, or an error.
        //
        // Options:
        // None.
        //
        // Example:
        // let result = instance.bind_channel_type(channel, value, line);

        let RuntimeValue::Channel { id } = channel else {
            return Err(
                RuntimeError::new("channel type binding requires channel", line).into_spanda(),
            );
        };
        let next = runtime_type_tag(value);
        if let Some(existing) = self.channel_type_tags.get(id) {
            if existing != &next {
                return Err(RuntimeError::new(
                    format!("Channel type mismatch: expected {existing}, got {next}"),
                    line,
                )
                .into_spanda());
            }
            return Ok(());
        }
        self.channel_type_tags.insert(*id, next);
        Ok(())
    }
}

fn runtime_type_tag(value: &RuntimeValue) -> String {
    // Runtime type tag.
    //
    // Parameters:
    // - `value` — input value
    //
    // Returns:
    // Text result.
    //
    // Options:
    // None.
    //
    // Example:
    // let result = spanda_core::concurrency::runtime_type_tag(value);

    match value {
        RuntimeValue::Object { type_name, .. } => format!("object:{type_name}"),
        RuntimeValue::Enum {
            enum_name, variant, ..
        } => format!("enum:{enum_name}::{variant}"),
        RuntimeValue::Number { unit, .. } => format!("number:{}", unit.as_str()),
        RuntimeValue::Result { .. } => "result".into(),
        RuntimeValue::Option { .. } => "option".into(),
        RuntimeValue::TraitObject { trait_name, .. } => format!("trait:{trait_name}"),
        RuntimeValue::Future { .. } => "future".into(),
        RuntimeValue::TaskHandle { .. } => "task_handle".into(),
        RuntimeValue::Channel { .. } => "channel".into(),
        RuntimeValue::String { .. } => "string".into(),
        RuntimeValue::Bool { .. } => "bool".into(),
        RuntimeValue::Void => "void".into(),
        RuntimeValue::Scan { .. } => "scan".into(),
        RuntimeValue::Pose { .. } => "pose".into(),
        RuntimeValue::Velocity { .. } => "velocity".into(),
        RuntimeValue::Trajectory { .. } => "trajectory".into(),
        RuntimeValue::Transform { .. } => "transform".into(),
        RuntimeValue::Sensor { .. } => "sensor".into(),
        RuntimeValue::Actuator { .. } => "actuator".into(),
        RuntimeValue::Topic { .. } => "topic".into(),
        RuntimeValue::Service { .. } => "service".into(),
        RuntimeValue::Action { .. } => "action".into(),
        RuntimeValue::Robot => "robot".into(),
        RuntimeValue::Agent { .. } => "agent".into(),
        RuntimeValue::Twin { .. } => "twin".into(),
        RuntimeValue::SafetyCtx => "safety_ctx".into(),
        RuntimeValue::AiModel { .. } => "ai_model".into(),
        RuntimeValue::ActionProposal { .. } => "action_proposal".into(),
        RuntimeValue::SafeAction { .. } => "safe_action".into(),
        RuntimeValue::Completion { .. } => "completion".into(),
        RuntimeValue::Embedding { .. } => "embedding".into(),
        RuntimeValue::Goal { .. } => "goal".into(),
        RuntimeValue::SensorFusion { .. } => "sensor_fusion".into(),
        RuntimeValue::AuditCtx => "audit_ctx".into(),
        RuntimeValue::LedgerCtx => "ledger_ctx".into(),
        RuntimeValue::Identity { .. } => "identity".into(),
        RuntimeValue::Secret { .. } => "secret".into(),
        RuntimeValue::Bytes { .. } => "bytes".into(),
        RuntimeValue::Null => "null".into(),
    }
}
