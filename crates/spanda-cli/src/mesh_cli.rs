//! CLI for Spanda Autonomous Entity Mesh commands.
//!
use spanda_config::{build_entity_registry, config_flag_from_args, ConfigResolver, SpandaManifest};
use spanda_entity_mesh::{
    apply_discovery, build_entity_mesh, build_merge_plan, compute_route,
    default_mesh_discovery_sources, discover_mesh_nodes, evaluate_mesh_health, find_capability,
    format_capability_results, format_health, format_merge_report, format_node_list, format_route,
    format_topology, inspect_node, list_nodes, merge_partitions, mesh_graph_json,
    parse_mesh_discovery_sources, simulate_partition, MeshFormat, MeshRouteOptions,
    MeshRoutingMode,
};
use std::env;
use std::path::{Path, PathBuf};
use std::process;

fn project_root(args: &[String]) -> PathBuf {
    if let Some(flag) = config_flag_from_args(args) {
        return flag.parent().unwrap_or(&flag).to_path_buf();
    }
    let cwd = env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    SpandaManifest::find_project_root(&cwd).unwrap_or(cwd)
}

fn load_resolved(root: &Path) -> spanda_config::ResolvedSystemConfig {
    ConfigResolver::new()
        .with_validation(false)
        .resolve_from_dir(root)
        .unwrap_or_else(|e| {
            eprintln!("error: failed to load mesh configuration: {e}");
            process::exit(1);
        })
}

fn json_output(args: &[String]) -> bool {
    args.iter().any(|a| a == "--json")
}

fn mesh_format(args: &[String]) -> MeshFormat {
    if json_output(args) {
        MeshFormat::Json
    } else {
        MeshFormat::Text
    }
}

fn capability_flag(args: &[String]) -> Option<String> {
    for (i, arg) in args.iter().enumerate() {
        if arg == "--capability" {
            return args.get(i + 1).cloned();
        }
    }
    None
}

fn entity_id_arg(args: &[String], usage: &str) -> String {
    args.iter()
        .find(|a| !a.starts_with('-'))
        .cloned()
        .unwrap_or_else(|| {
            eprintln!("{usage}");
            process::exit(1);
        })
}

fn load_mesh(args: &[String]) -> spanda_entity_mesh::EntityMesh {
    let root = project_root(args);
    let resolved = load_resolved(&root);
    let registry = build_entity_registry(&resolved);
    build_entity_mesh(&registry, "cli")
}

fn mesh_sources_from_args(args: &[String]) -> Vec<spanda_entity_mesh::MeshDiscoverySource> {
    let mut raw = Vec::new();
    let mut index = 0;
    while index < args.len() {
        if args[index] == "--source" {
            if let Some(value) = args.get(index + 1) {
                raw.push(value.clone());
                index += 2;
                continue;
            }
        }
        index += 1;
    }
    if raw.is_empty() {
        return default_mesh_discovery_sources();
    }
    let parsed = parse_mesh_discovery_sources(&raw);
    if parsed.is_empty() {
        default_mesh_discovery_sources()
    } else {
        parsed
    }
}

fn cmd_discover(args: &[String]) {
    let root = project_root(args);
    let resolved = load_resolved(&root);
    let registry = build_entity_registry(&resolved);
    let sources = mesh_sources_from_args(args);
    let result = discover_mesh_nodes(&registry, &sources);
    let mut mesh = build_entity_mesh(&registry, "cli");
    apply_discovery(&mut mesh, &result);
    if json_output(args) {
        println!(
            "{}",
            serde_json::to_string_pretty(&result).unwrap_or_default()
        );
    } else {
        println!(
            "Discovered {} entities ({} new)",
            result.discovered.len(),
            result.new_entities
        );
        for node in &result.discovered {
            println!("  {} — {}", node.entity_id, node.transport.as_str());
        }
    }
}

fn cmd_list(args: &[String]) {
    let mesh = load_mesh(args);
    let nodes = list_nodes(&mesh);
    print!("{}", format_node_list(&nodes, mesh_format(args)));
}

fn cmd_inspect(args: &[String]) {
    let mesh = load_mesh(args);
    let id = entity_id_arg(args, "Usage: spanda mesh inspect <entity-id> [--json]");
    match inspect_node(&mesh, &id) {
        Some(node) => {
            if json_output(args) {
                println!("{}", serde_json::to_string_pretty(node).unwrap_or_default());
            } else {
                println!("Entity: {}", node.entity_id);
                println!("  Node: {}", node.node_id);
                println!("  Transport: {}", node.transport.as_str());
                println!("  Reachable: {}", node.reachable);
                println!("  Trust: {:.2}", node.trust_score);
                println!("  Capabilities: {}", node.capabilities.join(", "));
                println!("  Neighbors: {}", node.neighbors.len());
            }
        }
        None => {
            eprintln!("Entity '{id}' not found in mesh");
            process::exit(1);
        }
    }
}

fn cmd_topology(args: &[String]) {
    let mesh = load_mesh(args);
    print!("{}", format_topology(&mesh.topology, mesh_format(args)));
}

