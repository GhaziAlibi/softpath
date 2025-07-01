use softpath::prelude::*;
use crate::common::{setup_test_dir, cleanup_test_dir};

mod common;

#[test]
fn test_path_traversal_errors() {
    // Test that path traversal attempts return PathTraversal error
    // Use paths that will definitely traverse above the current directory
    let malicious_paths = vec![
        "../../../../../../../../../etc/passwd",
        "../../../../../../../../../windows/system32", 
        "./../../../../../../../../sensitive/file.txt",
        "normal/../../../../../../../../etc/passwd",
    ];

    for path_str in malicious_paths {
        // Test exists() method
        match path_str.exists() {
            Err(SoftPathError::PathTraversal(_)) => {
                println!("✓ Correctly caught path traversal in exists(): {}", path_str);
            }
            Err(other_error) => {
                panic!("Expected PathTraversal error, got: {:?}", other_error);
            }
            Ok(_) => {
                panic!("Expected error for path traversal attempt: {}", path_str);
            }
        }

        // Test is_file() method
        match path_str.is_file() {
            Err(SoftPathError::PathTraversal(_)) => {
                println!("✓ Correctly caught path traversal in is_file(): {}", path_str);
            }
            Err(_) | Ok(_) => {
                panic!("Expected PathTraversal error for is_file(): {}", path_str);
            }
        }

        // Test is_dir() method
        match path_str.is_dir() {
            Err(SoftPathError::PathTraversal(_)) => {
                println!("✓ Correctly caught path traversal in is_dir(): {}", path_str);
            }
            Err(_) | Ok(_) => {
                panic!("Expected PathTraversal error for is_dir(): {}", path_str);
            }
        }

        // Test file_name() method
        match path_str.file_name() {
            Err(SoftPathError::PathTraversal(_)) => {
                println!("✓ Correctly caught path traversal in file_name(): {}", path_str);
            }
            Err(_) | Ok(_) => {
                panic!("Expected PathTraversal error for file_name(): {}", path_str);
            }
        }
    }
}

#[test]
fn test_path_depth_exceeded_errors() {
    // Create a very deep path that exceeds the maximum depth
    let mut deep_path = String::new();
    for i in 0..300 { // Assuming max depth is 256
        deep_path.push_str(&format!("dir{}/", i));
    }
    deep_path.push_str("file.txt");

    match deep_path.as_str().exists() {
        Err(SoftPathError::PathDepthExceeded(depth)) => {
            println!("✓ Correctly caught path depth exceeded: {} components", depth);
        }
        Err(other_error) => {
            panic!("Expected PathDepthExceeded error, got: {:?}", other_error);
        }
        Ok(_) => {
            panic!("Expected error for overly deep path");
        }
    }
}

#[test]
fn test_invalid_path_errors() {
    // Test paths with invalid characters (null bytes)
    let invalid_paths = vec![
        "file\0name.txt",
        "dir\0/file.txt",
        "\0invalid",
    ];

    for path_str in invalid_paths {
        match path_str.into_path() {
            Err(SoftPathError::InvalidPath(_)) => {
                println!("✓ Correctly caught invalid path: {}", path_str.replace('\0', "\\0"));
            }
            Err(other_error) => {
                println!("Got different error (may be platform-specific): {:?}", other_error);
            }
            Ok(_) => {
                panic!("Expected error for invalid path: {}", path_str.replace('\0', "\\0"));
            }
        }
    }
}

#[test]
fn test_io_errors() {
    // Test operations on non-existent files that should return IO errors
    let nonexistent = "/definitely/does/not/exist/file.txt";
    
    match nonexistent.read_to_string() {
        Err(SoftPathError::Io(io_error)) => {
            println!("✓ Correctly caught IO error: {}", io_error);
            assert_eq!(io_error.kind(), std::io::ErrorKind::NotFound);
        }
        Err(other_error) => {
            panic!("Expected IO error, got: {:?}", other_error);
        }
        Ok(_) => {
            panic!("Expected error when reading non-existent file");
        }
    }
}

#[test]
fn test_error_matching_patterns() {
    let malicious_path = "../../../../../../../../../etc/passwd";
    
    // Pattern 1: Using match with specific error variants
    match malicious_path.exists() {
        Err(SoftPathError::PathTraversal(path)) => {
            println!("Caught path traversal attempt: {}", path);
            assert!(path.contains("etc/passwd") || path.contains("<path>"));
        }
        Err(e) => panic!("Unexpected error type: {:?}", e),
        Ok(_) => panic!("Should have failed validation"),
    }

    // Pattern 2: Using if let for specific error types
    if let Err(SoftPathError::PathTraversal(_)) = malicious_path.is_file() {
        println!("✓ Successfully detected path traversal with if let");
    } else {
        panic!("Expected PathTraversal error");
    }

    // Pattern 3: Using matches! macro for quick checks
    assert!(matches!(
        malicious_path.is_dir(),
        Err(SoftPathError::PathTraversal(_))
    ));

    // Pattern 4: Using unwrap_err() when you're sure there's an error
    let error = malicious_path.file_name().unwrap_err();
    match error {
        SoftPathError::PathTraversal(_) => println!("✓ Got expected error type"),
        _ => panic!("Wrong error type: {:?}", error),
    }
}

#[test]
fn test_error_propagation() {
    // Test that errors are properly propagated through the ? operator
    fn test_function() -> Result<bool, SoftPathError> {
        let malicious_path = "../../../../../../../../../etc/passwd";
        malicious_path.exists() // This will return the PathTraversal error
    }

    match test_function() {
        Err(SoftPathError::PathTraversal(_)) => {
            println!("✓ Error properly propagated through ? operator");
        }
        Err(e) => panic!("Unexpected error: {:?}", e),
        Ok(_) => panic!("Expected error to be propagated"),
    }
}

#[test]
fn test_valid_paths_return_ok() -> Result<(), SoftPathError> {
    let test_dir = setup_test_dir();
    
    // Test that valid paths don't return errors
    let valid_paths = vec![
        "./valid_file.txt",
        "normal/path/file.txt",
        "simple.txt",
    ];

    for path_str in valid_paths {
        let full_path = test_dir.join(path_str);
        // These should not return validation errors (though they may return IO errors)
        match full_path.exists() {
            Ok(exists) => {
                println!("✓ Valid path processed successfully: {} (exists: {})", path_str, exists);
            }
            Err(SoftPathError::PathTraversal(_)) => {
                panic!("Valid path incorrectly flagged as traversal: {}", path_str);
            }
            Err(SoftPathError::PathDepthExceeded(_)) => {
                panic!("Valid path incorrectly flagged as too deep: {}", path_str);
            }
            Err(other_error) => {
                println!("Got other error (may be expected): {:?}", other_error);
            }
        }
    }
    
    cleanup_test_dir(&test_dir);
    Ok(())
}
