use softpath::prelude::*;
use crate::common::{setup_test_dir, cleanup_test_dir};

mod common;

#[test]
fn test_file_operations() -> Result<(), SoftPathError> {
    let test_dir = setup_test_dir();
    assert!(test_dir.exists()?, "Test directory should exist");

    // Create a test file
    let file_path = test_dir.join("test.txt");
    file_path.create_file()?;
    assert!(file_path.exists()?, "File should exist after creation");
    assert!(file_path.is_file()?, "Should be a file");

    // Write and read content
    let content = "Hello, SoftPath!";
    file_path.write_string(content)?;
    let read_content = file_path.read_to_string()?;
    assert_eq!(content, read_content, "File content should match");

    cleanup_test_dir(&test_dir);
    Ok(())
}

#[test]
fn test_copy_and_move() {
    let test_dir = setup_test_dir();

    // Create source file
    let source = test_dir.join("source.txt");
    source.create_file().unwrap(); // Create the file first
    source.write_string("test content").unwrap();
    assert!(source.exists().unwrap(), "Source file should exist");

    // Test copy
    let copy_dest = test_dir.join("copied.txt");
    source.copy_to(&copy_dest).unwrap();
    assert!(copy_dest.exists().unwrap(), "Copied file should exist");
    assert!(source.exists().unwrap(), "Source file should still exist after copy");
    assert_eq!(
        source.read_to_string().unwrap(),
        copy_dest.read_to_string().unwrap(),
        "Content should be identical"
    );

    // Test move
    let move_dest = test_dir.join("moved.txt");
    source.move_to(&move_dest).unwrap();
    assert!(!source.exists().unwrap(), "Source file should not exist after move");
    assert!(move_dest.exists().unwrap(), "Moved file should exist");
    assert_eq!(
        "test content",
        move_dest.read_to_string().unwrap(),
        "Content should be preserved after move"
    );

    cleanup_test_dir(&test_dir);
}

#[test]
fn test_is_empty() -> Result<(), SoftPathError> {
    let test_dir = setup_test_dir();

    // Test empty directory
    let empty_dir = test_dir.join("empty_dir");
    empty_dir.create_dir_all()?;
    assert!(
        empty_dir.is_empty()?,
        "Newly created directory should be empty"
    );

    // Test non-empty directory
    let non_empty_dir = test_dir.join("non_empty_dir");
    non_empty_dir.create_dir_all()?;
    non_empty_dir.join("file.txt").write_string("content")?;
    assert!(
        !non_empty_dir.is_empty()?,
        "Directory with file should not be empty"
    );

    // Test empty file
    let empty_file = test_dir.join("empty.txt");
    empty_file.write_string("")?;
    assert!(empty_file.is_empty()?, "Empty file should be empty");

    // Test non-empty file
    let non_empty_file = test_dir.join("non_empty.txt");
    non_empty_file.write_string("content")?;
    assert!(
        !non_empty_file.is_empty()?,
        "File with content should not be empty"
    );

    cleanup_test_dir(&test_dir);
    Ok(())
}

#[test]
fn test_hidden_files() -> Result<(), SoftPathError> {
    let test_dir = setup_test_dir();

    // Test hidden file (Unix-style)
    let hidden_file = test_dir.join(".hidden");
    hidden_file.write_string("content")?;

    // Test visible file
    let visible_file = test_dir.join("visible");
    visible_file.write_string("content")?;

    #[cfg(unix)]
    {
        assert!(
            hidden_file.is_hidden()?,
            "Dot-prefixed file should be hidden on Unix"
        );
        assert!(
            !visible_file.is_hidden()?,
            "Non-dot-prefixed file should not be hidden on Unix"
        );
    }

    cleanup_test_dir(&test_dir);
    Ok(())
}