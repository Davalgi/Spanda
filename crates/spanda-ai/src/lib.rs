//! ai support for Spanda.
//!
pub mod live;

use spanda_ast::nodes::{AgentDecl, AiModelDecl, ConfigValue, MemoryKind, Stmt, UnitKind};
use spanda_runtime::value::RuntimeValue;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AiRuntimeKind {
    Onnx,
    Tflite,
    Tensorrt,
    OpenVino,
}

impl AiRuntimeKind {
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

        // Dispatch based on the enum variant or current state.
        match self {
            AiRuntimeKind::Onnx => "onnx",
            AiRuntimeKind::Tflite => "tflite",
            AiRuntimeKind::Tensorrt => "tensorrt",
            AiRuntimeKind::OpenVino => "openvino",
        }
    }
}

#[derive(Debug, Clone)]
pub struct AiLibModule {
    pub id: String,
    pub vendor: String,
    pub name: String,
    pub version: String,
    pub description: String,
    pub runtime: AiRuntimeKind,
}

fn build_ai_registry() -> HashMap<String, AiLibModule> {
    // Description:
    //     Build ai registry.
    //
    // Inputs:
    //     None.
    //
    // Outputs:
    //     result: HashMap<String, AiLibModule>
    //         Return value from `build_ai_registry`.
    //
    // Example:
    //     let result = spanda_ai::build_ai_registry();

    // Produce from as the result.
    HashMap::from([
        (
            "onnx.runtime".to_string(),
            AiLibModule {
                id: "onnx.runtime".to_string(),
                vendor: "ONNX".to_string(),
                name: "runtime".to_string(),
                version: "1.0.0".to_string(),
                description: "ONNX Runtime inference backend".to_string(),
                runtime: AiRuntimeKind::Onnx,
            },
        ),
        (
            "tflite.runtime".to_string(),
            AiLibModule {
                id: "tflite.runtime".to_string(),
                vendor: "TensorFlow".to_string(),
                name: "runtime".to_string(),
                version: "1.0.0".to_string(),
                description: "TensorFlow Lite inference backend".to_string(),
                runtime: AiRuntimeKind::Tflite,
            },
        ),
        (
            "tensorrt.runtime".to_string(),
            AiLibModule {
                id: "tensorrt.runtime".to_string(),
                vendor: "NVIDIA".to_string(),
                name: "runtime".to_string(),
                version: "1.0.0".to_string(),
                description: "TensorRT inference backend for Jetson".to_string(),
                runtime: AiRuntimeKind::Tensorrt,
            },
        ),
        (
            "openvino.runtime".to_string(),
            AiLibModule {
                id: "openvino.runtime".to_string(),
                vendor: "Intel".to_string(),
                name: "runtime".to_string(),
                version: "1.0.0".to_string(),
                description: "OpenVINO inference backend for Intel CPUs and VPUs".to_string(),
                runtime: AiRuntimeKind::OpenVino,
            },
        ),
    ])
}

static AI_REGISTRY: std::sync::OnceLock<HashMap<String, AiLibModule>> = std::sync::OnceLock::new();

pub fn ai_lib_registry() -> &'static HashMap<String, AiLibModule> {
    // Description:
    //     Ai lib registry.
    //
    // Inputs:
    //     None.
    //
    // Outputs:
    //     result: &'static HashMap<String, AiLibModule>
    //         Return value from `ai_lib_registry`.
    //
    // Example:
    //     let result = spanda_ai::ai_lib_registry();

    // Produce get or init as the result.
    AI_REGISTRY.get_or_init(build_ai_registry)
}

pub fn resolve_ai_import(path: &str) -> Option<&'static AiLibModule> {
    // Description:
    //     Resolve ai import.
    //
    // Inputs:
    //     path: &str
    //         Caller-supplied path.
    //
    // Outputs:
    //     result: Option<&'static AiLibModule>
    //         Return value from `resolve_ai_import`.
    //
    // Example:
    //     let result = spanda_ai::resolve_ai_import(path);

    // Produce get as the result.
    ai_lib_registry().get(path)
}

pub fn ai_lib_registry_export() -> &'static HashMap<String, AiLibModule> {
    // Description:
    //     Ai lib registry export.
    //
    // Inputs:
    //     None.
    //
    // Outputs:
    //     result: &'static HashMap<String, AiLibModule>
    //         Return value from `ai_lib_registry_export`.
    //
    // Example:
    //     let result = spanda_ai::ai_lib_registry_export();

    // Produce ai lib registry as the result.
    ai_lib_registry()
}

pub fn list_ai_libraries() -> Vec<&'static AiLibModule> {
    // Description:
    //     List ai libraries.
    //
    // Inputs:
    //     None.
    //
    // Outputs:
    //     result: Vec<&'static AiLibModule>
    //         Return value from `list_ai_libraries`.
    //
    // Example:
    //     let result = spanda_ai::list_ai_libraries();

    // Collect filtered entries into a new list.
    ai_lib_registry().values().collect()
}

#[derive(Debug, Clone)]
pub struct CompletionRequest {
    pub prompt: String,
    pub input: Option<RuntimeValue>,
    pub model: String,
    pub provider: String,
    pub temperature: f64,
    pub max_tokens: usize,
}

#[derive(Debug, Clone)]
pub struct DetectionRequest {
    pub model: String,
    pub provider: String,
    pub frame: RuntimeValue,
}

#[derive(Debug, Clone)]
pub struct EmbedRequest {
    pub model: String,
    pub provider: String,
    pub text: String,
}

pub trait AiProvider: Send + Sync {
    fn complete(&self, request: &CompletionRequest) -> RuntimeValue;
    fn detect(&self, request: &DetectionRequest) -> RuntimeValue;
    fn embed(&self, request: &EmbedRequest) -> RuntimeValue;
}

