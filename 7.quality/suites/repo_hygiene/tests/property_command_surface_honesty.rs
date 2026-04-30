// Feature: stratumx-100-percent-canon-coverage
// Property 4: Root Command Execution Honesty
// Property 5: Repository Cleanliness
//
// **Validates: Requirements 2.1, 2.2, 3.1, 3.3**

use proptest::prelude::*;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Get the workspace root directory
fn get_workspace_root() -> PathBuf {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    PathBuf::from(manifest_dir)
        .parent()
        .and_then(|p| p.parent())
        .and_then(|p| p.parent())
        .expect("Failed to find workspace root")
        .to_path_buf()
}

/// Root commands that should be available
const ROOT_COMMANDS: &[&str] = &[
    "verify",
    "smoke",
    "full",
    "bench",
    "metrics",
    "evidence",
    "inventory",
];

/// Patterns for temporary/stray files that should not be in root
const STRAY_FILE_PATTERNS: &[&str] = &[
    r"\.orig$",
    r"\.bak$",
    r"_PLAN\.md$",
    r"_ROADMAP\.md$",
    r"_notes\.txt$",
];

/// Check if a command execution is honest (succeeds or fails with real defect, not topology error)
fn is_honest_execution(output: &std::process::Output) -> bool {
    let stderr = String::from_utf8_lossy(&output.stderr);
    let stdout = String::from_utf8_lossy(&output.stdout);

    // Topology errors indicate dishonest execution
    let topology_errors = [
        "could not find",
        "no such file or directory",
        "cannot find package",
        "failed to load manifest",
        "workspace member",
    ];

    for error in &topology_errors {
        if stderr.contains(error) || stdout.contains(error) {
            return false;
        }
    }

    // Either success or real defect error is honest
    true
}

/// Check if root directory is clean (no stray files)
fn check_root_cleanliness(repo_root: &Path) -> Result<Vec<PathBuf>, String> {
    let mut violations = Vec::new();

    let entries =
        fs::read_dir(repo_root).map_err(|e| format!("Failed to read root directory: {}", e))?;

    for entry in entries {
        let entry = entry.map_err(|e| format!("Failed to read directory entry: {}", e))?;
        let path = entry.path();

        if path.is_file() {
            let file_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");

            for pattern in STRAY_FILE_PATTERNS {
                let re = regex::Regex::new(pattern)
                    .map_err(|e| format!("Invalid regex pattern: {}", e))?;
                if re.is_match(file_name) {
                    violations.push(path.clone());
                    break;
                }
            }
        }
    }

    Ok(violations)
}

// ============================================================================
// Property 4: Root Command Execution Honesty
// ============================================================================

proptest! {
    #![proptest_config(ProptestConfig::with_cases(10))]

    /// Property 4: Root Command Execution Honesty
    ///
    /// For any root command (verify, full, bench, metrics, evidence, inventory, doctor),
    /// executing it must either succeed or fail with a real defect error,
    /// never with a workspace topology error.
    ///
    /// **Validates: Requirements 2.1, 2.2**
    #[test]
    fn property_root_command_execution_honesty(
        command_idx in 0..ROOT_COMMANDS.len()
    ) {
        let workspace_root = get_workspace_root();
        let command = ROOT_COMMANDS[command_idx];

        // Execute the command via cargo run
        let output = Command::new("cargo")
            .arg("run")
            .arg("-p")
            .arg("stratumx_quality_tasks")
            .arg("--")
            .arg(command)
            .current_dir(&workspace_root)
            .output()
            .expect("Failed to execute command");

        // Check that execution is honest (no topology errors)
        prop_assert!(
            is_honest_execution(&output),
            "Command '{}' failed with topology error, not honest execution. \
             Stderr: {}\nStdout: {}",
            command,
            String::from_utf8_lossy(&output.stderr),
            String::from_utf8_lossy(&output.stdout)
        );
    }
}

// ============================================================================
// Property 5: Repository Cleanliness
// ============================================================================

proptest! {
    #![proptest_config(ProptestConfig::with_cases(10))]

    /// Property 5: Repository Cleanliness
    ///
    /// For any file in the repository root directory, it must not match
    /// temporary file patterns (.orig, .bak, *_PLAN.md, *_ROADMAP.md, *_notes.txt)
    /// unless explicitly documented as intentional.
    ///
    /// **Validates: Requirements 3.1, 3.3**
    #[test]
    fn property_repository_cleanliness(
        _dummy in 0..1u8  // Dummy input since we're checking a fixed property
    ) {
        let workspace_root = get_workspace_root();

        let violations = check_root_cleanliness(&workspace_root)
            .expect("Failed to check root cleanliness");

        prop_assert!(
            violations.is_empty(),
            "Found {} stray files in root directory: {:?}. \
             These should be moved to 1.docs/history/ or 1.docs/plans/archive/",
            violations.len(),
            violations
        );
    }
}

// ============================================================================
// Unit Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_workspace_root() {
        let root = get_workspace_root();
        assert!(root.exists(), "Workspace root should exist");
        assert!(
            root.join("Cargo.toml").exists(),
            "Workspace root should have Cargo.toml"
        );
    }

    #[test]
    fn test_is_honest_execution() {
        // Honest execution (success)
        let output = std::process::Output {
            status: std::process::ExitStatus::default(),
            stdout: b"All checks passed".to_vec(),
            stderr: Vec::new(),
        };
        assert!(is_honest_execution(&output));

        // Honest execution (real error)
        let output = std::process::Output {
            status: std::process::ExitStatus::default(),
            stdout: Vec::new(),
            stderr: b"Test failed: assertion error".to_vec(),
        };
        assert!(is_honest_execution(&output));

        // Dishonest execution (topology error)
        let output = std::process::Output {
            status: std::process::ExitStatus::default(),
            stdout: Vec::new(),
            stderr: b"error: could not find package stratumx_quality_tasks".to_vec(),
        };
        assert!(!is_honest_execution(&output));
    }

    #[test]
    fn test_check_root_cleanliness() {
        let root = get_workspace_root();
        let violations = check_root_cleanliness(&root).expect("Failed to check root cleanliness");

        // Should have no violations in clean workspace
        assert!(
            violations.is_empty(),
            "Found stray files in root: {:?}",
            violations
        );
    }

    #[test]
    fn test_stratumx_quality_tasks_accessible() {
        let workspace_root = get_workspace_root();

        // Test that stratumx_quality_tasks is accessible
        let output = Command::new("cargo")
            .arg("run")
            .arg("-p")
            .arg("stratumx_quality_tasks")
            .arg("--")
            .arg("--help")
            .current_dir(&workspace_root)
            .output()
            .expect("Failed to execute command");

        assert!(
            output.status.success(),
            "stratumx_quality_tasks should be accessible via workspace"
        );
    }
}
