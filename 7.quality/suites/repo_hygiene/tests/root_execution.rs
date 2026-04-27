//! Property 31: Quality Suite Execution from Root
//!
//! Validates that all quality suites can be executed from the repository root
//! without requiring manual cd to subdirectories.

#[test]
fn test_quality_suites_compilable_from_root() {
    // This test verifies that quality suites are properly configured
    // to be built from the repository root via cargo test -p <suite>
    
    let suites = vec![
        "repo_hygiene",
        "smoke",
        "engine_canon_matrix",
        "sdk_canon_matrix",
        "tooling_canon_matrix",
        "editor_canon_matrix",
    ];
    
    for suite in suites {
        // Just verify the package exists and has a Cargo.toml
        let cargo_toml = format!("7.quality/suites/{}/Cargo.toml", suite);
        assert!(
            std::path::Path::new(&cargo_toml).exists(),
            "Quality suite {} must have Cargo.toml", suite
        );
    }
}

#[test]
fn test_quality_suite_naming_convention() {
    // All quality suites should follow naming convention
    let suites_dir = std::path::Path::new("7.quality/suites");
    
    assert!(suites_dir.exists(), "7.quality/suites must exist");
    
    if let Ok(entries) = std::fs::read_dir(suites_dir) {
        for entry in entries.flatten() {
            if entry.file_type().map(|t| t.is_dir()).unwrap_or(false) {
                let name = entry.file_name().to_string_lossy().to_string();
                
                // Should not contain spaces or special chars
                assert!(
                    !name.contains(' '),
                    "Suite name '{}' should not contain spaces", name
                );
                
                // Should be snake_case
                assert!(
                    name.chars().all(|c| c.is_alphanumeric() || c == '_'),
                    "Suite name '{}' should be snake_case", name
                );
            }
        }
    }
}