pub(crate) fn scan_distance(input: Option<&RuntimeValue>) -> f64 {
    // Description:
    //     Scan distance.
    //
    // Inputs:
    //     inp: Option<&RuntimeValue>
    //         Caller-supplied inp.
    //
    // Outputs:
    //     result: f64
    //         Return value from `scan_distance`.
    //
    // Example:
    //     let result = spanda_ai::scan_distance(inp);

    // Match on input and handle each case.
    match input {
        Some(RuntimeValue::Scan { nearest_distance }) => *nearest_distance,
        Some(RuntimeValue::Object { type_name, fields }) if type_name == "Detection" => {
            // Take this path when let Some(RuntimeValue::Number { value, .. }) = fields.get("nearest dis.
            if let Some(RuntimeValue::Number { value, .. }) = fields.get("nearest_distance") {
                *value
            } else {
                5.0
            }
        }
        _ => 5.0,
    }
}

pub(crate) fn action_proposal(
    linear: f64,
    angular: f64,
    source: impl Into<String>,
    trace: Vec<String>,
) -> RuntimeValue {
    // Description:
    //     Action proposal.
    //
    // Inputs:
    //     linear: f64
    //         Caller-supplied linear.
    //     angular: f64
    //         Caller-supplied angular.
    //     source: impl Into<String>
    //         Caller-supplied source.
    //     race: Vec<String>
    //         Caller-supplied race.
    //
    // Outputs:
    //     result: RuntimeValue
    //         Return value from `action_proposal`.
    //
    // Example:
    //     let result = spanda_ai::action_proposal(linear, angular, source, race);

    // Build a ActionProposal runtime value.
    RuntimeValue::ActionProposal {
        linear,
        angular,
        source: source.into(),
        trace,
    }
}

pub struct MockAiProvider;

impl AiProvider for MockAiProvider {
    fn complete(&self, request: &CompletionRequest) -> RuntimeValue {
        // Description:
        //     Complete.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //     request: &CompletionRequest
        //         Caller-supplied request.
        //
        // Outputs:
        //     result: RuntimeValue
        //         Return value from `complete`.
        //
        // Example:
        //     let result = spanda_ai::complete(&self, reques);

        // Compute prompt for the following logic.
        let prompt = request.prompt.clone();
        let dist = scan_distance(request.input.as_ref());

        // Take this path when regex stop halt wait(&request.prompt).
        if regex_stop_halt_wait(&request.prompt) {
            return action_proposal(
                0.0,
                0.0,
                &request.model,
                vec![
                    format!("model={}", request.model),
                    format!("prompt={prompt}"),
                    "decision=stop".into(),
                ],
            );
        }

        // Take this path when regex turn avoid obstacle(&request.prompt) || dist < 0.8.
        if regex_turn_avoid_obstacle(&request.prompt) || dist < 0.8 {
            let angular = if dist < 0.4 { 0.6 } else { 0.25 };
            let linear = if dist < 0.4 {
                0.0
            } else {
                (0.4_f64).min(dist * 0.3)
            };
            return action_proposal(
                linear,
                angular,
                &request.model,
                vec![
                    format!("model={}", request.model),
                    format!("prompt={prompt}"),
                    format!("nearest_distance={dist:.2}"),
                    "decision=avoid_obstacle".into(),
                ],
            );
        }
        let linear = (0.8_f64).min(dist * 0.45);
        action_proposal(
            linear,
            0.0,
            &request.model,
            vec![
                format!("model={}", request.model),
                format!("prompt={prompt}"),
                format!("nearest_distance={dist:.2}"),
                "decision=forward".into(),
            ],
        )
    }

    fn detect(&self, request: &DetectionRequest) -> RuntimeValue {
        // Description:
        //     Detect.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //     request: &DetectionRequest
        //         Caller-supplied request.
        //
        // Outputs:
        //     result: RuntimeValue
        //         Return value from `detect`.
        //
        // Example:
        //     let result = spanda_ai::detect(&self, reques);

        // Compute dist for the following logic.
        let dist = scan_distance(Some(&request.frame));
        let (label, confidence) = if dist < 0.6 {
            ("obstacle", 0.94)
        } else if dist < 1.2 {
            ("object", 0.82)
        } else {
            ("clear", 0.71)
        };
        let mut fields = HashMap::new();
        fields.insert("label".to_string(), RuntimeValue::string(label));
        fields.insert(
            "confidence".to_string(),
            RuntimeValue::number(confidence, UnitKind::None),
        );
        fields.insert(
            "nearest_distance".to_string(),
            RuntimeValue::number(dist, UnitKind::M),
        );
        RuntimeValue::object("Detection", fields)
    }

    fn embed(&self, request: &EmbedRequest) -> RuntimeValue {
        // Description:
        //     Embed.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //     request: &EmbedRequest
        //         Caller-supplied request.
        //
        // Outputs:
        //     result: RuntimeValue
        //         Return value from `embed`.
        //
        // Example:
        //     let result = spanda_ai::embed(&self, reques);

        // Compute vector for the following logic.
        let vector: Vec<f64> = (0..8)
            .map(|i| (request.text.len() as f64 * 0.13 + i as f64).sin() * 0.5 + 0.5)
            .collect();
        RuntimeValue::Embedding {
            dimensions: vector.len(),
            vector,
        }
    }
}

fn regex_stop_halt_wait(prompt: &str) -> bool {
    // Description:
    //     Regex stop halt wait.
    //
    // Inputs:
    //     promp: &str
    //         Caller-supplied promp.
    //
    // Outputs:
    //     result: bool
    //         Return value from `regex_stop_halt_wait`.
    //
    // Example:
    //     let result = spanda_ai::regex_stop_halt_wait(promp);

    // Compute lower for the following logic.
    let lower = prompt.to_lowercase();
    lower.contains("stop") || lower.contains("halt") || lower.contains("wait")
}

