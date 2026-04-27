// Feature: stratumx-100-percent-canon-coverage
// Property 19: Thin Host Discipline
// Property 20: App Test Minimalism
//
// **Validates: Requirements 9.1, 9.2, 9.3, 9.5, 11.3**

use proptest::prelude::*;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

fn get_workspace_root() -> PathBuf {
    let mut current = std::env::current_dir().expect("Failed to get current directory");
    loop {
        let engine_dir = current.join("2.engine");
        let sdk_dir = current.join("3.sdk");
        if engine_dir.exists() && sdk_dir.exists() {
            return current;
        }
        if !current.pop() {
            panic!("Could not find workspace root");
        }
    }
}

/// Get all Rust source files in 6.apps directory
fn get_app_source_files(root: &Path) -> Result<Vec<PathBuf>, String> {
    let apps_dir = root.join("6.apps");
    if !apps_dir.exists() {
        return Err("6.apps directory not found".to_string());
    }

    let mut files = Vec::new();
    for entry in WalkDir::new(&apps_dir)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("rs") {
            files.push(path.to_path_buf());
        }
    }

    Ok(files)
}

/// Check if a file contains domain logic patterns
fn contains_domain_logic(path: &Path) -> Result<Vec<String>, String> {
    let content = fs::read_to_string(path)
        .map_err(|e| format!("Failed to read {}: {}", path.display(), e))?;

    let mut violations = Vec::new();

    // Patterns that indicate domain logic (not coordination)
    // These are intentionally strict to catch real domain logic
    let domain_patterns = vec![
        // Business logic calculations (but not simple getters/setters)
        (r"fn calculate_\w+_from_\w+", "calculation logic"),
        (r"fn compute_\w+_state", "computation logic"),
        (r"fn validate_\w+_rules\(", "validation logic"),
        (r"fn transform_\w+_to_\w+", "transformation logic"),
        (r"fn apply_business_rules", "business rules"),
        
        // Truth ownership (actual state management, not just holders)
        (r"fn set_\w+_truth\(", "truth mutation"),
        (r"struct \w+Truth \{", "truth ownership"),
        
        // Duplication of tooling/editor pipelines
        (r"fn process_command_envelope\(", "command processing duplication"),
        (r"fn validate_ingress_packet\(", "validation duplication"),
        (r"struct CommandPipeline \{", "pipeline duplication"),
        (r"struct ValidationPipeline \{", "validation pipeline duplication"),
    ];

    for (pattern, description) in domain_patterns {
        if let Ok(re) = regex::Regex::new(pattern) {
            if re.is_match(&content) {
                violations.push(format!("{}: {}", path.display(), description));
            }
        }
    }

    // Check for direct engine truth mutation (not through SDK)
    // But allow simple reference holders
    if content.contains("WorldState") && content.contains(".mutate_truth(") {
        violations.push(format!("{}: direct engine truth mutation", path.display()));
    }

    Ok(violations)
}

/// Check if a file contains heavy tests
fn contains_heavy_tests(path: &Path) -> Result<bool, String> {
    let content = fs::read_to_string(path)
        .map_err(|e| format!("Failed to read {}: {}", path.display(), e))?;

    // Heavy test patterns
    let heavy_patterns = vec![
        "#[cfg(test)]",
        "#[test]",
        "proptest!",
        "#[tokio::test]",
        "mod tests {",
        "mod integration_tests {",
        "mod property_tests {",
    ];

    for pattern in heavy_patterns {
        if content.contains(pattern) {
            return Ok(true);
        }
    }

    Ok(false)
}

/// Count lines of code (excluding blank lines and comments)
fn count_loc(path: &Path) -> Result<usize, String> {
    let content = fs::read_to_string(path)
        .map_err(|e| format!("Failed to read {}: {}", path.display(), e))?;

    let loc = content
        .lines()
        .filter(|line| {
            let trimmed = line.trim();
            !trimmed.is_empty() && !trimmed.starts_with("//")
        })
        .count();

    Ok(loc)
}

// ============================================================================
// Property 19: Thin Host Discipline
// ============================================================================

