//! Entity governance projection and evaluation.
//!
use crate::certification::EntityCertificationSummary;
use crate::deployment_profile::deployment_profile_by_name;
use crate::human_accountability::HumanAccountability;
use crate::policy::{GovernancePolicyRef, StandardsProfileRef};
use crate::types::{
    AutonomyLevel, CertificationStatus, DeploymentMaturity, DeploymentProfileKind,
    OperationalConstraint, OperationalRisk, StandardsProfileKind, ValidationSeverity,
};
use serde::{Deserialize, Serialize};
use spanda_config::entity::{EntityHealthStatus, EntityReadinessStatus, EntityRecord, EntityRegistry, EntityTrustStatus};

/// Optional governance attributes for any entity.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct EntityGovernance {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub autonomy_level: Option<AutonomyLevel>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deployment_profile: Option<DeploymentProfileKind>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operational_maturity: Option<DeploymentMaturity>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub certification: Option<EntityCertificationSummary>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub risk_level: Option<OperationalRisk>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub governance_policies: Vec<GovernancePolicyRef>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub accountability: Option<HumanAccountability>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub standards_profiles: Vec<StandardsProfileRef>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub operational_constraints: Vec<OperationalConstraint>,
}

/// Governance evaluation report for a single entity.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EntityGovernanceReport {
    pub entity_id: String,
    pub governance: EntityGovernance,
    pub passed: bool,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub findings: Vec<GovernanceFinding>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub recommended_actions: Vec<String>,
}

/// Individual governance finding.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GovernanceFinding {
    pub severity: ValidationSeverity,
    pub code: String,
    pub message: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field: Option<String>,
}

/// Options for entity governance evaluation.
#[derive(Debug, Clone, Default)]
pub struct EntityGovernanceOptions {
    pub strict: bool,
}

