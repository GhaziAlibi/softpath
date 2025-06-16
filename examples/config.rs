use softpath::{Config, PathExt};
use std::path::PathBuf;

fn main() {
    // Create a custom configuration
    let config = Config::new()
        .with_max_path_depth(100) // Limit path depth to 100 components
        .with_max_symlink_depth(10); // Limit symlink following to 10 levels

    // Set the global configuration
    softpath::set_config(config);

    // Now all operations will use these limits
    let path = PathBuf::from("some/deep/path");

    // This will now use your custom limits for security checks
    match path.create_file() {
        Ok(_) => println!("File created successfully"),
        Err(e) => println!("Error: {}", e),
    }
}
