use super::super::operations::PathOps;
use crate::error::SoftPathError;
use crate::ops::PathExt;
use std::fs;
use std::path::{Path, PathBuf};

impl PathExt for PathBuf {
    fn into_path(self) -> Result<PathBuf, SoftPathError> {
        Ok(self)
    }

    fn exists(&self) -> bool {
        self.as_path().exists()
    }

    fn is_file(&self) -> bool {
        self.as_path().is_file()
    }

    fn is_dir(&self) -> bool {
        self.as_path().is_dir()
    }

    fn create_file(&self) -> Result<(), SoftPathError> {
        self.create_file_impl()
    }

    fn create_dir_all(&self) -> Result<(), SoftPathError> {
        crate::utils::check_path_traversal(self)?;
        crate::utils::check_symlink_cycles(self)?;
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
        let dest_path = dest.as_ref();
        crate::utils::check_path_traversal(dest_path)?;
        crate::utils::check_symlink_cycles(dest_path)?;
        fs::copy(self, dest)?;
        Ok(())
    }

    fn move_to<P: AsRef<Path>>(&self, dest: P) -> Result<(), SoftPathError> {
        let dest_path = dest.as_ref();
        crate::utils::check_path_traversal(dest_path)?;
        crate::utils::check_symlink_cycles(dest_path)?;
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
        Path::file_name(self)
            .and_then(|s| s.to_str())
            .map(String::from)
    }

    fn extension(&self) -> Option<String> {
        Path::extension(self)
            .and_then(|s| s.to_str())
            .map(String::from)
    }

    fn parent_name(&self) -> Option<String> {
        Path::parent(self)
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
