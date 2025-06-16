use crate::error::SoftPathError;
use std::path::{Path, PathBuf};

/// A trait for ergonomic, human-friendly path manipulation.
pub trait PathExt {
    /// Converts the path into a `PathBuf`.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The path contains invalid UTF-8
    /// - Home directory expansion is requested but the home directory cannot be determined
    fn into_path(self) -> Result<PathBuf, SoftPathError>;

    /// Returns true if the path exists.
    fn exists(&self) -> bool;

    /// Returns true if the path points to a regular file.
    fn is_file(&self) -> bool;

    /// Returns true if the path points to a directory.
    fn is_dir(&self) -> bool;

    /// Creates a new, empty file at this path.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The file already exists
    /// - The user lacks permissions
    /// - Any parent directories are missing
    fn create_file(&self) -> Result<(), SoftPathError>;

    /// Creates all parent directories if they are missing.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The user lacks permissions
    /// - Any parent component is not a directory
    fn create_dir_all(&self) -> Result<(), SoftPathError>;

    /// Removes a file or directory at this path.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The path does not exist
    /// - The user lacks permissions
    /// - The path is a non-empty directory
    fn remove(&self) -> Result<(), SoftPathError>;

    /// Reads the entire file into a string.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The path does not exist
    /// - The path is not a file
    /// - The file cannot be read
    /// - The file contains invalid UTF-8
    fn read_to_string(&self) -> Result<String, SoftPathError>;

    /// Writes a string slice to a file.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The file cannot be created
    /// - The user lacks permissions
    /// - Any parent directories are missing
    fn write_string(&self, contents: &str) -> Result<(), SoftPathError>;

    /// Copies the file to a new path.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The source path does not exist
    /// - The destination path already exists
    /// - The user lacks permissions
    /// - Any parent directories of the destination are missing
    fn copy_to<P: AsRef<Path>>(&self, dest: P) -> Result<(), SoftPathError>;

    /// Moves the file or directory to a new path.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The source path does not exist
    /// - The destination path already exists
    /// - The user lacks permissions
    fn move_to<P: AsRef<Path>>(&self, dest: P) -> Result<(), SoftPathError>;

    /// Returns true if the path points to an empty file or directory.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The path does not exist
    /// - The user lacks permissions to read the path
    fn is_empty(&self) -> Result<bool, SoftPathError>;

    /// Returns true if the path is hidden (platform-specific implementation).
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The path does not exist
    /// - The user lacks permissions to read the path attributes
    fn is_hidden(&self) -> Result<bool, SoftPathError>;

    /// Returns the file name as a String, or None if the path terminates in '..' or '.'.
    fn file_name(&self) -> Option<String>;

    /// Returns the extension of the file as a String, or None if there is no extension.
    fn extension(&self) -> Option<String>;

    /// Returns the name of the parent directory as a String, or None if there is no parent.
    fn parent_name(&self) -> Option<String>;

    /// Returns the absolute path with all components normalized.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The path does not exist
    /// - The current directory cannot be determined
    /// - The user lacks permissions to resolve the path
    fn absolute(&self) -> Result<PathBuf, SoftPathError>;
}
