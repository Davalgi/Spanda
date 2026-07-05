//! CLI commands for bio-inspired resilient autonomy architecture.
//!
use crate::config_load::{ensure_config_valid, load_system_config_from_cli_args};
use spanda_autonomy::format::format_report;
use spanda_autonomy::types::AutonomyReportFormat;
use spanda_autonomy::{
    analyze_alert_fatigue, apply_habituation, apply_sensitization, compute_recovery_confidence,
    evaluate_homeostasis, evaluate_quarantine_decision, evaluate_reflex_priority,
    fuse_observations, list_reflex_actions, AdaptiveRecoveryPolicy, ConfidencePolicy,
    HabituationPolicy, HomeostasisPolicy, ImmunePolicy, RecoveryHistory,
    RepetitionPattern, SensitizationPolicy, SensorConfidence, StabilityMetric,
};
use spanda_config::build_entity_registry;
use std::process;

fn parse_format(args: &[String]) -> AutonomyReportFormat {
    if args.iter().any(|a| a == "--json") {
        AutonomyReportFormat::Json
    } else if args.iter().any(|a| a == "--markdown") {
        AutonomyReportFormat::Markdown
    } else {
        AutonomyReportFormat::Text
    }
}

fn entity_id_arg(args: &[String]) -> Option<String> {
    args.iter()
        .find(|a| !a.starts_with('-'))
        .cloned()
        .or_else(|| {
            load_system_config_from_cli_args(args)
                .and_then(|cfg| build_entity_registry(&cfg).entities.keys().next().cloned())
        })
}

pub fn reflex_dispatch(args: &[String]) {
    match args.first().map(String::as_str).unwrap_or("") {
        "list" => cmd_reflex_list(args),
        "simulate" => cmd_reflex_simulate(args),
        "trace" => cmd_reflex_trace(args),
        _ => {
            eprintln!("Usage: spanda reflex {{list|simulate|trace}} [--json] [trigger_hint]");
            process::exit(1);
        }
    }
}

fn cmd_reflex_list(args: &[String]) {
    let format = parse_format(args);
    let actions = list_reflex_actions();
    println!("{}", format_report(&actions, format));
}

fn reflex_hint(args: &[String]) -> &str {
    args.iter()
        .find(|a| {
            !a.starts_with('-')
                && !matches!(a.as_str(), "list" | "simulate" | "trace" | "check" | "report")
        })
        .map(String::as_str)
        .unwrap_or("emergency")
}

fn cmd_reflex_simulate(args: &[String]) {
    let format = parse_format(args);
    let hint = reflex_hint(args);
    let actions = list_reflex_actions();
    let selected = evaluate_reflex_priority(&actions, hint);
    let report = serde_json::json!({
        "trigger_hint": hint,
        "selected": selected,
        "available": actions.len(),
    });
    println!("{}", format_report(&report, format));
}

fn cmd_reflex_trace(args: &[String]) {
    let format = parse_format(args);
    let hint = reflex_hint(args);
    let actions = list_reflex_actions();
    if let Some(action) = evaluate_reflex_priority(&actions, hint) {
        let trace = spanda_autonomy::ReflexTrace {
            reflex_id: action.id.clone(),
            entity_id: "local".into(),
            trigger: action.trigger.clone(),
            action_taken: action.action.clone(),
            timestamp: chrono::Utc::now().to_rfc3339(),
            priority: action.priority,
        };
        println!("{}", format_report(&trace, format));
    } else {
        eprintln!("No reflex matched hint: {hint}");
        process::exit(1);
    }
}

pub fn fusion_dispatch(args: &[String]) {
    match args.first().map(String::as_str).unwrap_or("") {
        "check" => cmd_fusion_check(args),
        _ => {
            eprintln!("Usage: spanda fusion check [--json] [entity_id]");
            process::exit(1);
        }
    }
}

