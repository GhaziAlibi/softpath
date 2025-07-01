use softpath::prelude::*;
use crate::common::{setup_test_dir, cleanup_test_dir};
use std::sync::Arc;
use std::thread;

mod common;

#[test]
fn test_concurrent_operations() {
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