/// Evaluate governance posture for a single entity.
pub fn evaluate_entity_governance(
    entity_id: &str,
    registry: &EntityRegistry,
    governance: &EntityGovernance,
    opts: &EntityGovernanceOptions,
) -> EntityGovernanceReport {
    // Build findings by cross-checking governance attributes against entity posture.
    //
    // Parameters:
    // - `entity_id` — target entity identifier
    // - `registry` — unified entity graph
    // - `governance` — governance attributes to validate
    // - `opts` — evaluation options
    //
    // Returns:
    // Governance report with pass/fail and findings.
    //
    // Options:
    // `strict` — treat warnings as failures.
    //
    // Example:
    // let report = evaluate_entity_governance("robot:alpha", &registry, &gov, &opts);

    let mut findings = Vec::new();
    let mut actions = Vec::new();
    let entity = registry.get(entity_id);

    if let Some(autonomy) = governance.autonomy_level {
        if autonomy.requires_human_approval() {
            if governance
                .accountability
                .as_ref()
                .and_then(|a| a.responsible_person.as_ref())
                .is_none()
            {
                findings.push(GovernanceFinding {
                    severity: ValidationSeverity::Missing,
                    code: "GOV_HUMAN_APPROVAL".into(),
                    message: format!(
                        "Autonomy level {} requires a responsible person for human approval",
                        autonomy.as_str()
                    ),
                    field: Some("accountability.responsible_person".into()),
                });
                actions.push("Assign a responsible person for human approval workflows".into());
            }
        }
        if let Some(record) = entity {
            let trust_sufficient = matches!(
                record.trust_status,
                EntityTrustStatus::Trusted | EntityTrustStatus::Verified
            );
            if autonomy.level_number() >= 3 && !trust_sufficient {
                findings.push(GovernanceFinding {
                    severity: ValidationSeverity::Warning,
                    code: "GOV_TRUST_AUTONOMY".into(),
                    message: format!(
                        "Autonomy level {} requires trusted posture; entity is {:?}",
                        autonomy.as_str(),
                        record.trust_status
                    ),
                    field: Some("trust_status".into()),
                });
            }
        }
    }

    if let Some(risk) = governance.risk_level {
        if risk.requires_human_approval() {
            let has_approval_chain = governance
                .accountability
                .as_ref()
                .map(|a| !a.approval_chain.is_empty())
                .unwrap_or(false);
            if !has_approval_chain {
                findings.push(GovernanceFinding {
                    severity: ValidationSeverity::Missing,
                    code: "GOV_APPROVAL_CHAIN".into(),
                    message: format!(
                        "Risk level {} requires an approval chain",
                        risk.as_str()
                    ),
                    field: Some("accountability.approval_chain".into()),
                });
                actions.push("Define an approval chain for high-risk operations".into());
            }
        }
        if risk.requires_simulation() {
            if let Some(maturity) = governance.operational_maturity {
                if maturity < DeploymentMaturity::Simulation {
                    findings.push(GovernanceFinding {
                        severity: ValidationSeverity::Warning,
                        code: "GOV_SIMULATION_MATURITY".into(),
                        message: "Medium+ risk should reach simulation maturity before deployment"
                            .into(),
                        field: Some("operational_maturity".into()),
                    });
                }
            }
        }
    }

    if let Some(maturity) = governance.operational_maturity {
        if maturity.allows_live_deployment() {
            let cert_ok = governance
                .certification
                .as_ref()
                .map(|c| c.status.is_operational())
                .unwrap_or(false);
            if !cert_ok {
                findings.push(GovernanceFinding {
                    severity: ValidationSeverity::Missing,
                    code: "GOV_CERT_LIVE".into(),
                    message: "Live deployment maturity requires validated or certified status"
                        .into(),
                    field: Some("certification.status".into()),
                });
                actions.push("Complete certification validation before live deployment".into());
            }
            if let Some(accountability) = governance.accountability.as_ref() {
                if !accountability.is_complete_for_production() {
                    findings.push(GovernanceFinding {
                        severity: ValidationSeverity::Missing,
                        code: "GOV_ACCOUNTABILITY".into(),
                        message: "Production deployment requires complete human accountability"
                            .into(),
                        field: Some("accountability".into()),
                    });
                }
            }
        }
    }

    if let Some(profile_kind) = governance.deployment_profile.as_ref() {
        let profile = deployment_profile_by_name(profile_kind.as_str());
        if let Some(profile) = profile {
            if let Some(record) = entity {
                let capability_applicable = matches!(
                    record.entity_type,
                    spanda_config::entity::EntityKind::Robot
                        | spanda_config::entity::EntityKind::Drone
                        | spanda_config::entity::EntityKind::Vehicle
                );
                if capability_applicable {
                    for cap in &profile.required_capabilities {
                        if !record.capabilities.iter().any(|c| c == cap) {
                            findings.push(GovernanceFinding {
                                severity: ValidationSeverity::Missing,
                                code: "GOV_CAPABILITY".into(),
                                message: format!(
                                    "Deployment profile {} requires capability '{}'",
                                    profile_kind.as_str(),
                                    cap
                                ),
                                field: Some("capabilities".into()),
                            });
                        }
                    }
                }
            }
            if let Some(autonomy) = governance.autonomy_level {
                if autonomy > profile.decision_authority.max_autonomy_level {
                    findings.push(GovernanceFinding {
                        severity: ValidationSeverity::Warning,
                        code: "GOV_AUTONOMY_PROFILE".into(),
                        message: format!(
                            "Autonomy {} exceeds profile maximum {}",
                            autonomy.as_str(),
                            profile.decision_authority.max_autonomy_level.as_str()
                        ),
                        field: Some("autonomy_level".into()),
                    });
                }
            }
        }
    }

    if let Some(record) = entity {
        if record.readiness_status == EntityReadinessStatus::NotReady
            && governance
                .operational_maturity
                .map(|m| m.allows_live_deployment())
                .unwrap_or(false)
        {
            findings.push(GovernanceFinding {
                severity: ValidationSeverity::Warning,
                code: "GOV_READINESS".into(),
                message: "Entity is not ready but maturity allows live deployment".into(),
                field: Some("readiness_status".into()),
            });
        }
        if record.health_status == EntityHealthStatus::Critical {
            findings.push(GovernanceFinding {
                severity: ValidationSeverity::Action,
                code: "GOV_HEALTH_CRITICAL".into(),
                message: "Entity health is critical — governance review required".into(),
                field: Some("health_status".into()),
            });
            actions.push("Resolve critical health before continuing governed operations".into());
        }
    }

    let has_blocking = findings.iter().any(|f| {
        matches!(
            f.severity,
            ValidationSeverity::Missing | ValidationSeverity::Action
        )
    });
    let has_warnings = findings.iter().any(|f| f.severity == ValidationSeverity::Warning);
    let passed = !has_blocking && (!opts.strict || !has_warnings);

    EntityGovernanceReport {
        entity_id: entity_id.to_string(),
        governance: governance.clone(),
        passed,
        findings,
        recommended_actions: actions,
    }
}