pub fn confidence_dispatch(args: &[String]) {
    match args.first().map(String::as_str).unwrap_or("") {
        "report" => cmd_confidence_report(args),
        _ => {
            eprintln!("Usage: spanda confidence report [--json] [entity_id]");
            process::exit(1);
        }
    }
}

fn cmd_fusion_check(args: &[String]) {
    let format = parse_format(args);
    let readings = demo_sensor_readings();
    let fused = fuse_observations("navigation", &readings, &ConfidencePolicy::default());
    println!("{}", format_report(&fused, format));
}

fn cmd_confidence_report(args: &[String]) {
    let format = parse_format(args);
    let readings = demo_sensor_readings();
    let fused = fuse_observations("navigation", &readings, &ConfidencePolicy::default());
    let report = serde_json::json!({
        "confidence_score": fused.confidence,
        "conflicts": fused.conflicts,
        "recommendation": fused.confidence.recommendation,
    });
    println!("{}", format_report(&report, format));
}

fn demo_sensor_readings() -> Vec<SensorConfidence> {
    vec![
        SensorConfidence {
            source: "gps".into(),
            value: 1.0,
            confidence: 0.85,
            timestamp: None,
        },
        SensorConfidence {
            source: "imu".into(),
            value: 1.05,
            confidence: 0.9,
            timestamp: None,
        },
        SensorConfidence {
            source: "wheel_odometry".into(),
            value: 0.98,
            confidence: 0.75,
            timestamp: None,
        },
    ]
}

pub fn homeostasis_dispatch(args: &[String]) {
    match args.first().map(String::as_str).unwrap_or("") {
        "check" => cmd_homeostasis_check(args),
        "report" => cmd_homeostasis_report(args),
        _ => {
            eprintln!("Usage: spanda homeostasis {{check|report}} [--json] [entity_id]");
            process::exit(1);
        }
    }
}

fn cmd_homeostasis_check(args: &[String]) {
    let format = parse_format(args);
    let (entity, metrics) = load_entity_metrics(args);
    let report = evaluate_homeostasis(&entity, &metrics, &HomeostasisPolicy::platform_defaults());
    println!("{}", format_report(&report, format));
}

fn cmd_homeostasis_report(args: &[String]) {
    cmd_homeostasis_check(args);
}

pub fn immunity_dispatch(args: &[String]) {
    match args.first().map(String::as_str).unwrap_or("") {
        "scan" => cmd_immunity_scan(args),
        "quarantine" => cmd_immunity_quarantine(args),
        "report" => cmd_immunity_report(args),
        _ => {
            eprintln!("Usage: spanda immunity {{scan|quarantine|report}} [--json] [entity_id]");
            process::exit(1);
        }
    }
}

fn cmd_immunity_scan(args: &[String]) {
    let format = parse_format(args);
    let registry = load_registry(args);
    let policy = ImmunePolicy::platform_defaults();
    let events: Vec<_> = registry
        .entities
        .values()
        .flat_map(|e| spanda_autonomy::evaluate_immunity(e, &policy))
        .collect();
    println!("{}", format_report(&events, format));
}

fn cmd_immunity_quarantine(args: &[String]) {
    let format = parse_format(args);
    let entity = load_entity(args);
    let decision = evaluate_quarantine_decision(&entity, &ImmunePolicy::platform_defaults());
    println!("{}", format_report(&decision, format));
}

fn cmd_immunity_report(args: &[String]) {
    cmd_immunity_scan(args);
}

pub fn alerts_dispatch(args: &[String]) {
    match args.first().map(String::as_str).unwrap_or("") {
        "analyze" => cmd_alerts_analyze(args),
        "fatigue-report" => cmd_alerts_fatigue_report(args),
        _ => {
            eprintln!("Usage: spanda alerts {{analyze|fatigue-report}} [--json]");
            process::exit(1);
        }
    }
}

