//! CLI commands for self-healing and recovery framework.

use crate::config_load::{ensure_config_valid, load_system_config_from_cli_args};

use spanda_assurance::{
    analyze_failure_with_recovery, diagnose_from_trace, evaluate_recovery, format_recovery,
    load_merged_recovery_knowledge, recovery_from_diagnosis, simulate_failure_recovery,
    RecoveryContext, RecoveryLevel, RecoveryReport,
};
use spanda_config::build_entity_registry;
use spanda_lexer::tokenize;
use spanda_parser::parse;
use spanda_readiness::{format_failure_analysis, ReportFormat};
use spanda_recovery::{
    format_decision, format_graph, format_history, format_metrics, format_orchestrator_report,
    format_playbooks, OrchestratorContext, RecoveryOrchestrator, RecoveryOrchestratorRequest,
    RecoverySimulationMode,
};
use std::fs;
use std::path::Path;
use std::process;

const MINIMAL_PROGRAM: &str = "robot Placeholder { behavior idle() {} }";

fn read_file(path: &str) -> String {
    // Description:
    //     Read file.
    //
    // Inputs:
    //     path: &str
    //         Caller-supplied path.
    //
    // Outputs:
    //     result: String
    //         Return value from `read_file`.
    //
    // Example:

    //     let result = spanda_cli::recovery_cli::read_file(path);

    fs::read_to_string(path).unwrap_or_else(|e| {
        eprintln!("Failed to read {path}: {e}");
        process::exit(1);
    })
}

fn parse_program(source: &str) -> spanda_ast::nodes::Program {
    // Description:
    //     Parse program.
    //
    // Inputs:
    //     source: &str
    //         Caller-supplied source.
    //
    // Outputs:
    //     result: spanda_ast::nodes::Program
    //         Return value from `parse_program`.
    //
    // Example:

    //     let result = spanda_cli::recovery_cli::parse_program(source);

    let tokens = tokenize(source).unwrap_or_else(|e| {
        eprintln!("{e}");
        process::exit(1);
    });
    parse(tokens).unwrap_or_else(|e| {
        eprintln!("{e}");
        process::exit(1);
    })
}

fn parse_format(args: &[String]) -> ReportFormat {
    // Description:
    //     Parse format.
    //
    // Inputs:
    //     args: &[String]
    //         Caller-supplied args.
    //
    // Outputs:
    //     result: ReportFormat
    //         Return value from `parse_format`.
    //
    // Example:

    //     let result = spanda_cli::recovery_cli::parse_format(args);

    if args.iter().any(|a| a == "--json") {
        ReportFormat::Json
    } else if args.iter().any(|a| a == "--markdown") {
        ReportFormat::Markdown
    } else if args.iter().any(|a| a == "--html") {
        ReportFormat::Html
    } else {
        ReportFormat::Text
    }
}

fn file_arg(args: &[String]) -> String {
    // Description:
    //     File arg.
    //
    // Inputs:
    //     args: &[String]
    //         Caller-supplied args.
    //
    // Outputs:
    //     result: String
    //         Return value from `file_arg`.
    //
    // Example:

    //     let result = spanda_cli::recovery_cli::file_arg(args);

    args.iter()
        .find(|a| !a.starts_with('-'))
        .cloned()
        .unwrap_or_else(|| {
            eprintln!("Missing file path");
            process::exit(1);
        })
}

fn failure_arg(args: &[String]) -> Option<String> {
    // Description:
    //     Failure arg.
    //
    // Inputs:
    //     args: &[String]
    //         Caller-supplied args.
    //
    // Outputs:
    //     result: Option<String>
    //         Return value from `failure_arg`.
    //
    // Example:

    //     let result = spanda_cli::recovery_cli::failure_arg(args);

    args.iter()
        .position(|a| a == "--inject-failure" || a == "--failure")
        .and_then(|i| args.get(i + 1).cloned())
}

fn device_registry_from_args(
    args: &[String],
) -> Option<std::sync::Arc<spanda_config::ResolvedSystemConfig>> {
    load_system_config_from_cli_args(args)
}

fn entity_id_arg(args: &[String]) -> Option<String> {
    args.iter()
        .position(|a| a == "--entity" || a == "--entity-id")
        .and_then(|i| args.get(i + 1).cloned())
}

fn orchestrator_context(
    args: &[String],
) -> (
    spanda_ast::nodes::Program,
    spanda_config::entity::EntityRegistry,
    Option<std::sync::Arc<spanda_config::ResolvedSystemConfig>>,
) {
    let file = file_arg(args);
    let source = read_file(&file);
    let program = parse_program(&source);
    let config = device_registry_from_args(args);
    let registry = config
        .as_ref()
        .map(|c| build_entity_registry(c))
        .unwrap_or_default();
    (program, registry, config)
}

