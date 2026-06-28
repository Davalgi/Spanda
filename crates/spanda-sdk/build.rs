fn main() -> Result<(), Box<dyn std::error::Error>> {
    if std::env::var("CARGO_FEATURE_GRPC").is_ok() {
        tonic_build::configure()
            .build_client(true)
            .build_server(false)
            .compile_protos(
                &["../spanda-api/proto/spanda/v1/control_center.proto"],
                &["../spanda-api/proto"],
            )?;
    }
    Ok(())
}
