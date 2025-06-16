//! Configuration options for the softpath library.

/// Configuration for path operations and security settings.
#[derive(Debug, Clone)]
pub struct Config {
    /// Maximum allowed depth of a path in components.
    /// Default: 256
    pub max_path_depth: usize,

    /// Maximum number of symlinks to follow when resolving paths.
    /// Default: 40
    pub max_symlink_depth: usize,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            max_path_depth: 256,
            max_symlink_depth: 40,
        }
    }
}

impl Config {
    /// Creates a new configuration with default values.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the maximum path depth.
    #[must_use]
    pub fn with_max_path_depth(mut self, depth: usize) -> Self {
        self.max_path_depth = depth;
        self
    }

    /// Sets the maximum symlink depth.
    #[must_use]
    pub fn with_max_symlink_depth(mut self, depth: usize) -> Self {
        self.max_symlink_depth = depth;
        self
    }
}
