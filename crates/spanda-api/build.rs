fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = std::env::var("OUT_DIR")?;
    let descriptor_path = std::path::Path::new(&out_dir).join("proto_descriptor.bin");
    tonic_build::configure()
        .build_server(true)
        .file_descriptor_set_path(descriptor_path)
        .compile_protos(&["proto/spanda/v1/control_center.proto"], &["proto"])?;
    Ok(())
}
