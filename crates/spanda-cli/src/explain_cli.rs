//! CLI commands for explainability reports.

use spanda_explain::{
    explain_program, explain_readiness, explain_safety, explain_trace, explain_verify,
    format_explain_report,
};
use spanda_lexer::tokenize;
use spanda_parser::parse;
use std::fs;
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

fn file_arg(args: &[String]) -> String {
    args.iter()
        .find(|a| !a.starts_with('-'))
        .cloned()
        .unwrap_or_else(|| {
            eprintln!("Missing file path");
            process::exit(1);
        })
}

fn json_flag(args: &[String]) -> bool {
    args.iter().any(|a| a == "--json")
}

/// `spanda explain <file.sd> [--json]`
pub fn cmd_explain_program(args: &[String]) {
    let file = file_arg(args);
    let source = read_file(&file);
    let program = parse_program(&source);
    let report = explain_program(&program, &file);
    println!("{}", format_explain_report(&report, json_flag(args)));
}

/// `spanda explain readiness --file <file.sd> [--json]`
pub fn cmd_explain_readiness(args: &[String]) {
    let file = args
        .windows(2)
        .find(|w| w[0] == "--file")
        .map(|w| w[1].clone())
        .or_else(|| args.iter().find(|a| !a.starts_with('-')).cloned())
        .unwrap_or_else(|| {
            eprintln!("Missing --file <path>");
            process::exit(1);
        });
    let source = read_file(&file);
    let program = parse_program(&source);
    let report = explain_readiness(&program, &file);
    println!("{}", format_explain_report(&report, json_flag(args)));
}

/// `spanda explain verify --file <file.sd> [--json]`
pub fn cmd_explain_verify(args: &[String]) {
    let file = args
        .windows(2)
        .find(|w| w[0] == "--file")
        .map(|w| w[1].clone())
        .or_else(|| args.iter().find(|a| !a.starts_with('-')).cloned())
        .unwrap_or_else(|| {
            eprintln!("Missing --file <path>");
            process::exit(1);
        });
    let source = read_file(&file);
    let program = parse_program(&source);
    let report = explain_verify(&program, &file);
    println!("{}", format_explain_report(&report, json_flag(args)));
}

/// `spanda explain safety --file <file.sd> [--json]`
pub fn cmd_explain_safety(args: &[String]) {
    let file = args
        .windows(2)
        .find(|w| w[0] == "--file")
        .map(|w| w[1].clone())
        .or_else(|| args.iter().find(|a| !a.starts_with('-')).cloned())
        .unwrap_or_else(|| {
            eprintln!("Missing --file <path>");
            process::exit(1);
        });
    let source = read_file(&file);
    let program = parse_program(&source);
    let report = explain_safety(&program, &file);
    println!("{}", format_explain_report(&report, json_flag(args)));
}

/// `spanda explain <trace> [--json]` when path ends with `.trace`
pub fn cmd_explain_trace(args: &[String]) {
    let file = file_arg(args);
    match explain_trace(&file) {
        Ok(report) => println!("{}", format_explain_report(&report, json_flag(args))),
        Err(error) => {
            eprintln!("{error}");
            process::exit(1);
        }
    }
}

/// Dispatch `spanda explain` subcommands.
pub fn explain_dispatch(args: &[String]) {
    let sub = args.first().map(String::as_str).unwrap_or("");
    match sub {
        "readiness" => cmd_explain_readiness(&args[1..]),
        "verify" => cmd_explain_verify(&args[1..]),
        "safety" => cmd_explain_safety(&args[1..]),
        "" => {
            eprintln!(
                "Usage:\n  spanda explain <file.sd> [--json]\n  spanda explain readiness|verify|safety --file <file.sd> [--json]\n  spanda explain <mission.trace> [--json]"
            );
            process::exit(1);
        }
        other if other.ends_with(".trace") || other.ends_with(".json") => {
            cmd_explain_trace(args);
        }
        other if other.ends_with(".sd") => cmd_explain_program(args),
        _ => {
            eprintln!(
                "Usage:\n  spanda explain <file.sd> [--json]\n  spanda explain readiness|verify|safety --file <file.sd> [--json]\n  spanda explain <mission.trace> [--json]"
            );
            process::exit(1);
        }
    }
}
