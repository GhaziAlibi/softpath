use softpath::prelude::*;
use crate::common::{setup_test_dir, cleanup_test_dir};

mod common;

#[test]
fn test_path_components() {
    let test_dir = setup_test_dir();

    let file_path = test_dir.join("parent_dir").join("test.txt");

    // Test file_name
    assert_eq!(file_path.file_name().unwrap(), Some("test.txt".to_string()));

    // Test extension
    assert_eq!(file_path.extension().unwrap(), Some("txt".to_string()));

    // Test parent_name
    assert_eq!(file_path.parent_name().unwrap(), Some("parent_dir".to_string()));

    // Test path without extension
    let no_ext_path = test_dir.join("file_without_extension");
    assert_eq!(
        no_ext_path.file_name().unwrap(),
        Some("file_without_extension".to_string())
    );
    assert_eq!(no_ext_path.extension().unwrap(), None);

    cleanup_test_dir(&test_dir);
}

#[test]
fn test_absolute_path() -> Result<(), SoftPathError> {
    let test_dir = setup_test_dir();

    // Create a file
    let rel_path = test_dir.join("some_dir/../file.txt");
    rel_path.write_string("content")?;

    let abs_path = rel_path.absolute()?;
    assert!(abs_path.starts_with(test_dir.absolute()?));
    assert!(abs_path.ends_with("file.txt"));
    assert!(!abs_path.to_str().unwrap().contains(".."));

    cleanup_test_dir(&test_dir);
    Ok(())
}