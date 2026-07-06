//! Live cross-interface probes against a running Control Center started by
//! `scripts/cross_interface_consistency.sh`.

use spanda_api::grpc::spanda_v1::control_center_client::ControlCenterClient;
use spanda_api::grpc::spanda_v1::{Empty, JsonBodyRequest};
use std::time::{Duration, Instant};
use tonic::transport::Channel;

async fn connect_grpc(bind: &str) -> ControlCenterClient<Channel> {
    let deadline = Instant::now() + Duration::from_secs(15);
    loop {
        match Channel::from_shared(format!("http://{bind}"))
            .unwrap()
            .connect()
            .await
        {
            Ok(channel) => return ControlCenterClient::new(channel),
            Err(error) if Instant::now() < deadline => {
                tokio::time::sleep(Duration::from_millis(50)).await;
                let _ = error;
            }
            Err(error) => panic!("grpc connect to {bind}: {error:?}"),
        }
    }
}

fn cross_interface_http_env() -> Option<(String, String)> {
    let grpc_bind = std::env::var("SPANDA_XIFACE_GRPC_BIND").ok()?;
    let http_base = std::env::var("SPANDA_XIFACE_HTTP_BASE").ok()?;
    Some((grpc_bind, http_base))
}

#[tokio::test]
async fn grpc_health_matches_rest() {
    let Some((grpc_bind, http_base)) = cross_interface_http_env() else {
        return;
    };

    let rest = ureq::get(&format!("{http_base}/v1/health"))
        .call()
        .expect("rest health")
        .into_json::<serde_json::Value>()
        .expect("rest health json");
    assert!(
        rest.get("status")
            .and_then(|value| value.as_str())
            .is_some()
            || rest
                .get("ok")
                .and_then(|value| value.as_bool())
                .unwrap_or(false),
        "rest health missing status/ok: {rest}"
    );

    let mut client = connect_grpc(&grpc_bind).await;
    let grpc = client
        .health(Empty {})
        .await
        .expect("grpc health")
        .into_inner();
    assert!(
        grpc.status.starts_with("ok"),
        "unexpected grpc health: {}",
        grpc.status
    );
}

#[tokio::test]
async fn grpc_recovery_plan_matches_rest() {
    let Some((grpc_bind, http_base)) = cross_interface_http_env() else {
        return;
    };
    let Some(healing) = std::env::var("SPANDA_XIFACE_SELF_HEALING").ok() else {
        return;
    };
    let api_key = std::env::var("SPANDA_API_KEY").unwrap_or_default();

    let rest_body = serde_json::json!({
        "file": healing,
        "failure": "gps",
    });
    let mut rest_request = ureq::post(&format!("{http_base}/v1/recovery/plan"));
    rest_request = rest_request.set("Content-Type", "application/json");
    if !api_key.is_empty() {
        rest_request = rest_request.set("Authorization", &format!("Bearer {api_key}"));
    }
    let rest = rest_request
        .send_json(rest_body)
        .expect("rest recovery plan")
        .into_json::<serde_json::Value>()
        .expect("rest recovery plan json");
    assert!(
        rest.get("report").is_some() || rest.get("plans").is_some() || rest.get("passed").is_some(),
        "rest recovery plan missing expected fields: {rest}"
    );

    let mut client = connect_grpc(&grpc_bind).await;
    let grpc = client
        .plan_recovery(JsonBodyRequest {
            body_json: serde_json::json!({
                "file": healing,
                "failure": "gps",
            })
            .to_string(),
        })
        .await
        .expect("grpc recovery plan")
        .into_inner();
    assert!(
        grpc.json.contains("report") || grpc.json.contains("plans") || grpc.json.contains("passed"),
        "grpc recovery plan missing expected fields: {}",
        grpc.json
    );
}

#[tokio::test]
async fn grpc_autonomy_matches_rest() {
    let Some((grpc_bind, http_base)) = cross_interface_http_env() else {
        return;
    };

    let rest = ureq::get(&format!("{http_base}/v1/autonomy/reflex"))
        .call()
        .expect("rest autonomy reflex")
        .into_json::<serde_json::Value>()
        .expect("rest autonomy reflex json");
    assert!(
        rest.get("reflexes").is_some(),
        "rest reflex missing: {rest}"
    );

    let mut client = connect_grpc(&grpc_bind).await;
    let grpc = client
        .list_autonomy_reflexes(Empty {})
        .await
        .expect("grpc autonomy reflex")
        .into_inner();
    assert!(
        grpc.json.contains("reflex.emergency_stop"),
        "grpc reflex missing catalog: {}",
        grpc.json
    );

    let rest_homeo = ureq::get(&format!("{http_base}/v1/autonomy/homeostasis"))
        .call()
        .expect("rest homeostasis")
        .into_json::<serde_json::Value>()
        .expect("rest homeostasis json");
    assert!(
        rest_homeo.get("reports").is_some(),
        "rest homeostasis: {rest_homeo}"
    );

    let grpc_homeo = client
        .get_autonomy_homeostasis(Empty {})
        .await
        .expect("grpc homeostasis")
        .into_inner();
    assert!(
        grpc_homeo.json.contains("reports"),
        "grpc homeostasis missing reports: {}",
        grpc_homeo.json
    );

    let rest_fusion = ureq::get(&format!("{http_base}/v1/autonomy/fusion"))
        .call()
        .expect("rest fusion")
        .into_json::<serde_json::Value>()
        .expect("rest fusion json");
    assert!(
        rest_fusion.get("fusion").is_some(),
        "rest fusion: {rest_fusion}"
    );

    let grpc_fusion = client
        .get_autonomy_fusion(Empty {})
        .await
        .expect("grpc fusion")
        .into_inner();
    assert!(
        grpc_fusion.json.contains("fusion"),
        "grpc fusion missing field: {}",
        grpc_fusion.json
    );

    let rest_memory = ureq::get(&format!("{http_base}/v1/autonomy/memory"))
        .call()
        .expect("rest memory")
        .into_json::<serde_json::Value>()
        .expect("rest memory json");
    assert!(
        rest_memory.get("memory").is_some(),
        "rest memory: {rest_memory}"
    );

    let grpc_memory = client
        .get_autonomy_memory(Empty {})
        .await
        .expect("grpc memory")
        .into_inner();
    assert!(
        grpc_memory.json.contains("memory"),
        "grpc memory missing field: {}",
        grpc_memory.json
    );
}
