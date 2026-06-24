//! Live AI provider backends (OpenAI via Python bridge or native HTTP).

use crate::{
    action_proposal, build_prompt, scan_distance, AiProvider, CompletionRequest, DetectionRequest,
    EmbedRequest, MockAiProvider,
};
use spanda_runtime::value::RuntimeValue;
use std::io::{Read, Write};
use std::process::{Command, Stdio};

/// Return true when live AI providers should be used instead of mock-only mode.
pub fn live_ai_enabled() -> bool {
    // Description:
    //     Live ai enabled.
    //
    // Inputs:
    //     None.
    //
    // Outputs:
    //     result: bool
    //         Return value from `live_ai_enabled`.
    //
    // Example:

    //     let result = spanda_ai::live::live_ai_enabled();

    std::env::var("SPANDA_LIVE_AI").ok().as_deref() != Some("0")
        && std::env::var("OPENAI_API_KEY")
            .ok()
            .is_some_and(|key| !key.is_empty())
}

/// Return true when live Anthropic providers should be used.
pub fn live_anthropic_enabled() -> bool {
    // Description:
    //     Live anthropic enabled.
    //
    // Inputs:
    //     None.
    //
    // Outputs:
    //     result: bool
    //         Return value from `live_anthropic_enabled`.
    //
    // Example:

    //     let result = spanda_ai::live::live_anthropic_enabled();

    std::env::var("SPANDA_LIVE_AI").ok().as_deref() != Some("0")
        && std::env::var("ANTHROPIC_API_KEY")
            .ok()
            .is_some_and(|key| !key.is_empty())
}

/// Return true when live ONNX inference should be used.
pub fn live_onnx_enabled() -> bool {
    // Description:
    //     Live onnx enabled.
    //
    // Inputs:
    //     None.
    //
    // Outputs:
    //     result: bool
    //         Return value from `live_onnx_enabled`.
    //
    // Example:

    //     let result = spanda_ai::live::live_onnx_enabled();

    std::env::var("SPANDA_LIVE_AI").ok().as_deref() != Some("0")
        && std::env::var("SPANDA_ONNX_MODEL_PATH")
            .ok()
            .is_some_and(|path| !path.is_empty())
}

/// Select a runtime AI provider for the configured provider name.
pub fn resolve_ai_provider(provider: &str) -> Box<dyn AiProvider> {
    // Description:
    //     Resolve ai provider.
    //
    // Inputs:
    //     provider: &str
    //         Caller-supplied provider.
    //
    // Outputs:
    //     result: Box<dyn AiProvider>
    //         Return value from `resolve_ai_provider`.
    //
    // Example:

    //     let result = spanda_ai::live::resolve_ai_provider(provider);

    match provider.to_ascii_lowercase().as_str() {
        "openai" if live_ai_enabled() => Box::new(OpenAiProvider),
        "anthropic" if live_anthropic_enabled() => Box::new(AnthropicProvider),
        "onnx" if live_onnx_enabled() => Box::new(OnnxProvider),
        _ => Box::new(MockAiProvider),
    }
}

pub struct OpenAiProvider;

impl AiProvider for OpenAiProvider {
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

        //     let result = spanda_ai::live::complete(&self, reques);

        let prompt = build_prompt(&request.prompt, request.input.as_ref(), None);
        if let Some(text) = call_openai_complete(&prompt) {
            return proposal_from_completion(&text, request);
        }
        MockAiProvider.complete(request)
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

        //     let result = spanda_ai::live::detect(&self, reques);

        MockAiProvider.detect(request)
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

        //     let result = spanda_ai::live::embed(&self, reques);

        MockAiProvider.embed(request)
    }
}

pub struct AnthropicProvider;

impl AiProvider for AnthropicProvider {
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

        //     let result = spanda_ai::live::complete(&self, reques);

        let prompt = build_prompt(&request.prompt, request.input.as_ref(), None);
        if let Some(text) = call_anthropic_complete(&prompt) {
            return proposal_from_completion(&text, request);
        }
        MockAiProvider.complete(request)
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

        //     let result = spanda_ai::live::detect(&self, reques);

        MockAiProvider.detect(request)
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

        //     let result = spanda_ai::live::embed(&self, reques);

        MockAiProvider.embed(request)
    }
}

pub struct OnnxProvider;

impl AiProvider for OnnxProvider {
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

        //     let result = spanda_ai::live::complete(&self, reques);

        let prompt = build_prompt(&request.prompt, request.input.as_ref(), None);
        if let Some(text) = call_onnx_complete(&prompt) {
            return proposal_from_completion(&text, request);
        }
        MockAiProvider.complete(request)
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

        //     let result = spanda_ai::live::detect(&self, reques);

        MockAiProvider.detect(request)
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

        //     let result = spanda_ai::live::embed(&self, reques);

        MockAiProvider.embed(request)
    }
}

fn proposal_from_completion(text: &str, request: &CompletionRequest) -> RuntimeValue {
    // Description:
    //     Proposal from completion.
    //
    // Inputs:
    //     ex: &str
    //         Caller-supplied ex.
    //     request: &CompletionRequest
    //         Caller-supplied request.
    //
    // Outputs:
    //     result: RuntimeValue
    //         Return value from `proposal_from_completion`.
    //
    // Example:

    //     let result = spanda_ai::live::proposal_from_completion(ex, reques);

    let dist = scan_distance(request.input.as_ref());
    let lower = text.to_ascii_lowercase();
    if lower.contains("stop") || lower.contains("halt") || lower.contains("wait") {
        return action_proposal(
            0.0,
            0.0,
            &request.model,
            vec![
                format!("model={}", request.model),
                format!("provider={}", request.provider),
                format!("completion={text}"),
                "decision=stop".into(),
            ],
        );
    }
    if lower.contains("turn") || lower.contains("avoid") || dist < 0.8 {
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
                format!("provider={}", request.provider),
                format!("completion={text}"),
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
            format!("provider={}", request.provider),
            format!("completion={text}"),
            "decision=forward".into(),
        ],
    )
}

