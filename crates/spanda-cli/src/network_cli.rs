//! Network scanning CLI for device discovery.

use spanda_config::scan_subnet;
use std::process;

/// `spanda network scan --subnet <CIDR> [--json] [--ports 80,443,554]`
pub fn cmd_network_scan(args: &[String]) {
    let json = args.iter().any(|a| a == "--json");
    let mut subnet: Option<String> = None;
    let mut ports: Vec<u16> = Vec::new();
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--subnet" => {
                subnet = args.get(i + 1).cloned();
                i += 2;
            }
            "--ports" => {
                if let Some(raw) = args.get(i + 1) {
                    ports = raw
                        .split(',')
                        .filter_map(|p| p.trim().parse().ok())
                        .collect();
                }
                i += 2;
            }
            _ => i += 1,
        }
    }
    let Some(cidr) = subnet else {
        eprintln!(
            "Usage: spanda network scan --subnet 192.168.1.0/24 [--json] [--ports 80,443,554]"
        );
        process::exit(1);
    };
    let results = scan_subnet(&cidr, &ports, 250);
    if json {
        println!("{}", serde_json::to_string_pretty(&results).unwrap());
        return;
    }
    let reachable = results.iter().filter(|r| r.reachable).count();
    println!("Scan {cidr}: {reachable}/{} hosts reachable", results.len());
    for host in results.iter().filter(|r| r.reachable) {
        println!(
            "  {} ports={:?} latency={:?}ms",
            host.ip, host.open_ports, host.latency_ms
        );
    }
}
