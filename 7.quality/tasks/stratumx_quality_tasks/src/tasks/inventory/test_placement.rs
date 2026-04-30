use std::fs;
use std::path::{Path, PathBuf};

/// Represents a misplaced heavy test found outside 7.quality
#[derive(Debug, Clone)]
pub struct MisplacedTest {
    pub path: PathBuf,
    pub test_type: TestType,
    pub reason: String,
}

/// Types of heavy tests that must be in 7.quality
#[derive(Debug, Clone, PartialEq)]
pub enum TestType {
    Integration, // tests/ directory
    Property,    // proptest/quickcheck usage
    Matrix,      // matrix test infrastructure
    Hygiene,     // hygiene test infrastructure
    Benchmark,   // benches/ directory
}

impl TestType {
    pub fn as_str(&self) -> &str {
        match self {
            TestType::Integration => "Integration Test",
            TestType::Property => "Property-Based Test",
            TestType::Matrix => "Matrix Test",
            TestType::Hygiene => "Hygiene Test",
            TestType::Benchmark => "Benchmark Test",
        }
    }
}

/// Check for misplaced heavy tests outside 7.quality
pub fn check_test_placement(repo_root: &Path) -> Result<Vec<MisplacedTest>, String> {
    let mut violations = Vec::new();

    // Directories to scan (exclude 7.quality)
    let scan_dirs = vec![
        repo_root.join("2.engine"),
        repo_root.join("3.sdk"),
        repo_root.join("4.tooling"),
        repo_root.join("5.editor"),
        repo_root.join("6.apps"),
    ];

    for dir in scan_dirs {
        if !dir.exists() {
            continue;
        }

        // Scan for integration tests (tests/ directories)
        scan_for_integration_tests(&dir, &mut violations)?;

        // Scan for property-based tests (proptest/quickcheck usage)
        scan_for_property_tests(&dir, &mut violations)?;

        // Scan for benchmark tests (benches/ directories)
        scan_for_benchmark_tests(&dir, &mut violations)?;
    }

    Ok(violations)
}

/// Scan for integration tests in tests/ directories
fn scan_for_integration_tests(
    dir: &Path,
    violations: &mut Vec<MisplacedTest>,
) -> Result<(), String> {
    let entries =
        fs::read_dir(dir).map_err(|e| format!("Failed to read directory {:?}: {}", dir, e))?;

    for entry in entries {
        let entry = entry.map_err(|e| format!("Failed to read entry: {}", e))?;
        let path = entry.path();

        if path.is_dir() {
            // Skip target and hidden directories
            if let Some(name) = path.file_name() {
                let name_str = name.to_string_lossy();
                if name_str == "target" || name_str.starts_with('.') {
                    continue;
                }
            }

            // Check if this is a tests/ directory
            if path.file_name() == Some(std::ffi::OsStr::new("tests")) {
                // Check if it contains any test files
                if let Ok(test_entries) = fs::read_dir(&path) {
                    for test_entry in test_entries.flatten() {
                        let test_path = test_entry.path();
                        if test_path.extension() == Some(std::ffi::OsStr::new("rs")) {
                            violations.push(MisplacedTest {
                                path: test_path.clone(),
                                test_type: TestType::Integration,
                                reason:
                                    "Integration test found in tests/ directory outside 7.quality"
                                        .to_string(),
                            });
                        }
                    }
                }
            } else {
                // Recurse into subdirectories
                scan_for_integration_tests(&path, violations)?;
            }
        }
    }

    Ok(())
}

/// Scan for property-based tests (proptest/quickcheck usage)
fn scan_for_property_tests(dir: &Path, violations: &mut Vec<MisplacedTest>) -> Result<(), String> {
    let entries =
        fs::read_dir(dir).map_err(|e| format!("Failed to read directory {:?}: {}", dir, e))?;

    for entry in entries {
        let entry = entry.map_err(|e| format!("Failed to read entry: {}", e))?;
        let path = entry.path();

        if path.is_dir() {
            // Skip target and hidden directories
            if let Some(name) = path.file_name() {
                let name_str = name.to_string_lossy();
                if name_str == "target" || name_str.starts_with('.') {
                    continue;
                }
            }
            // Recurse into subdirectories
            scan_for_property_tests(&path, violations)?;
        } else if path.extension() == Some(std::ffi::OsStr::new("rs")) {
            // Read file content
            let content = match fs::read_to_string(&path) {
                Ok(c) => c,
                Err(_) => continue, // Skip files we can't read
            };

            // Check for proptest/quickcheck usage
            let has_proptest = content.contains("use proptest::") || content.contains("proptest!");
            let has_quickcheck =
                content.contains("use quickcheck::") || content.contains("quickcheck!");

            if has_proptest || has_quickcheck {
                violations.push(MisplacedTest {
                    path: path.to_path_buf(),
                    test_type: TestType::Property,
                    reason: "Property-based test (proptest/quickcheck) found outside 7.quality"
                        .to_string(),
                });
            }
        }
    }

    Ok(())
}

