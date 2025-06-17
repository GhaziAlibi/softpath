use crate::error::SoftPathError;
use std::fs;
use std::path::{Path, PathBuf};

/// Internal trait providing default implementations for common path operations
pub(crate) trait PathOps {
    fn as_path(&self) -> &Path;

    fn create_file_impl(&self) -> Result<(), SoftPathError> {
        let path = self.as_path().to_path_buf();
        crate::utils::check_path_traversal(&path)?;
        crate::utils::check_symlink_cycles(&path)?;
        
        // Validate parent directory separately to prevent TOCTOU
        if let Some(parent) = path.parent() {
            crate::utils::check_path_traversal(parent)?;
            crate::utils::check_symlink_cycles(parent)?;
            fs::create_dir_all(parent)?;
        }
        fs::File::create(path)?;
        Ok(())
    }

    fn write_string_impl(&self, contents: &str) -> Result<(), SoftPathError> {
        let path = self.as_path().to_path_buf();
        crate::utils::check_path_traversal(&path)?;
        crate::utils::check_symlink_cycles(&path)?;
        
        // Validate parent directory separately to prevent TOCTOU
        if let Some(parent) = path.parent() {
            crate::utils::check_path_traversal(parent)?;
            crate::utils::check_symlink_cycles(parent)?;
            fs::create_dir_all(parent)?;
        }
        fs::write(path, contents)?;
        Ok(())
    }

    fn remove_impl(&self) -> Result<(), SoftPathError> {
        let path = self.as_path().to_path_buf();
        crate::utils::check_path_traversal(&path)?;
        crate::utils::check_symlink_cycles(&path)?;
        if path.is_dir() {
            fs::remove_dir_all(path)?;
        } else {
            fs::remove_file(path)?;
        }
        Ok(())
    }

    fn is_empty_impl(&self) -> Result<bool, SoftPathError> {
        let path = self.as_path().to_path_buf();
        crate::utils::check_path_traversal(&path)?;
        crate::utils::check_symlink_cycles(&path)?;
        if path.is_file() {
            Ok(fs::metadata(&path)?.len() == 0)
        } else if path.is_dir() {
            Ok(fs::read_dir(&path)?.next().is_none())
        } else {
            Ok(false)
        }
    }

    fn is_hidden_impl(&self) -> Result<bool, SoftPathError> {
        let path = self.as_path().to_path_buf();
        crate::utils::check_path_traversal(&path)?;
        crate::utils::check_symlink_cycles(&path)?;
        #[cfg(windows)]
        {
            use std::os::windows::fs::MetadataExt;
            let attr = fs::metadata(&path)?.file_attributes();
            Ok((attr & 0x2) != 0) // FILE_ATTRIBUTE_HIDDEN
        }
        #[cfg(unix)]
        {
            Ok(path
                .file_name()
                .map(|s| s.to_string_lossy().to_string())
                .is_some_and(|s| s.starts_with('.')))
        }
    }
}

// Basic PathOps implementations
impl PathOps for PathBuf {
    fn as_path(&self) -> &Path {
        self.as_path()
    }
}

impl PathOps for &Path {
    fn as_path(&self) -> &Path {
        self
    }
}