/// Parse governance attributes from entity metadata and labels.
pub fn governance_from_entity(record: &EntityRecord) -> EntityGovernance {
    // Project governance fields from entity metadata keys and tags.
    let autonomy_level = record
        .metadata
        .get("governance.autonomy_level")
        .map(|s| AutonomyLevel::parse(s));
    let deployment_profile = record
        .metadata
        .get("governance.deployment_profile")
        .map(|s| DeploymentProfileKind::parse(s));
    let operational_maturity = record
        .metadata
        .get("governance.operational_maturity")
        .map(|s| DeploymentMaturity::parse(s));
    let risk_level = record
        .metadata
        .get("governance.risk_level")
        .map(|s| OperationalRisk::parse(s));
    let certification_status = record
        .metadata
        .get("governance.certification_status")
        .map(|s| CertificationStatus::parse(s));

    let mut constraints: Vec<OperationalConstraint> = record
        .tags
        .iter()
        .filter_map(|tag| {
            tag.strip_prefix("constraint:")
                .map(OperationalConstraint::parse)
        })
        .collect();

    if let Some(raw) = record.metadata.get("governance.constraints") {
        for part in raw.split(',') {
            let trimmed = part.trim();
            if !trimmed.is_empty() {
                constraints.push(OperationalConstraint::parse(trimmed));
            }
        }
    }

    let standards_profiles: Vec<StandardsProfileRef> = record
        .metadata
        .get("governance.standards_profiles")
        .map(|raw| {
            raw.split(',')
                .map(|s| StandardsProfileRef::builtin(StandardsProfileKind::parse(s.trim())))
                .collect()
        })
        .unwrap_or_default();

    let mut accountability = HumanAccountability::default();
    if let Some(person) = record
        .owner
        .as_ref()
        .or(record.metadata.get("governance.responsible_person"))
    {
        accountability.responsible_person = Some(person.clone());
    }
    if let Some(org) = record.metadata.get("governance.responsible_organization") {
        accountability.responsible_organization = Some(org.clone());
    }
    if let Some(owner) = record.metadata.get("governance.mission_owner") {
        accountability.mission_owner = Some(owner.clone());
    }
    if let Some(owner) = record.metadata.get("governance.deployment_owner") {
        accountability.deployment_owner = Some(owner.clone());
    }
    if let Some(email) = record.metadata.get("governance.emergency_contact") {
        accountability.emergency_contact =
            Some(crate::human_accountability::AccountabilityContact {
                email: Some(email.clone()),
                name: record.metadata.get("governance.emergency_contact_name").cloned(),
                ..Default::default()
            });
    }
    if let Some(email) = record.metadata.get("governance.escalation_contact") {
        accountability.escalation_contact =
            Some(crate::human_accountability::AccountabilityContact {
                email: Some(email.clone()),
                name: record.metadata.get("governance.escalation_contact_name").cloned(),
                ..Default::default()
            });
    }
    if let Some(raw) = record.metadata.get("governance.approval_chain") {
        accountability.approval_chain = raw
            .split('|')
            .filter(|s| !s.is_empty())
            .map(|role| crate::human_accountability::ApprovalChainStep {
                role: role.to_string(),
                assignee: None,
                required: true,
                approved_at: None,
                approved_by: None,
            })
            .collect();
    }

    let certification = certification_status.map(|status| EntityCertificationSummary {
        status,
        records: vec![],
        primary_record_id: record.metadata.get("governance.certification_id").cloned(),
    });

    EntityGovernance {
        autonomy_level,
        deployment_profile,
        operational_maturity,
        certification,
        risk_level,
        governance_policies: vec![],
        accountability: if accountability.responsible_person.is_some()
            || accountability.deployment_owner.is_some()
        {
            Some(accountability)
        } else {
            None
        },
        standards_profiles,
        operational_constraints: constraints,
    }
}

