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
            // Use create_dir_all which is more atomic than separate checks
            fs::create_dir_all(parent)?;
        }
        
        // Use OpenOptions for more controlled file creation
        use std::fs::OpenOptions;
        OpenOptions::new()
            .write(true)
            .create_new(true) // Fails if file already exists, preventing race conditions
            .open(&path)
            .map_err(|e| {
                if e.kind() == std::io::ErrorKind::AlreadyExists {
                    std::io::Error::new(
                        std::io::ErrorKind::AlreadyExists,
                        format!("File already exists: {}", path.display())
                    )
                } else {
                    e
                }
            })?;
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
        
        // Use atomic write operation with temporary file to reduce TOCTOU risks
        use std::io::Write;
        let temp_path = path.with_extension("tmp");
        
        // Write to temporary file first
        {
            let mut temp_file = fs::File::create(&temp_path)?;
            temp_file.write_all(contents.as_bytes())?;
            temp_file.sync_all()?; // Ensure data is written to disk
        }
        
        // Atomically rename temporary file to target (atomic on most filesystems)
        fs::rename(&temp_path, &path)?;
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
