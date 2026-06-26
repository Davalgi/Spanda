//! Enterprise operations primitives for Spanda Control Center.
//!
pub mod alerting;
pub mod slack;

pub use alerting::{
    Alert, AlertChannel, AlertDispatcher, AlertSeverity, AlertStore, AlertType,
};
pub use slack::slack_webhook_payload;
