//! Tauri desktop shell for Spanda Control Center.
//!

use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Manager,
};

/// Start the Control Center desktop application.
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Launch the Tauri desktop shell for Control Center.
    //
    // Parameters:
    // None.
    //
    // Returns:
    // Does not return on success (runs the event loop).
    //
    // Options:
    // Set `SPANDA_CONTROL_CENTER_URL` for the default API base exposed to the webview.
    //
    // Example:
    // spanda_control_center_desktop_lib::run();

    // Build the Tauri app with shell, notification, and tray integrations.
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .setup(|app| {
            // Build a minimal tray menu so the icon is visible on Linux.
            let show_item = MenuItem::with_id(app, "show", "Show Control Center", true, None::<&str>)?;
            let quit_item = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&show_item, &quit_item])?;

            // Attach the system tray with fleet status tooltip.
            let _tray = TrayIconBuilder::with_id("main")
                .icon(
                    app.default_window_icon()
                        .ok_or_else(|| tauri::Error::FailedToReceiveMessage)?
                        .clone(),
                )
                .menu(&menu)
                .tooltip("Spanda Control Center — connecting…")
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "show" => {
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                    "quit" => {
                        app.exit(0);
                    }
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } = event
                    {
                        let app = tray.app_handle();
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                })
                .build(app)?;
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            default_api_base,
            desktop_features,
            spawn_control_center_api,
            desktop_notify,
            update_tray_status
        ])
        .run(tauri::generate_context!())
        .expect("error while running Spanda Control Center desktop");
}

/// Return the default Control Center API URL for the webview.
#[tauri::command]
fn default_api_base() -> String {
    // Resolve the API base URL from the environment or local dev default.
    //
    // Parameters:
    // None (Tauri command).
    //
    // Returns:
    // Control Center REST base URL string.
    //
    // Options:
    // `SPANDA_CONTROL_CENTER_URL` overrides the default `http://127.0.0.1:8080`.
    //
    // Example:
    // SPANDA_CONTROL_CENTER_URL=http://fleet:8080 spanda-control-center-desktop

    // Prefer SPANDA_CONTROL_CENTER_URL; fall back to local dev server.
    std::env::var("SPANDA_CONTROL_CENTER_URL").unwrap_or_else(|_| "http://127.0.0.1:8080".into())
}

/// Desktop feature flags exposed to the webview (notifications, tray, bundled API).
#[tauri::command]
fn desktop_features() -> serde_json::Value {
    // Return desktop shell capabilities for the Control Center webview.
    //
    // Parameters:
    // None (Tauri command).
    //
    // Returns:
    // JSON object describing optional desktop integrations.
    //
    // Options:
    // Set `SPANDA_DESKTOP_BUNDLED_API=1` to advertise local API bundling.
    //
    // Example:
    // desktop_features() -> {"notifications": true, "system_tray": true, ...}

    // Surface optional desktop integrations to the React shell.
    let bundled_api = std::env::var("SPANDA_DESKTOP_BUNDLED_API")
        .map(|value| value == "1" || value.eq_ignore_ascii_case("true"))
        .unwrap_or(false);
    serde_json::json!({
        "notifications": true,
        "system_tray": true,
        "offline_cache": true,
        "bundled_api": bundled_api,
        "bundled_api_hint": "Run: spanda control-center serve --bind 127.0.0.1:8080",
        "spawn_api_command": "spawn_control_center_api",
        "app_version": env!("CARGO_PKG_VERSION")
    })
}

/// Spawn a local Control Center API via `spanda control-center serve` (desktop shell).
#[tauri::command]
async fn spawn_control_center_api(
    app: tauri::AppHandle,
    bind: Option<String>,
) -> Result<String, String> {
    // Launch the Control Center REST API as a background child process.
    //
    // Parameters:
    // - `bind` — optional listen address (defaults to 127.0.0.1:8080).
    //
    // Returns:
    // Human-readable status string on success.
    //
    // Options:
    // Requires `spanda` on PATH; uses tauri-plugin-shell.
    //
    // Example:
    // spawn_control_center_api(Some("127.0.0.1:8080".into()))

    use tauri_plugin_shell::ShellExt;

    // Resolve bind address from argument or local dev default.
    let listen = bind.unwrap_or_else(|| "127.0.0.1:8080".into());

    // Spawn the Control Center serve subcommand detached from the desktop shell.
    app.shell()
        .command("spanda")
        .args(["control-center", "serve", "--bind", listen.as_str()])
        .spawn()
        .map_err(|error| format!("spawn failed: {error}"))?;

    Ok(format!("spawned spanda control-center serve --bind {listen}"))
}

/// Show a native OS notification from the webview.
#[tauri::command]
fn desktop_notify(app: tauri::AppHandle, title: String, body: String) -> Result<(), String> {
    // Deliver a native notification for SRE or fleet alerts.
    //
    // Parameters:
    // - `title` — notification headline.
    // - `body` — notification detail text.
    //
    // Returns:
    // Ok on success, or an error string when the OS rejects the notification.
    //
    // Options:
    // Requires notification permissions on macOS and Windows.
    //
    // Example:
    // desktop_notify(app, "SLO fast-burn".into(), "Investigate incidents".into())

    use tauri_plugin_notification::NotificationExt;

    // Build and show the native notification.
    app.notification()
        .builder()
        .title(title)
        .body(body)
        .show()
        .map_err(|error| error.to_string())
}

/// Update the system tray tooltip with fleet health summary text.
#[tauri::command]
fn update_tray_status(
    app: tauri::AppHandle,
    status: String,
    detail: Option<String>,
) -> Result<(), String> {
    // Refresh tray tooltip from polled instance status.
    //
    // Parameters:
    // - `status` — overall fleet health label.
    // - `detail` — optional secondary detail (robot count, tenant, etc.).
    //
    // Returns:
    // Ok when the tray icon exists and accepts the tooltip update.
    //
    // Options:
    // None.
    //
    // Example:
    // update_tray_status(app, "healthy".into(), Some("robots: 4".into()))

    // Compose tooltip text from status and optional detail.
    let tooltip = match detail {
        Some(extra) if !extra.is_empty() => format!("Spanda — {status} ({extra})"),
        _ => format!("Spanda — {status}"),
    };

    // Apply tooltip to the main tray icon when present.
    if let Some(tray) = app.tray_by_id("main") {
        tray.set_tooltip(Some(&tooltip))
            .map_err(|error| error.to_string())?;
    }
    Ok(())
}
