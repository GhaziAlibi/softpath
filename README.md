# SoftPath

A human-friendly file and directory path manipulation library for Rust.

## Features

- ✨ Intuitive path operations
- 🔒 Safe path handling with security checks
- 🌍 Cross-platform support (Windows, Unix, macOS)
- 🏃‍♂️ High-performance operations
- 🧪 Comprehensive test suite
- 📝 Well-documented API

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
softpath = "0.1.0"
```

## Quick Start

```rust
use softpath::prelude::*;

fn main() -> Result<(), softpath::SoftPathError> {
    // Create and write to a file
    let config_file = "~/config/app.json".into_path()?;
    config_file.write_string("{\"version\": 1}")?;

    // Copy to backup location
    let backup = "~/config/backup/app.json".into_path()?;
    config_file.copy_to(&backup)?;

    // Read the content
    let content = backup.read_to_string()?;
    println!("Backup content: {}", content);

    Ok(())
}
```

## Safety Features

- Path traversal protection
- Symlink cycle detection
- Maximum path depth checks
- Secure temporary file handling
- Cross-platform path normalization

## Performance

The library is designed with performance in mind:

- Minimal allocations
- Efficient path manipulation
- Smart caching when appropriate
- Benchmarked operations

## License

Licensed under:

- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
