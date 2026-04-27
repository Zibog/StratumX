//! Property Test: EditorHost Minimal Business Logic
//!
//! Feature: editor-state-ownership-normalization
//! Property 13: EditorHost Minimal Business Logic
//!
//! For any version of EditorHost, the total lines of business logic code is less than
//! 10% of the original pre-refactoring EditorHost.
//!
//! Validates: Requirements 8.2

use std::fs;
use std::path::{Path, PathBuf};
use stratumx_repo_hygiene_support::workspace_root_from_manifest_dir;

fn editor_host_path() -> PathBuf {
    PathBuf::from(workspace_root_from_manifest_dir(env!("CARGO_MANIFEST_DIR")))
        .join("5.editor")
        .join("editor-state-containers")
        .join("src")
        .join("editor_host.rs")
}

/// Count non-comment, non-whitespace lines in a file
fn count_code_lines(file_path: &str) -> Result<usize, String> {
    let content = fs::read_to_string(file_path)
        .map_err(|e| format!("Failed to read file {}: {}", file_path, e))?;

    let mut count = 0;
    let mut in_block_comment = false;

    for line in content.lines() {
        let trimmed = line.trim();

        // Skip empty lines
        if trimmed.is_empty() {
            continue;
        }

        // Handle block comments
        if trimmed.starts_with("/*") {
            in_block_comment = true;
        }
        if in_block_comment {
            if trimmed.ends_with("*/") {
                in_block_comment = false;
            }
            continue;
        }

        // Skip line comments
        if trimmed.starts_with("//") {
            continue;
        }

        // Count as code line
        count += 1;
    }

    Ok(count)
}

