//! CLI for mission resource estimation.
//!
use spanda_estimate::{estimate_mission, format_mission_estimate, EstimateFormat, EstimateOptions};
use spanda_lexer::tokenize;
use spanda_parser::parse;
use std::fs;
use std::path::Path;
use std::process;

fn parse_program(path: &Path) -> spanda_ast::nodes::Program {
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

fn file_arg(args: &[String]) -> String {
    let mut index = 0;
    while index < args.len() {
        match args[index].as_str() {
            "--target" => index += 2,
            "--json" => index += 1,
            other if !other.starts_with('-') => return other.to_string(),
            _ => index += 1,
        }
    }
    eprintln!("Usage: spanda estimate <file.sd> [--target <profile>] [--json]");
    process::exit(1);
}

fn parse_target(args: &[String]) -> Option<String> {
    for (index, arg) in args.iter().enumerate() {
        if arg == "--target" {
            return args.get(index + 1).cloned();
        }
    }
    None
}

/// `spanda estimate <file.sd> [--target <profile>] [--json]`
pub fn estimate_dispatch(args: &[String]) {
    let file = file_arg(args);
    let path = Path::new(&file);
    let program = parse_program(path);
    let report = estimate_mission(
        &program,
        &file,
        &EstimateOptions {
            target: parse_target(args),
        },
    );
    let format = if args.iter().any(|a| a == "--json") {
        EstimateFormat::Json
    } else {
        EstimateFormat::Text
    };
    println!("{}", format_mission_estimate(&report, format));
    if !report.within_budget {
        process::exit(1);
    }
}