fn regex_turn_avoid_obstacle(prompt: &str) -> bool {
    // Description:
    //     Regex turn avoid obstacle.
    //
    // Inputs:
    //     promp: &str
    //         Caller-supplied promp.
    //
    // Outputs:
    //     result: bool
    //         Return value from `regex_turn_avoid_obstacle`.
    //
    // Example:
    //     let result = spanda_ai::regex_turn_avoid_obstacle(promp);

    // Compute lower for the following logic.
    let lower = prompt.to_lowercase();
    lower.contains("turn") || lower.contains("avoid") || lower.contains("obstacle")
}

pub fn mock_summarize(input: Option<&RuntimeValue>, model: &str) -> RuntimeValue {
    // Description:
    //     Mock summarize.
    //
    // Inputs:
    //     inp: Option<&RuntimeValue>
    //         Caller-supplied inp.
    //     odel: &str
    //         Caller-supplied odel.
    //
    // Outputs:
    //     result: RuntimeValue
    //         Return value from `mock_summarize`.
    //
    // Example:
    //     let result = spanda_ai::mock_summarize(inp, odel);

    // Compute summary for the following logic.
    let summary = match input {
        Some(RuntimeValue::Scan { nearest_distance }) => {
            format!("Nearest obstacle at {nearest_distance:.2} m")
        }
        Some(RuntimeValue::Object { type_name, fields }) if type_name == "Detection" => {
            let label = match fields.get("label") {
                Some(RuntimeValue::String { value }) => value.as_str(),
                _ => "object",
            };
            format!("Detected {label}")
        }
        _ => "Environment stable".to_string(),
    };
    RuntimeValue::Completion {
        text: format!("[{model}] {summary}"),
        model: Some(model.to_string()),
    }
}

pub fn mock_analyze_frame(frame: Option<&RuntimeValue>, _model: &str) -> RuntimeValue {
    // Description:
    //     Mock analyze frame.
    //
    // Inputs:
    //     frame: Option<&RuntimeValue>
    //         Caller-supplied frame.
    //     _model: &str
    //         Caller-supplied model.
    //
    // Outputs:
    //     result: RuntimeValue
    //         Return value from `mock_analyze_frame`.
    //
    // Example:
    //     let result = spanda_ai::mock_analyze_frame(frame, _model);

    // Compute dist for the following logic.
    let dist = scan_distance(frame);
    let mut fields = HashMap::new();
    fields.insert(
        "label".to_string(),
        RuntimeValue::string(if dist < 0.7 {
            "cluttered_scene"
        } else {
            "open_scene"
        }),
    );
    fields.insert(
        "confidence".to_string(),
        RuntimeValue::number(0.86, UnitKind::None),
    );
    fields.insert(
        "nearest_distance".to_string(),
        RuntimeValue::number(dist, UnitKind::M),
    );
    RuntimeValue::object("Detection", fields)
}

pub fn mock_camera_frame() -> RuntimeValue {
    // Description:
    //     Mock camera frame.
    //
    // Inputs:
    //     None.
    //
    // Outputs:
    //     result: RuntimeValue
    //         Return value from `mock_camera_frame`.
    //
    // Example:
    //     let result = spanda_ai::mock_camera_frame();

    // Create mutable fields for accumulating results.
    let mut fields = HashMap::new();
    fields.insert(
        "width".to_string(),
        RuntimeValue::number(640.0, UnitKind::None),
    );
    fields.insert(
        "height".to_string(),
        RuntimeValue::number(480.0, UnitKind::None),
    );
    fields.insert(
        "nearest_distance".to_string(),
        RuntimeValue::number(1.5, UnitKind::M),
    );
    RuntimeValue::object("CameraFrame", fields)
}

pub fn build_prompt(base: &str, input: Option<&RuntimeValue>, goal: Option<&str>) -> String {
    // Description:
    //     Build prompt.
    //
    // Inputs:
    //     base: &str
    //         Caller-supplied base.
    //     inp: Option<&RuntimeValue>
    //         Caller-supplied inp.
    //     goal: Option<&str>
    //         Caller-supplied goal.
    //
    // Outputs:
    //     result: String
    //         Return value from `build_prompt`.
    //
    // Example:
    //     let result = spanda_ai::build_prompt(base, inp, goal);

    // Create mutable header for accumulating results.
    let mut header = String::new();

    // Emit output when is empty provides a g.
    if let Some(g) = goal.filter(|s| !s.is_empty()) {
        header.push_str(&format!("Goal: {g}"));
    }
    let base = base.trim();

    // Skip further work when !base is empty.
    if !base.is_empty() {
        // Skip further work when !header is empty.
        if !header.is_empty() {
            header.push_str("\n\n");
        }
        header.push_str(base);
    }
    let input_summary = summarize_input(input);

    // Skip further work when header is empty.
    if header.is_empty() {
        format!("Context:\n{input_summary}")
    } else {
        format!("{header}\n\nContext:\n{input_summary}")
    }
}

fn summarize_input(input: Option<&RuntimeValue>) -> String {
    // Description:
    //     Summarize input.
    //
    // Inputs:
    //     inp: Option<&RuntimeValue>
    //         Caller-supplied inp.
    //
    // Outputs:
    //     result: String
    //         Return value from `summarize_input`.
    //
    // Example:
    //     let result = spanda_ai::summarize_input(inp);

    // Match on input and handle each case.
    match input {
        None | Some(RuntimeValue::Void) => "(no input)".to_string(),
        Some(RuntimeValue::Scan { nearest_distance }) => {
            format!("LiDAR scan — nearest obstacle {nearest_distance:.2} m")
        }
        Some(RuntimeValue::String { value }) => value.clone(),
        Some(RuntimeValue::Object { type_name, fields }) if type_name == "Detection" => {
            let label = match fields.get("label") {
                Some(RuntimeValue::String { value }) => value.as_str(),
                _ => "object",
            };
            let conf = match fields.get("confidence") {
                Some(RuntimeValue::Number { value, .. }) => *value,
                _ => 0.0,
            };
            format!("Vision scene — {label} ({conf:.2} confidence)")
        }
        Some(RuntimeValue::Object { type_name, fields }) if type_name == "Detections" => {
            let count = match fields.get("count") {
                Some(RuntimeValue::Number { value, .. }) => *value,
                _ => 0.0,
            };
            format!("Detections — {count} object(s) in view")
        }
        Some(RuntimeValue::Object { type_name, .. }) => format!("{type_name} object"),
        Some(RuntimeValue::Completion { text, .. }) => text.clone(),
        Some(RuntimeValue::Goal { text }) => format!("Goal — {text}"),
        Some(other) => format!("({} value)", runtime_value_kind(other)),
    }
}

