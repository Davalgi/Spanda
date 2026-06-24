//! Message schema registry used during compile-time type checking.
//!
use spanda_ast::comm_decl::{MessageDecl, MessageSchema};
use spanda_ast::foundations::StructDecl;
use spanda_ast::nodes::SpandaType;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Default)]
pub struct MessageRegistry {
    schemas: HashMap<String, MessageSchema>,
    builtin: HashSet<String>,
}

impl MessageRegistry {
    pub fn new() -> Self {
        // Description:
        //     Construct a new instance.
        //
        // Inputs:
        //     None.
        //
        // Outputs:
        //     result: Self
        //         Return value from `new`.
        //
        // Example:
        //     let value = spanda_typecheck::message_registry::new();
        let mut reg = Self::default();
        for name in ["Velocity", "Pose", "Scan", "String"] {
            reg.builtin.insert(name.into());
        }
        reg
    }

    pub fn register(&mut self, decl: &MessageDecl) {
        // Description:
        //     Register.
        //
        // Inputs:
        //     &mut self: input value
        //         Caller-supplied &mut self.
        //     decl: &MessageDecl
        //         Caller-supplied decl.
        //
        // Outputs:
        //     None.
        //
        // Example:

        //     let result = spanda_typecheck::message_registry::register(&mut self, decl);

        let MessageDecl::MessageDecl {
            name,
            fields,
            version,
            ..
        } = decl;
        self.schemas.insert(
            name.clone(),
            MessageSchema {
                name: name.clone(),
                fields: fields
                    .iter()
                    .map(|f| (f.name.clone(), f.type_name.clone()))
                    .collect(),
                version: *version,
            },
        );
    }

    pub fn from_program(messages: &[MessageDecl], structs: &[StructDecl]) -> Self {
        // Description:
        //     From program.
        //
        // Inputs:
        //     essages: &[MessageDecl]
        //         Caller-supplied essages.
        //     structs: &[StructDecl]
        //         Caller-supplied structs.
        //
        // Outputs:
        //     result: Self
        //         Return value from `from_program`.
        //
        // Example:

        //     let result = spanda_typecheck::message_registry::from_program(essages, structs);

        let mut reg = Self::new();
        for msg in messages {
            reg.register(msg);
        }
        for s in structs {
            let StructDecl::StructDecl { name, fields, .. } = s;
            reg.schemas.insert(
                name.clone(),
                MessageSchema {
                    name: name.clone(),
                    fields: fields
                        .iter()
                        .map(|f| (f.name.clone(), f.type_name.clone()))
                        .collect(),
                    version: None,
                },
            );
        }
        reg
    }

    pub fn is_known(&self, name: &str) -> bool {
        // Description:
        //     Is known.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //     name: &str
        //         Caller-supplied name.
        //
        // Outputs:
        //     result: bool
        //         Return value from `is_known`.
        //
        // Example:

        //     let result = spanda_typecheck::message_registry::is_known(&self, name);

        self.builtin.contains(name) || self.schemas.contains_key(name)
    }

    pub fn get(&self, name: &str) -> Option<&MessageSchema> {
        // Description:
        //     Get.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //     name: &str
        //         Caller-supplied name.
        //
        // Outputs:
        //     result: Option<&MessageSchema>
        //         Return value from `get`.
        //
        // Example:

        //     let result = spanda_typecheck::message_registry::get(&self, name);

        self.schemas.get(name)
    }

    pub fn resolve_type(&self, name: &str) -> Option<SpandaType> {
        // Description:
        //     Resolve type.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //     name: &str
        //         Caller-supplied name.
        //
        // Outputs:
        //     result: Option<SpandaType>
        //         Return value from `resolve_type`.
        //
        // Example:

        //     let result = spanda_typecheck::message_registry::resolve_type(&self, name);

        match name {
            "Velocity" => Some(SpandaType::Velocity),
            "Pose" => Some(SpandaType::Pose),
            "Scan" => Some(SpandaType::Scan),
            "String" => Some(SpandaType::String),
            "Command" | "Conversation" | "Feedback" | "Approval" | "Intent" => {
                Some(SpandaType::Named { name: name.into() })
            }
            "SafeMessage" | "VerifiedMessage" | "TrustedSource" | "ActionProposal"
            | "SafeAction" | "CommandMessage" | "EncryptedMessage" | "SignedMessage"
            | "Certificate" | "PublicKey" | "PrivateKey" | "SessionKey" => {
                Some(SpandaType::Named { name: name.into() })
            }
            "BatteryRequest" | "BatteryStatus" | "NavigationFeedback" | "NavigationResult"
            | "LidarReading" | "LidarScan" | "Timestamp" | "PathPlan" => {
                Some(SpandaType::Named { name: name.into() })
            }
            other if self.schemas.contains_key(other) => {
                Some(SpandaType::Named { name: other.into() })
            }
            other
                if other.starts_with("Topic<")
                    || other.starts_with("Service<")
                    || other.starts_with("Action<") =>
            {
                Some(SpandaType::Named { name: other.into() })
            }
            _ => None,
        }
    }
}

/// Agent capability actions that map to comm-bus operations.
pub const COMM_CAPABILITIES: &[&str] = &["subscribe", "publish", "call", "execute", "discover"];

pub fn is_comm_capability(action: &str) -> bool {
    // Description:
    //     Is comm capability.
    //
    // Inputs:
    //     action: &str
    //         Caller-supplied action.
    //
    // Outputs:
    //     result: bool
    //         Return value from `is_comm_capability`.
    //
    // Example:

    //     let result = spanda_typecheck::message_registry::is_comm_capability(action);

    COMM_CAPABILITIES.contains(&action)
}