fn cmd_graph(args: &[String]) {
    let mesh = load_mesh(args);
    if json_output(args) {
        println!(
            "{}",
            serde_json::to_string_pretty(&mesh_graph_json(&mesh)).unwrap_or_default()
        );
    } else {
        cmd_topology(args);
    }
}

fn cmd_health(args: &[String]) {
    let mesh = load_mesh(args);
    let health = evaluate_mesh_health(&mesh, &Default::default());
    print!("{}", format_health(&health, mesh_format(args)));
}

fn cmd_route(args: &[String]) {
    let mesh = load_mesh(args);
    let positional: Vec<_> = args.iter().filter(|a| !a.starts_with('-')).collect();
    if positional.len() < 2 {
        eprintln!("Usage: spanda mesh route <source> <target> [--json]");
        process::exit(1);
    }
    let route = compute_route(
        &mesh,
        positional[0],
        positional[1],
        &MeshRouteOptions {
            mode: Some(MeshRoutingMode::TrustWeighted),
            min_trust: 0.5,
            ..Default::default()
        },
    )
    .unwrap_or_else(|e| {
        eprintln!("Route failed: {e}");
        process::exit(1);
    });
    print!("{}", format_route(&route, mesh_format(args)));
}

fn cmd_find_capability(args: &[String]) {
    let mesh = load_mesh(args);
    let cap = capability_flag(args).unwrap_or_else(|| {
        eprintln!("Usage: spanda mesh find --capability <name> [--json]");
        process::exit(1);
    });
    let results = find_capability(&mesh, &cap);
    print!("{}", format_capability_results(&results, mesh_format(args)));
}

fn cmd_capabilities(args: &[String]) {
    let mesh = load_mesh(args);
    if json_output(args) {
        println!(
            "{}",
            serde_json::to_string_pretty(&mesh.capability_ads).unwrap_or_default()
        );
    } else {
        for ad in &mesh.capability_ads {
            println!("{}: {}", ad.entity_id, ad.capabilities.join(", "));
        }
    }
}

fn cmd_simulate_partition(args: &[String]) {
    let mut mesh = load_mesh(args);
    let ids: Vec<String> = args
        .iter()
        .filter(|a| !a.starts_with('-'))
        .cloned()
        .collect();
    if ids.is_empty() {
        eprintln!("Usage: spanda mesh simulate-partition <entity-id> [...] [--json]");
        process::exit(1);
    }
    let report = simulate_partition(&mut mesh, &ids);
    if json_output(args) {
        println!(
            "{}",
            serde_json::to_string_pretty(&report).unwrap_or_default()
        );
    } else {
        println!("Partition simulated affecting: {}", ids.join(", "));
        println!("Local coordinator: {:?}", report.local_coordinator);
    }
}

fn cmd_merge_report(args: &[String]) {
    let mesh = load_mesh(args);
    let plan = build_merge_plan(
        &mesh,
        &mesh
            .partitions
            .iter()
            .filter(|p| p.active)
            .map(|p| p.partition_id.clone())
            .collect::<Vec<_>>(),
    );
    let report = merge_partitions(&mut { mesh.clone() }, &plan);
    print!("{}", format_merge_report(&report, mesh_format(args)));
}

/// Dispatch `spanda mesh` subcommands.
pub fn mesh_dispatch(args: &[String]) {
    let sub = args.first().map(String::as_str).unwrap_or("");
    match sub {
        "discover" => cmd_discover(&args[1..]),
        "list" => cmd_list(&args[1..]),
        "inspect" => cmd_inspect(&args[1..]),
        "topology" => cmd_topology(&args[1..]),
        "graph" => cmd_graph(&args[1..]),
        "health" => cmd_health(&args[1..]),
        "route" => cmd_route(&args[1..]),
        "find" => cmd_find_capability(&args[1..]),
        "capabilities" => cmd_capabilities(&args[1..]),
        "simulate-partition" => cmd_simulate_partition(&args[1..]),
        "merge-report" => cmd_merge_report(&args[1..]),
        "" | "help" | "--help" | "-h" => print_usage(),
        other => {
            eprintln!("Unknown mesh subcommand: {other}");
            print_usage();
            process::exit(1);
        }
    }
}

fn print_usage() {
    eprintln!(
        "Spanda Autonomous Entity Mesh\n\
         \n\
         Usage:\n\
           spanda mesh discover [--source <name>]... [--json] [--config <spanda.toml>]\n\
           spanda mesh list [--json]\n\
           spanda mesh inspect <entity-id> [--json]\n\
           spanda mesh topology [--json]\n\
           spanda mesh graph [--json]\n\
           spanda mesh health [--json]\n\
           spanda mesh route <source> <target> [--json]\n\
           spanda mesh find --capability <name> [--json]\n\
           spanda mesh capabilities [--json]\n\
           spanda mesh simulate-partition <entity-id> [...] [--json]\n\
           spanda mesh merge-report [--json]\n\
         \n\
         Entity Mesh sits above transports — it is not packet routing.\n\
         See docs/entity-mesh.md"
    );
}
