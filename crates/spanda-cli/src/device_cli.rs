//! CLI commands for device discovery and inspection.

use spanda_config::{
    discover_matches, generate_report_bundle, scan_subnet, ConfigResolver, SpandaManifest,
};
use std::env;
use std::path::PathBuf;
use std::process;

fn project_root(args: &[String]) -> PathBuf {
    for (i, arg) in args.iter().enumerate() {
        if arg == "--config" {
            if let Some(path) = args.get(i + 1) {
                let p = PathBuf::from(path);
                return p.parent().unwrap_or(&p).to_path_buf();
            }
        }
    }
    let cwd = env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    SpandaManifest::find_project_root(&cwd).unwrap_or(cwd)
}

fn load_resolved(root: &PathBuf) -> spanda_config::ResolvedSystemConfig {
    ConfigResolver::new()
        .with_validation(false)
        .resolve_from_dir(root)
        .unwrap_or_else(|e| {
            eprintln!("{e}");
            process::exit(1);
        })
}

/// Dispatch `spanda device` subcommands.
pub fn device_dispatch(args: &[String]) {
    let sub = args.first().map(String::as_str).unwrap_or("");
    match sub {
        "discover" => cmd_discover(&args[1..]),
        "inspect" => cmd_inspect(&args[1..]),
        _ => {
            eprintln!(
                "Usage:\n  \
                 spanda device discover [--subnet CIDR] [--json] [--config <spanda.toml>]\n  \
                 spanda device inspect <id> [--json] [--config <spanda.toml>]"
            );
            process::exit(1);
        }
    }
}

fn cmd_discover(args: &[String]) {
    let json = args.iter().any(|a| a == "--json");
    let mut subnet: Option<String> = None;
    let mut i = 0;
    while i < args.len() {
        if args[i] == "--subnet" {
            subnet = args.get(i + 1).cloned();
            i += 2;
            continue;
        }
        i += 1;
    }
    let root = project_root(args);
    let resolved = load_resolved(&root);
    let bundle = generate_report_bundle(&resolved);
    let probes = subnet
        .as_deref()
        .map(|cidr| scan_subnet(cidr, &[], 200))
        .unwrap_or_default();
    let matches = discover_matches(&resolved.device_registry, &probes);
    if json {
        println!(
            "{}",
            serde_json::to_string_pretty(&serde_json::json!({
                "devices": resolved.device_registry.devices,
                "network": bundle.network,
                "probes": probes,
                "matches": matches,
            }))
            .unwrap()
        );
        return;
    }
    println!(
        "Configured devices: {}",
        resolved.device_registry.devices.len()
    );
    for device in &resolved.device_registry.devices {
        println!(
            "  {} logical={:?} ip={:?} mac={:?} provider={:?}",
            device.id, device.logical_name, device.ip_address, device.mac_address, device.provider
        );
    }
    if let Some(ref cidr) = subnet {
        println!("\nScan {cidr}: {} hosts probed", probes.len());
        for probe in probes.iter().filter(|p| p.reachable) {
            println!("  reachable {} ports={:?}", probe.ip, probe.open_ports);
        }
        if !matches.is_empty() {
            println!("\nMatched configured devices:");
            for m in &matches {
                println!("  {} <= {}", m.device_id, m.configured_ip);
            }
        }
    }
}

fn cmd_inspect(args: &[String]) {
    let json = args.iter().any(|a| a == "--json");
    let device_id = args
        .iter()
        .find(|a| !a.starts_with('-'))
        .cloned()
        .unwrap_or_else(|| {
            eprintln!("Missing device id");
            process::exit(1);
        });
    let root = project_root(args);
    let resolved = load_resolved(&root);
    let device = resolved.device_registry.get(&device_id).unwrap_or_else(|| {
        eprintln!("Device '{device_id}' not found");
        process::exit(1);
    });
    if json {
        println!("{}", serde_json::to_string_pretty(device).unwrap());
    } else {
        println!("Device: {}", device.id);
        if let Some(ref logical) = device.logical_name {
            println!("Logical name: {logical}");
        }
        println!("Type: {}", device.device_type);
        if let Some(ref provider) = device.provider {
            println!("Provider: {provider}");
        }
        if let Some(ref ip) = device.ip_address {
            println!("IP: {ip}");
        }
        if let Some(ref mac) = device.mac_address {
            println!("MAC: {mac}");
        }
        if let Some(ref endpoint) = device.endpoint_url {
            println!("Endpoint: {endpoint}");
        }
        if let Some(ref protocol) = device.protocol {
            println!("Protocol: {protocol}");
        }
        if let Some(ref trust) = device.trust_level {
            println!("Trust: {trust}");
        }
        if !device.capabilities.is_empty() {
            println!("Capabilities: {}", device.capabilities.join(", "));
        }
    }
}
