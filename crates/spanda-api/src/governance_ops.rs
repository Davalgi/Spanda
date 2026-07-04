//! Operational governance REST handlers — compliance, certification, risk, deployment profiles.
//!
use crate::handlers::{bad_request, json_ok, parse_query, unauthorized};
use crate::state::ControlCenterState;
use serde::Deserialize;
use spanda_deploy_http::HttpResponse;
use spanda_governance::{
    default_certification_store_path, default_policy_store_path, deployment_profile_by_name,
    format_certification_report, format_compliance_report, format_deployment_verify,
    format_governance_report, governance_framework_summary, list_deployment_profiles, policy_ref,
    run_compliance_check, validate_governance, verify_deployment, CertificationReport,
    CertificationStore, GovernancePolicyKind, PolicyStore, ValidationOptions,
};
use spanda_security::{ApiKeyStore, RbacAction, RbacContext};

#[derive(Deserialize, Default)]
struct ComplianceCheckRequest {
    #[serde(default)]
    strict: bool,
    entity_id: Option<String>,
}

#[derive(Deserialize, Default)]
struct GovernanceValidateRequest {
    #[serde(default)]
    strict: bool,
    entity_id: Option<String>,
}

pub fn governance_summary() -> HttpResponse {
    json_ok(&governance_framework_summary())
}

pub fn compliance_summary(state: &ControlCenterState, ctx: Option<&RbacContext>) -> HttpResponse {
    if !ApiKeyStore::check(ctx, RbacAction::Operate) {
        return unauthorized();
    }
    let registry = state.entity_registry();
    let Some(resolved) = state.resolved_config() else {
        return bad_request("No configuration loaded");
    };
    let report = run_compliance_check(
        &registry,
        resolved,
        &ValidationOptions::default(),
    );
    json_ok(&serde_json::json!({
        "version": "v1",
        "passed": report.passed,
        "summary": report.summary,
        "disclaimer": "Spanda provides governance abstractions and validation mechanisms — not legal or regulatory advice."
    }))
}

pub fn compliance_check(
    state: &ControlCenterState,
    query: &str,
    body: Option<&str>,
    ctx: Option<&RbacContext>,
) -> HttpResponse {
    if !ApiKeyStore::check(ctx, RbacAction::Operate) {
        return unauthorized();
    }
    let mut opts = ValidationOptions::default();
    if let Some(raw) = body {
        if let Ok(req) = serde_json::from_str::<ComplianceCheckRequest>(raw) {
            opts.strict = req.strict;
            opts.entity_id = req.entity_id;
        }
    }
    let params = parse_query(query);
    if params.get("strict").map(|s| s == "true").unwrap_or(false) {
        opts.strict = true;
    }
    if let Some(id) = params.get("entity_id") {
        opts.entity_id = Some(id.clone());
    }

    let registry = state.entity_registry();
    let Some(resolved) = state.resolved_config() else {
        return bad_request("No configuration loaded");
    };
    let report = run_compliance_check(&registry, resolved, &opts);
    json_ok(&serde_json::json!({
        "version": "v1",
        "report": report,
        "text": format_compliance_report(&report, false),
    }))
}

pub fn governance_validate(
    state: &ControlCenterState,
    query: &str,
    body: Option<&str>,
    ctx: Option<&RbacContext>,
) -> HttpResponse {
    if !ApiKeyStore::check(ctx, RbacAction::Operate) {
        return unauthorized();
    }
    let mut opts = ValidationOptions::default();
    if let Some(raw) = body {
        if let Ok(req) = serde_json::from_str::<GovernanceValidateRequest>(raw) {
            opts.strict = req.strict;
            opts.entity_id = req.entity_id;
        }
    }
    let params = parse_query(query);
    if params.get("strict").map(|s| s == "true").unwrap_or(false) {
        opts.strict = true;
    }

    let registry = state.entity_registry();
    let Some(resolved) = state.resolved_config() else {
        return bad_request("No configuration loaded");
    };
    let validation = validate_governance(&registry, resolved, &opts);
    let compliance = run_compliance_check(&registry, resolved, &opts);
    json_ok(&serde_json::json!({
        "version": "v1",
        "validation": validation,
        "compliance": compliance,
        "text": format_governance_report(&compliance, &validation, false),
    }))
}

pub fn certifications_list(state: &ControlCenterState, ctx: Option<&RbacContext>) -> HttpResponse {
    if !ApiKeyStore::check(ctx, RbacAction::Operate) {
        return unauthorized();
    }
    let registry = state.entity_registry();
    let certs: Vec<serde_json::Value> = registry
        .entities
        .values()
        .filter_map(|entity| {
            entity.governance.as_ref().and_then(|gov| {
                gov.certification_status.as_ref().map(|status| {
                    serde_json::json!({
                        "entity_id": entity.id,
                        "entity_type": entity.entity_type,
                        "certification_status": status,
                        "deployment_profile": gov.deployment_profile,
                        "operational_maturity": gov.operational_maturity,
                    })
                })
            })
        })
        .collect();
    json_ok(&serde_json::json!({
        "version": "v1",
        "certifications": certs,
    }))
}

