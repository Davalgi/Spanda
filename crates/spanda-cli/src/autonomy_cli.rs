//! CLI commands for bio-inspired resilient autonomy architecture.
//!
use crate::config_load::{ensure_config_valid, load_system_config_from_cli_args};
use spanda_ast::assurance_decl::{AttentionPolicyDecl, HomeostasisPolicyDecl};
use spanda_ast::nodes::Program;
use spanda_autonomy::format::format_report;
use spanda_autonomy::types::AutonomyReportFormat;
use spanda_autonomy::{
    analyze_alert_fatigue, apply_habituation, apply_sensitization, compute_attention_score,
    compute_recovery_confidence, evaluate_homeostasis, evaluate_quarantine_decision,
    evaluate_reflex_priority, fuse_observations, list_reflex_actions, rank_events,
    AdaptiveRecoveryPolicy, AttentionPolicy, ConfidencePolicy, EventPriority, HabituationPolicy,
    HomeostasisPolicy, ImmunePolicy, RecoveryHistory, RepetitionPattern, SensitizationPolicy,
    SensorConfidence, StabilityMetric,
};
use spanda_autonomy::types::AutonomySeverity;
use spanda_config::build_entity_registry;
use spanda_lexer::tokenize;
use spanda_parser::parse;
use std::fs;
use std::path::Path;
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
        .find(|a| !a.starts_with('-') && !a.ends_with(".sd") && *a != "check" && *a != "report")
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
                && !matches!(
                    a.as_str(),
                    "list" | "simulate" | "trace" | "check" | "report"
                )
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
            eprintln!(
                "Usage: spanda homeostasis {{check|report}} [--json] [--program <file.sd>] [entity_id]"
            );
            process::exit(1);
        }
    }
}

fn cmd_homeostasis_check(args: &[String]) {
    // Evaluate homeostasis using platform defaults or metrics declared in `--program`.
    //
    // Parameters:
    // - `args` — CLI args after `homeostasis check`
    //
    // Returns:
    // Nothing; prints a formatted StabilityReport.
    //
    // Options:
    // `--program <file.sd>` loads `@policy(kind: "homeostasis")` / `homeostasis_policy` metrics.
    //
    // Example:
    // cmd_homeostasis_check(&args);

    let format = parse_format(args);
    let (entity, metrics) = load_entity_metrics(args);
    let policy = homeostasis_policy_from_args(args);
    let report = evaluate_homeostasis(&entity, &metrics, &policy);
    println!("{}", format_report(&report, format));
}

fn cmd_homeostasis_report(args: &[String]) {
    cmd_homeostasis_check(args);
}

pub fn attention_dispatch(args: &[String]) {
    match args.first().map(String::as_str).unwrap_or("") {
        "check" => cmd_attention_check(args),
        _ => {
            eprintln!("Usage: spanda attention check [--json] [--program <file.sd>]");
            process::exit(1);
        }
    }
}

fn cmd_attention_check(args: &[String]) {
    // Rank demo events using attention rules from `--program` when provided.
    //
    // Parameters:
    // - `args` — CLI args after `attention check`
    //
    // Returns:
    // Nothing; prints a ranked AttentionWindow.
    //
    // Options:
    // `--program <file.sd>` loads `@policy(kind: "attention")` / `attention_policy` rules.
    //
    // Example:
    // cmd_attention_check(&args);

    let format = parse_format(args);
    let policy = attention_policy_from_args(args);
    let scores = vec![
        compute_attention_score("e1", "routine_telemetry", EventPriority::Routine, AutonomySeverity::Info),
        compute_attention_score("e2", "battery_low", EventPriority::Important, AutonomySeverity::High),
        compute_attention_score(
            "e3",
            "emergency_stop",
            EventPriority::Critical,
            AutonomySeverity::Critical,
        ),
    ];
    let window = rank_events(scores, &policy);
    let report = serde_json::json!({
        "policy": {
            "critical_first": policy.critical_first,
            "suppress_routine_when_critical": policy.suppress_routine_when_critical,
            "suppression_rules": policy.suppression_rules,
        },
        "window": window,
    });
    println!("{}", format_report(&report, format));
}

fn program_path_arg(args: &[String]) -> Option<String> {
    // Resolve `--program <path>` or a positional `*.sd` path.
    //
    // Parameters:
    // - `args` — CLI argument list
    //
    // Returns:
    // Path to a `.sd` program when present.
    //
    // Options:
    // None.
    //
    // Example:
    // let path = program_path_arg(args);

    for (i, arg) in args.iter().enumerate() {
        if arg == "--program" {
            return args.get(i + 1).cloned();
        }
    }
    args.iter()
        .find(|a| a.ends_with(".sd") && !a.starts_with('-'))
        .cloned()
}

fn parse_program_file(path: &Path) -> Program {
    // Parse a Spanda source file into a Program AST.
    //
    // Parameters:
    // - `path` — filesystem path to `.sd`
    //
    // Returns:
    // Parsed program, or exits on failure.
    //
    // Options:
    // None.
    //
    // Example:
    // let program = parse_program_file(Path::new("rover.sd"));

    let source = fs::read_to_string(path).unwrap_or_else(|e| {
        eprintln!("Failed to read {}: {e}", path.display());
        process::exit(1);
    });
    let tokens = tokenize(&source).unwrap_or_else(|e| {
        eprintln!("{e}");
        process::exit(1);
    });
    parse(tokens).unwrap_or_else(|e| {
        eprintln!("{e}");
        process::exit(1);
    })
}

fn homeostasis_policy_from_args(args: &[String]) -> HomeostasisPolicy {
    // Build a homeostasis policy from optional `--program` declarations.
    //
    // Parameters:
    // - `args` — CLI args that may include `--program`
    //
    // Returns:
    // `HomeostasisPolicy` filtered to declared metrics, or platform defaults.
    //
    // Options:
    // None.
    //
    // Example:
    // let policy = homeostasis_policy_from_args(args);

    let Some(path) = program_path_arg(args) else {
        return HomeostasisPolicy::platform_defaults();
    };
    let Program::Program {
        homeostasis_policies,
        ..
    } = parse_program_file(Path::new(&path));
    let mut names = Vec::new();
    for policy in homeostasis_policies {
        let HomeostasisPolicyDecl::HomeostasisPolicyDecl { metrics, .. } = policy;
        names.extend(metrics);
    }
    HomeostasisPolicy::from_declared_metrics(&names)
}

fn attention_policy_from_args(args: &[String]) -> AttentionPolicy {
    // Build an attention policy from optional `--program` declarations.
    //
    // Parameters:
    // - `args` — CLI args that may include `--program`
    //
    // Returns:
    // `AttentionPolicy` derived from declared rules, or critical-first defaults.
    //
    // Options:
    // None.
    //
    // Example:
    // let policy = attention_policy_from_args(args);

    let Some(path) = program_path_arg(args) else {
        return AttentionPolicy::from_declared_rules(&[]);
    };
    let Program::Program {
        attention_policies, ..
    } = parse_program_file(Path::new(&path));
    let mut names = Vec::new();
    for policy in attention_policies {
        let AttentionPolicyDecl::AttentionPolicyDecl { rules, .. } = policy;
        names.extend(rules);
    }
    AttentionPolicy::from_declared_rules(&names)
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
