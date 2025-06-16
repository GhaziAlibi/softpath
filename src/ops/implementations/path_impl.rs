use std::fs;
use std::path::{Path, PathBuf};
use crate::error::SoftPathError;
use crate::ops::PathExt;
use super::super::operations::PathOps;

impl PathExt for &Path {
    fn into_path(self) -> Result<PathBuf, SoftPathError> {
        Ok(self.to_path_buf())
    }

    fn exists(&self) -> bool {
        Path::exists(self)
    }

    fn is_file(&self) -> bool {
        Path::is_file(self)
    }

    fn is_dir(&self) -> bool {
        Path::is_dir(self)
    }

    fn create_file(&self) -> Result<(), SoftPathError> {
        self.create_file_impl()
    }

    fn create_dir_all(&self) -> Result<(), SoftPathError> {
        fs::create_dir_all(self).map_err(SoftPathError::from)
    }

    fn remove(&self) -> Result<(), SoftPathError> {
        self.remove_impl()
    }

    fn read_to_string(&self) -> Result<String, SoftPathError> {
        fs::read_to_string(self).map_err(SoftPathError::from)
    }

    fn write_string(&self, contents: &str) -> Result<(), SoftPathError> {
        self.write_string_impl(contents)
    }

    fn copy_to<P: AsRef<Path>>(&self, dest: P) -> Result<(), SoftPathError> {
        fs::copy(self, dest)?;
        Ok(())
    }

    fn move_to<P: AsRef<Path>>(&self, dest: P) -> Result<(), SoftPathError> {
        fs::rename(self, dest)?;
        Ok(())
    }

    fn is_empty(&self) -> Result<bool, SoftPathError> {
        self.as_path().is_empty_impl()
    }

    fn is_hidden(&self) -> Result<bool, SoftPathError> {
        self.as_path().is_hidden_impl()
    }

    fn file_name(&self) -> Option<String> {
        self.as_path()
            .file_name()
            .and_then(|s| s.to_str())
            .map(String::from)
    }

    fn extension(&self) -> Option<String> {
        self.as_path()
            .extension()
            .and_then(|s| s.to_str())
            .map(String::from)
    }

    fn parent_name(&self) -> Option<String> {
        self.as_path()
            .parent()
            .and_then(Path::file_name)
            .and_then(|s| s.to_str())
            .map(String::from)
    }

    fn absolute(&self) -> Result<PathBuf, SoftPathError> {
        // First canonicalize the path to resolve any . or .. components
        let canonical = fs::canonicalize(self).map_err(SoftPathError::Io)?;

        // Then run security checks on the resolved path
        crate::utils::check_path_traversal(&canonical)?;
        crate::utils::check_symlink_cycles(&canonical)?;

        Ok(canonical)
    }
}
