use softpath::prelude::*;
use std::env;
use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU32, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

// Use a counter to make each test directory unique
static COUNTER: AtomicU32 = AtomicU32::new(0);

// Helper function to get a unique temporary test directory
fn setup_test_dir() -> PathBuf {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let count = COUNTER.fetch_add(1, Ordering::SeqCst);

    let mut test_dir = env::temp_dir();
    test_dir.push(format!(
        "softpath_test_{}_{}_{}",
        timestamp,
        std::process::id(),
        count
    ));

    // Clean up any existing directory (shouldn't exist due to unique name)
    if test_dir.exists().unwrap_or(false) {
        let _ = fs::remove_dir_all(&test_dir);
    }

    // Create fresh directory
    fs::create_dir_all(&test_dir).expect("Failed to create test directory");
    test_dir
}

// Helper function to clean up after tests
fn cleanup_test_dir(test_dir: &PathBuf) {
    // Give the filesystem a moment to release any file handles
    std::thread::sleep(std::time::Duration::from_millis(100));

    let mut retries = 3;
    while retries > 0 {
        match fs::remove_dir_all(test_dir) {
            Ok(_) => break,
            Err(e) => {
                if retries == 1 {
                    // Only panic on the last retry
                    panic!(
                        "Failed to clean up test directory after {} retries: {}",
                        4 - retries,
                        e
                    );
                }
                // Wait a bit before retrying
                std::thread::sleep(std::time::Duration::from_millis(100));
                retries -= 1;
            }
        }
    }
}

#[test]
fn test_path_creation() {
    let test_dir = setup_test_dir();
    assert!(test_dir.exists().unwrap(), "Base test directory should exist");
    assert!(
        test_dir.is_dir().unwrap(),
        "Base test directory should be a directory"
    );

    // Test nested directory creation
    let config_dir = test_dir.join("config");
    assert!(
        !config_dir.exists().unwrap(),
        "Config directory should not exist yet"
    );
    config_dir.create_dir_all().unwrap();
    assert!(
        config_dir.exists().unwrap(),
        "Config directory should exist after creation"
    );
    assert!(
        config_dir.is_dir().unwrap(),
        "Config directory should be a directory"
    );

    // Test creating directory with existing parent
    let sub_config = config_dir.join("subdir");
    sub_config.create_dir_all().unwrap();
    assert!(sub_config.exists().unwrap(), "Subdirectory should exist");
    assert!(sub_config.is_dir().unwrap(), "Subdirectory should be a directory");

    cleanup_test_dir(&test_dir);
}

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
fn test_nested_directories() {
    let test_dir = setup_test_dir();

    // Create nested directory structure
    let nested_dir = test_dir.join("deeply").join("nested").join("dir");
    nested_dir.create_dir_all().unwrap();
    assert!(nested_dir.exists().unwrap());

    // Create file in nested directory
    let nested_file = nested_dir.join("test.txt");
    nested_file.create_file().unwrap();
    assert!(nested_file.exists().unwrap());

    cleanup_test_dir(&test_dir);
}

#[test]
fn test_directory_removal() {
    let test_dir = setup_test_dir();

    // Create directory with contents
    let sub_dir = test_dir.join("subdir");
    sub_dir.create_dir_all().unwrap();
    sub_dir.join("file1.txt").create_file().unwrap();
    sub_dir.join("file2.txt").create_file().unwrap();

    // Test recursive removal
    test_dir.remove().unwrap();
    assert!(!test_dir.exists().unwrap());
}

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

#[test]
#[cfg(unix)]
fn test_unix_specific_paths() {
    let test_dir = setup_test_dir();

    // Test absolute path
    let abs_path = test_dir.join("unix_test.txt");
    abs_path.create_file().unwrap();
    assert!(abs_path.exists().unwrap());

    cleanup_test_dir(&test_dir);
}

#[test]
#[cfg(windows)]
fn test_windows_specific_paths() {
    let test_dir = setup_test_dir();

    // Test Windows-style paths
    let file_path = test_dir.join("windows_test.txt");
    file_path.create_file().unwrap();
    assert!(file_path.exists().unwrap());

    // Test path with spaces
    let space_path = test_dir.join("with spaces.txt");
    space_path.create_file().unwrap();
    assert!(space_path.exists().unwrap());

    cleanup_test_dir(&test_dir);
}

#[test]
fn test_concurrent_operations() {
    use std::sync::Arc;
    use std::thread;

    let test_dir = Arc::new(setup_test_dir());
    let dir_clone = Arc::clone(&test_dir);

    // Spawn thread for concurrent operations
    let handle = thread::spawn(move || -> Result<(), SoftPathError> {
        let thread_file = dir_clone.join("thread.txt");
        thread_file.create_file()?;
        thread_file.write_string("thread content")?;
        assert!(thread_file.exists()?);
        Ok(())
    });

    // Main thread operations
    let main_file = test_dir.join("main.txt");
    main_file.create_file().unwrap();
    main_file.write_string("main content").unwrap();
    assert!(main_file.exists().unwrap());

    // Wait for thread to complete
    handle.join().unwrap().unwrap();

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