fn orchestrator_request_from_args(
    args: &[String],
    mode: RecoverySimulationMode,
) -> RecoveryOrchestratorRequest {
    RecoveryOrchestratorRequest {
        entity_id: entity_id_arg(args),
        failure: failure_arg(args),
        mode,
        playbook: args
            .iter()
            .position(|a| a == "--playbook")
            .and_then(|i| args.get(i + 1).cloned()),
        max_escalation_level: None,
        force_execute: args.iter().any(|a| a == "--execute" || a == "--force"),
    }
}

/// `spanda recovery plan <file.sd> [--entity <id>] [--failure <kind>] [--json]`
pub fn cmd_orchestrator_plan(args: &[String]) {
    let format = parse_format(args);
    let (program, registry, config) = orchestrator_context(args);
    let orchestrator = RecoveryOrchestrator::new();
    let request = orchestrator_request_from_args(args, RecoverySimulationMode::Plan);
    let report = orchestrator.plan_recovery(&program, &registry, config.as_deref(), &request);
    println!("{}", format_orchestrator_report(&report, format));
    if !report.passed {
        process::exit(1);
    }
}

/// `spanda recovery simulate <file.sd> [--entity <id>] [--failure <kind>]`
pub fn cmd_orchestrator_simulate(args: &[String]) {
    let format = parse_format(args);
    let (program, registry, config) = orchestrator_context(args);
    let orchestrator = RecoveryOrchestrator::new();
    let request = orchestrator_request_from_args(args, RecoverySimulationMode::Simulate);
    let report =
        orchestrator.simulate_recovery(&program, &registry, config.as_deref(), &request, None);
    println!("{}", format_orchestrator_report(&report, format));
}

/// `spanda recovery execute <file.sd> [--entity <id>] [--failure <kind>] [--force]`
pub fn cmd_orchestrator_execute(args: &[String]) {
    let format = parse_format(args);
    let (program, registry, config) = orchestrator_context(args);
    let mut orchestrator = RecoveryOrchestrator::new();
    let request = orchestrator_request_from_args(args, RecoverySimulationMode::Validate);
    let ctx = OrchestratorContext {
        skip_execution: !request.force_execute,
        ..Default::default()
    };
    let report =
        orchestrator.execute_recovery(&program, &registry, config.as_deref(), &request, &ctx);
    println!("{}", format_orchestrator_report(&report, format));
    if !report.passed {
        process::exit(1);
    }
}

/// `spanda recovery validate <file.sd> [--entity <id>] [--failure <kind>]`
pub fn cmd_orchestrator_validate(args: &[String]) {
    let format = parse_format(args);
    let (program, registry, config) = orchestrator_context(args);
    let orchestrator = RecoveryOrchestrator::new();
    let request = orchestrator_request_from_args(args, RecoverySimulationMode::Validate);
    let report = orchestrator.dry_run_recovery(&program, &registry, config.as_deref(), &request);
    println!("{}", format_orchestrator_report(&report, format));
}

/// `spanda recovery history [--json]`
pub fn cmd_orchestrator_history(args: &[String]) {
    let format = parse_format(args);
    let orchestrator = RecoveryOrchestrator::new();
    let history = orchestrator.get_history(50);
    println!("{}", format_history(&history, format));
}

/// `spanda recovery metrics <file.sd> [--json]`
pub fn cmd_orchestrator_metrics(args: &[String]) {
    let format = parse_format(args);
    let (program, _, _) = orchestrator_context(args);
    let orchestrator = RecoveryOrchestrator::new();
    let metrics = orchestrator.get_metrics(&program);
    println!("{}", format_metrics(&metrics, format));
}

/// `spanda recovery graph <file.sd> [--entity <id>] [--json]`
pub fn cmd_orchestrator_graph(args: &[String]) {
    let format = parse_format(args);
    let (_, registry, _) = orchestrator_context(args);
    let orchestrator = RecoveryOrchestrator::new();
    let entity_id = entity_id_arg(args);
    let graph = orchestrator.build_graph(&registry, entity_id.as_deref());
    println!("{}", format_graph(&graph, format));
}

/// `spanda recovery playbooks [--json] [--config <spanda.toml>]`
pub fn cmd_orchestrator_playbooks(args: &[String]) {
    let format = parse_format(args);
    let config = device_registry_from_args(args);
    let orchestrator = RecoveryOrchestrator::new();
    let playbooks = orchestrator.list_playbooks(config.as_deref());
    println!("{}", format_playbooks(&playbooks, format));
}

