//! Mesh message security validation using secure messaging.
//!
use crate::types::*;
use spanda_security::{RobotIdentity, SignedMessage};

/// Security validation result for mesh messages.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MeshSecurityVerdict {
    Accepted,
    Rejected(String),
}

/// Validate a mesh message against security policies.
pub fn validate_mesh_message(
    mesh: &mut EntityMesh,
    message: &MeshMessage,
    now_epoch: i64,
) -> MeshSecurityVerdict {
    if message.nonce.is_empty() {
        return MeshSecurityVerdict::Rejected("missing nonce (replay protection)".into());
    }

    if let Err(err) = mesh.nonce_registry.register(&message.nonce) {
        return MeshSecurityVerdict::Rejected(err);
    }

    if message.ttl_secs == 0 {
        return MeshSecurityVerdict::Rejected("expired message: ttl=0".into());
    }

    if let Ok(ts) = chrono::DateTime::parse_from_rfc3339(&message.timestamp) {
        let age = now_epoch - ts.timestamp();
        if age > i64::from(message.ttl_secs) {
            return MeshSecurityVerdict::Rejected("expired message: ttl exceeded".into());
        }
        if age < -60 {
            return MeshSecurityVerdict::Rejected("message timestamp in future".into());
        }
    }

    if (message.priority == MeshMessagePriority::SafetyCritical
        || message.priority == MeshMessagePriority::Emergency)
        && message.trust_requirement.require_signed
        && message.signature.is_none()
    {
        return MeshSecurityVerdict::Rejected(
            "unsigned high-risk mesh command rejected".into(),
        );
    }

    if let Some(route) = &message.route {
        if message.trust_requirement.block_untrusted_relays && !route.trusted {
            return MeshSecurityVerdict::Rejected(
                "route through compromised or untrusted entity rejected".into(),
            );
        }
        for hop in &route.hops {
            if let Some(node) = mesh.nodes.get(hop) {
                if node.trust_score < message.trust_requirement.minimum_trust_score {
                    return MeshSecurityVerdict::Rejected(format!(
                        "route hop '{hop}' below trust requirement"
                    ));
                }
            }
        }
    }

    if message.trust_requirement.require_identity_match {
        if let Some(source) = mesh.nodes.get(&message.source_entity) {
            if source.security_identity.identity.is_none() && message.encryption_required {
                return MeshSecurityVerdict::Rejected(
                    "identity validation failed for encrypted message".into(),
                );
            }
        }
    }

    if let Some(envelope) = &message.signed_envelope {
        if !verify_signed_envelope(envelope) {
            return MeshSecurityVerdict::Rejected("secure messaging signature invalid".into());
        }
    }

    MeshSecurityVerdict::Accepted
}

fn verify_signed_envelope(envelope: &SignedMessage) -> bool {
    let identity = RobotIdentity::new(&envelope.signature.signer_id, "mesh-verify-key");
    envelope.verify(&identity).unwrap_or(false)
}

/// Wrap mesh payload in secure messaging signed envelope.
pub fn sign_mesh_message(message: &mut MeshMessage, identity: &RobotIdentity) {
    let payload = serde_json::to_string(message).unwrap_or_default();
    let signed = SignedMessage::sign(payload.clone(), identity);
    message.signature = Some(signed.signature.value.clone());
    message.signed_envelope = Some(signed);
    message.payload_hash = spanda_audit::sha256(&payload).0;
}

/// Build an auditable mesh message with defaults.
pub fn build_mesh_message(
    source_entity: &str,
    target_entity: Option<&str>,
    priority: MeshMessagePriority,
) -> MeshMessage {
    let payload_hash = spanda_audit::sha256("").0;
    MeshMessage {
        message_id: format!("msg-{}", uuid_simple()),
        source_entity: source_entity.into(),
        target_entity: target_entity.map(String::from),
        target_capability: None,
        route: None,
        priority,
        ttl_secs: 300,
        timestamp: chrono::Utc::now().to_rfc3339(),
        nonce: uuid_simple(),
        signature: None,
        encryption_required: matches!(
            priority,
            MeshMessagePriority::SafetyCritical | MeshMessagePriority::Emergency
        ),
        trust_requirement: MeshTrustRequirement::default(),
        payload_hash,
        signed_envelope: None,
        payload: None,
    }
}

fn uuid_simple() -> String {
    format!(
        "{:x}{:x}",
        chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0),
        mesh_random()
    )
}

fn mesh_random() -> u64 {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    let mut hasher = DefaultHasher::new();
    chrono::Utc::now().to_rfc3339().hash(&mut hasher);
    hasher.finish()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::discovery::build_entity_mesh;
    use spanda_config::entity::{EntityKind, EntityRecord, EntityRegistry};

    #[test]
    fn replayed_mesh_message_rejected() {
        let registry = EntityRegistry::default();
        let mut mesh = build_entity_mesh(&registry, "sec-test");
        let msg = build_mesh_message("src", None, MeshMessagePriority::Normal);
        let nonce = msg.nonce.clone();
        assert!(matches!(
            validate_mesh_message(&mut mesh, &msg, chrono::Utc::now().timestamp()),
            MeshSecurityVerdict::Accepted
        ));
        let replay = MeshMessage {
            nonce,
            ..build_mesh_message("src", None, MeshMessagePriority::Normal)
        };
        assert!(matches!(
            validate_mesh_message(&mut mesh, &replay, chrono::Utc::now().timestamp()),
            MeshSecurityVerdict::Rejected(_)
        ));
    }

    #[test]
    fn unsigned_safety_critical_rejected() {
        let registry = EntityRegistry::default();
        let mut mesh = build_entity_mesh(&registry, "sec-test");
        let msg = build_mesh_message("src", None, MeshMessagePriority::SafetyCritical);
        assert!(matches!(
            validate_mesh_message(&mut mesh, &msg, chrono::Utc::now().timestamp()),
            MeshSecurityVerdict::Rejected(_)
        ));
    }

    #[test]
    fn secure_messaging_wraps_mesh_messages() {
        let identity = RobotIdentity::new("robot-a", "test-signing-key");
        let mut msg = build_mesh_message("robot-a", Some("robot-b"), MeshMessagePriority::Normal);
        sign_mesh_message(&mut msg, &identity);
        assert!(msg.signature.is_some());
        assert!(msg.signed_envelope.is_some());
    }
}