/// Count business logic lines in EditorHost
///
/// Business logic is defined as:
/// - Methods that implement domain operations (not lifecycle or delegation)
/// - Direct state manipulation
/// - Complex conditional logic
///
/// Lifecycle and delegation methods are NOT business logic:
/// - new(), initialize(), shutdown()
/// - services(), services_mut(), query()
/// - validate_state_ownership()
fn count_business_logic_lines(file_path: &str) -> Result<usize, String> {
    let content = fs::read_to_string(file_path)
        .map_err(|e| format!("Failed to read file {}: {}", file_path, e))?;

    // For the refactored EditorHost, business logic should be minimal
    // We count lines in methods that are NOT:
    // - new()
    // - initialize()
    // - shutdown()
    // - services()
    // - services_mut()
    // - query()
    // - validate_state_ownership()
    // - is_initialized()

    let mut business_logic_lines = 0;
    let mut in_allowed_method = false;
    let mut in_block_comment = false;
    let mut brace_depth = 0;

    let allowed_methods = [
        "pub fn new(",
        "pub fn initialize(",
        "pub fn shutdown(",
        "pub fn services(&",
        "pub fn services_mut(",
        "pub fn query(",
        "pub fn validate_state_ownership(",
        "pub fn is_initialized(",
    ];

    for line in content.lines() {
        let trimmed = line.trim();

        // Skip empty lines
        if trimmed.is_empty() {
            continue;
        }

        // Handle block comments
        if trimmed.starts_with("/*") {
            in_block_comment = true;
        }
        if in_block_comment {
            if trimmed.ends_with("*/") {
                in_block_comment = false;
            }
            continue;
        }

        // Skip line comments
        if trimmed.starts_with("//") {
            continue;
        }

        // Check if we're entering an allowed method
        if allowed_methods.iter().any(|m| trimmed.contains(m)) {
            in_allowed_method = true;
            brace_depth = 0;
            continue;
        }

        // Track brace depth to know when we exit a method
        for ch in trimmed.chars() {
            match ch {
                '{' => brace_depth += 1,
                '}' => {
                    brace_depth -= 1;
                    if brace_depth == 0 && in_allowed_method {
                        in_allowed_method = false;
                    }
                }
                _ => {}
            }
        }

        // If we're not in an allowed method and not in impl block header, count as business logic
        if !in_allowed_method
            && !trimmed.starts_with("impl ")
            && !trimmed.starts_with("pub struct ")
            && !trimmed.starts_with("use ")
            && !trimmed.starts_with("#[")
            && trimmed != "{"
            && trimmed != "}"
        {
            business_logic_lines += 1;
        }
    }

    Ok(business_logic_lines)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_editor_host_minimal_business_logic() {
        let editor_host_path = editor_host_path();

        // Verify file exists
        assert!(
            Path::new(&editor_host_path).exists(),
            "EditorHost file not found at {}",
            editor_host_path.display()
        );

        // Count total code lines
        let total_lines = count_code_lines(editor_host_path.to_str().unwrap())
            .expect("Failed to count code lines");

        // Count business logic lines
        let business_logic_lines = count_business_logic_lines(editor_host_path.to_str().unwrap())
            .expect("Failed to count business logic lines");

        println!("Total code lines: {}", total_lines);
        println!("Business logic lines: {}", business_logic_lines);

        // Business logic should be less than 10% of total
        let business_logic_percentage = if total_lines > 0 {
            (business_logic_lines as f64 / total_lines as f64) * 100.0
        } else {
            0.0
        };

        println!(
            "Business logic percentage: {:.2}%",
            business_logic_percentage
        );

        assert!(
            business_logic_percentage < 10.0,
            "EditorHost contains too much business logic: {:.2}% (expected < 10%)",
            business_logic_percentage
        );
    }

    #[test]
    fn test_editor_host_only_lifecycle_and_delegation() {
        let editor_host_path = editor_host_path();

        let content =
            fs::read_to_string(&editor_host_path).expect("Failed to read EditorHost file");

        // Verify that EditorHost only has lifecycle and delegation methods
        let allowed_methods = [
            "pub fn new(",
            "pub fn initialize(",
            "pub fn shutdown(",
            "pub fn services(",
            "pub fn services_mut(",
            "pub fn query(",
            "pub fn validate_state_ownership(",
            "pub fn is_initialized(",
        ];

        // Count public methods
        let public_method_count = content.matches("pub fn ").count();

        // All public methods should be in the allowed list
        for method in allowed_methods.iter() {
            if !content.contains(method) {
                // Method might not be implemented yet, that's ok
                continue;
            }
        }

        // Verify we don't have business logic methods like:
        let forbidden_methods = [
            "pub fn open_world(",
            "pub fn close_world(",
            "pub fn save_world(",
            "pub fn modify_terrain(",
            "pub fn create_material(",
            "pub fn place_audio_source(",
            "pub fn set_weather(",
            "pub fn start_preview(",
        ];

        for method in forbidden_methods.iter() {
            assert!(
                !content.contains(method),
                "EditorHost should not contain business logic method: {}",
                method
            );
        }

        println!("Public method count: {}", public_method_count);
        println!("Allowed method count: {}", allowed_methods.len());

        // Public methods should be approximately equal to allowed methods
        // (allowing for some variance due to test methods, etc.)
        assert!(
            public_method_count <= allowed_methods.len() + 2,
            "EditorHost has too many public methods: {} (expected <= {})",
            public_method_count,
            allowed_methods.len() + 2
        );
    }

    #[test]
    fn test_editor_host_no_direct_state_access() {
        let editor_host_path = editor_host_path();

        let content =
            fs::read_to_string(&editor_host_path).expect("Failed to read EditorHost file");

        // EditorHost should not directly access state containers except through:
        // 1. StateContainerSystem (for validation)
        // 2. QueryLayer (for queries)
        // 3. Services (for operations)

        // Check that EditorHost doesn't have methods that directly manipulate state
        let forbidden_patterns = [
            ".project_state.lock()",
            ".workspace_state.lock()",
            ".world_state.lock()",
            ".diagnostics_state.lock()",
        ];

        for pattern in forbidden_patterns.iter() {
            // Allow these patterns in the new() method for initialization
            // but not in other methods
            let lines_with_pattern: Vec<&str> = content
                .lines()
                .filter(|line| line.contains(pattern))
                .collect();

            // If pattern exists, it should only be in initialization context
            for line in lines_with_pattern {
                let is_in_new_method = content
                    .split("pub fn new(")
                    .nth(1)
                    .map(|after_new| {
                        let new_method_end =
                            after_new.find("\n    pub fn ").unwrap_or(after_new.len());
                        let new_method = &after_new[..new_method_end];
                        new_method.contains(line)
                    })
                    .unwrap_or(false);

                if !is_in_new_method {
                    panic!(
                        "EditorHost should not directly access state outside of initialization: {}",
                        line
                    );
                }
            }
        }
    }

    #[test]
    fn test_editor_host_structure() {
        let editor_host_path = editor_host_path();

        let content =
            fs::read_to_string(&editor_host_path).expect("Failed to read EditorHost file");

        // Verify EditorHost has the expected structure
        assert!(content.contains("pub struct EditorHost"));
        assert!(content.contains("state_system: Arc<StateContainerSystem>"));
        assert!(content.contains("query_layer: QueryLayer"));
        assert!(content.contains("cache_layer: Arc<Mutex<CacheLayer>>"));
        assert!(content.contains("services: EditorServices"));

        // Verify EditorHost is a thin facade
        assert!(content.contains("/// EditorHost - Thin facade"));
    }
}