fn runtime_value_kind(value: &RuntimeValue) -> &'static str {
    // Description:
    //     Runtime value kind.
    //
    // Inputs:
    //     value: &RuntimeValue
    //         Caller-supplied value.
    //
    // Outputs:
    //     result: &'static str
    //         Return value from `runtime_value_kind`.
    //
    // Example:
    //     let result = spanda_ai::runtime_value_kind(value);

    // Match on value and handle each case.
    match value {
        RuntimeValue::Number { .. } => "number",
        RuntimeValue::Bool { .. } => "bool",
        RuntimeValue::String { .. } => "string",
        RuntimeValue::Void => "void",
        RuntimeValue::Scan { .. } => "scan",
        RuntimeValue::Pose { .. } => "pose",
        RuntimeValue::Velocity { .. } => "velocity",
        RuntimeValue::Trajectory { .. } => "trajectory",
        RuntimeValue::Transform { .. } => "transform",
        RuntimeValue::Object { .. } => "object",
        RuntimeValue::Enum { .. } => "enum",
        RuntimeValue::Sensor { .. } => "sensor",
        RuntimeValue::Actuator { .. } => "actuator",
        RuntimeValue::Topic { .. } => "topic",
        RuntimeValue::Service { .. } => "service",
        RuntimeValue::Action { .. } => "action",
        RuntimeValue::Robot => "robot",
        RuntimeValue::Agent { .. } => "agent",
        RuntimeValue::Twin { .. } => "twin",
        RuntimeValue::SafetyCtx => "safety_ctx",
        RuntimeValue::AiModel { .. } => "ai_model",
        RuntimeValue::ActionProposal { .. } => "action_proposal",
        RuntimeValue::SafeAction { .. } => "safe_action",
        RuntimeValue::Completion { .. } => "completion",
        RuntimeValue::Embedding { .. } => "embedding",
        RuntimeValue::Goal { .. } => "goal",
        RuntimeValue::SensorFusion { .. } => "sensor_fusion",
        RuntimeValue::MissionControl { .. } => "mission_control",
        RuntimeValue::NavigationControl { .. } => "navigation_control",
        RuntimeValue::SlamControl => "slam_control",
        RuntimeValue::FleetControl { .. } => "fleet_control",
        RuntimeValue::AuditCtx => "audit_ctx",
        RuntimeValue::LedgerCtx => "ledger_ctx",
        RuntimeValue::WorldModelCtx => "world_model_ctx",
        RuntimeValue::Identity { .. } => "identity",
        RuntimeValue::Secret { .. } => "secret",
        RuntimeValue::Result { .. } => "result",
        RuntimeValue::Option { .. } => "option",
        RuntimeValue::Bytes { .. } => "bytes",
        RuntimeValue::Null => "null",
        RuntimeValue::Future { .. } => "future",
        RuntimeValue::TaskHandle { .. } => "task_handle",
        RuntimeValue::Channel { .. } => "channel",
        RuntimeValue::TraitObject { .. } => "trait_object",
        RuntimeValue::Regex { .. } => "regex",
        RuntimeValue::Capture { .. } => "capture",
    }
}

#[derive(Debug, Clone)]
pub struct AiModelConfig {
    pub provider: String,
    pub model: String,
    pub temperature: f64,
    pub max_tokens: usize,
}

pub struct AiModel {
    pub name: String,
    pub model_type: String,
    pub config: AiModelConfig,
    provider: Box<dyn AiProvider>,
}

impl AiModel {
    pub fn new(decl: &AiModelDecl, provider: Option<Box<dyn AiProvider>>) -> Self {
        // Description:
        //     Construct a new instance.
        //
        // Inputs:
        //     decl: &AiModelDecl
        //         Caller-supplied decl.
        //     provider: Option<Box<dyn AiProvider>>
        //         Caller-supplied provider.
        //
        // Outputs:
        //     result: Self
        //         Return value from `new`.
        //
        // Example:
        //     let value = spanda_ai::new(decl, provider);

        // Assemble the struct fields and return it.
        let config = parse_config(decl);
        Self {
            name: match decl {
                AiModelDecl::AiModelDecl { name, .. } => name.clone(),
            },
            model_type: match decl {
                AiModelDecl::AiModelDecl { model_type, .. } => model_type.clone(),
            },
            config: config.clone(),
            provider: provider.unwrap_or_else(|| live::resolve_ai_provider(&config.provider)),
        }
    }

    pub fn reason(
        &self,
        prompt: &str,
        input: Option<RuntimeValue>,
        goal: Option<&str>,
    ) -> Result<RuntimeValue, String> {
        // Description:
        //     Reason.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //     promp: &str
        //         Caller-supplied promp.
        //     inp: Option<RuntimeValue>
        //         Caller-supplied inp.
        //     goal: Option<&str>
        //         Caller-supplied goal.
        //
        // Outputs:
        //     result: Result<RuntimeValue, String>
        //         Return value from `reason`.
        //
        // Example:
        //     let result = spanda_ai::reason(&self, promp, inp, goal);

        // take the branch when model type differs from "LLM".
        if self.model_type != "LLM" {
            return Err(format!(
                "Model '{}' is {}, not LLM",
                self.name, self.model_type
            ));
        }
        Ok(self.provider.complete(&CompletionRequest {
            prompt: build_prompt(prompt, input.as_ref(), goal),
            input,
            model: self.config.model.clone(),
            provider: self.config.provider.clone(),
            temperature: self.config.temperature,
            max_tokens: self.config.max_tokens,
        }))
    }