/// Scan for benchmark tests in benches/ directories
fn scan_for_benchmark_tests(dir: &Path, violations: &mut Vec<MisplacedTest>) -> Result<(), String> {
    let entries =
        fs::read_dir(dir).map_err(|e| format!("Failed to read directory {:?}: {}", dir, e))?;

    for entry in entries {
        let entry = entry.map_err(|e| format!("Failed to read entry: {}", e))?;
        let path = entry.path();

        if path.is_dir() {
            // Skip target and hidden directories
            if let Some(name) = path.file_name() {
                let name_str = name.to_string_lossy();
                if name_str == "target" || name_str.starts_with('.') {
                    continue;
                }
            }

            // Check if this is a benches/ directory
            if path.file_name() == Some(std::ffi::OsStr::new("benches")) {
                // Check if it contains any benchmark files
                if let Ok(bench_entries) = fs::read_dir(&path) {
                    for bench_entry in bench_entries.flatten() {
                        let bench_path = bench_entry.path();
                        if bench_path.extension() == Some(std::ffi::OsStr::new("rs")) {
                            violations.push(MisplacedTest {
                                path: bench_path.clone(),
                                test_type: TestType::Benchmark,
                                reason:
                                    "Benchmark test found in benches/ directory outside 7.quality"
                                        .to_string(),
                            });
                        }
                    }
                }
            } else {
                // Recurse into subdirectories
                scan_for_benchmark_tests(&path, violations)?;
            }
        }
    }

    Ok(())
}

/// Format violations for display
pub fn format_violations(violations: &[MisplacedTest]) -> String {
    if violations.is_empty() {
        return String::from(
            "✓ Test Placement Check: PASS\n  All heavy tests are correctly located in 7.quality\n",
        );
    }

    let mut output = String::new();
    output.push_str("✗ Test Placement Check: FAIL\n");
    output.push_str(&format!(
        "  Found {} misplaced heavy test(s) outside 7.quality:\n\n",
        violations.len()
    ));

    // Group by test type
    for test_type in &[
        TestType::Integration,
        TestType::Property,
        TestType::Matrix,
        TestType::Hygiene,
        TestType::Benchmark,
    ] {
        let type_violations: Vec<_> = violations
            .iter()
            .filter(|v| &v.test_type == test_type)
            .collect();
        if type_violations.is_empty() {
            continue;
        }

        output.push_str(&format!(
            "  {} ({} found):\n",
            test_type.as_str(),
            type_violations.len()
        ));
        for violation in type_violations {
            output.push_str(&format!("    - {}\n", violation.path.display()));
            output.push_str(&format!("      Reason: {}\n", violation.reason));
        }
        output.push('\n');
    }

    output.push_str("  Remediation:\n");
    output.push_str("    1. Determine appropriate quality suite for each test\n");
    output.push_str("    2. Move test to 7.quality/suites/<appropriate-suite>/\n");
    output.push_str("    3. Update test to work in new location\n");
    output.push_str("    4. Document move in 1.docs/history/SANITATION_LEDGER.md\n");
    output.push_str("    5. Re-run inventory check to verify\n\n");

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_test_type_as_str() {
        assert_eq!(TestType::Integration.as_str(), "Integration Test");
        assert_eq!(TestType::Property.as_str(), "Property-Based Test");
        assert_eq!(TestType::Matrix.as_str(), "Matrix Test");
        assert_eq!(TestType::Hygiene.as_str(), "Hygiene Test");
        assert_eq!(TestType::Benchmark.as_str(), "Benchmark Test");
    }

    #[test]
    fn test_format_violations_empty() {
        let violations = vec![];
        let output = format_violations(&violations);
        assert!(output.contains("PASS"));
        assert!(output.contains("All heavy tests are correctly located in 7.quality"));
    }

    #[test]
    fn test_format_violations_with_tests() {
        let violations = vec![MisplacedTest {
            path: PathBuf::from("2.engine/some-package/tests/integration_test.rs"),
            test_type: TestType::Integration,
            reason: "Integration test found in tests/ directory outside 7.quality".to_string(),
        }];
        let output = format_violations(&violations);
        assert!(output.contains("FAIL"));
        assert!(output.contains("Integration Test"));
        assert!(output.contains("integration_test.rs"));
    }
}
