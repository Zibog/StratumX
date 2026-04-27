//! Temporary filesystem fixture for testing

use std::fs;
use std::path::{Path, PathBuf};

/// Temporary directory that cleans up on drop
pub struct TempDir {
    path: PathBuf,
}

impl Default for TempDir {
    fn default() -> Self {
        Self::new()
    }
}

impl TempDir {
    pub fn new() -> Self {
        let path = std::env::temp_dir().join(format!("stratumx_test_{}", uuid::Uuid::new_v4()));
        fs::create_dir_all(&path).unwrap();
        Self { path }
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn create_file(&self, name: &str, content: &str) -> PathBuf {
        let file_path = self.path.join(name);
        fs::write(&file_path, content).unwrap();
        file_path
    }

    pub fn create_dir(&self, name: &str) -> PathBuf {
        let dir_path = self.path.join(name);
        fs::create_dir_all(&dir_path).unwrap();
        dir_path
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.path);
    }
}

/// Creates a temporary directory for testing
pub fn create_temp_dir() -> TempDir {
    TempDir::new()
}
