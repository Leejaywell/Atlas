/// The core component of the Atlas system.
pub struct AtlasCore {
    /// The current version of the Atlas Core.
    pub version: String,
}

impl AtlasCore {
    /// Creates a new instance of `AtlasCore` with the version from `Cargo.toml`.
    pub fn new() -> Self {
        Self {
            version: env!("CARGO_PKG_VERSION").to_string(),
        }
    }

    /// Returns a status message indicating that the Atlas Core is running.
    pub fn get_status(&self) -> String {
        format!("Atlas Core v{} is running", self.version)
    }
}

impl Default for AtlasCore {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_core_status() {
        let core = AtlasCore::new();
        assert_eq!(core.get_status(), format!("Atlas Core v{} is running", env!("CARGO_PKG_VERSION")));
    }

    #[test]
    fn test_default_implementation() {
        let core = AtlasCore::default();
        assert_eq!(core.version, env!("CARGO_PKG_VERSION"));
    }
}