pub fn deployment_profiles_list() -> HttpResponse {
    let profiles: Vec<serde_json::Value> = list_deployment_profiles()
        .iter()
        .map(|p| {
            serde_json::json!({
                "kind": p.kind.as_str(),
                "display_name": p.display_name,
                "description": p.description,
                "default_risk_level": p.default_risk_level.as_str(),
                "max_autonomy_level": p.decision_authority.max_autonomy_level.as_str(),
                "required_capabilities": p.required_capabilities,
                "required_certifications": p.required_certifications,
                "standards_profiles": p.standards_profiles.iter().map(|s| s.as_str()).collect::<Vec<_>>(),
            })
        })
        .collect();
    json_ok(&serde_json::json!({
        "version": "v1",
        "profiles": profiles,
    }))
}

pub fn deployment_profile_detail(query: &str) -> HttpResponse {
    let params = parse_query(query);
    let name = params
        .get("name")
        .or_else(|| params.get("profile"))
        .cloned()
        .unwrap_or_else(|| "warehouse".to_string());
    match deployment_profile_by_name(&name) {
        Some(profile) => json_ok(&serde_json::json!({
            "version": "v1",
            "profile": profile,
        })),
        None => bad_request(&format!("Unknown deployment profile '{name}'")),
    }
}

pub fn risk_summary(state: &ControlCenterState, ctx: Option<&RbacContext>) -> HttpResponse {
    if !ApiKeyStore::check(ctx, RbacAction::Operate) {
        return unauthorized();
    }
    let registry = state.entity_registry();
    let entities: Vec<serde_json::Value> = registry
        .entities
        .values()
        .filter_map(|entity| {
            entity.governance.as_ref().and_then(|gov| {
                gov.risk_level.as_ref().map(|risk| {
                    serde_json::json!({
                        "entity_id": entity.id,
                        "entity_type": entity.entity_type,
                        "risk_level": risk,
                        "autonomy_level": gov.autonomy_level,
                        "deployment_profile": gov.deployment_profile,
                        "health_status": entity.health_status,
                        "readiness_status": entity.readiness_status,
                    })
                })
            })
        })
        .collect();
    json_ok(&serde_json::json!({
        "version": "v1",
        "entities": entities,
    }))
}

pub fn certification_report(
    state: &ControlCenterState,
    query: &str,
    ctx: Option<&RbacContext>,
) -> HttpResponse {
    if !ApiKeyStore::check(ctx, RbacAction::Operate) {
        return unauthorized();
    }
    let params = parse_query(query);
    let entity_id = params.get("entity_id").cloned();
    let mut store = CertificationStore::load(&default_certification_store_path());
    seed_certifications_from_registry(&mut store, state);
    let report = CertificationReport::from_store(&store, entity_id.as_deref());
    json_ok(&serde_json::json!({
        "version": "v1",
        "report": report,
        "text": format_certification_report(&report, false),
    }))
}

pub fn deployment_verify(
    state: &ControlCenterState,
    query: &str,
    body: Option<&str>,
    ctx: Option<&RbacContext>,
) -> HttpResponse {
    if !ApiKeyStore::check(ctx, RbacAction::Operate) {
        return unauthorized();
    }
    let mut opts = ValidationOptions::default();
    if let Some(raw) = body {
        if let Ok(req) = serde_json::from_str::<GovernanceValidateRequest>(raw) {
            opts.strict = req.strict;
            opts.entity_id = req.entity_id;
        }
    }
    let params = parse_query(query);
    if params.get("strict").map(|s| s == "true").unwrap_or(false) {
        opts.strict = true;
    }
    if let Some(id) = params.get("entity_id") {
        opts.entity_id = Some(id.clone());
    }
    let registry = state.entity_registry();
    let Some(resolved) = state.resolved_config() else {
        return bad_request("No configuration loaded");
    };
    let report = verify_deployment(&registry, resolved, &opts);
    json_ok(&serde_json::json!({
        "version": "v1",
        "report": report,
        "text": format_deployment_verify(&report, false),
    }))
}

pub fn policies_list(ctx: Option<&RbacContext>) -> HttpResponse {
    if !ApiKeyStore::check(ctx, RbacAction::Operate) {
        return unauthorized();
    }
    let store = PolicyStore::load(&default_policy_store_path());
    json_ok(&serde_json::json!({
        "version": "v1",
        "assignments": store.assignments,
        "audit": store.audit,
    }))
}

