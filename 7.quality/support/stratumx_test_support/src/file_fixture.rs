//! File fixture for testing

use std::path::PathBuf;

/// Creates a temporary test file path
pub fn test_file_path() -> PathBuf {
    PathBuf::from("/test/fixtures/world.test")
}

/// Creates a temporary directory path
pub fn test_dir_path() -> PathBuf {
    PathBuf::from("/test/fixtures")
}