/// `spanda recovery explain <file.sd> --entity <id> [--failure <kind>]`
pub fn cmd_orchestrator_explain(args: &[String]) {
    let format = parse_format(args);
    let (_, registry, config) = orchestrator_context(args);
    let entity_id = entity_id_arg(args).unwrap_or_else(|| {
        eprintln!("--entity <id> required for explain");
        process::exit(1);
    });
    let failure = failure_arg(args).unwrap_or_else(|| "degraded".into());
    let orchestrator = RecoveryOrchestrator::new();
    let decision = orchestrator
        .explain_recovery(&registry, config.as_deref(), &entity_id, &failure)
        .unwrap_or_else(|| {
            eprintln!("entity '{entity_id}' not found");
            process::exit(1);
        });
    println!("{}", format_decision(&decision, format));
}

/// `spanda recovery dry-run <file.sd> [--entity <id>] [--failure <kind>]`
pub fn cmd_orchestrator_dry_run(args: &[String]) {
    let format = parse_format(args);
    let (program, registry, config) = orchestrator_context(args);
    let orchestrator = RecoveryOrchestrator::new();
    let request = orchestrator_request_from_args(args, RecoverySimulationMode::DryRun);
    let report = orchestrator.dry_run_recovery(&program, &registry, config.as_deref(), &request);
    println!("{}", format_orchestrator_report(&report, format));
}

fn build_report(file: &str, args: &[String]) -> RecoveryReport {
    // Description:
    //     Build report.
    //
    // Inputs:
    //     file: &str
    //         Caller-supplied file.
    //     args: &[String]
    //         Caller-supplied args.
    //
    // Outputs:
    //     result: RecoveryReport
    //         Return value from `build_report`.
    //
    // Example:

    //     let result = spanda_cli::recovery_cli::build_report(file, args);

    let config = device_registry_from_args(args);
    let device_registry = config.as_ref().map(|cfg| &cfg.device_registry);

    if file.ends_with(".trace") {
        let diagnosis = diagnose_from_trace(Path::new(file)).unwrap_or_else(|e| {
            eprintln!("{e}");
            process::exit(1);
        });
        let sd_path = file.replacen(".trace", ".sd", 1);
        let program = if Path::new(&sd_path).exists() {
            parse_program(&read_file(&sd_path))
        } else {
            parse_program(MINIMAL_PROGRAM)
        };
        recovery_from_diagnosis(&program, &diagnosis)
    } else {
        let source = read_file(file);
        let program = parse_program(&source);
        if let Some(failure) = failure_arg(args) {
            simulate_failure_recovery(&program, &failure, device_registry)
        } else {
            evaluate_recovery(&program, None, device_registry)
        }
    }
}

/// `spanda heal <file.sd|mission.trace> [--json|--markdown|--html] [--failure <kind>]`
pub fn cmd_heal(args: &[String]) {
    if let Some(cfg) = load_system_config_from_cli_args(args) {
        ensure_config_valid(Some(cfg.as_ref()));
    }
    // Description:
    //     Cmd heal.
    //
    // Inputs:
    //     args: &[String]
    //         Caller-supplied args.
    //
    // Outputs:
    //     None.
    //
    // Example:

    //     let result = spanda_cli::recovery_cli::cmd_heal(args);

    let format = parse_format(args);
    let file = file_arg(args);
    let report = build_report(&file, args);
    println!("{}", format_recovery(&report, format));
    if !report.passed {
        process::exit(1);
    }
}

/// `spanda recover <file.sd> [--json] [--failure <kind>]`
pub fn cmd_recover(args: &[String]) {
    if let Some(cfg) = load_system_config_from_cli_args(args) {
        ensure_config_valid(Some(cfg.as_ref()));
    }
    // Description:
    //     Cmd recover.
    //
    // Inputs:
    //     args: &[String]
    //         Caller-supplied args.
    //
    // Outputs:
    //     None.
    //
    // Example:

    //     let result = spanda_cli::recovery_cli::cmd_recover(args);

    let format = parse_format(args);
    let file = file_arg(args);
    let source = read_file(&file);
    let program = parse_program(&source);
    let config = device_registry_from_args(args);
    let device_registry = config.as_ref().map(|cfg| &cfg.device_registry);
    let report = if let Some(failure) = failure_arg(args) {
        simulate_failure_recovery(&program, &failure, device_registry)
    } else {
        let ctx = RecoveryContext {
            issue: "gps.failed".into(),
            diagnosis: Some("Satellite lock lost".into()),
            classification: None,
            level: RecoveryLevel::Level3AutomaticWithValidation,
        };
        evaluate_recovery(&program, Some(&ctx), device_registry)
    };
    println!("{}", format_recovery(&report, format));
    if !report.passed {
        process::exit(1);
    }
}