fn cmd_alerts_analyze(args: &[String]) {
    let format = parse_format(args);
    let patterns = demo_alert_patterns();
    let suppressions = apply_habituation(&patterns, &HabituationPolicy::default());
    let escalations = apply_sensitization(&patterns, &SensitizationPolicy::default());
    let report = serde_json::json!({
        "patterns": patterns,
        "suppressions": suppressions,
        "escalations": escalations,
    });
    println!("{}", format_report(&report, format));
}

fn cmd_alerts_fatigue_report(args: &[String]) {
    let format = parse_format(args);
    let patterns = demo_alert_patterns();
    let suppressions = apply_habituation(&patterns, &HabituationPolicy::default());
    let escalations = apply_sensitization(&patterns, &SensitizationPolicy::default());
    let fatigue = analyze_alert_fatigue(&suppressions, &escalations);
    println!("{}", format_report(&fatigue, format));
}

fn demo_alert_patterns() -> Vec<RepetitionPattern> {
    vec![
        RepetitionPattern {
            label: "routine_telemetry".into(),
            count: 120,
            trend: "stable".into(),
        },
        RepetitionPattern {
            label: "network_glitch".into(),
            count: 6,
            trend: "worsening".into(),
        },
        RepetitionPattern {
            label: "recovery_attempt".into(),
            count: 4,
            trend: "stable".into(),
        },
    ]
}

pub fn recovery_confidence_dispatch(args: &[String]) {
    match args.first().map(String::as_str).unwrap_or("") {
        "confidence" => cmd_recovery_confidence(args),
        "learning-report" => cmd_recovery_learning_report(args),
        _ => {
            eprintln!("Usage: spanda recovery confidence|learning-report [--json] [entity_id]");
            process::exit(1);
        }
    }
}

fn cmd_recovery_confidence(args: &[String]) {
    let format = parse_format(args);
    let entity_id = entity_id_arg(args).unwrap_or_else(|| "local".into());
    let history = demo_recovery_history(&entity_id);
    let rc = compute_recovery_confidence(
        &entity_id,
        &history,
        &AdaptiveRecoveryPolicy::platform_defaults(),
    );
    println!("{}", format_report(&rc, format));
}

fn cmd_recovery_learning_report(args: &[String]) {
    cmd_recovery_confidence(args);
}

fn demo_recovery_history(entity_id: &str) -> Vec<RecoveryHistory> {
    vec![
        RecoveryHistory {
            entity_id: entity_id.into(),
            strategy: "reconnect_camera".into(),
            success: true,
            duration_ms: 400,
        },
        RecoveryHistory {
            entity_id: entity_id.into(),
            strategy: "reconnect_camera".into(),
            success: true,
            duration_ms: 500,
        },
        RecoveryHistory {
            entity_id: entity_id.into(),
            strategy: "restart_provider".into(),
            success: false,
            duration_ms: 3000,
        },
    ]
}

fn load_registry(args: &[String]) -> spanda_config::EntityRegistry {
    if let Some(resolved) = load_system_config_from_cli_args(args) {
        ensure_config_valid(Some(resolved.as_ref()));
        return build_entity_registry(resolved.as_ref());
    }
    spanda_config::EntityRegistry::default()
}

fn load_entity(args: &[String]) -> spanda_config::EntityRecord {
    let registry = load_registry(args);
    if let Some(id) = entity_id_arg(args) {
        if let Some(entity) = registry.get(&id) {
            return entity.clone();
        }
    }
    registry
        .entities
        .values()
        .next()
        .cloned()
        .unwrap_or_default()
}

fn load_entity_metrics(args: &[String]) -> (spanda_config::EntityRecord, Vec<StabilityMetric>) {
    let entity = load_entity(args);
    let metrics = vec![
        StabilityMetric {
            name: "cpu_pct".into(),
            value: 55.0,
            unit: "pct".into(),
        },
        StabilityMetric {
            name: "memory_pct".into(),
            value: 62.0,
            unit: "pct".into(),
        },
        StabilityMetric {
            name: "battery_pct".into(),
            value: 72.0,
            unit: "pct".into(),
        },
    ];
    (entity, metrics)
}
