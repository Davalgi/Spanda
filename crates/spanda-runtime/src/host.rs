//! Host hooks for domain-specific interpreter behavior in `spanda-core`.
//!

use spanda_ast::comm_decl::TransportKind;

/// Domain-specific runtime services supplied by the embedding application.
pub trait RuntimeHost {
    /// Whether an import path enables SLAM adapter hooks.
    fn slam_import_known(&self, path: &str) -> bool;

    /// Whether an import path enables navigation adapter hooks.
    fn navigation_import_known(&self, path: &str) -> bool;

    /// Invoke an external Nav2 bridge when the host configures one.
    fn invoke_nav2_bridge(&self, goal: &str) -> Option<String> {
        let _ = goal;
        None
    }

    /// Invoke an external SLAM bridge when the host configures one.
    fn invoke_slam_bridge(&self, op: &str) -> Option<String> {
        let _ = op;
        None
    }

    /// Map an active connectivity link name to the default transport backend.
    fn connectivity_link_to_transport(&self, link: &str) -> TransportKind {
        let _ = link;
        TransportKind::Sim
    }
}

/// Return true when any import path enables SLAM adapter behavior.
pub fn imports_enable_slam(paths: &[&str], host: &dyn RuntimeHost) -> bool {
    paths.iter().any(|path| host.slam_import_known(path))
}

/// Return true when any import path enables navigation adapter behavior.
pub fn imports_enable_navigation(paths: &[&str], host: &dyn RuntimeHost) -> bool {
    paths.iter().any(|path| host.navigation_import_known(path))
}
