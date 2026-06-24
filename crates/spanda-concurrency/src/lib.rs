//! Cooperative channels, spawn handles, and select for concurrent Spanda tasks.

use spanda_error::SpandaError;
use spanda_runtime::error::RuntimeError;
use spanda_runtime::value::RuntimeValue;
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
        // Description:
        //     Provide the default value for this type.
        //
        // Inputs:
        //     None.
        //
        // Outputs:
        //     result: Self
        //         Return value from `default`.
        //
        // Example:
        //     let result = spanda_concurrency::default();

        // Build the result via new.
        Self::new()
    }
}

impl ConcurrencyRuntime {
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
        //     let value = spanda_concurrency::new();

        // Assemble the struct fields and return it.
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
        // Description:
        //     Register agent route.
        //
        // Inputs:
        //     &mut self: input value
        //         Caller-supplied &mut self.
        //     fro: &str
        //         Caller-supplied fro.
        //     o: &str
        //         Caller-supplied o.
        //     essage_type: &str
        //         Caller-supplied essage type.
        //
        // Outputs:
        //     None.
        //
        // Example:
        //     let result = spanda_concurrency::register_agent_route(&mut self, fro, o, essage_type);

        // Append into self.
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
        // Description:
        //     Send agent.
        //
        // Inputs:
        //     &mut self: input value
        //         Caller-supplied &mut self.
        //     fro: &str
        //         Caller-supplied fro.
        //     o: &str
        //         Caller-supplied o.
        //     value: RuntimeValue
        //         Caller-supplied value.
        //     line: u32
        //         Caller-supplied line.
        //
        // Outputs:
        //     result: Result<(), SpandaError>
        //         Return value from `send_agent`.
        //
        // Example:
        //     let result = spanda_concurrency::send_agent(&mut self, fro, o, value, line);

        // Compute allowed for the following logic.
        let allowed = self
            .agent_routes
            .iter()
            .any(|route| route.from == from && route.to == to);

        // Take the branch when allowed is false.
        if !allowed {
            return Err(SpandaError::from(RuntimeError::new(
                format!("No agent channel from '{from}' to '{to}'"),
                line,
            )));
        }

