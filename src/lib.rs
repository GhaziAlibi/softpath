//! # `SoftPath`
//!
//! A human-friendly file and directory path manipulation library for Rust.
//!
//! This crate provides an ergonomic interface for common file system operations,
//! with a focus on safety, cross-platform compatibility, and ease of use.
//!
//! ## Features
//!
//! - Intuitive path manipulation methods
//! - Automatic directory creation when needed
//! - Home directory expansion (`~` support)
//! - Cross-platform path handling
//! - Strong error handling
//! - Thread-safe operations
//!
//! ## Configuration
//!
//! `SoftPath` provides configurable security limits that can be adjusted globally:
//!
//! ```rust
//! use softpath::{Config, PathExt};
//! use std::path::PathBuf;
//!
//! // Create custom configuration
//! let config = Config::new()
//!     .with_max_path_depth(100)      // Limit path depth to 100 components
//!     .with_max_symlink_depth(10);   // Limit symlink following to 10 levels
//!
//! // Set the global configuration
//! softpath::set_config(config);
//!
//! // Now all operations will use these limits
//! let path = PathBuf::from("some/path");
//! path.create_file().expect("Failed to create file");
//! ```
//!
//! ## Basic Examples
//!
//! ```rust
//! use softpath::prelude::*;
//! use std::path::PathBuf;
//!
//! # fn main() -> Result<(), softpath::SoftPathError> {
//! # let temp_dir = std::env::temp_dir();
//! # let config_path = temp_dir.join("softpath_example/config");
//! # let backup_path = temp_dir.join("softpath_example/backup");
//! # std::fs::create_dir_all(&config_path)?;
//! # std::fs::create_dir_all(&backup_path)?;
//!
//! // Create and write to a file
//! let config_file = config_path.join("app.json").into_path()?;
//! config_file.write_string("{\"version\": 1}")?;
//!
//! // Copy to backup location
//! let backup = backup_path.join("app.json").into_path()?;
//! config_file.copy_to(&backup)?;
//! # Ok(())
//! # }
//! ```

mod config;
mod error;
mod ops;
mod utils;

pub use config::Config;
pub use error::SoftPathError;
pub use ops::PathExt;
pub use utils::security::set_config;

// Re-export commonly used types
/// Commonly used imports for convenient access to `SoftPath` functionality.
///
/// The `prelude` module re-exports traits and types that are most frequently used,
/// allowing for easier imports in user code.
pub mod prelude {
    pub use crate::ops::PathExt;
    pub use crate::SoftPathError;
}
