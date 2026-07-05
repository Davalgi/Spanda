//! Embedded React Control Center UI (built from `@davalgi-spanda/web`).
//!
//! Regenerate with `scripts/sync_control_center_embedded_ui.sh` after UI changes.

use rust_embed::RustEmbed;
use spanda_deploy_http::HttpResponse;
use std::borrow::Cow;

#[derive(RustEmbed)]
#[folder = "src/static/control-center-ui/"]
struct ControlCenterUi;

/// Map a request path to an embedded asset path.
fn asset_key(path: &str) -> Option<String> {
    match path {
        "/" | "/control-center" => Some("control-center.html".to_string()),
        asset if asset.starts_with("/assets/") => Some(asset[1..].to_string()),
        _ => None,
    }
}

/// MIME type for a static asset path.
pub fn content_type_for_path(path: &str) -> Option<&'static str> {
    let key = asset_key(path)?;
    let mime = match key.rsplit('.').next() {
        Some("html") => "text/html; charset=utf-8",
        Some("js") => "application/javascript; charset=utf-8",
        Some("css") => "text/css; charset=utf-8",
        Some("json") => "application/json",
        Some("svg") => "image/svg+xml",
        Some("png") => "image/png",
        Some("ico") => "image/x-icon",
        Some("woff2") => "font/woff2",
        Some("woff") => "font/woff",
        _ => "application/octet-stream",
    };
    Some(mime)
}

/// Serve an embedded Control Center static asset when present.
pub fn serve_static(path: &str) -> Option<HttpResponse> {
    let key = asset_key(path)?;
    let file = ControlCenterUi::get(&key)?;
    let body = match file.data {
        Cow::Borrowed(bytes) => String::from_utf8_lossy(bytes).into_owned(),
        Cow::Owned(bytes) => String::from_utf8_lossy(&bytes).into_owned(),
    };
    Some(HttpResponse { status: 200, body })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn embedded_index_is_available() {
        assert!(ControlCenterUi::get("control-center.html").is_some());
    }

    #[test]
    fn serve_root_returns_html() {
        let response = serve_static("/").expect("index");
        assert!(response.body.contains("root"));
    }

    #[test]
    fn serve_bundled_assets() {
        let css = content_type_for_path("/assets/control-center-Ch0dDWmQ.css");
        assert_eq!(css, Some("text/css; charset=utf-8"));
    }
}
