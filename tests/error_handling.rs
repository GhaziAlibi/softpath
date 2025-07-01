use softpath::prelude::*;
use crate::common::{setup_test_dir, cleanup_test_dir};

mod common;

#[test]
fn test_error_handling() -> Result<(), SoftPathError> {
    let test_dir = setup_test_dir();

    // Test reading from nonexistent file
    let nonexistent = test_dir.join("nonexistent.txt");
    match nonexistent.read_to_string() {
        Err(e) => println!("Expected error when reading nonexistent file: {:?}", e),
        Ok(_) => panic!("Reading nonexistent file should fail"),
    }

    // Test writing to nonexistent parent directory (should create it)
    let nested_file = test_dir.join("new_dir").join("file.txt");
    nested_file.write_string("test")?;
    assert!(
        nested_file.exists().unwrap(),
        "File should be created in new directory"
    );

    // Test removing nonexistent path
    let nonexistent_dir = test_dir.join("does_not_exist");
    match nonexistent_dir.remove() {
        Err(e) => println!("Expected error when removing nonexistent path: {:?}", e),
        Ok(_) => panic!("Removing nonexistent path should fail"),
    }

    // Test invalid path handling
    let invalid_path = test_dir.join("invalid\0file.txt"); // Null byte in filename
    match invalid_path.create_file() {
        Err(e) => println!("Expected error for invalid path: {:?}", e),
        Ok(_) => panic!("Creating file with invalid path should fail"),
    }

    // Clean up
    cleanup_test_dir(&test_dir);
    Ok(())
}