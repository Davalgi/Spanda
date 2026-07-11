//! CLI commands for maintenance windows and sleep-mode scheduling.
//!
use spanda_autonomy::{
    list_maintenance_windows, set_maintenance_window, MaintenanceWindow,
};
use spanda_autonomy::format::format_report;
use spanda_autonomy::types::AutonomyReportFormat;
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

/// Dispatch `spanda maintenance` subcommands.
pub fn maintenance_dispatch(args: &[String]) {
    // Route maintenance CLI verbs to window list/set handlers.
    //
    // Parameters:
    // - `args` — subcommand tokens after `spanda maintenance`
    //
    // Returns:
    // Nothing; exits on usage errors.
    //
    // Options:
    // None.
    //
    // Example:
    // maintenance_dispatch(&["window", "list"]);

    match args.first().map(String::as_str).unwrap_or("") {
        "window" => window_dispatch(&args[1..]),
        _ => {
            eprintln!("Usage: spanda maintenance window {{list|set}} ...");
            process::exit(1);
        }
    }
}

fn window_dispatch(args: &[String]) {
    match args.first().map(String::as_str).unwrap_or("") {
        "list" => cmd_window_list(args),
        "set" => cmd_window_set(args),
        _ => {
            eprintln!(
                "Usage: spanda maintenance window list [--json]\n       spanda maintenance window set --id ID --start ISO --end ISO [--activity NAME]..."
            );
            process::exit(1);
        }
    }
}

fn cmd_window_list(args: &[String]) {
    // Print the scheduled maintenance windows.
    //
    // Parameters:
    // - `args` — optional `--json` / `--markdown`
    //
    // Returns:
    // Nothing; prints the schedule.
    //
    // Options:
    // Format flags.
    //
    // Example:
    // spanda maintenance window list --json

    let format = parse_format(args);
    let windows = list_maintenance_windows();
    println!("{}", format_report(&windows, format));
}

fn flag_value<'a>(args: &'a [String], name: &str) -> Option<&'a str> {
    args.windows(2).find_map(|pair| {
        if pair[0] == name {
            Some(pair[1].as_str())
        } else {
            None
        }
    })
}

fn cmd_window_set(args: &[String]) {
    // Upsert a maintenance window into the persistent schedule.
    //
    // Parameters:
    // - `args` — `--id`, `--start`, `--end`, optional repeated `--activity`
    //
    // Returns:
    // Nothing; prints the saved window.
    //
    // Options:
    // `--json` for machine-readable output.
    //
    // Example:
    // spanda maintenance window set --id nightly --start 2026-07-12T02:00:00Z --end 2026-07-12T04:00:00Z --activity ota

    let format = parse_format(args);
    let id = flag_value(args, "--id").unwrap_or("");
    let start = flag_value(args, "--start").unwrap_or("");
    let end = flag_value(args, "--end").unwrap_or("");
    if id.is_empty() || start.is_empty() || end.is_empty() {
        eprintln!(
            "Usage: spanda maintenance window set --id ID --start ISO --end ISO [--activity NAME]..."
        );
        process::exit(1);
    }
    let activities: Vec<String> = args
        .windows(2)
        .filter(|pair| pair[0] == "--activity")
        .map(|pair| pair[1].clone())
        .collect();
    let saved = set_maintenance_window(MaintenanceWindow {
        id: id.into(),
        start: start.into(),
        end: end.into(),
        activities,
    });
    println!("{}", format_report(&saved, format));
}