proptest! {
    #![proptest_config(ProptestConfig::with_cases(10))]

    /// **Property 19: Thin Host Discipline**
    ///
    /// For any package in 6.apps, it must contain only bootstrap code,
    /// dependency wiring, and app entry commands, with no domain logic
    /// or truth ownership.
    ///
    /// **Validates: Requirements 9.1, 9.2, 9.3, 11.3**
    #[test]
    fn property_19_thin_host_discipline(_seed in 0u64..10) {
        let root = get_workspace_root();
        let files = get_app_source_files(&root)
            .expect("Failed to get app source files");

        let mut all_violations = Vec::new();

        for file in &files {
            let violations = contains_domain_logic(file)
                .expect("Failed to check for domain logic");
            all_violations.extend(violations);
        }

        if !all_violations.is_empty() {
            panic!(
                "Property 19 FAILED: Found domain logic in app packages:\n{}",
                all_violations.join("\n")
            );
        }
    }
}

// ============================================================================
// Property 20: App Test Minimalism
// ============================================================================

proptest! {
    #![proptest_config(ProptestConfig::with_cases(10))]

    /// **Property 20: App Test Minimalism**
    ///
    /// For any package in 6.apps, it must contain only minimal sanity tests
    /// (no heavy integration or property tests).
    ///
    /// **Validates: Requirements 9.5**
    #[test]
    fn property_20_app_test_minimalism(_seed in 0u64..10) {
        let root = get_workspace_root();
        let files = get_app_source_files(&root)
            .expect("Failed to get app source files");

        let mut heavy_test_files = Vec::new();

        for file in &files {
            if contains_heavy_tests(&file).unwrap_or(false) {
                heavy_test_files.push(file.display().to_string());
            }
        }

        if !heavy_test_files.is_empty() {
            panic!(
                "Property 20 FAILED: Found heavy tests in app packages:\n{}",
                heavy_test_files.join("\n")
            );
        }
    }
}

// ============================================================================
// Unit Tests for Helper Functions
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_workspace_root() {
        let root = get_workspace_root();
        assert!(root.join("2.engine").exists());
        assert!(root.join("3.sdk").exists());
        assert!(root.join("6.apps").exists());
    }

    #[test]
    fn test_get_app_source_files() {
        let root = get_workspace_root();
        let files = get_app_source_files(&root).expect("Failed to get app source files");
        assert!(!files.is_empty(), "Should find at least one Rust file in 6.apps");
        
        // All files should be in 6.apps
        for file in &files {
            assert!(file.starts_with(&root.join("6.apps")));
        }
    }

    #[test]
    fn test_contains_domain_logic() {
        let root = get_workspace_root();
        
        // Test a known thin app (stack utility)
        let stack_utility = root.join("6.apps/stack/stratumx_stack_utility/src/main.rs");
        if stack_utility.exists() {
            let violations = contains_domain_logic(&stack_utility)
                .expect("Failed to check stack utility");
            assert!(violations.is_empty(), "Stack utility should have no domain logic");
        }
    }

    #[test]
    fn test_contains_heavy_tests() {
        let root = get_workspace_root();
        
        // Test a known app without tests
        let stack_utility = root.join("6.apps/stack/stratumx_stack_utility/src/main.rs");
        if stack_utility.exists() {
            let has_tests = contains_heavy_tests(&stack_utility)
                .expect("Failed to check for tests");
            assert!(!has_tests, "Stack utility should have no heavy tests");
        }
    }

    #[test]
    fn test_count_loc() {
        let root = get_workspace_root();
        
        // Test LOC counting on a known file
        let stack_utility = root.join("6.apps/stack/stratumx_stack_utility/src/main.rs");
        if stack_utility.exists() {
            let loc = count_loc(&stack_utility).expect("Failed to count LOC");
            assert!(loc > 0, "Should count at least some lines");
            assert!(loc < 200, "Stack utility should be small");
        }
    }

    #[test]
    fn test_thin_host_discipline_integration() {
        let root = get_workspace_root();
        let files = get_app_source_files(&root).expect("Failed to get app source files");

        let mut all_violations = Vec::new();

        for file in &files {
            let violations = contains_domain_logic(&file)
                .expect("Failed to check for domain logic");
            all_violations.extend(violations);
        }

        // This is a real check - apps should have no domain logic
        assert!(
            all_violations.is_empty(),
            "Found domain logic violations in apps:\n{}",
            all_violations.join("\n")
        );
    }

    #[test]
    fn test_app_test_minimalism_integration() {
        let root = get_workspace_root();
        let files = get_app_source_files(&root).expect("Failed to get app source files");

        let mut heavy_test_files = Vec::new();

        for file in &files {
            if contains_heavy_tests(&file).unwrap_or(false) {
                heavy_test_files.push(file.display().to_string());
            }
        }

        // This is a real check - apps should have no heavy tests
        assert!(
            heavy_test_files.is_empty(),
            "Found heavy tests in apps:\n{}",
            heavy_test_files.join("\n")
        );
    }
}
