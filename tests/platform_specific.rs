use softpath::prelude::*;
use crate::common::{setup_test_dir, cleanup_test_dir};

mod common;

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