/// Stamp governance metadata onto an entity record from TOML governance config.
pub fn stamp_entity_governance(record: &mut EntityRecord, governance: &EntityGovernance) {
    if let Some(level) = governance.autonomy_level {
        record
            .metadata
            .insert("governance.autonomy_level".into(), level.as_str().into());
    }
    if let Some(profile) = governance.deployment_profile.as_ref() {
        record.metadata.insert(
            "governance.deployment_profile".into(),
            profile.as_str().into(),
        );
    }
    if let Some(maturity) = governance.operational_maturity {
        record.metadata.insert(
            "governance.operational_maturity".into(),
            maturity.as_str().into(),
        );
    }
    if let Some(risk) = governance.risk_level {
        record
            .metadata
            .insert("governance.risk_level".into(), risk.as_str().into());
    }
    if let Some(cert) = governance.certification.as_ref() {
        record.metadata.insert(
            "governance.certification_status".into(),
            cert.status.as_str().into(),
        );
    }
    if !governance.operational_constraints.is_empty() {
        let joined = governance
            .operational_constraints
            .iter()
            .map(|c| c.as_str())
            .collect::<Vec<_>>()
            .join(",");
        record
            .metadata
            .insert("governance.constraints".into(), joined);
    }
    if let Some(accountability) = governance.accountability.as_ref() {
        if let Some(person) = accountability.responsible_person.as_ref() {
            record.owner = Some(person.clone());
            record.metadata.insert(
                "governance.responsible_person".into(),
                person.clone(),
            );
        }
        if let Some(org) = accountability.responsible_organization.as_ref() {
            record.metadata.insert(
                "governance.responsible_organization".into(),
                org.clone(),
            );
        }
        if let Some(owner) = accountability.mission_owner.as_ref() {
            record
                .metadata
                .insert("governance.mission_owner".into(), owner.clone());
        }
        if let Some(owner) = accountability.deployment_owner.as_ref() {
            record
                .metadata
                .insert("governance.deployment_owner".into(), owner.clone());
        }
        if let Some(contact) = accountability.emergency_contact.as_ref() {
            if let Some(email) = contact.email.as_ref() {
                record
                    .metadata
                    .insert("governance.emergency_contact".into(), email.clone());
            }
            if let Some(name) = contact.name.as_ref() {
                record.metadata.insert(
                    "governance.emergency_contact_name".into(),
                    name.clone(),
                );
            }
        }
        if let Some(contact) = accountability.escalation_contact.as_ref() {
            if let Some(email) = contact.email.as_ref() {
                record
                    .metadata
                    .insert("governance.escalation_contact".into(), email.clone());
            }
            if let Some(name) = contact.name.as_ref() {
                record.metadata.insert(
                    "governance.escalation_contact_name".into(),
                    name.clone(),
                );
            }
        }
        if !accountability.approval_chain.is_empty() {
            let roles = accountability
                .approval_chain
                .iter()
                .map(|s| s.role.as_str())
                .collect::<Vec<_>>()
                .join("|");
            record
                .metadata
                .insert("governance.approval_chain".into(), roles);
        }
    }
}

