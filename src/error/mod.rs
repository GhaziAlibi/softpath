use std::path::PathBuf;
use thiserror::Error;

/// Represents errors that can occur during path operations
#[derive(Error, Debug)]
pub enum SoftPathError {
    /// An IO error occurred during the operation
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// Home directory could not be determined
    #[error("Home directory not found")]
    HomeDirNotFound,

    /// Path is invalid or malformed
    #[error("Invalid path: {0}")]
    InvalidPath(String),

    /// Path exceeds maximum depth
    #[error("Path depth exceeds maximum of {0} components")]
    PathTooDeep(usize),

    /// Path contains directory traversal attempts
    #[error("Path contains directory traversal attempt: {0}")]
    PathTraversal(String),

    /// Path contains a symlink cycle
    #[error("Symlink cycle detected in path: {0:?}")]
    SymlinkCycle(PathBuf),

    /// Path depth exceeded the allowed limit
    #[error("Path depth exceeded: {0} components")]
    PathDepthExceeded(usize),

    /// Path contains too many symlink levels
    #[error("Symlink depth exceeds maximum of {0} levels")]
    SymlinkDepthExceeded(usize),

    /// A symlink cycle was detected
    #[error("Symlink cycle detected in path: {0}")]
    SymlinkCycleDetected(String),

    /// Permission denied for the operation
    #[error("Permission denied: {0}")]
    PermissionDenied(String),
}
