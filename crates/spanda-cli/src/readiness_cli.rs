//! CLI commands for operational readiness and mission assurance.

use spanda_lexer::tokenize;
use spanda_parser::parse;
use spanda_readiness::{
    analyze_failure, audit_program, diagnose_trace, evaluate_fleet_readiness, evaluate_readiness,
    evaluate_twin_readiness, format_audit, format_failure_analysis, format_fleet_readiness,
    format_mission_verification, format_readiness, format_root_cause, format_safety_report,
    generate_safety_report, verify_approvals, verify_fleet, verify_mission, ReadinessOptions,
    ReportFormat,
};
use std::fs;
use std::path::Path;
use std::process;

fn read_file(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|e| {
        eprintln!("Failed to read {path}: {e}");
        process::exit(1);
    })
}

fn parse_program(source: &str) -> spanda_ast::nodes::Program {
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

fn file_from_args(args: &[String]) -> String {
    args.iter()
        .find(|a| !a.starts_with('-'))
        .cloned()
        .unwrap_or_else(|| {
            eprintln!("Missing file path");
            process::exit(1);
        })
}

/// `spanda readiness <file.sd> [--json|--markdown|--html]`
pub fn cmd_readiness(args: &[String]) {
    let format = parse_format(args);
    let file = file_from_args(args);
    let source = read_file(&file);
    let program = parse_program(&source);
    let report = evaluate_readiness(&program, &ReadinessOptions::default());
    println!("{}", format_readiness(&report, format));
    if !report.mission_ready {
        process::exit(1);
    }
}

/// `spanda verify mission` — mission verification (also used when verify targets mission file).
pub fn cmd_verify_mission(args: &[String]) {
    let json = args.iter().any(|a| a == "--json");
    let file = file_from_args(args);
    let source = read_file(&file);
    let program = parse_program(&source);
    let reports = verify_mission(&program, None);
    if json {
        println!("{}", serde_json::to_string_pretty(&reports).unwrap());
    } else {
        print!("{}", format_mission_verification(&reports));
    }
    if reports.iter().any(|r| !r.achievable) {
        process::exit(1);
    }
}

/// `spanda analyze-failure <file.sd>`
pub fn cmd_analyze_failure(args: &[String]) {
    let json = args.iter().any(|a| a == "--json");
    let file = file_from_args(args);
    let source = read_file(&file);
    let program = parse_program(&source);
    let report = analyze_failure(&program);
    if json {
        println!("{}", serde_json::to_string_pretty(&report).unwrap());
    } else {
        print!("{}", format_failure_analysis(&report));
    }
}

/// `spanda safety-report <file.sd> [--json|--markdown|--html]`
pub fn cmd_safety_report(args: &[String]) {
    let format = parse_format(args);
    let file = file_from_args(args);
    let source = read_file(&file);
    let program = parse_program(&source);
    let report = generate_safety_report(&program, &file);
    println!("{}", format_safety_report(&report, format));
    if !report.deployable {
        process::exit(1);
    }
}

/// `spanda twin readiness <file.sd> [--trace <path>]`
pub fn cmd_twin_readiness(args: &[String]) {
    let json = args.iter().any(|a| a == "--json");
    let file = file_from_args(args);
    let trace_path = args
        .windows(2)
        .find(|w| w[0] == "--trace")
        .map(|w| w[1].clone());
    let source = read_file(&file);
    let program = parse_program(&source);
    let status = evaluate_twin_readiness(&program, trace_path.as_deref().map(Path::new));
    if json {
        println!("{}", serde_json::to_string_pretty(&status).unwrap());
    } else {
        print!("{}", spanda_readiness::format_twin_readiness(&status));
    }
}

/// `spanda fleet readiness <file.sd>`
pub fn cmd_fleet_readiness(args: &[String]) {
    let json = args.iter().any(|a| a == "--json");
    let file = file_from_args(args);
    let source = read_file(&file);
    let program = parse_program(&source);
    let report = evaluate_fleet_readiness(&program, &ReadinessOptions::default());
    if json {
        println!("{}", serde_json::to_string_pretty(&report).unwrap());
    } else {
        print!("{}", format_fleet_readiness(&report));
    }
}

/// `spanda diagnose <trace>`
pub fn cmd_diagnose(args: &[String]) {
    let json = args.iter().any(|a| a == "--json");
    let file = file_from_args(args);
    let report = diagnose_trace(Path::new(&file)).unwrap_or_else(|e| {
        eprintln!("{e}");
        process::exit(1);
    });
    if json {
        println!("{}", serde_json::to_string_pretty(&report).unwrap());
    } else {
        print!("{}", format_root_cause(&report));
    }
}

/// `spanda audit <file.sd>`
pub fn cmd_audit(args: &[String]) {
    let json = args.iter().any(|a| a == "--json");
    let file = file_from_args(args);
    let source = read_file(&file);
    let program = parse_program(&source);
    let report = audit_program(&program, &source);
    if json {
        println!("{}", serde_json::to_string_pretty(&report).unwrap());
    } else {
        print!("{}", format_audit(&report));
    }
    if report.critical_count > 0 {
        process::exit(1);
    }
}

/// `spanda verify-fleet <file.sd>`
pub fn cmd_verify_fleet(args: &[String]) {
    let json = args.iter().any(|a| a == "--json");
    let file = file_from_args(args);
    let source = read_file(&file);
    let program = parse_program(&source);
    let report = verify_fleet(&program);
    if json {
        println!("{}", serde_json::to_string_pretty(&report).unwrap());
    } else {
        for f in &report.findings {
            println!("[{}] {} — {}", f.severity, f.category, f.message);
        }
    }
    if !report.compatible {
        process::exit(1);
    }
}

/// `spanda verify-approval <file.sd>` (internal) or part of verify extensions.
pub fn cmd_verify_approval(args: &[String]) {
    let json = args.iter().any(|a| a == "--json");
    let file = file_from_args(args);
    let source = read_file(&file);
    let program = parse_program(&source);
    let report = verify_approvals(&program);
    if json {
        println!("{}", serde_json::to_string_pretty(&report).unwrap());
    } else {
        for row in &report.rows {
            println!(
                "{} / {} — path:{} actor:{} fallback:{} [{}]",
                row.actor,
                row.action,
                row.approval_path_exists,
                row.actor_exists,
                row.fallback_exists,
                row.status
            );
        }
    }
    if !report.compatible {
        process::exit(1);
    }
}

/// Top-level readiness dispatch for subcommands.
pub fn readiness_dispatch(args: &[String]) {
    if args.is_empty() {
        eprintln!("Usage: spanda readiness <file.sd> [--json|--markdown|--html]");
        process::exit(1);
    }
    cmd_readiness(args);
}
