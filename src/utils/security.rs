//! Security utilities for path operations.
//!
//! This module provides functions for checking path security, including:
//! - Directory traversal detection
//! - Symlink cycle detection

use crate::error::SoftPathError;
use crate::Config;
use std::collections::HashSet;
use std::fs;
use std::path::Path;
use std::sync::OnceLock;

static CONFIG: OnceLock<Config> = OnceLock::new();

/// Gets the current configuration, initializing with defaults if not set
pub(crate) fn get_config() -> &'static Config {
    CONFIG.get_or_init(Config::default)
}

/// Sets the global configuration for path operations.
///
/// This function allows you to customize the security limits used by the library:
/// - Maximum path depth (number of path components allowed)
/// - Maximum symlink depth (number of symlinks to follow)
///
/// If this function is not called, default values will be used:
/// - Default max path depth: 256
/// - Default max symlink depth: 40
///
/// # Examples
/// ```
/// use softpath::Config;
///
/// // Create custom configuration
/// let config = Config::new()
///     .with_max_path_depth(128)      // More restrictive path depth
///     .with_max_symlink_depth(20);   // More restrictive symlink following
///
/// // Apply the configuration globally
/// softpath::set_config(config);
/// ```
///
/// # Thread Safety
///
/// This function uses a thread-safe `OnceLock` internally, so it's safe to call
/// from multiple threads. However, the configuration can only be set once - subsequent
/// calls will have no effect.
pub fn set_config(config: Config) {
    let _ = CONFIG.set(config);
}

/// Checks if a path contains potential directory traversal attacks.
pub(crate) fn check_path_traversal(path: &Path) -> Result<(), SoftPathError> {
    use std::path::Component;

    let config = get_config();

    // Check for path depth
    if path.components().count() > config.max_path_depth {
        return Err(SoftPathError::PathDepthExceeded(config.max_path_depth));
    }

    // Convert to absolute path for analysis
    let absolute = if path.is_absolute() {
        path.to_path_buf()
    } else {
        std::env::current_dir()?.join(path)
    };

    // Count the net level of directory traversal
    let mut depth = 0;
    for component in absolute.components() {
        match component {
            Component::ParentDir => depth -= 1,
            Component::Normal(_) => depth += 1,
            Component::RootDir => depth = 0,
            _ => {}
        }
        // If we ever try to go above root, that's a traversal attempt
        if depth < 0 {
            return Err(SoftPathError::PathTraversal(
                sanitize_path_for_error(&absolute),
            ));
        }
    }

    Ok(())
}

/// Checks if a path contains symlink cycles or exceeds the maximum symlink depth.
pub(crate) fn check_symlink_cycles(path: &Path) -> Result<(), SoftPathError> {
    let config = get_config();
    let mut followed = 0;
    let mut current = path.to_path_buf();
    let mut visited = HashSet::new();

    while current.is_symlink() {
        if !visited.insert(current.clone()) {
            return Err(SoftPathError::SymlinkCycleDetected(
                sanitize_path_for_error(&current).into(),
            ));
        }

        if followed >= config.max_symlink_depth {
            return Err(SoftPathError::SymlinkDepthExceeded(
                config.max_symlink_depth,
            ));
        }

        followed += 1;
        current = fs::read_link(&current)?;
    }

    Ok(())
}

/// Sanitizes a path for inclusion in error messages to prevent information disclosure.
/// In production builds, this should hide sensitive directory structures.
fn sanitize_path_for_error(path: &Path) -> String {
    #[cfg(debug_assertions)]
    {
        // In debug builds, show full path for development
        path.to_string_lossy().into()
    }
    #[cfg(not(debug_assertions))]
    {
        // In release builds, only show the filename to prevent information disclosure
        path.file_name()
            .map(|name| format!("<path>/{}", name.to_string_lossy()))
            .unwrap_or_else(|| "<path>".to_string())
    }
}
