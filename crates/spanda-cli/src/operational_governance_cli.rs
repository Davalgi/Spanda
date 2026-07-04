//! CLI for operational governance — compliance check, governance validate/report, deployment profiles.
//!
use spanda_config::{ConfigResolver, SpandaManifest};
use spanda_governance::{
    default_certification_store_path, deployment_profile_by_name, format_certification_report,
    format_compliance_report, format_deployment_verify, format_governance_report,
    format_governance_validation, governance_framework_summary, list_deployment_profiles,
    run_compliance_check, validate_governance, verify_deployment, CertificationReport,
    CertificationRecord, CertificationStatus, CertificationStore, ValidationOptions,
};
use std::env;
use std::process;

fn load_registry() -> (spanda_config::EntityRegistry, spanda_config::ResolvedSystemConfig) {
    let cwd = env::current_dir().unwrap_or_else(|_| ".".into());
    let root = SpandaManifest::find_project_root(&cwd).unwrap_or(cwd);
    let resolved = ConfigResolver::new()
        .resolve_from_dir(&root)
        .unwrap_or_else(|e| {
            eprintln!("Failed to resolve config: {e}");
            process::exit(1);
        });
    let registry = resolved.entity_registry();
    (registry, resolved)
}

fn parse_opts(args: &[String]) -> ValidationOptions {
    let mut opts = ValidationOptions::default();
    let mut index = 0;
    while index < args.len() {
        match args[index].as_str() {
            "--strict" => opts.strict = true,
            "--json" => {}
            "--entity" | "--entity-id" => {
                index += 1;
                if index < args.len() {
                    opts.entity_id = Some(args[index].clone());
                }
            }
            _ => {}
        }
        index += 1;
    }
    opts
}

fn wants_json(args: &[String]) -> bool {
    args.iter().any(|a| a == "--json")
}

pub fn compliance_check_dispatch(args: &[String]) {
    let opts = parse_opts(args);
    let json = wants_json(args);
    let (registry, resolved) = load_registry();
    let report = run_compliance_check(&registry, &resolved, &opts);
    println!("{}", format_compliance_report(&report, json));
    if !report.passed {
        process::exit(1);
    }
}

pub fn governance_validate_dispatch(args: &[String]) {
    let opts = parse_opts(args);
    let json = wants_json(args);
    let (registry, resolved) = load_registry();
    let validation = validate_governance(&registry, &resolved, &opts);
    println!("{}", format_governance_validation(&validation, json));
    if !validation.passed {
        process::exit(1);
    }
}

pub fn governance_report_dispatch(args: &[String]) {
    let opts = parse_opts(args);
    let json = wants_json(args);
    let (registry, resolved) = load_registry();
    let compliance = run_compliance_check(&registry, &resolved, &opts);
    let validation = validate_governance(&registry, &resolved, &opts);
    println!("{}", format_governance_report(&compliance, &validation, json));
    if !compliance.passed || !validation.passed {
        process::exit(1);
    }
}

pub fn deployment_profile_dispatch(args: &[String]) {
    let json = wants_json(args);
    let name = args
        .iter()
        .find(|a| !a.starts_with('-'))
        .map(String::as_str);

    if let Some(name) = name {
        let profile = deployment_profile_by_name(name).unwrap_or_else(|| {
            eprintln!("Unknown deployment profile '{name}'");
            process::exit(1);
        });
        if json {
            println!("{}", serde_json::to_string_pretty(&profile).unwrap_or_default());
        } else {
            println!("Deployment Profile: {}", profile.display_name.as_deref().unwrap_or(name));
            println!("Kind: {}", profile.kind.as_str());
            println!("Default Risk: {}", profile.default_risk_level.as_str());
            println!(
                "Max Autonomy: {}",
                profile.decision_authority.max_autonomy_level.as_str()
            );
            if !profile.required_capabilities.is_empty() {
                println!("Required Capabilities: {}", profile.required_capabilities.join(", "));
            }
            if !profile.required_certifications.is_empty() {
                println!(
                    "Required Certifications: {}",
                    profile.required_certifications.join(", ")
                );
            }
        }
        return;
    }

    let profiles = list_deployment_profiles();
    if json {
        println!("{}", serde_json::to_string_pretty(&profiles).unwrap_or_default());
    } else {
        println!("Deployment Profiles:");
        for profile in profiles {
            println!(
                "  {} — risk: {}, max autonomy: {}",
                profile.kind.as_str(),
                profile.default_risk_level.as_str(),
                profile.decision_authority.max_autonomy_level.as_str()
            );
        }
    }
}

pub fn governance_framework_dispatch(args: &[String]) {
    let json = wants_json(args);
    let summary = governance_framework_summary();
    if json {
        println!("{}", serde_json::to_string_pretty(&summary).unwrap_or_default());
    } else {
        println!("Operational Governance Framework");
        println!("================================");
        println!("Capabilities: standards awareness, compliance validation, deployment governance,");
        println!("certification tracking, risk assessment, audit support, human accountability");
        println!();
        println!("Disclaimer: Spanda provides governance abstractions — not legal or regulatory advice.");
    }
}

