use softpath::prelude::*;
use crate::common::{setup_test_dir, cleanup_test_dir};

mod common;

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