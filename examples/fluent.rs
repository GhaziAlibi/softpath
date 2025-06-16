use softpath::prelude::*;
use std::env;

fn main() -> Result<(), SoftPathError> {
    println!("SoftPath Fluent API Demo\n");

    // Get base directory for our demo
    let base_dir = env::current_dir()?.join("fluent_demo");
    println!("Working directory: {:?}\n", base_dir);

    // Example 1: Create a directory structure and file in one chain
    let config_dir = base_dir.join("config");
    config_dir.create_dir_all()?;
    config_dir.join("settings.json").create_file()?;
    println!("1. Created directory structure with file");

    // Example 2: Write and read file in one chain
    let settings_path = config_dir.join("settings.json");
    settings_path.write_string("{\"name\": \"fluent-demo\"}")?;
    let content = settings_path.read_to_string()?;
    println!("2. File content: {}", content);

    // Example 3: Chain multiple checks
    let file_status = if settings_path.exists() && settings_path.is_file() {
        "exists and is a file"
    } else {
        "does not exist or is not a file"
    };
    println!("3. File status: {}", file_status);

    // Example 4: Create backup directory and copy file in chain
    let backup_path = base_dir.join("backup");
    backup_path.create_dir_all()?;
    settings_path.copy_to(backup_path.join("settings.json.bak"))?;
    println!("4. File copied to backup location");

    // Example 5: Move file with verification
    let source = backup_path.join("settings.json.bak");
    let target = backup_path.join("settings_final.json");
    source.move_to(&target)?;
    println!(
        "5. File moved successfully: {}",
        !source.exists() && target.exists()
    );

    // Example 6: Recursive cleanup
    println!("6. Cleaning up test files and directories...");
    base_dir.remove()?;
    println!("   Directory removed successfully");

    // Example 7: Complex operation chain with temporary variables
    {
        let deep_config = base_dir.join("nested").join("deep").join("config");
        deep_config.create_dir_all()?;

        let test_file = deep_config.join("test.txt");
        test_file.create_file()?;
        test_file.write_string("Hello, Fluent API!")?;

        let content = test_file.read_to_string()?;
        println!("7. Complex operation result: {}", content);
    }

    // Final cleanup
    if base_dir.exists() {
        base_dir.remove()?;
    }

    println!("\nFluent API demo completed successfully!");
    Ok(())
}
