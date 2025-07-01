use std::env;
use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU32, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

// Use a counter to make each test directory unique
static COUNTER: AtomicU32 = AtomicU32::new(0);

/// Helper function to get a unique temporary test directory
pub fn setup_test_dir() -> PathBuf {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let count = COUNTER.fetch_add(1, Ordering::SeqCst);

    let mut test_dir = env::temp_dir();
    test_dir.push(format!(
        "softpath_test_{}_{}_{}" ,
        timestamp,
        std::process::id(),
        count
    ));

    // Clean up any existing directory (shouldn't exist due to unique name)
    if test_dir.exists() {
        let _ = fs::remove_dir_all(&test_dir);
    }

    // Create fresh directory
    fs::create_dir_all(&test_dir).expect("Failed to create test directory");
    test_dir
}

/// Helper function to clean up after tests
pub fn cleanup_test_dir(test_dir: &PathBuf) {
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