//! SLAM package adapter hooks for external mapping/localization stacks.

use spanda_ast::nodes::ImportDecl;

/// Import paths that enable SLAM adapter behavior.
pub fn slam_import_paths() -> &'static [&'static str] {
    // Description:
    //     Slam import paths.
    //
    // Inputs:
    //     None.
    //
    // Outputs:
    //     result: &'static [&'static str]
    //         Return value from `slam_import_paths`.
    //
    // Example:

    //     let result = spanda_runtime_host::slam_adapter::slam_import_paths();

    &[
        "navigation.slam",
        "navigation.cartographer",
        "navigation.rtabmap",
    ]
}

/// Return true when the program imports a SLAM-related module path.
pub fn program_uses_slam(imports: &[ImportDecl]) -> bool {
    // Description:
    //     Program uses slam.
    //
    // Inputs:
    //     imports: &[ImportDecl]
    //         Caller-supplied imports.
    //
    // Outputs:
    //     result: bool
    //         Return value from `program_uses_slam`.
    //
    // Example:

    //     let result = spanda_runtime_host::slam_adapter::program_uses_slam(imports);

    imports.iter().any(|imp| {
        let ImportDecl::ImportDecl { path, .. } = imp;
        slam_import_paths().contains(&path.as_str())
    })
}