pub fn policies_assign(
    body: &str,
    ctx: Option<&RbacContext>,
) -> HttpResponse {
    if !ApiKeyStore::check(ctx, RbacAction::Deploy) {
        return unauthorized();
    }
    #[derive(Deserialize)]
    struct AssignRequest {
        entity_id: String,
        kind: String,
        name: String,
        version: Option<String>,
        #[serde(default)]
        sign: bool,
    }
    let req: AssignRequest = match serde_json::from_str(body) {
        Ok(value) => value,
        Err(error) => return bad_request(&error.to_string()),
    };
    let path = default_policy_store_path();
    let mut store = PolicyStore::load(&path);
    let policy = policy_ref(
        GovernancePolicyKind::parse(&req.kind),
        &req.name,
        req.version.as_deref(),
    );
    let actor = ctx.map(|c| c.key_id.as_str());
    let assignment = store.assign(
        &req.entity_id,
        policy,
        actor,
        req.sign.then_some("spanda-governance-policy"),
    );
    if let Err(error) = store.save(&path) {
        return bad_request(&error);
    }
    json_ok(&serde_json::json!({
        "version": "v1",
        "assignment": assignment,
    }))
}

pub fn governance_audit(ctx: Option<&RbacContext>) -> HttpResponse {
    if !ApiKeyStore::check(ctx, RbacAction::Operate) {
        return unauthorized();
    }
    let policies = PolicyStore::load(&default_policy_store_path());
    let certifications = CertificationStore::load(&default_certification_store_path());
    json_ok(&serde_json::json!({
        "version": "v1",
        "policy_audit": policies.audit,
        "certification_records": certifications.records.len(),
    }))
}

pub fn accountability_summary(
    state: &ControlCenterState,
    ctx: Option<&RbacContext>,
) -> HttpResponse {
    if !ApiKeyStore::check(ctx, RbacAction::Operate) {
        return unauthorized();
    }
    let registry = state.entity_registry();
    let rows: Vec<serde_json::Value> = registry
        .entities
        .values()
        .filter_map(|entity| {
            entity.governance.as_ref().map(|gov| {
                serde_json::json!({
                    "entity_id": entity.id,
                    "responsible_person": gov.responsible_person,
                    "responsible_organization": gov.responsible_organization,
                    "mission_owner": gov.mission_owner,
                    "deployment_owner": gov.deployment_owner,
                    "emergency_contact": gov.emergency_contact,
                    "escalation_contact": gov.escalation_contact,
                    "approval_chain": gov.approval_chain,
                    "operational_maturity": gov.operational_maturity,
                })
            })
        })
        .collect();
    json_ok(&serde_json::json!({
        "version": "v1",
        "entities": rows,
    }))
}

fn seed_certifications_from_registry(store: &mut CertificationStore, state: &ControlCenterState) {
    let registry = state.entity_registry();
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
        let mut record = spanda_governance::CertificationRecord::draft(&id);
        record.status = spanda_governance::CertificationStatus::parse(status);
        record.applicable_scope = vec![entity.id.clone()];
        record.certified_by = gov.responsible_person.clone();
        store.upsert(record);
    }
}

/// JSON body for gRPC `GetGovernance` (parity with `GET /v1/governance`).
pub fn governance_summary_json() -> String {
    governance_summary().body
}

/// JSON body for gRPC `GetCompliance` (parity with `GET /v1/compliance`).
pub fn compliance_summary_json(
    state: &ControlCenterState,
    ctx: Option<&RbacContext>,
) -> String {
    compliance_summary(state, ctx).body
}

/// JSON body for gRPC `CheckCompliance` (parity with `POST /v1/compliance/check`).
pub fn compliance_check_json(
    state: &ControlCenterState,
    body: &str,
    ctx: Option<&RbacContext>,
) -> String {
    compliance_check(state, "", Some(body), ctx).body
}

/// JSON body for gRPC `ValidateGovernance` (parity with `POST /v1/governance/validate`).
pub fn governance_validate_json(
    state: &ControlCenterState,
    body: &str,
    ctx: Option<&RbacContext>,
) -> String {
    governance_validate(state, "", Some(body), ctx).body
}

/// JSON body for gRPC `ListCertifications` (parity with `GET /v1/certifications`).
pub fn certifications_list_json(
    state: &ControlCenterState,
    ctx: Option<&RbacContext>,
) -> String {
    certifications_list(state, ctx).body
}

/// JSON body for gRPC `ListDeploymentProfiles` (parity with `GET /v1/deployment-profiles`).
pub fn deployment_profiles_list_json() -> String {
    deployment_profiles_list().body
}

/// JSON body for gRPC `GetDeploymentProfile` (parity with `GET /v1/deployment-profiles?name=`).
pub fn deployment_profile_detail_json(query: &str) -> String {
    deployment_profile_detail(query).body
}

/// JSON body for gRPC `GetOperationalRisk` (parity with `GET /v1/risk`).
pub fn risk_summary_json(state: &ControlCenterState, ctx: Option<&RbacContext>) -> String {
    risk_summary(state, ctx).body
}
