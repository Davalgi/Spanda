//! Slack-compatible webhook payload formatting for alerts.
//!
use crate::alerting::Alert;
use serde_json::json;

/// Format an alert as a Slack incoming-webhook JSON body.
pub fn slack_webhook_payload(alert: &Alert) -> String {
    json!({
        "text": format!(
            "[Spanda] {:?} {:?} — {} ({})",
            alert.severity, alert.alert_type, alert.message, alert.source
        )
    })
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::alerting::{AlertSeverity, AlertType};

    #[test]
    fn slack_payload_contains_message() {
        let alert = Alert {
            id: "a1".into(),
            alert_type: AlertType::RobotOffline,
            severity: AlertSeverity::Critical,
            message: "rover offline".into(),
            source: "fleet".into(),
            timestamp_ms: 1.0,
            delivered_via: vec![],
        };
        let body = slack_webhook_payload(&alert);
        assert!(body.contains("rover offline"));
    }
}