    pub fn summarize(&self, input: Option<RuntimeValue>) -> Result<RuntimeValue, String> {
        // Description:
        //     Summarize.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //     inp: Option<RuntimeValue>
        //         Caller-supplied inp.
        //
        // Outputs:
        //     result: Result<RuntimeValue, String>
        //         Return value from `summarize`.
        //
        // Example:
        //     let result = spanda_ai::summarize(&self, inp);

        // take the branch when model type differs from "LLM".
        if self.model_type != "LLM" {
            return Err(format!(
                "Model '{}' is {}, not LLM",
                self.name, self.model_type
            ));
        }
        Ok(mock_summarize(input.as_ref(), &self.config.model))
    }

    pub fn detect(&self, frame: RuntimeValue) -> Result<RuntimeValue, String> {
        // Description:
        //     Detect.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //     frame: RuntimeValue
        //         Caller-supplied frame.
        //
        // Outputs:
        //     result: Result<RuntimeValue, String>
        //         Return value from `detect`.
        //
        // Example:
        //     let result = spanda_ai::detect(&self, frame);

        // take the branch when model type differs from "VisionModel".
        if self.model_type != "VisionModel" {
            return Err(format!(
                "Model '{}' is {}, not VisionModel",
                self.name, self.model_type
            ));
        }
        Ok(self.provider.detect(&DetectionRequest {
            model: self.config.model.clone(),
            provider: self.config.provider.clone(),
            frame,
        }))
    }

    pub fn to_runtime_value(&self) -> RuntimeValue {
        // Description:
        //     To runtime value.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //
        // Outputs:
        //     result: RuntimeValue
        //         Return value from `to_runtime_value`.
        //
        // Example:
        //     let result = spanda_ai::to_runtime_value(&self);

        // Build a AiModel runtime value.
        RuntimeValue::AiModel {
            name: self.name.clone(),
            model_type: self.model_type.clone(),
            provider: self.config.provider.clone(),
        }
    }
}

fn parse_config(decl: &AiModelDecl) -> AiModelConfig {
    // Description:
    //     Parse config.
    //
    // Inputs:
    //     decl: &AiModelDecl
    //         Caller-supplied decl.
    //
    // Outputs:
    //     result: AiModelConfig
    //         Return value from `parse_config`.
    //
    // Example:
    //     let result = spanda_ai::parse_config(decl);

    // Compute AiModelDecl for the following logic.
    let AiModelDecl::AiModelDecl { config, name, .. } = decl;
    let map: HashMap<String, ConfigValue> = config
        .iter()
        .map(|e| (e.key.clone(), e.value.clone()))
        .collect();
    AiModelConfig {
        provider: config_string(&map, "provider", "mock"),
        model: config_string(&map, "model", name),
        temperature: config_number(&map, "temperature", 0.2),
        max_tokens: config_number(&map, "max_tokens", 512.0) as usize,
    }
}

fn config_string(map: &HashMap<String, ConfigValue>, key: &str, default: &str) -> String {
    // Description:
    //     Config string.
    //
    // Inputs:
    //     ap: &HashMap<String, ConfigValue>
    //         Caller-supplied ap.
    //     key: &str
    //         Caller-supplied key.
    //     defaul: &str
    //         Caller-supplied defaul.
    //
    // Outputs:
    //     result: String
    //         Return value from `config_string`.
    //
    // Example:
    //     let result = spanda_ai::config_string(ap, key, defaul);

    // Match on get and handle each case.
    match map.get(key) {
        Some(ConfigValue::String(s)) => s.clone(),
        Some(ConfigValue::EnumVariant { variant, .. }) => variant.clone(),
        Some(ConfigValue::Number(n)) => n.to_string(),
        Some(ConfigValue::Bool(b)) => b.to_string(),
        None => default.to_string(),
    }
}

fn config_number(map: &HashMap<String, ConfigValue>, key: &str, default: f64) -> f64 {
    // Description:
    //     Config number.
    //
    // Inputs:
    //     ap: &HashMap<String, ConfigValue>
    //         Caller-supplied ap.
    //     key: &str
    //         Caller-supplied key.
    //     defaul: f64
    //         Caller-supplied defaul.
    //
    // Outputs:
    //     result: f64
    //         Return value from `config_number`.
    //
    // Example:
    //     let result = spanda_ai::config_number(ap, key, defaul);

    // Match on get and handle each case.
    match map.get(key) {
        Some(ConfigValue::Number(n)) => *n,
        Some(ConfigValue::String(s)) => s.parse().unwrap_or(default),
        _ => default,
    }
}

pub fn create_ai_model(decl: &AiModelDecl) -> AiModel {
    // Description:
    //     Create ai model.
    //
    // Inputs:
    //     decl: &AiModelDecl
    //         Caller-supplied decl.
    //
    // Outputs:
    //     result: AiModel
    //         Return value from `create_ai_model`.
    //
    // Example:
    //     let result = spanda_ai::create_ai_model(decl);

    // Produce new as the result.
    AiModel::new(decl, None)
}

#[derive(Clone)]
pub struct AgentRuntime {
    pub decl: AgentDecl,
    pub memory: Option<MemoryStore>,
}

pub fn create_agent_runtime(decl: AgentDecl, memory: Option<MemoryStore>) -> AgentRuntime {
    // Description:
    //     Create agent runtime.
    //
    // Inputs:
    //     decl: AgentDecl
    //         Caller-supplied decl.
    //     emory: Option<MemoryStore>
    //         Caller-supplied emory.
    //
    // Outputs:
    //     result: AgentRuntime
    //         Return value from `create_agent_runtime`.
    //
    // Example:
    //     let result = spanda_ai::create_agent_runtime(decl, emory);

    // Produce AgentRuntime { decl, memory } as the result.
    AgentRuntime { decl, memory }
}

