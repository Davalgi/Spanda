//! One-time stderr notices when mock or simulated backends are active.
//!
use spanda_ast::comm_decl::{BusDecl, TransportKind};
use spanda_ast::nodes::{Program, RobotDecl, TopicDecl};
use std::collections::HashSet;
use std::sync::Mutex;

static WARNED: Mutex<Option<HashSet<String>>> = Mutex::new(None);

fn warned_keys() -> std::sync::MutexGuard<'static, Option<HashSet<String>>> {
    WARNED.lock().unwrap_or_else(|error| error.into_inner())
}

fn quiet() -> bool {
    matches!(
        std::env::var("SPANDA_QUIET").ok().as_deref(),
        Some("1") | Some("true") | Some("yes") | Some("on")
    )
}

fn warn_once(key: &str, message: &str) {
    if quiet() {
        return;
    }
    let mut guard = warned_keys();
    let set = guard.get_or_insert_with(HashSet::new);
    if set.insert(key.to_string()) {
        eprintln!("[spanda] {message}");
    }
}

fn ai_any_live_configured() -> bool {
    let live_ai = std::env::var("SPANDA_LIVE_AI").ok().as_deref() != Some("0");
    live_ai
        && (std::env::var("OPENAI_API_KEY")
            .ok()
            .is_some_and(|key| !key.is_empty())
            || std::env::var("ANTHROPIC_API_KEY")
                .ok()
                .is_some_and(|key| !key.is_empty())
            || std::env::var("SPANDA_ONNX_MODEL_PATH")
                .ok()
                .is_some_and(|path| !path.is_empty()))
}

fn transport_live_configured(kind: TransportKind) -> bool {
    match kind {
        TransportKind::Ros2 => std::env::var("SPANDA_ROS2_LIVE").ok().as_deref() == Some("1"),
        TransportKind::Mqtt => {
            std::env::var("SPANDA_LIVE_MQTT").ok().as_deref() == Some("1")
                || std::env::var("SPANDA_MQTT_LIVE").is_ok()
        }
        TransportKind::Dds => std::env::var("SPANDA_LIVE_DDS").ok().as_deref() == Some("1"),
        TransportKind::Websocket => std::env::var("SPANDA_LIVE_WEBSOCKET").ok().as_deref() == Some("1"),
        TransportKind::Local | TransportKind::Sim => true,
    }
}

/// Warn once when a named live AI provider falls back to the mock backend.
pub fn warn_ai_mock_fallback(requested_provider: &str) {
    let fix = match requested_provider.to_ascii_lowercase().as_str() {
        "openai" => {
            "set OPENAI_API_KEY (unset SPANDA_LIVE_AI=0) — see docs/troubleshooting.md#live-ai-and-extern-bridges"
        }
        "anthropic" => {
            "set ANTHROPIC_API_KEY (unset SPANDA_LIVE_AI=0) — see docs/troubleshooting.md#live-ai-and-extern-bridges"
        }
        "onnx" => "set SPANDA_ONNX_MODEL_PATH — see docs/live-ai-provider.md",
        _ => "see docs/known-limitations.md#ai-and-providers",
    };
    warn_once(
        &format!("ai:{}", requested_provider.to_ascii_lowercase()),
        &format!(
            "AI provider '{requested_provider}' is using the mock backend — {fix}"
        ),
    );
}

/// Warn once when an external transport uses the in-memory/simulated path.
pub fn warn_transport_mock_fallback(transport: &str) {
    let fix = match transport {
        "ros2" => "export SPANDA_ROS2_LIVE=1 and source ROS 2 — see docs/ros2-golden-path.md",
        "mqtt" => "export SPANDA_LIVE_MQTT=1 (or SPANDA_MQTT_LIVE) — see docs/iot.md",
        "dds" => "export SPANDA_LIVE_DDS=1 (UDP JSON shim) — see docs/known-limitations.md",
        "websocket" => "export SPANDA_LIVE_WEBSOCKET=1 — see docs/iot.md",
        "modbus" => "export SPANDA_LIVE_MODBUS=1 — see docs/iot.md",
        "opcua" | "opc-ua" => "export SPANDA_LIVE_OPCUA=1 — see docs/iot.md",
        _ => "see docs/troubleshooting.md#live-iot-and-transports",
    };
    warn_once(
        &format!("transport:{transport}"),
        &format!("Transport '{transport}' is using the in-memory/simulated path — {fix}"),
    );
}

/// Emit program-aware notices before execution when mock backends are likely.
pub fn emit_program_backend_notices(program: &Program) {
    let Program::Program { robots, .. } = program;
    let mut uses_ai = false;
    let mut external_transports = HashSet::new();

    for robot in robots {
        let RobotDecl::RobotDecl {
            ai_models,
            agents,
            topics,
            buses,
            ..
        } = robot;
        if !ai_models.is_empty() || !agents.is_empty() {
            uses_ai = true;
        }
        for topic in topics {
            if let TopicDecl::TopicDecl {
                transport: Some(transport),
                ..
            } = topic
            {
                if !matches!(transport, TransportKind::Local | TransportKind::Sim) {
                    external_transports.insert(*transport);
                }
            }
        }
        for bus in buses {
            let BusDecl::BusDecl { transport, .. } = bus;
            if !matches!(transport, TransportKind::Local | TransportKind::Sim) {
                external_transports.insert(*transport);
            }
        }
    }

    if uses_ai && !ai_any_live_configured() {
        warn_once(
            "program:ai",
            "Program uses AI models/agents; live AI is not configured — mock backend active. \
             See docs/known-limitations.md#ai-and-providers (SPANDA_QUIET=1 to hide).",
        );
    }

    for transport in external_transports {
        if !transport_live_configured(transport) {
            warn_transport_mock_fallback(transport.as_str());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn warn_once_respects_quiet_and_deduplicates() {
        std::env::set_var("SPANDA_QUIET", "1");
        warn_ai_mock_fallback("openai");
        std::env::remove_var("SPANDA_QUIET");
        *warned_keys() = Some(HashSet::new());
        warn_ai_mock_fallback("openai");
        let count = warned_keys().as_ref().map(|set| set.len()).unwrap_or(0);
        assert_eq!(count, 1);
        warn_ai_mock_fallback("openai");
        let count = warned_keys().as_ref().map(|set| set.len()).unwrap_or(0);
        assert_eq!(count, 1);
    }
}
