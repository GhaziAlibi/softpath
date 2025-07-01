use softpath::prelude::*;
use crate::common::{setup_test_dir, cleanup_test_dir};

mod common;

#[test]
fn test_string_path_operations() -> Result<(), SoftPathError> {
    let test_dir = setup_test_dir();
    assert!(test_dir.exists().unwrap(), "Test directory should exist");

    // Test string-based operations using platform-specific separator
    let file_path = test_dir
        .join("test.txt")
        .to_str()
        .ok_or_else(|| SoftPathError::InvalidPath("Invalid UTF-8 in path".to_string()))?
        .to_string();

    // Test with string slice
    let file_ref: &str = &file_path;
    file_ref.create_file()?;
    assert!(file_ref.exists()?, "File should exist");
    assert!(file_ref.is_file()?, "Should be a file");

    // Test content operations with string paths
    file_ref.write_string("test content")?;
    let content = file_ref.read_to_string()?;
    assert_eq!(content, "test content", "Content should match");

    cleanup_test_dir(&test_dir);
    Ok(())
}

#[test]
fn test_path_reference_operations() {
    let test_dir = setup_test_dir();

    // Test operations with Path references
    let path = test_dir.as_path();
    assert!(path.exists());
    assert!(path.is_dir());

    // Test file operations with Path references
    let file_path = path.join("test.txt");
    let file_ref = file_path.as_path();
    file_ref.create_file().unwrap();
    assert!(file_ref.exists());
    assert!(file_ref.is_file());

    // Test content operations with Path references
    file_ref.write_string("test content").unwrap();
    assert_eq!(file_ref.read_to_string().unwrap(), "test content");

    cleanup_test_dir(&test_dir);
}