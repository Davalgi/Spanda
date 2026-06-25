//! CLI commands for decision audit trails.

use spanda_decision::{audit_decisions_from_trace, format_decision_audit, format_decision_explanations};
use std::process;

fn file_arg(args: &[String]) -> String {
    args.iter()
        .find(|a| !a.starts_with('-'))
        .cloned()
        .unwrap_or_else(|| {
            eprintln!("Missing trace path");
            process::exit(1);
        })
}

/// `spanda audit decisions <mission.trace> [--json]`
pub fn cmd_audit_decisions(args: &[String]) {
    let json = args.iter().any(|a| a == "--json");
    let explain = args.iter().any(|a| a == "--explain");
    let file = file_arg(args);
    match audit_decisions_from_trace(&file) {
        Ok(report) => {
            if explain {
                println!("{}", format_decision_explanations(&report));
            } else {
                println!("{}", format_decision_audit(&report, json));
            }
        }
        Err(error) => {
            eprintln!("{error}");
            process::exit(1);
        }
    }
}

/// Dispatch `spanda audit decisions` (used from main.rs).
pub fn audit_dispatch(args: &[String]) {
    let sub = args.first().map(String::as_str).unwrap_or("");
    match sub {
        "decisions" => cmd_audit_decisions(&args[1..]),
        _ => {
            eprintln!("Usage:\n  spanda audit decisions <mission.trace> [--json] [--explain]");
            process::exit(1);
        }
    }
}
