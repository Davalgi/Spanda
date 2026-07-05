fn control_center_ui_version() -> String {
    let manifest_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let web_pkg = manifest_dir.join("../../packages/web/package.json");
    println!("cargo:rerun-if-changed={}", web_pkg.display());
    let text = std::fs::read_to_string(&web_pkg).unwrap_or_default();
    text.lines()
        .find_map(|line| {
            let trimmed = line.trim();
            trimmed
                .strip_prefix("\"version\":")
                .or_else(|| trimmed.strip_prefix("\"version\" :"))
                .map(|rest| rest.trim().trim_matches('"').trim_end_matches(',').to_string())
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