/// `spanda recovery-report <file.sd> [--json|--markdown|--html]`
pub fn cmd_recovery_report(args: &[String]) {
    // Description:
    //     Cmd recovery report.
    //
    // Inputs:
    //     args: &[String]
    //         Caller-supplied args.
    //
    // Outputs:
    //     None.
    //
    // Example:

    //     let result = spanda_cli::recovery_cli::cmd_recovery_report(args);

    let format = parse_format(args);
    let file = file_arg(args);
    let source = read_file(&file);
    let program = parse_program(&source);
    let config = device_registry_from_args(args);
    let device_registry = config.as_ref().map(|cfg| &cfg.device_registry);
    let report = evaluate_recovery(&program, None, device_registry);
    println!("{}", format_recovery(&report, format));
    if !report.passed {
        process::exit(1);
    }
}

/// `spanda recovery knowledge <file.sd> [--json]`
pub fn cmd_recovery_knowledge(args: &[String]) {
    // Description:
    //     Cmd recovery knowledge.
    //
    // Inputs:
    //     args: &[String]
    //         Caller-supplied args.
    //
    // Outputs:
    //     None.
    //
    // Example:

    //     let result = spanda_cli::recovery_cli::cmd_recovery_knowledge(args);

    let file = file_arg(args);
    let source = read_file(&file);
    let program = parse_program(&source);
    let kb = load_merged_recovery_knowledge(&program);
    if args.iter().any(|a| a == "--json") {
        println!("{}", serde_json::to_string_pretty(&kb).unwrap_or_default());
    } else {
        for entry in &kb.entries {
            println!(
                "{} -> {} ({:.0}% success)\n  {}",
                entry.failure_pattern,
                entry.recovery_pattern,
                entry.success_rate * 100.0,
                entry.recommendation
            );
        }
        if kb.entries.is_empty() {
            println!("No recovery knowledge entries.");
        }
    }
}

/// Dispatch `spanda recovery` subcommands.
pub fn recovery_dispatch(args: &[String]) {
    // Description:
    //     Recovery dispatch.
    //
    // Inputs:
    //     args: &[String]
    //         Caller-supplied args.
    //
    // Outputs:
    //     None.
    //
    // Example:

    //     let result = spanda_cli::recovery_cli::recovery_dispatch(args);

    match args.first().map(String::as_str).unwrap_or("") {
        "plan" => cmd_orchestrator_plan(&args[1..]),
        "simulate" => cmd_orchestrator_simulate(&args[1..]),
        "dry-run" => cmd_orchestrator_dry_run(&args[1..]),
        "execute" => cmd_orchestrator_execute(&args[1..]),
        "validate" => cmd_orchestrator_validate(&args[1..]),
        "history" => cmd_orchestrator_history(&args[1..]),
        "metrics" => cmd_orchestrator_metrics(&args[1..]),
        "graph" => cmd_orchestrator_graph(&args[1..]),
        "playbooks" => cmd_orchestrator_playbooks(&args[1..]),
        "explain" => cmd_orchestrator_explain(&args[1..]),
        "report" => cmd_recovery_report(&args[1..]),
        "knowledge" => cmd_recovery_knowledge(&args[1..]),
        _ => {
            eprintln!(
                "Usage: spanda recovery plan|simulate|dry-run|execute|validate|history|metrics|graph|playbooks|explain|report|knowledge ..."
            );
            process::exit(1);
        }
    }
}

/// Extended failure analysis with recovery planning (`--with-recovery`).
pub fn cmd_analyze_failure_recovery(args: &[String]) {
    // Description:
    //     Cmd analyze failure recovery.
    //
    // Inputs:
    //     args: &[String]
    //         Caller-supplied args.
    //
    // Outputs:
    //     None.
    //
    // Example:

    //     let result = spanda_cli::recovery_cli::cmd_analyze_failure_recovery(args);

    let file = file_arg(args);
    let source = read_file(&file);
    let program = parse_program(&source);
    let report = analyze_failure_with_recovery(&program);
    if args.iter().any(|a| a == "--json") {
        println!(
            "{}",
            serde_json::to_string_pretty(&report).unwrap_or_default()
        );
    } else {
        let mut out = format_failure_analysis(&report.failure);
        out.push_str("\nRecovery Plans:\n");
        for plan in &report.recovery_plans {
            out.push_str(&format!(
                "  Failure: {}\n  Impact: see above\n  Fallback: {}\n  Risk: {}\n",
                plan.failure,
                plan.actions
                    .first()
                    .map(|a| a.description.as_str())
                    .unwrap_or("none"),
                plan.risk
            ));
            for action in &plan.actions {
                out.push_str(&format!("    - {}\n", action.description));
            }
        }
        out.push_str(&format!("\nOverall Risk: {}\n", report.risk));
        println!("{out}");
    }
}