pub fn agent_tool_names(decl: &AgentDecl) -> Vec<String> {
    // Description:
    //     Agent tool names.
    //
    // Inputs:
    //     decl: &AgentDecl
    //         Caller-supplied decl.
    //
    // Outputs:
    //     result: Vec<String>
    //         Return value from `agent_tool_names`.
    //
    // Example:
    //     let result = spanda_ai::agent_tool_names(decl);

    // Match on decl and handle each case.
    match decl {
        AgentDecl::AgentDecl { tools, .. } => tools.clone(),
    }
}

pub fn agent_uses_models(decl: &AgentDecl) -> Vec<String> {
    // Description:
    //     Agent uses models.
    //
    // Inputs:
    //     decl: &AgentDecl
    //         Caller-supplied decl.
    //
    // Outputs:
    //     result: Vec<String>
    //         Return value from `agent_uses_models`.
    //
    // Example:
    //     let result = spanda_ai::agent_uses_models(decl);

    // Match on decl and handle each case.
    match decl {
        AgentDecl::AgentDecl { uses_ai, .. } => uses_ai.clone(),
    }
}

pub trait PlanExecutor {
    fn execute_block(&mut self, stmts: &[Stmt]) -> Result<(), spanda_error::SpandaError>;
}

pub fn execute_agent_plan(
    agent: &AgentRuntime,
    executor: &mut dyn PlanExecutor,
) -> Result<(), spanda_error::SpandaError> {
    // Description:
    //     Execute agent plan.
    //
    // Inputs:
    //     agen: &AgentRuntime
    //         Caller-supplied agen.
    //     executor: &mut dyn PlanExecutor
    //         Caller-supplied executor.
    //
    // Outputs:
    //     result: Result<(), spanda_error::SpandaError>
    //         Return value from `execute_agent_plan`.
    //
    // Example:
    //     let result = spanda_ai::execute_agent_plan(agen, executor);

    // Compute plan body for the following logic.
    let plan_body = match &agent.decl {
        AgentDecl::AgentDecl { plan_body, .. } => plan_body,
    };
    executor.execute_block(plan_body)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AiMemoryKind {
    ShortTerm,
    LongTerm,
}

impl From<MemoryKind> for AiMemoryKind {
    fn from(kind: MemoryKind) -> Self {
        // Description:
        //     From.
        //
        // Inputs:
        //     kind: MemoryKind
        //         Caller-supplied kind.
        //
        // Outputs:
        //     result: Self
        //         Return value from `from`.
        //
        // Example:
        //     let result = spanda_ai::from(kind);

        // Match on kind and handle each case.
        match kind {
            MemoryKind::ShortTerm => AiMemoryKind::ShortTerm,
            MemoryKind::LongTerm => AiMemoryKind::LongTerm,
        }
    }
}

#[derive(Clone)]
pub struct MemoryStore {
    pub kind: AiMemoryKind,
    entries: Vec<MemoryEntry>,
    limit: usize,
}

#[derive(Clone)]
struct MemoryEntry {
    key: String,
    value: RuntimeValue,
    #[allow(dead_code)]
    at: u128,
}

impl MemoryStore {
    pub fn new(kind: AiMemoryKind, limit: Option<usize>) -> Self {
        // Description:
        //     Construct a new instance.
        //
        // Inputs:
        //     kind: AiMemoryKind
        //         Caller-supplied kind.
        //     limi: Option<usize>
        //         Caller-supplied limi.
        //
        // Outputs:
        //     result: Self
        //         Return value from `new`.
        //
        // Example:
        //     let value = spanda_ai::new(kind, limi);

        // Compute default limit for the following logic.
        let default_limit = match kind {
            AiMemoryKind::ShortTerm => 32,
            AiMemoryKind::LongTerm => 256,
        };
        Self {
            kind,
            entries: Vec::new(),
            limit: limit.unwrap_or(default_limit),
        }
    }

    pub fn remember(&mut self, key: impl Into<String>, value: RuntimeValue) {
        // Description:
        //     Remember.
        //
        // Inputs:
        //     &mut self: input value
        //         Caller-supplied &mut self.
        //     key: impl Into<String>
        //         Caller-supplied key.
        //     value: RuntimeValue
        //         Caller-supplied value.
        //
        // Outputs:
        //     None.
        //
        // Example:
        //     let result = spanda_ai::remember(&mut self, key, value);

        // Append into self.
        self.entries.push(MemoryEntry {
            key: key.into(),
            value,
            at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_millis())
                .unwrap_or(0),
        });

        // Take this path when self.entries.len() > self.limit.
        if self.entries.len() > self.limit {
            self.entries.remove(0);
        }
    }

    pub fn recall(&self, key: &str) -> Option<&RuntimeValue> {
        // Description:
        //     Recall.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //     key: &str
        //         Caller-supplied key.
        //
        // Outputs:
        //     result: Option<&RuntimeValue>
        //         Return value from `recall`.
        //
        // Example:
        //     let result = spanda_ai::recall(&self, key);

        // Call entries on the current instance.
        self.entries
            .iter()
            .rev()
            .find(|e| e.key == key)
            .map(|e| &e.value)
    }

    pub fn recent(&self, count: usize) -> Vec<&RuntimeValue> {
        // Description:
        //     Recent.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //     coun: usize
        //         Caller-supplied coun.
        //
        // Outputs:
        //     result: Vec<&RuntimeValue>
        //         Return value from `recent`.
        //
        // Example:
        //     let result = spanda_ai::recent(&self, coun);

        // Call entries on the current instance.
        self.entries
            .iter()
            .rev()
            .take(count)
            .map(|e| &e.value)
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .collect()
    }

    pub fn clear(&mut self) {
        // Description:
        //     Clear.
        //
        // Inputs:
        //     &mut self: input value
        //         Caller-supplied &mut self.
        //
        // Outputs:
        //     None.
        //
        // Example:
        //     let result = spanda_ai::clear(&mut self);

        // Call clear on the current instance.
        self.entries.clear();
    }

    pub fn summary_for_prompt(&self) -> Option<String> {
        // Description:
        //     Summary for prompt.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //
        // Outputs:
        //     result: Option<String>
        //         Return value from `summary_for_prompt`.
        //
        // Example:
        //     let result = spanda_ai::summary_for_prompt(&self);

        // skip further work when entries is empty.
        if self.entries.is_empty() {
            return None;
        }
        let kind = match self.kind {
            AiMemoryKind::ShortTerm => "short_term",
            AiMemoryKind::LongTerm => "long_term",
        };
        let keys: Vec<&str> = self
            .entries
            .iter()
            .rev()
            .take(5)
            .map(|e| e.key.as_str())
            .collect();
        Some(format!("Agent memory ({kind}): {}", keys.join(", ")))
    }
}

