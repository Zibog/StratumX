//! Test repository fixtures for repo hygiene tests
//!
//! Provides helper functions to create temporary test repositories
//! with files and directory structures.

use std::fs;
use tempfile::TempDir;

/// Create a temporary test repository directory
///
/// Returns a TempDir that will be automatically cleaned up when dropped.
pub fn create_test_repo() -> TempDir {
    TempDir::new().expect("Failed to create temp dir")
}

/// Create a file in the test repository with the specified content
///
/// # Arguments
/// * `repo` - The temporary directory representing the test repo
/// * `path` - Relative path within the repo (e.g., "src/lib.rs")
/// * `content` - File content to write
pub fn create_file(repo: &TempDir, path: &str, content: &str) {
    let full_path = repo.path().join(path);
    if let Some(parent) = full_path.parent() {
        fs::create_dir_all(parent).expect("Failed to create parent directories");
    }
    fs::write(&full_path, content).expect("Failed to write file");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_test_repo() {
        let repo = create_test_repo();
        assert!(repo.path().exists());
    }

    #[test]
    fn test_create_file_in_repo() {
        let repo = create_test_repo();
        create_file(&repo, "src/lib.rs", "fn main() {}");

        let file_path = repo.path().join("src/lib.rs");
        assert!(file_path.exists());

        let content = fs::read_to_string(&file_path).unwrap();
        assert_eq!(content, "fn main() {}");
    }

    #[test]
    fn test_create_file_creates_parent_dirs() {
        let repo = create_test_repo();
        create_file(&repo, "deep/nested/path/test.rs", "test content");

        let file_path = repo.path().join("deep/nested/path/test.rs");
        assert!(file_path.exists());
    }
}
