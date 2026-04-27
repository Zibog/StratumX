use std::path::{Path, PathBuf};

/// Represents a test file candidate for migration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TestFileCandidate {
    pub path: PathBuf,
    pub test_type: TestType,
    pub target_suite: String,
    pub dependencies: Vec<String>,
}

/// Classification of test types.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TestType {
    Unit,
    Integration,
    Property,
    Invariant,
    Smoke,
}

impl TestType {
    /// Detect test type from file path and content.
    pub fn detect_from_file(path: &Path, content: &str) -> Self {
        if content.contains("proptest!") || content.contains("prop_compose!") {
            TestType::Property
        } else if content.contains("invariant!") {
            TestType::Invariant
        } else if path
            .file_name()
            .and_then(|name| name.to_str())
            .is_some_and(|name| name.contains("smoke"))
        {
            TestType::Smoke
        } else if path.ancestors().any(|ancestor| ancestor.ends_with("tests")) {
            TestType::Integration
        } else {
            TestType::Unit
        }
    }
}

/// Result of a test file migration operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MigrationResult {
    pub source_path: PathBuf,
    pub destination_path: PathBuf,
    pub imports_updated: usize,
    pub regression_files_moved: Vec<PathBuf>,
    pub compilation_status: CompilationStatus,
}

/// Status of compilation verification after migration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompilationStatus {
    Success,
    Failed,
    NotAttempted,
}