        // Emit output when self provides a route.
        if let Some(route) = self
            .agent_routes
            .iter()
            .find(|route| route.from == from && route.to == to)
        {
            // Skip further work when message type is empty.
            if !route.message_type.is_empty() {
                let actual = runtime_type_tag(&value);
                let expected = format!("object:{}", route.message_type);

                // Take the branch when actual differs from message type.
                if actual != expected && actual != route.message_type {
                    return Err(SpandaError::from(RuntimeError::new(
                        format!(
                            "Agent message type mismatch: expected {}, got {actual}",
                            route.message_type
                        ),
                        line,
                    )));
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
        // Description:
        //     Try recv agent.
        //
        // Inputs:
        //     &mut self: input value
        //         Caller-supplied &mut self.
        //     agen: &str
        //         Caller-supplied agen.
        //     _line: u32
        //         Caller-supplied line.
        //
        // Outputs:
        //     result: Option<RuntimeValue>
        //         Return value from `try_recv_agent`.
        //
        // Example:
        //     let result = spanda_concurrency::try_recv_agent(&mut self, agen, _line);

        // Call agent inboxes on the current instance.
        self.agent_inboxes
            .get_mut(agent)
            .and_then(|inbox| inbox.pop_front())
    }

    pub fn agent_inbox_len(&self, agent: &str) -> usize {
        // Description:
        //     Agent inbox len.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //     agen: &str
        //         Caller-supplied agen.
        //
        // Outputs:
        //     result: usize
        //         Return value from `agent_inbox_len`.
        //
        // Example:
        //     let result = spanda_concurrency::agent_inbox_len(&self, agen);

        // Call agent inboxes on the current instance.
        self.agent_inboxes
            .get(agent)
            .map(|inbox| inbox.len())
            .unwrap_or(0)
    }

    pub fn create_channel(&mut self) -> RuntimeValue {
        // Description:
        //     Create channel.
        //
        // Inputs:
        //     &mut self: input value
        //         Caller-supplied &mut self.
        //
        // Outputs:
        //     result: RuntimeValue
        //         Return value from `create_channel`.
        //
        // Example:
        //     let result = spanda_concurrency::create_channel(&mut self);

        // Compute id for the following logic.
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
        // Description:
        //     Send.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //     channel: &RuntimeValue
        //         Caller-supplied channel.
        //     value: RuntimeValue
        //         Caller-supplied value.
        //     line: u32
        //         Caller-supplied line.
        //
        // Outputs:
        //     result: Result<(), SpandaError>
        //         Return value from `send`.
        //
        // Example:
        //     let result = spanda_concurrency::send(&self, channel, value, line);

        // Compute RuntimeValue for the following logic.
        let RuntimeValue::Channel { id } = channel else {
            return Err(SpandaError::from(RuntimeError::new(
                "send requires a channel",
                line,
            )));
        };
        let handle = self.channels.get(id).ok_or_else(|| {
            SpandaError::from(RuntimeError::new(format!("Unknown channel id {id}"), line))
        })?;

        // Emit output when get provides a expected.
        if let Some(expected) = self.channel_type_tags.get(id) {
            let actual = runtime_type_tag(&value);

            // Take the branch when expected differs from actual.
            if expected != &actual {
                return Err(SpandaError::from(RuntimeError::new(
                    format!("Channel type mismatch: expected {expected}, got {actual}"),
                    line,
                )));
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
        // Description:
        //     Try recv.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //     channel: &RuntimeValue
        //         Caller-supplied channel.
        //     line: u32
        //         Caller-supplied line.
        //
        // Outputs:
        //     result: Result<Option<RuntimeValue>, SpandaError>
        //         Return value from `try_recv`.
        //
        // Example:
        //     let result = spanda_concurrency::try_recv(&self, channel, line);

        // Compute RuntimeValue for the following logic.
        let RuntimeValue::Channel { id } = channel else {
            return Err(SpandaError::from(RuntimeError::new(
                "recv requires a channel",
                line,
            )));
        };
        let handle = self.channels.get(id).ok_or_else(|| {
            SpandaError::from(RuntimeError::new(format!("Unknown channel id {id}"), line))
        })?;
        Ok(handle.borrow_mut().pop_front())
    }

    pub fn create_task_handle(
        &mut self,
        func_name: String,
        args: Vec<RuntimeValue>,
    ) -> RuntimeValue {
        // Description:
        //     Create task handle.
        //
        // Inputs:
        //     &mut self: input value
        //         Caller-supplied &mut self.
        //     func_name: String
        //         Caller-supplied func name.
        //     args: Vec<RuntimeValue>
        //         Caller-supplied args.
        //
        // Outputs:
        //     result: RuntimeValue
        //         Return value from `create_task_handle`.
        //
        // Example:
        //     let result = spanda_concurrency::create_task_handle(&mut self, func_name, args);

        // Compute id for the following logic.
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
        // Description:
        //     Queue fire and forget.
        //
        // Inputs:
        //     &mut self: input value
        //         Caller-supplied &mut self.
        //     func_name: String
        //         Caller-supplied func name.
        //     args: Vec<RuntimeValue>
        //         Caller-supplied args.
        //
        // Outputs:
        //     None.
        //
        // Example:
        //     let result = spanda_concurrency::queue_fire_and_forget(&mut self, func_name, args);

        // Compute handle for the following logic.
        let handle = self.create_task_handle(func_name, args);

        // Take this path when let RuntimeValue::TaskHandle { id } = handle.
        if let RuntimeValue::TaskHandle { id } = handle {
            self.fire_and_forget_queue.push(id);
        }
    }

    pub fn handle(&self, id: u64) -> Option<&SpawnHandle> {
        // Description:
        //     Handle.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //     id: u64
        //         Caller-supplied id.
        //
        // Outputs:
        //     result: Option<&SpawnHandle>
        //         Return value from `handle`.
        //
        // Example:
        //     let result = spanda_concurrency::handle(&self, id);

        // Call get on the current instance.
        self.handles.get(&id)
    }

    pub fn handle_mut(&mut self, id: u64) -> Option<&mut SpawnHandle> {
        // Description:
        //     Handle mut.
        //
        // Inputs:
        //     &mut self: input value
        //         Caller-supplied &mut self.
        //     id: u64
        //         Caller-supplied id.
        //
        // Outputs:
        //     result: Option<&mut SpawnHandle>
        //         Return value from `handle_mut`.
        //
        // Example:
        //     let result = spanda_concurrency::handle_mut(&mut self, id);

        // Call get mut on the current instance.
        self.handles.get_mut(&id)
    }

    pub fn set_handle_result(&mut self, id: u64, result: RuntimeValue) {
        // Description:
        //     Set handle result.
        //
        // Inputs:
        //     &mut self: input value
        //         Caller-supplied &mut self.
        //     id: u64
        //         Caller-supplied id.
        //     resul: RuntimeValue
        //         Caller-supplied resul.
        //
        // Outputs:
        //     None.
        //
        // Example:
        //     let result = spanda_concurrency::set_handle_result(&mut self, id, resul);
        // use handle when get mut is present.

        // Emit output when get mut provides a handle.
        if let Some(handle) = self.handles.get_mut(&id) {
            handle.result = Some(result);
        }
    }

    pub fn drain_fire_and_forget_queue(&mut self) -> Vec<u64> {
        // Description:
        //     Drain fire and forget queue.
        //
        // Inputs:
        //     &mut self: input value
        //         Caller-supplied &mut self.
        //
        // Outputs:
        //     result: Vec<u64>
        //         Return value from `drain_fire_and_forget_queue`.
        //
        // Example:
        //     let result = spanda_concurrency::drain_fire_and_forget_queue(&mut self);

        // Move out the stored value and leave a default behind.
        std::mem::take(&mut self.fire_and_forget_queue)
    }

    pub fn bind_channel_type(
        &mut self,
        channel: &RuntimeValue,
        value: &RuntimeValue,
        line: u32,
    ) -> Result<(), SpandaError> {
        // Description:
        //     Bind channel type.
        //
        // Inputs:
        //     &mut self: input value
        //         Caller-supplied &mut self.
        //     channel: &RuntimeValue
        //         Caller-supplied channel.
        //     value: &RuntimeValue
        //         Caller-supplied value.
        //     line: u32
        //         Caller-supplied line.
        //
        // Outputs:
        //     result: Result<(), SpandaError>
        //         Return value from `bind_channel_type`.
        //
        // Example:
        //     let result = spanda_concurrency::bind_channel_type(&mut self, channel, value, line);

        // Compute RuntimeValue for the following logic.
        let RuntimeValue::Channel { id } = channel else {
            return Err(SpandaError::from(RuntimeError::new(
                "channel type binding requires channel",
                line,
            )));
        };
        let next = runtime_type_tag(value);

        // Emit output when get provides a existing.
        if let Some(existing) = self.channel_type_tags.get(id) {
            // Take the branch when existing differs from next.
            if existing != &next {
                return Err(SpandaError::from(RuntimeError::new(
                    format!("Channel type mismatch: expected {existing}, got {next}"),
                    line,
                )));
            }
            return Ok(());
        }
        self.channel_type_tags.insert(*id, next);
        Ok(())
    }
}

fn runtime_type_tag(value: &RuntimeValue) -> String {
    // Description:
    //     Runtime type tag.
    //
    // Inputs:
    //     value: &RuntimeValue
    //         Caller-supplied value.
    //
    // Outputs:
    //     result: String
    //         Return value from `runtime_type_tag`.
    //
    // Example:
    //     let result = spanda_concurrency::runtime_type_tag(value);

    // Match on value and handle each case.
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
        RuntimeValue::MissionControl { .. } => "mission_control".into(),
        RuntimeValue::NavigationControl { .. } => "navigation_control".into(),
        RuntimeValue::SlamControl => "slam_control".into(),
        RuntimeValue::FleetControl { .. } => "fleet_control".into(),
        RuntimeValue::AuditCtx => "audit_ctx".into(),
        RuntimeValue::LedgerCtx => "ledger_ctx".into(),
        RuntimeValue::WorldModelCtx => "world_model_ctx".into(),
        RuntimeValue::Identity { .. } => "identity".into(),
        RuntimeValue::Secret { .. } => "secret".into(),
        RuntimeValue::Bytes { .. } => "bytes".into(),
        RuntimeValue::Null => "null".into(),
        RuntimeValue::Regex { .. } => "regex".into(),
        RuntimeValue::Capture { .. } => "capture".into(),
    }
}
