//! CLI commands for mission contract verification.

use spanda_contract::{format_contract_report, verify_contract};
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

/// `spanda contract verify <file.sd> [--json]`
pub fn cmd_contract_verify(args: &[String]) {
    let json = args.iter().any(|a| a == "--json");
    let file = file_arg(args);
    let source = read_file(&file);
    let program = parse_program(&source);
    let report = verify_contract(&program, &file);
    println!("{}", format_contract_report(&report, json));
    if !report.passed {
        process::exit(1);
    }
}

/// Dispatch `spanda contract` subcommands.
pub fn contract_dispatch(args: &[String]) {
    let sub = args.first().map(String::as_str).unwrap_or("");
    match sub {
        "verify" => cmd_contract_verify(&args[1..]),
        _ => {
            eprintln!(
                "Usage:\n  spanda contract verify <file.sd> [--json]"
            );
            process::exit(1);
        }
    }
}