pub fn runtime_safe_action(linear: f64, angular: f64) -> RuntimeValue {
    // Description:
    //     Runtime safe action.
    //
    // Inputs:
    //     linear: f64
    //         Caller-supplied linear.
    //     angular: f64
    //         Caller-supplied angular.
    //
    // Outputs:
    //     result: RuntimeValue
    //         Return value from `runtime_safe_action`.
    //
    // Example:
    //     let result = spanda_ai::runtime_safe_action(linear, angular);

    // Build a SafeAction runtime value.
    RuntimeValue::SafeAction { linear, angular }
}

pub fn runtime_action_proposal(
    linear: f64,
    angular: f64,
    source: impl Into<String>,
) -> RuntimeValue {
    // Description:
    //     Runtime action proposal.
    //
    // Inputs:
    //     linear: f64
    //         Caller-supplied linear.
    //     angular: f64
    //         Caller-supplied angular.
    //     source: impl Into<String>
    //         Caller-supplied source.
    //
    // Outputs:
    //     result: RuntimeValue
    //         Return value from `runtime_action_proposal`.
    //
    // Example:
    //     let result = spanda_ai::runtime_action_proposal(linear, angular, source);

    // Build a ActionProposal runtime value.
    RuntimeValue::ActionProposal {
        linear,
        angular,
        source: source.into(),
        trace: Vec::new(),
    }
}

pub fn is_action_proposal(value: &RuntimeValue) -> bool {
    // Description:
    //     Is action proposal.
    //
    // Inputs:
    //     value: &RuntimeValue
    //         Caller-supplied value.
    //
    // Outputs:
    //     result: bool
    //         Return value from `is_action_proposal`.
    //
    // Example:
    //     let result = spanda_ai::is_action_proposal(value);

    // Produce }) as the result.
    matches!(value, RuntimeValue::ActionProposal { .. })
}

/// Estimate model confidence from an ActionProposal (0.0–1.0).
pub fn proposal_confidence(value: &RuntimeValue) -> f64 {
    // Description:
    //     Proposal confidence.
    //
    // Inputs:
    //     value: &RuntimeValue
    //         Caller-supplied value.
    //
    // Outputs:
    //     result: f64
    //         Return value from `proposal_confidence`.
    //
    // Example:
    //     let result = spanda_ai::proposal_confidence(value);

    // Match on value and handle each case.
    match value {
        RuntimeValue::ActionProposal { trace, .. } => {
            // Handle each input line.
            for line in trace {
                // Emit output when strip prefix provides a dist str.
                if let Some(dist_str) = line.strip_prefix("nearest_distance=") {
                    // Handle the success value from <f64>.
                    if let Ok(dist) = dist_str.parse::<f64>() {
                        return (dist / 5.0).clamp(0.05, 1.0);
                    }
                }

                // Check membership before continuing.
                if line.contains("decision=stop") {
                    return 0.95;
                }
            }
            0.75
        }
        RuntimeValue::Object { fields, .. } => {
            // Take this path when let Some(RuntimeValue::Number { value, .. }) = fields.get("confidence".
            if let Some(RuntimeValue::Number { value, .. }) = fields.get("confidence") {
                return value.clamp(0.0, 1.0);
            }
            0.75
        }
        _ => 1.0,
    }
}

pub const AI_CONFIDENCE_LOW_THRESHOLD: f64 = 0.5;

pub fn is_safe_action(value: &RuntimeValue) -> bool {
    // Description:
    //     Is safe action.
    //
    // Inputs:
    //     value: &RuntimeValue
    //         Caller-supplied value.
    //
    // Outputs:
    //     result: bool
    //         Return value from `is_safe_action`.
    //
    // Example:
    //     let result = spanda_ai::is_safe_action(value);

    // Produce }) as the result.
    matches!(value, RuntimeValue::SafeAction { .. })
}

#[derive(Debug, Clone, PartialEq)]
pub struct ActionProposalFields {
    pub linear: f64,
    pub angular: f64,
    pub source: String,
}

pub fn proposal_from_value(value: &RuntimeValue) -> Option<ActionProposalFields> {
    // Description:
    //     Proposal from value.
    //
    // Inputs:
    //     value: &RuntimeValue
    //         Caller-supplied value.
    //
    // Outputs:
    //     result: Option<ActionProposalFields>
    //         Return value from `proposal_from_value`.
    //
    // Example:
    //     let result = spanda_ai::proposal_from_value(value);

    // Match on value and handle each case.
    match value {
        RuntimeValue::ActionProposal {
            linear,
            angular,
            source,
            trace: _,
        } => Some(ActionProposalFields {
            linear: *linear,
            angular: *angular,
            source: source.clone(),
        }),
        RuntimeValue::Object { type_name, fields } if type_name == "ActionProposal" => {
            let linear = match fields.get("linear") {
                Some(RuntimeValue::Number { value, .. }) => *value,
                _ => 0.0,
            };
            let angular = match fields.get("angular") {
                Some(RuntimeValue::Number { value, .. }) => *value,
                _ => 0.0,
            };
            Some(ActionProposalFields {
                linear,
                angular,
                source: "object".to_string(),
            })
        }
        RuntimeValue::Velocity { linear, angular } => Some(ActionProposalFields {
            linear: *linear,
            angular: *angular,
            source: "velocity".to_string(),
        }),
        _ => None,
    }
}