pub fn certification_list_dispatch(args: &[String]) {
    let json = wants_json(args);
    let (registry, _) = load_registry();
    let certs: Vec<serde_json::Value> = registry
        .entities
        .values()
        .filter_map(|entity| {
            entity.governance.as_ref().and_then(|gov| {
                gov.certification_status.as_ref().map(|status| {
                    serde_json::json!({
                        "entity_id": entity.id,
                        "certification_status": status,
                        "deployment_profile": gov.deployment_profile,
                        "operational_maturity": gov.operational_maturity,
                    })
                })
            })
        })
        .collect();
    if json {
        println!("{}", serde_json::to_string_pretty(&certs).unwrap_or_default());
    } else if certs.is_empty() {
        println!("No certification records found in entity registry.");
    } else {
        println!("Certifications:");
        for cert in certs {
            println!(
                "  {} — {}",
                cert["entity_id"].as_str().unwrap_or("-"),
                cert["certification_status"].as_str().unwrap_or("-")
            );
        }
    }
}

pub fn certification_inspect_dispatch(args: &[String]) {
    let json = wants_json(args);
    let entity_id = args
        .iter()
        .find(|a| !a.starts_with('-'))
        .unwrap_or_else(|| {
            eprintln!("Usage: spanda certification inspect <entity-id> [--json]");
            process::exit(1);
        });
    let (registry, _) = load_registry();
    let Some(entity) = registry.get(entity_id) else {
        eprintln!("Entity '{entity_id}' not found");
        process::exit(1);
    };
    if json {
        println!("{}", serde_json::to_string_pretty(entity).unwrap_or_default());
    } else {
        println!("Entity: {}", entity.id);
        println!("Type: {}", entity.entity_type.as_str());
        if let Some(gov) = entity.governance.as_ref() {
            println!("Autonomy Level: {}", gov.autonomy_level.as_deref().unwrap_or("-"));
            println!("Deployment Profile: {}", gov.deployment_profile.as_deref().unwrap_or("-"));
            println!("Operational Maturity: {}", gov.operational_maturity.as_deref().unwrap_or("-"));
            println!("Certification Status: {}", gov.certification_status.as_deref().unwrap_or("-"));
            println!("Risk Level: {}", gov.risk_level.as_deref().unwrap_or("-"));
            println!("Responsible Person: {}", gov.responsible_person.as_deref().unwrap_or("-"));
            println!("Deployment Owner: {}", gov.deployment_owner.as_deref().unwrap_or("-"));
            println!("Mission Owner: {}", gov.mission_owner.as_deref().unwrap_or("-"));
            println!("Emergency Contact: {}", gov.emergency_contact.as_deref().unwrap_or("-"));
            println!("Escalation Contact: {}", gov.escalation_contact.as_deref().unwrap_or("-"));
            if !gov.approval_chain.is_empty() {
                println!("Approval Chain: {}", gov.approval_chain.join(" → "));
            }
        } else {
            println!("No governance metadata configured.");
        }
    }
}

pub fn risk_report_dispatch(args: &[String]) {
    let json = wants_json(args);
    let (registry, _) = load_registry();
    let risks: Vec<serde_json::Value> = registry
        .entities
        .values()
        .filter_map(|entity| {
            entity.governance.as_ref().and_then(|gov| {
                gov.risk_level.as_ref().map(|risk| {
                    serde_json::json!({
                        "entity_id": entity.id,
                        "risk_level": risk,
                        "autonomy_level": gov.autonomy_level,
                        "deployment_profile": gov.deployment_profile,
                        "health_status": entity.health_status,
                    })
                })
            })
        })
        .collect();
    if json {
        println!("{}", serde_json::to_string_pretty(&risks).unwrap_or_default());
    } else if risks.is_empty() {
        println!("No risk metadata found in entity registry.");
    } else {
        println!("Operational Risk Report:");
        for row in risks {
            println!(
                "  {} — risk: {}, health: {:?}",
                row["entity_id"].as_str().unwrap_or("-"),
                row["risk_level"].as_str().unwrap_or("-"),
                row["health_status"]
            );
        }
    }
}

pub fn certification_report_dispatch(args: &[String]) {
    let json = wants_json(args);
    let entity_id = args.iter().find(|a| !a.starts_with('-')).map(String::as_str);
    let (registry, _) = load_registry();
    let mut store = CertificationStore::load(&default_certification_store_path());
    for entity in registry.entities.values() {
        let Some(gov) = entity.governance.as_ref() else {
            continue;
        };
        let Some(status) = gov.certification_status.as_ref() else {
            continue;
        };
        let id = format!("cert-{}", entity.id);
        if store.get(&id).is_some() {
            continue;
        }
        let mut record = CertificationRecord::draft(&id);
        record.status = CertificationStatus::parse(status);
        record.applicable_scope = vec![entity.id.clone()];
        record.certified_by = gov.responsible_person.clone();
        store.upsert(record);
    }
    let report = CertificationReport::from_store(&store, entity_id);
    println!("{}", format_certification_report(&report, json));
}

pub fn deployment_verify_dispatch(args: &[String]) {
    let opts = parse_opts(args);
    let json = wants_json(args);
    let (registry, resolved) = load_registry();
    let report = verify_deployment(&registry, &resolved, &opts);
    println!("{}", format_deployment_verify(&report, json));
    if !report.passed {
        process::exit(1);
    }
}
