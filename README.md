[![Crates.io](https://img.shields.io/crates/v/softpath.svg)](https://crates.io/crates/softpath)[![Docs.rs](https://docs.rs/softpath/badge.svg)](https://docs.rs/softpath)[![Build Status](https://github.com/GhaziAlibi/softpath/actions/workflows/rust.yml/badge.svg)](https://github.com/GhaziAlibi/softpath/actions/workflows/rust.yml)[![License](https://img.shields.io/crates/l/softpath.svg)](https://crates.io/crates/softpath)[![Rust Version](https://img.shields.io/badge/rust-1.70+-blue.svg)](https://blog.rust-lang.org/)[![Downloads](https://img.shields.io/crates/d/softpath.svg)](https://crates.io/crates/softpath)
[![🇵🇸 Free Palestine](https://img.shields.io/badge/🇵🇸_Free_Palestine-red.svg)](https://en.wikipedia.org/wiki/State_of_Palestine)


# SoftPath

A safe and intuitive path manipulation library for Rust that actually cares about security.

## Why SoftPath?

Working with file paths in Rust shouldn't be a security nightmare. We built SoftPath because we got tired of seeing the same path traversal vulnerabilities pop up in codebases over and over again.

**What you get:**
- Simple, chainable API that feels natural to use
- Automatic protection against path traversal attacks
- Cross-platform support (Windows, Unix, macOS)
- Fast operations that don't sacrifice security for speed
- Comprehensive tests (because we actually run them)
- Documentation that doesn't assume you're a security expert

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
softpath = "0.2.1"
```

## Quick Start

Here's how easy it is to work with paths safely:

```rust
use softpath::prelude::*;

fn main() -> Result<(), softpath::SoftPathError> {
    // Create and write to a file
    let config_file = "~/config/app.json".into_path()?;
    config_file.write_string("{\"version\": 1}")?;

    // Copy it somewhere else
    let backup = "~/config/backup/app.json".into_path()?;
    config_file.copy_to(&backup)?;

    // Create directories as needed
    "~/data/logs".into_path()?.create_dir_all()?;

    // Read it back
    let content = backup.read_to_string()?;
    println!("Backup content: {}", content);

    Ok(())
}
```

That's it. No `../../../etc/passwd` nonsense will get through.



## Security Features

Here's what protects you from the usual path-related disasters:

**Path Traversal Protection**  
No more `../../../etc/passwd` attacks. We check every path before doing anything with it.

**Symlink Cycle Detection**  
Prevents infinite symlink loops that could crash your program or eat up resources.

**TOCTOU Prevention**  
We validate paths right before using them, not way earlier when things might have changed.

**Destination Validation**  
Before copying or moving files, we make sure you're not accidentally overwriting something important.

**Cross-platform Consistency**  
Same security behavior whether you're on Windows, Linux, or macOS.

## Performance

Security doesn't mean slow. We've made sure the safety checks don't kill your performance:

- Minimal memory allocations (even with all the validation)
- Security checks are fast and don't block operations
- We cache validation results when it makes sense
- The safety features add virtually no overhead to normal operations

## How to Use This Safely

**Do this:**
```rust
// Let SoftPath handle the validation
let user_path = user_input.into_path()?;
user_path.write_string(content)?;
```

**Don't do this:**
```rust
// Bypasses all our safety checks
std::fs::write(user_input, content);
```

**A few simple rules:**
1. Use SoftPath methods instead of `std::fs` when you can
2. Always call `.into_path()` on user input before doing file operations
3. Handle the errors - they'll tell you when something sketchy is happening
4. Keep the library updated (we fix things when we find them)

**Error handling example:**
```rust
match sketchy_path.into_path() {
    Ok(safe_path) => safe_path.create_dir_all()?,
    Err(SoftPathError::PathTraversal(_)) => {
        // Someone tried something sneaky
        return Err("Nice try, but no".into());
    }
    Err(e) => return Err(e),
}
```

## License

Licensed under:

- MIT license ([LICENSE-MIT](LICENSE) or http://opensource.org/licenses/MIT)

## Contributing

Want to help make this better? Great! Just keep security in mind.

**If you're adding new features:**
- Make sure they don't bypass our path validation
- Add tests (especially for the security stuff)
- Update the docs if you change the API
- Run `cargo test` before submitting

**If you find a security issue:**
- Please use GitHub Security Advisories instead of opening a public issue
- We'll fix it quickly and give you credit

**Before submitting a PR:**
```bash
# Make sure everything still works
cargo test

# Check that security tests pass
cargo test security

# Make sure you didn't break performance
cargo bench

# Check for known vulnerabilities
cargo audit
```

We're pretty responsive to PRs, especially if they fix bugs or improve security.
