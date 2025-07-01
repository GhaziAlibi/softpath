use std::fs;
use std::path::{Path, PathBuf};
use crate::error::SoftPathError;
use crate::ops::PathExt;

impl PathExt for &str {
    fn into_path(self) -> Result<PathBuf, SoftPathError> {
        // Check for null bytes which are invalid in paths
        if self.contains('\0') {
            return Err(SoftPathError::InvalidPath(
                "Path contains null bytes".to_string()
            ));
        }
        
        // First validate the input string for basic path traversal patterns
        let input_path = PathBuf::from(self);
        
        let path = if let Some(path) = self.strip_prefix("~/").or_else(|| self.strip_prefix("~\\")) {
            // Validate the path component after tilde expansion before joining
            let path_component = PathBuf::from(path);
            crate::utils::check_path_traversal(&path_component)?;
            
            let home = dirs::home_dir().ok_or_else(|| {
                SoftPathError::InvalidPath("Could not determine home directory.".to_string())
            })?;
            home.join(path)
        } else {
            input_path
        };
        
        // Final validation of the complete path
        crate::utils::check_path_traversal(&path)?;
        crate::utils::check_symlink_cycles(&path)?;
        Ok(path)
    }

    fn exists(&self) -> Result<bool, SoftPathError> {
        // Convert to path and validate, then check existence
        let path = self.into_path()?;
        Ok(path.exists()?)
    }

    fn is_file(&self) -> Result<bool, SoftPathError> {
        // Convert to path and validate, then check if it's a file
        let path = self.into_path()?;
        Ok(path.is_file()?)
    }

    fn is_dir(&self) -> Result<bool, SoftPathError> {
        // Convert to path and validate, then check if it's a directory
        let path = self.into_path()?;
        Ok(path.is_dir()?)
    }

    fn create_file(&self) -> Result<(), SoftPathError> {
        let path = self.into_path()?;
        path.create_file()
    }

    fn create_dir_all(&self) -> Result<(), SoftPathError> {
        fs::create_dir_all(self.into_path()?).map_err(SoftPathError::from)
    }

    fn remove(&self) -> Result<(), SoftPathError> {
        let path = self.into_path()?;
        path.remove()
    }

    fn read_to_string(&self) -> Result<String, SoftPathError> {
        let path = self.into_path()?;
        fs::read_to_string(path).map_err(SoftPathError::from)
    }

    fn write_string(&self, contents: &str) -> Result<(), SoftPathError> {
        let path = self.into_path()?;
        path.write_string(contents)
    }

    fn copy_to<P: AsRef<Path>>(&self, dest: P) -> Result<(), SoftPathError> {
        let from = self.into_path()?;
        let dest_path = dest.as_ref();
        crate::utils::check_path_traversal(dest_path)?;
        crate::utils::check_symlink_cycles(dest_path)?;
        fs::copy(&from, dest)?;
        Ok(())
    }

    fn move_to<P: AsRef<Path>>(&self, dest: P) -> Result<(), SoftPathError> {
        let from = self.into_path()?;
        let dest_path = dest.as_ref();
        crate::utils::check_path_traversal(dest_path)?;
        crate::utils::check_symlink_cycles(dest_path)?;
        fs::rename(&from, dest)?;
        Ok(())
    }

    fn is_empty(&self) -> Result<bool, SoftPathError> {
        let path = self.into_path()?;
        path.is_empty()
    }

    fn is_hidden(&self) -> Result<bool, SoftPathError> {
        let path = self.into_path()?;
        path.is_hidden()
    }

    fn file_name(&self) -> Result<Option<String>, SoftPathError> {
        // Convert to path and validate, then get the file name
        let path = self.into_path()?;
        Ok(path.as_path()
            .file_name()
            .and_then(|s| s.to_str())
            .map(String::from))
    }

    fn extension(&self) -> Result<Option<String>, SoftPathError> {
        // Convert to path and validate, then get the extension
        let path = self.into_path()?;
        Ok(path.as_path()
            .extension()
            .and_then(|s| s.to_str())
            .map(String::from))
    }

    fn parent_name(&self) -> Result<Option<String>, SoftPathError> {
        // Convert to path and validate, then get the parent name
        let path = self.into_path()?;
        Ok(path.parent()
            .and_then(|p| p.file_name())
            .and_then(|s| s.to_str())
            .map(String::from))
    }

    fn absolute(&self) -> Result<PathBuf, SoftPathError> {
        let path = self.into_path()?;
        Ok(fs::canonicalize(path)?)
    }
}
