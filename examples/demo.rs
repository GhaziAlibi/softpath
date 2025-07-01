use softpath::prelude::*;
use std::env;

fn main() -> Result<(), SoftPathError> {
    println!("SoftPath Demo - Path manipulation made easy!\n");

    // Get the current directory for our demo
    let current_dir = env::current_dir()?;
    let test_dir = current_dir.join("softpath_demo");
    println!("1. Working directory: {:?}", test_dir);

    // Create directory and check existence
    test_dir.create_dir_all()?;
    assert!(test_dir.exists()?);
    assert!(test_dir.is_dir()?);
    println!("2. Created directory and verified its existence");

    // Create a file in the directory
    let file_path = test_dir.join("hello.txt");
    file_path.create_file()?;
    assert!(file_path.exists()?);
    assert!(file_path.is_file()?);
    println!("3. Created file: {:?}", file_path);

    // Write content to the file
    let content = "Hello from SoftPath!\nThis is a demo of path manipulation.";
    file_path.write_string(content)?;
    println!("4. Wrote content to file");

    // Read content back
    let read_content = file_path.read_to_string()?;
    assert_eq!(content, read_content);
    println!("5. Successfully read back the content");

    // Create subdirectories for copy/move operations
    let source_dir = test_dir.join("source");
    let backup_dir = test_dir.join("backup");
    source_dir.create_dir_all()?;
    backup_dir.create_dir_all()?;

    // Create a file for copy/move operations
    let source_file = source_dir.join("data.txt");
    source_file.write_string("This file will be copied and moved")?;
    println!("6. Created source file: {:?}", source_file);

    // Copy the file
    let copy_dest = backup_dir.join("data-backup.txt");
    source_file.copy_to(&copy_dest)?;
    assert!(copy_dest.exists()?);
    println!("7. Copied file to: {:?}", copy_dest);

    // Move the file
    let move_dest = backup_dir.join("data-final.txt");
    source_file.move_to(&move_dest)?;
    assert!(!source_file.exists()?);
    assert!(move_dest.exists()?);
    println!("8. Moved file to: {:?}", move_dest);

    // Clean up - remove the whole test directory
    println!("\nCleaning up...");
    test_dir.remove()?;
    assert!(!test_dir.exists()?);
    println!("Successfully removed test directory and all its contents");

    Ok(())
}
