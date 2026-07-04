//! Property-style plugin manifest parser tests.

use spanda_plugin::manifest::PluginManifest;

#[test]
fn plugin_manifest_parser_never_panics_on_garbage() {
    // Garbage plugin manifests must not panic.
    let samples = [
        "",
        "not toml",
        "[plugin]\nname = ",
        "[plugin]\nname = \"x\"\nversion = \"0.1.0\"\npublisher = \"p\"\ndescription = \"d\"\nlicense = \"Apache-2.0\"\ntype = \"readiness\"",
        &"[".repeat(100),
    ];
    for sample in samples {
        let _ = std::panic::catch_unwind(|| {
            let _ = PluginManifest::parse_str(sample);
        })
        .expect("plugin manifest parser must not panic");
    }
}