pub fn safe_action_from_proposal(linear: f64, angular: f64) -> RuntimeValue {
    // Description:
    //     Safe action from proposal.
    //
    // Inputs:
    //     linear: f64
    //         Caller-supplied linear.
    //     angular: f64
    //         Caller-supplied angular.
    //
    // Outputs:
    //     result: RuntimeValue
    //         Return value from `safe_action_from_proposal`.
    //
    // Example:
    //     let result = spanda_ai::safe_action_from_proposal(linear, angular);

    // Produce runtime safe action as the result.
    runtime_safe_action(linear, angular)
}

pub fn wrap_completion(text: impl Into<String>, model: impl Into<String>) -> RuntimeValue {
    // Description:
    //     Wrap completion.
    //
    // Inputs:
    //     ex: impl Into<String>
    //         Caller-supplied ex.
    //     odel: impl Into<String>
    //         Caller-supplied odel.
    //
    // Outputs:
    //     result: RuntimeValue
    //         Return value from `wrap_completion`.
    //
    // Example:
    //     let result = spanda_ai::wrap_completion(ex, odel);

    // Build a Completion runtime value.
    RuntimeValue::Completion {
        text: text.into(),
        model: Some(model.into()),
    }
}

pub fn wrap_detection(label: &str, confidence: f64, nearest_distance: f64) -> RuntimeValue {
    // Description:
    //     Wrap detection.
    //
    // Inputs:
    //     label: &str
    //         Caller-supplied label.
    //     confidence: f64
    //         Caller-supplied confidence.
    //     nearest_distance: f64
    //         Caller-supplied nearest distance.
    //
    // Outputs:
    //     result: RuntimeValue
    //         Return value from `wrap_detection`.
    //
    // Example:
    //     let result = spanda_ai::wrap_detection(label, confidence, nearest_distance);

    // Create mutable fields for accumulating results.
    let mut fields = HashMap::new();
    fields.insert("label".to_string(), RuntimeValue::string(label));
    fields.insert(
        "confidence".to_string(),
        RuntimeValue::number(confidence, UnitKind::None),
    );
    fields.insert(
        "nearest_distance".to_string(),
        RuntimeValue::number(nearest_distance, UnitKind::M),
    );
    RuntimeValue::object("Detection", fields)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mock_provider_proposes_motion() {
        // Description:
        //     Mock provider proposes motion.
        //
        // Inputs:
        //     None.
        //
        // Outputs:
        //     None.
        //
        // Example:

        //     let result = spanda_ai::mock_provider_proposes_motion();

        let provider = MockAiProvider;
        let result = provider.complete(&CompletionRequest {
            prompt: "Go forward".to_string(),
            input: Some(RuntimeValue::scan(2.0)),
            model: "mock".to_string(),
            provider: "mock".to_string(),
            temperature: 0.2,
            max_tokens: 512,
        });
        assert!(is_action_proposal(&result));
    }

    #[test]
    fn mock_provider_stops_on_halt_prompt() {
        // Description:
        //     Mock provider stops on halt prompt.
        //
        // Inputs:
        //     None.
        //
        // Outputs:
        //     None.
        //
        // Example:

        //     let result = spanda_ai::mock_provider_stops_on_halt_prompt();

        let provider = MockAiProvider;
        let result = provider.complete(&CompletionRequest {
            prompt: "stop now".to_string(),
            input: None,
            model: "mock".to_string(),
            provider: "mock".to_string(),
            temperature: 0.2,
            max_tokens: 512,
        });
        if let RuntimeValue::ActionProposal {
            linear, angular, ..
        } = result
        {
            assert_eq!(linear, 0.0);
            assert_eq!(angular, 0.0);
        } else {
            panic!("expected action proposal");
        }
    }

    #[test]
    fn mock_summarize_scan() {
        // Description:
        //     Mock summarize scan.
        //
        // Inputs:
        //     None.
        //
        // Outputs:
        //     None.
        //
        // Example:

        //     let result = spanda_ai::mock_summarize_scan();

        let summary = mock_summarize(Some(&RuntimeValue::scan(1.25)), "mock");
        if let RuntimeValue::Completion { text, .. } = summary {
            assert!(text.contains("1.25"));
        } else {
            panic!("expected completion");
        }
    }

    #[test]
    fn memory_store_recalls_latest() {
        // Description:
        //     Memory store recalls latest.
        //
        // Inputs:
        //     None.
        //
        // Outputs:
        //     None.
        //
        // Example:

        //     let result = spanda_ai::memory_store_recalls_latest();

        let mut store = MemoryStore::new(AiMemoryKind::ShortTerm, None);
        store.remember("a", RuntimeValue::number(1.0, UnitKind::None));
        store.remember("a", RuntimeValue::number(2.0, UnitKind::None));
        assert_eq!(
            store.recall("a"),
            Some(&RuntimeValue::number(2.0, UnitKind::None))
        );
    }

    #[test]
    fn resolves_ai_imports() {
        // Description:
        //     Resolves ai imports.
        //
        // Inputs:
        //     None.
        //
        // Outputs:
        //     None.
        //
        // Example:

        //     let result = spanda_ai::resolves_ai_imports();

        assert!(resolve_ai_import("onnx.runtime").is_some());
        assert!(resolve_ai_import("openvino.runtime").is_some());
        assert!(list_ai_libraries().len() >= 4);
    }

    #[test]
    fn proposal_from_velocity() {
        // Description:
        //     Proposal from velocity.
        //
        // Inputs:
        //     None.
        //
        // Outputs:
        //     None.
        //
        // Example:

        //     let result = spanda_ai::proposal_from_velocity();

        let proposal = proposal_from_value(&RuntimeValue::Velocity {
            linear: 0.5,
            angular: 0.1,
        });
        assert_eq!(proposal.unwrap().source, "velocity");
    }
}
