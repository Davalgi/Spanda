fn control_center_ui_version() -> String {
    // Read packages/web semver for CONTROL_CENTER_UI_VERSION (API /v1/version).
    //
    // Parameters:
    // None (uses CARGO_MANIFEST_DIR).
    //
    // Returns:
    // Semver string from package.json, or `"0.0.0"` if missing/unparseable.
    //
    // Options:
    // None.
    //
    // Example:

    // assert_eq!(control_center_ui_version(), "1.0.0");

    let manifest_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let web_pkg = manifest_dir.join("../../packages/web/package.json");
    println!("cargo:rerun-if-changed={}", web_pkg.display());
    let text = std::fs::read_to_string(&web_pkg).unwrap_or_default();

    // Prefer JSON parse so trailing commas/quotes cannot leave a mangled semver.
    if let Ok(value) = serde_json::from_str::<serde_json::Value>(&text) {
        if let Some(version) = value.get("version").and_then(|v| v.as_str()) {
            if !version.is_empty() {
                return version.to_string();
            }
        }
    }

    // Fall back to a line scan when JSON parse fails (e.g. incomplete checkout).
    text.lines()
        .find_map(|line| {
            let trimmed = line.trim();
            trimmed
                .strip_prefix("\"version\":")
                .or_else(|| trimmed.strip_prefix("\"version\" :"))
                .map(|rest| {

                    // Strip comma first, then quotes — `"1.0.0",` must not become `1.0.0"`.
                    rest.trim()
                        .trim_end_matches(',')
                        .trim_matches('"')
                        .to_string()
                })
        })
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| "0.0.0".to_string())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!(
        "cargo:rustc-env=CONTROL_CENTER_UI_VERSION={}",
        control_center_ui_version()
    );

    let out_dir = std::env::var("OUT_DIR")?;
    let descriptor_path = std::path::Path::new(&out_dir).join("proto_descriptor.bin");
    tonic_build::configure()
        .build_server(true)
        .file_descriptor_set_path(descriptor_path)
        .compile_protos(&["proto/spanda/v1/control_center.proto"], &["proto"])?;
    Ok(())
}