/// Parse governance config from a TOML value section.
pub fn parse_governance_config(value: &toml::Value) -> EntityGovernance {
    let table = value.as_table();
    let get_str = |key: &str| table.and_then(|t| t.get(key)).and_then(|v| v.as_str());
    let get_table = |key: &str| table.and_then(|t| t.get(key)).and_then(|v| v.as_table());

    let contact_from = |key: &str| -> Option<crate::human_accountability::AccountabilityContact> {
        let contact = get_table(key)?;
        let get = |k: &str| contact.get(k).and_then(|v| v.as_str()).map(String::from);
        Some(crate::human_accountability::AccountabilityContact {
            name: get("name"),
            role: get("role"),
            email: get("email"),
            phone: get("phone"),
            organization: get("organization"),
        })
    };

    let approval_chain = table
        .and_then(|t| t.get("approval_chain"))
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|item| {
                    let step = item.as_table()?;
                    let role = step.get("role")?.as_str()?.to_string();
                    Some(crate::human_accountability::ApprovalChainStep {
                        role,
                        assignee: step
                            .get("assignee")
                            .and_then(|v| v.as_str())
                            .map(String::from),
                        required: step
                            .get("required")
                            .and_then(|v| v.as_bool())
                            .unwrap_or(true),
                        approved_at: None,
                        approved_by: None,
                    })
                })
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();

    let operator_certifications = table
        .and_then(|t| t.get("operator_certifications"))
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|item| {
                    let cert = item.as_table()?;
                    let id = cert.get("id")?.as_str()?.to_string();
                    Some(crate::human_accountability::OperatorCertification {
                        id,
                        issuer: cert
                            .get("issuer")
                            .and_then(|v| v.as_str())
                            .map(String::from),
                        issued_at: cert
                            .get("issued_at")
                            .and_then(|v| v.as_str())
                            .map(String::from),
                        expires_at: cert
                            .get("expires_at")
                            .and_then(|v| v.as_str())
                            .map(String::from),
                        scopes: cert
                            .get("scopes")
                            .and_then(|v| v.as_array())
                            .map(|scopes| {
                                scopes
                                    .iter()
                                    .filter_map(|s| s.as_str().map(String::from))
                                    .collect()
                            })
                            .unwrap_or_default(),
                    })
                })
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();

    let person = get_str("responsible_person");
    let org = get_str("responsible_organization");
    let mission = get_str("mission_owner");
    let deployment = get_str("deployment_owner");
    let emergency = contact_from("emergency_contact");
    let escalation = contact_from("escalation_contact");
    let has_accountability = person.is_some()
        || org.is_some()
        || mission.is_some()
        || deployment.is_some()
        || emergency.is_some()
        || escalation.is_some()
        || !approval_chain.is_empty()
        || !operator_certifications.is_empty();

    EntityGovernance {
        autonomy_level: get_str("autonomy_level").map(AutonomyLevel::parse),
        deployment_profile: get_str("deployment_profile").map(DeploymentProfileKind::parse),
        operational_maturity: get_str("operational_maturity").map(DeploymentMaturity::parse),
        certification: get_str("certification_status").map(|status| EntityCertificationSummary {
            status: CertificationStatus::parse(status),
            records: vec![],
            primary_record_id: get_str("certification_id").map(String::from),
        }),
        risk_level: get_str("risk_level").map(OperationalRisk::parse),
        governance_policies: vec![],
        accountability: has_accountability.then(|| HumanAccountability {
            responsible_person: person.map(String::from),
            responsible_organization: org.map(String::from),
            mission_owner: mission.map(String::from),
            deployment_owner: deployment.map(String::from),
            approval_chain,
            emergency_contact: emergency,
            escalation_contact: escalation,
            operator_certifications,
        }),
        standards_profiles: get_str("standards_profiles")
            .map(|raw| {
                raw.split(',')
                    .map(|s| StandardsProfileRef::builtin(StandardsProfileKind::parse(s.trim())))
                    .collect()
            })
            .unwrap_or_default(),
        operational_constraints: get_str("constraints")
            .map(|raw| {
                raw.split(',')
                    .map(|s| OperationalConstraint::parse(s.trim()))
                    .collect()
            })
            .unwrap_or_default(),
    }
}
