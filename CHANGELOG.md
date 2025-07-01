# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0] - 2025-01-20

### Added
- Comprehensive test examples for SoftPathError variants
- Test patterns for PathTraversal, PathDepthExceeded, InvalidPath, and Io errors
- Error matching examples using match statements, if let, matches! macro, and unwrap_err()
- Error propagation test examples

## [0.1.2] - 2025-01-20

### Security
- Implemented atomic file operations to prevent TOCTOU race conditions
- Added path sanitization in error messages for production builds
- Enhanced error handling with debug logging in string implementations
- Updated documentation examples to use proper error handling
- Fixed error enum consistency for symlink cycle detection

### Fixed
- File creation now uses `OpenOptions::create_new(true)` for atomic creation
- File writing uses temporary files with atomic rename operations
- Silent error masking in string path operations now includes debug logging
- Documentation examples no longer use panic-prone `expect()` calls
- Error messages now sanitize paths in production builds

### Changed
- Enhanced security posture with multiple atomic operation improvements
- Improved debugging capabilities with conditional logging
- Better error handling patterns demonstrated in documentation

## [0.1.1] - 2025-06-18

### Security
- Fixed critical vulnerabilities in `create_dir_all()`, `copy_to()`, and `move_to()` (they weren't validating paths at all)
- Resolved TOCTOU race conditions in `absolute()`, `create_file()`, and `write_string()`
- Added proper validation before file operations instead of after
- Security rating improved from HIGH RISK to LOW RISK

## [0.1.0] - Initial Release

### Added
- Initial implementation of SoftPath library
- Basic path manipulation operations
- Cross-platform support (Windows, Unix, macOS)
- Path traversal protection
- Symlink cycle detection