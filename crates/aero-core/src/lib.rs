//! Simulation core for aero (working title).
//!
//! This crate currently contains only the workspace smoke test. Flight mechanics
//! will be added in later tasks, independently of rendering and file access.

#[cfg(test)]
mod tests {
    #[test]
    fn package_name_matches_workspace_library() {
        assert_eq!(env!("CARGO_PKG_NAME"), "aero-core");
    }
}