fn call_onnx_complete(prompt: &str) -> Option<String> {
    // Description:
    //     Call onnx complete.
    //
    // Inputs:
    //     promp: &str
    //         Caller-supplied promp.
    //
    // Outputs:
    //     result: Option<String>
    //         Return value from `call_onnx_complete`.
    //
    // Example:

    //     let result = spanda_ai::live::call_onnx_complete(promp);

    let response = call_python_bridge(
        "onnx_complete",
        vec![serde_json::Value::String(prompt.to_string())],
    )?;
    match response.get("result") {
        Some(serde_json::Value::String(text)) if !text.is_empty() => Some(text.clone()),
        _ => None,
    }
}

fn call_anthropic_complete(prompt: &str) -> Option<String> {
    // Description:
    //     Call anthropic complete.
    //
    // Inputs:
    //     promp: &str
    //         Caller-supplied promp.
    //
    // Outputs:
    //     result: Option<String>
    //         Return value from `call_anthropic_complete`.
    //
    // Example:

    //     let result = spanda_ai::live::call_anthropic_complete(promp);

    let response = call_python_bridge(
        "anthropic_complete",
        vec![serde_json::Value::String(prompt.to_string())],
    )?;
    match response.get("result") {
        Some(serde_json::Value::String(text)) if !text.is_empty() => Some(text.clone()),
        _ => None,
    }
}

fn call_openai_complete(prompt: &str) -> Option<String> {
    // Description:
    //     Call openai complete.
    //
    // Inputs:
    //     promp: &str
    //         Caller-supplied promp.
    //
    // Outputs:
    //     result: Option<String>
    //         Return value from `call_openai_complete`.
    //
    // Example:

    //     let result = spanda_ai::live::call_openai_complete(promp);

    let response = call_python_bridge(
        "openai_complete",
        vec![serde_json::Value::String(prompt.to_string())],
    )?;
    match response.get("result") {
        Some(serde_json::Value::String(text)) if !text.is_empty() => Some(text.clone()),
        _ => None,
    }
}

fn call_python_bridge(fn_name: &str, args: Vec<serde_json::Value>) -> Option<serde_json::Value> {
    // Description:
    //     Call python bridge.
    //
    // Inputs:
    //     fn_name: &str
    //         Caller-supplied fn name.
    //     args: Vec<serde_json::Value>
    //         Caller-supplied args.
    //
    // Outputs:
    //     result: Option<serde_json::Value>
    //         Return value from `call_python_bridge`.
    //
    // Example:

    //     let result = spanda_ai::live::call_python_bridge(fn_name, args);

    let script = bridge_script_path()?;
    let python = std::env::var("SPANDA_PYTHON").unwrap_or_else(|_| "python3".into());
    let request = serde_json::json!({ "fn": fn_name, "args": args });
    let mut child = Command::new(python)
        .arg(script)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .ok()?;
    {
        let stdin = child.stdin.as_mut()?;
        let payload = serde_json::to_string(&request).ok()?;
        stdin.write_all(payload.as_bytes()).ok()?;
    }
    let mut stdout = String::new();
    child.stdout.as_mut()?.read_to_string(&mut stdout).ok()?;
    let _ = child.wait();
    let parsed: serde_json::Value = serde_json::from_str(stdout.trim()).ok()?;
    if parsed.get("ok") == Some(&serde_json::Value::Bool(true)) {
        Some(parsed)
    } else {
        None
    }
}

fn bridge_script_path() -> Option<String> {
    // Description:
    //     Bridge script path.
    //
    // Inputs:
    //     None.
    //
    // Outputs:
    //     result: Option<String>
    //         Return value from `bridge_script_path`.
    //
    // Example:

    //     let result = spanda_ai::live::bridge_script_path();

    if let Ok(path) = std::env::var("SPANDA_PYTHON_BRIDGE") {
        if std::path::Path::new(&path).is_file() {
            return Some(path);
        }
    }
    let candidates = [
        "scripts/spanda_python_bridge.py".to_string(),
        format!(
            "{}/../../scripts/spanda_python_bridge.py",
            env!("CARGO_MANIFEST_DIR")
        ),
    ];
    for candidate in candidates {
        if std::path::Path::new(&candidate).is_file() {
            return Some(candidate);
        }
    }
    std::env::current_dir()
        .ok()
        .map(|cwd| cwd.join("scripts/spanda_python_bridge.py"))
        .filter(|p| p.is_file())
        .map(|p| p.to_string_lossy().into_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn live_ai_disabled_without_api_key() {
        // Description:
        //     Live ai disabled without api key.
        //
        // Inputs:
        //     None.
        //
        // Outputs:
        //     None.
        //
        // Example:

        //     let result = spanda_ai::live::live_ai_disabled_without_api_key();

        std::env::remove_var("OPENAI_API_KEY");
        std::env::remove_var("SPANDA_LIVE_AI");
        assert!(!live_ai_enabled());
    }
}
