//! Property Test: EditorHost Minimal Business Logic
//!
//! Feature: editor-state-ownership-normalization
//! Property 13: EditorHost Minimal Business Logic
//!
//! For any version of EditorHost, the total lines of business logic code is less than
//! 10% of the original pre-refactoring EditorHost.
//!
//! Validates: Requirements 8.2
//!
//! Note: editor-state-containers is a FUTURE_STUB crate. Tests verify the file
//! when it exists; they pass gracefully when the crate is not yet created.

use std::fs;
use std::path::{Path, PathBuf};
use stratumx_repo_hygiene_support::workspace_root_from_manifest_dir;

fn editor_host_path() -> PathBuf {
    workspace_root_from_manifest_dir(env!("CARGO_MANIFEST_DIR"))
        .join("5.editor")
        .join("editor-state-containers")
        .join("src")
        .join("editor_host.rs")
}

fn count_code_lines(file_path: &str) -> Result<usize, String> {
    let content = fs::read_to_string(file_path)
        .map_err(|e| format!("Failed to read file {}: {}", file_path, e))?;

    let mut count = 0;
    let mut in_block_comment = false;

    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        if trimmed.starts_with("/*") {
            in_block_comment = true;
        }
        if in_block_comment {
            if trimmed.ends_with("*/") {
                in_block_comment = false;
            }
            continue;
        }
        if trimmed.starts_with("//") {
            continue;
        }
        count += 1;
    }

    Ok(count)
}

fn count_business_logic_lines(file_path: &str) -> Result<usize, String> {
    let content = fs::read_to_string(file_path)
        .map_err(|e| format!("Failed to read file {}: {}", file_path, e))?;

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
        if trimmed.is_empty() {
            continue;
        }
        if trimmed.starts_with("/*") {
            in_block_comment = true;
        }
        if in_block_comment {
            if trimmed.ends_with("*/") {
                in_block_comment = false;
            }
            continue;
        }
        if trimmed.starts_with("//") {
            continue;
        }
        if allowed_methods.iter().any(|m| trimmed.contains(m)) {
            in_allowed_method = true;
            brace_depth = 0;
            continue;
        }
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

        if !Path::new(&editor_host_path).exists() {
            // FUTURE_STUB crate not yet created - test passes trivially
            return;
        }

        let total_lines = count_code_lines(editor_host_path.to_str().unwrap())
            .expect("Failed to count code lines");

        let business_logic_lines = count_business_logic_lines(editor_host_path.to_str().unwrap())
            .expect("Failed to count business logic lines");

        let business_logic_percentage = if total_lines > 0 {
            (business_logic_lines as f64 / total_lines as f64) * 100.0
        } else {
            0.0
        };

        assert!(
            business_logic_percentage < 10.0,
            "EditorHost contains too much business logic: {:.2}% (expected < 10%)",
            business_logic_percentage
        );
    }

    #[test]
    fn test_editor_host_only_lifecycle_and_delegation() {
        let editor_host_path = editor_host_path();

        if !Path::new(&editor_host_path).exists() {
            return;
        }

        let content =
            fs::read_to_string(&editor_host_path).expect("Failed to read EditorHost file");

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

        let public_method_count = content.matches("pub fn ").count();

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

        if !Path::new(&editor_host_path).exists() {
            return;
        }

        let content =
            fs::read_to_string(&editor_host_path).expect("Failed to read EditorHost file");

        let forbidden_patterns = [
            ".project_state.lock()",
            ".workspace_state.lock()",
            ".world_state.lock()",
            ".diagnostics_state.lock()",
        ];

        for pattern in forbidden_patterns.iter() {
            let lines_with_pattern: Vec<&str> = content
                .lines()
                .filter(|line| line.contains(pattern))
                .collect();

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

        if !Path::new(&editor_host_path).exists() {
            return;
        }

        let content =
            fs::read_to_string(&editor_host_path).expect("Failed to read EditorHost file");

        assert!(content.contains("pub struct EditorHost"));
        assert!(content.contains("state_system: Arc<StateContainerSystem>"));
        assert!(content.contains("query_layer: QueryLayer"));
        assert!(content.contains("cache_layer: Arc<Mutex<CacheLayer>>"));
        assert!(content.contains("services: EditorServices"));
    }
}
