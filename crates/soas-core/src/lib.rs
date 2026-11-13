//! Simulation core for Showdown of Aerial Systems.
//!
//! Pure simulation helpers, independent of rendering and file access.

pub mod aircraft;
pub mod atmosphere;
pub mod attitude;
pub mod dynamics;
pub mod units;

#[cfg(test)]
mod tests {
    #[test]
    fn package_name_matches_workspace_library() {
        assert_eq!(env!("CARGO_PKG_NAME"), "soas-core");
    }
}
