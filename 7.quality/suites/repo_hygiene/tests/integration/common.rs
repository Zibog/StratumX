// Integration test utilities and mock repository helpers

use std::fs;
use std::path::{Path, PathBuf};
use tempfile::TempDir;

/// Mock repository structure for integration testing
pub struct MockRepo {
    pub _temp_dir: TempDir,
    pub root: PathBuf,
}

impl MockRepo {
    /// Create a new mock repository with basic structure
    pub fn new() -> Self {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let root = temp_dir.path().to_path_buf();

        Self {
            _temp_dir: temp_dir,
            root,
        }
    }

    /// Create a directory structure
    pub fn create_dir(&self, path: &str) {
        let full_path = self.root.join(path);
        fs::create_dir_all(full_path).expect("Failed to create directory");
    }

    /// Create a file with content
    pub fn create_file(&self, path: &str, content: &str) {
        let full_path = self.root.join(path);
        if let Some(parent) = full_path.parent() {
            fs::create_dir_all(parent).expect("Failed to create parent directory");
        }
        fs::write(full_path, content).expect("Failed to write file");
    }

    /// Check if a file exists
    pub fn file_exists(&self, path: &str) -> bool {
        self.root.join(path).exists()
    }

    /// Read file content
    pub fn read_file(&self, path: &str) -> String {
        fs::read_to_string(self.root.join(path)).expect("Failed to read file")
    }

    /// Get the root path
    pub fn root_path(&self) -> &Path {
        &self.root
    }
}

/// Create a mock repository with test files in src/ directories
pub fn create_repo_with_test_files() -> MockRepo {
    let repo = MockRepo::new();

    // Create production code structure
    repo.create_dir("5.editor/l7.0-editor-command-spine/src");
    repo.create_dir("5.editor/l8.0-editor-shell/src");
    repo.create_dir("6.apps/editor/stratumx_editor_app/src/desktop_app");
    repo.create_dir("4.tooling/l6.0-tool-session/src");
    repo.create_dir("5.editor/editor-state-containers/src");

    // Create test files in src/ (violations)
    repo.create_file(
        "5.editor/l7.0-editor-command-spine/src/command_tests.rs",
        r#"
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_command_dispatch() {
        assert_eq!(2 + 2, 4);
    }
}
"#,
    );

    repo.create_file(
        "5.editor/l8.0-editor-shell/src/shell_tests.rs",
        r#"
#[cfg(test)]
mod tests {
    #[test]
    fn test_shell_init() {
        assert_eq!(2 + 2, 4);
    }
}
"#,
    );

    repo.create_file(
        "6.apps/editor/stratumx_editor_app/src/desktop_app/panel_tests.rs",
        r#"
#[cfg(test)]
mod tests {
    #[test]
    fn test_panel_render() {
        assert_eq!(3 * 3, 9);
    }
}
"#,
    );

    repo
}

/// Create a mock repository with hygiene violations
pub fn create_repo_with_violations() -> MockRepo {
    let repo = MockRepo::new();

    repo.create_dir("2.engine/l1-foundation/src");

    // File exceeding 200 lines
    let mut long_content = String::new();
    for i in 0..250 {
        long_content.push_str(&format!("// Line {}\n", i));
    }
    repo.create_file("2.engine/l1-foundation/src/long_file.rs", &long_content);

    // File with TODO comment
    repo.create_file(
        "2.engine/l1-foundation/src/with_todo.rs",
        r#"
pub fn some_function() {
    // TODO: Implement this properly
    unimplemented!()
}
"#,
    );

    // File with #[allow(...)] attribute
    repo.create_file(
        "2.engine/l1-foundation/src/with_allow.rs",
        r#"
#[allow(dead_code)]
pub fn unused_function() {
    println!("This is never called");
}
"#,
    );

    // File with static mut
    repo.create_file(
        "2.engine/l1-foundation/src/with_static_mut.rs",
        r#"
static mut GLOBAL_STATE: i32 = 0;

pub fn modify_global() {
    unsafe {
        GLOBAL_STATE += 1;
    }
}
"#,
    );

    repo
}

/// Create a mock repository with waiver registry
pub fn create_repo_with_waivers() -> MockRepo {
    let repo = MockRepo::new();

    repo.create_dir("2.engine/l1-foundation/src");
    repo.create_dir("7.quality/suites/repo_hygiene");

    // Create a file that would violate line limit
    let mut long_content = String::new();
    for i in 0..250 {
        long_content.push_str(&format!("// Line {}\n", i));
    }
    repo.create_file("2.engine/l1-foundation/src/waived_file.rs", &long_content);

    // Create waiver registry
    repo.create_file(
        "7.quality/suites/repo_hygiene/waivers.toml",
        r#"
[[line_limit_waivers]]
path = "2.engine/l1-foundation/src/waived_file.rs"
justification = "Legacy file scheduled for refactoring"
"#,
    );

    repo
}

/// Create a mock repository for migration rollback testing
pub fn create_repo_for_rollback_test() -> MockRepo {
    let repo = MockRepo::new();

    repo.create_dir("5.editor/l7.0-editor-command-spine/src");

    // Create a test file with invalid imports that will fail compilation
    repo.create_file(
        "5.editor/l7.0-editor-command-spine/src/failing_test.rs",
        r#"
use non_existent_module::SomeType;

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_something() {
        let _ = SomeType::new();
    }
}
"#,
    );

    repo
}
