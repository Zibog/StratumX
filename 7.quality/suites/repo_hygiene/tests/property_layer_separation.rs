// Feature: editor-state-truth-normalization-phase3
// Property 10: Owner Services In Correct Layer
// Property 11: App Layer Knows Only Façades
//
// **Validates: Requirements 5.4, 5.5**

use proptest::prelude::*;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

fn get_workspace_root() -> PathBuf {
    // Try to find the workspace root by looking for Cargo.toml
    let mut current = std::env::current_dir().expect("Failed to get current directory");

    // Walk up until we find the workspace root (contains 5.editor and 6.apps directories)
    loop {
        let editor_dir = current.join("5.editor");
        let apps_dir = current.join("6.apps");
        if editor_dir.exists() && apps_dir.exists() {
            return current;
        }

        if !current.pop() {
            panic!("Could not find workspace root");
        }
    }
}

/// Scans a Rust file for import statements
fn extract_imports(file_path: &Path) -> Result<Vec<String>, String> {
    let content = fs::read_to_string(file_path)
        .map_err(|e| format!("Failed to read file {}: {}", file_path.display(), e))?;

    let mut imports = Vec::new();

    for line in content.lines() {
        let trimmed = line.trim();

        // Match various import patterns
        if trimmed.starts_with("use ") {
            // Extract the import path
            if let Some(import_path) = trimmed
                .strip_prefix("use ")
                .and_then(|s| s.split(';').next())
                .map(|s| s.trim().to_string())
            {
                imports.push(import_path);
            }
        }
    }

    Ok(imports)
}

/// Checks if a file path is in the 5.editor layer
fn is_in_editor_layer(path: &str) -> bool {
    // Normalize path separators for cross-platform compatibility
    let normalized = path.replace('\\', "/");
    normalized.contains("/5.editor/") || normalized.starts_with("5.editor/")
}

/// Checks if a file path is in the 6.apps layer
fn is_in_apps_layer(path: &str) -> bool {
    // Normalize path separators for cross-platform compatibility
    let normalized = path.replace('\\', "/");
    normalized.contains("/6.apps/") || normalized.starts_with("6.apps/")
}

/// Checks if an import is a façade or public API (not internal implementation)
fn is_facade_or_public_api(import_path: &str) -> bool {
    // Façade patterns:
    // - Direct service imports (e.g., "editor_state_containers::ProjectService")
    // - Public API modules (e.g., "editor_state_containers::api")
    // - Owner types exposed through public API

    // Internal implementation patterns (NOT façades):
    // - Internal modules like "::owners::", "::queries::", "::cache::", "::persistence::"
    // - Private implementation details

    let is_internal = import_path.contains("::owners::")
        || import_path.contains("::queries::")
        || import_path.contains("::cache::")
        || import_path.contains("::persistence::")
        || import_path.contains("::model::")
        || import_path.contains("::internal::")
        || import_path.contains("::private::");

    !is_internal
}

/// Finds all owner service types in the codebase
fn find_owner_services(root: &Path) -> Result<Vec<(String, String)>, String> {
    let mut services = Vec::new();

    // Domain authoring services that MUST be in 5.editor (not 6.apps)
    // These are the services that own domain truth
    let domain_service_patterns = vec![
        "TerrainAuthoringService",
        "MaterialAuthoringService",
        "EnvironmentAuthoringService",
        "AudioAuthoringService",
    ];

    // Scan both 5.editor and 6.apps to find where these services are defined
    for scan_path in &["5.editor", "6.apps"] {
        let full_path = root.join(scan_path);
        if !full_path.exists() {
            continue;
        }

        for entry in WalkDir::new(&full_path)
            .follow_links(false)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("rs") {
                if let Ok(content) = fs::read_to_string(path) {
                    for service_name in &domain_service_patterns {
                        // Look for struct definitions
                        if content.contains(&format!("pub struct {}", service_name))
                            || content.contains(&format!("struct {}", service_name))
                        {
                            services.push((service_name.to_string(), path.display().to_string()));
                        }
                    }
                }
            }
        }
    }

    Ok(services)
}

// Property 10: Owner Services In Correct Layer
// **Validates: Requirements 5.4**
//
// Test that domain authoring services (which own domain truth) live in 5.editor/*, not 6.apps/*
// Note: Shell services like DiagnosticsService, RuntimeModeService can be in 6.apps (L8 layer)
proptest! {
    #![proptest_config(ProptestConfig::with_cases(10))]

    #[test]
    fn property_owner_services_in_correct_layer(
        _seed in 0u64..100u64
    ) {
        let workspace_root = get_workspace_root();

        // Find all domain authoring services in the codebase
        let services = find_owner_services(&workspace_root)
            .expect("Failed to find owner services");

        // Verify that each domain authoring service is in the correct layer
        for (service_name, file_path) in services {
            // Domain authoring services should be in 5.editor, not 6.apps
            prop_assert!(
                is_in_editor_layer(&file_path),
                "Domain authoring service '{}' is defined in '{}' but should be in 5.editor/* layer",
                service_name,
                file_path
            );

            prop_assert!(
                !is_in_apps_layer(&file_path),
                "Domain authoring service '{}' is incorrectly defined in 6.apps/* layer at '{}'",
                service_name,
                file_path
            );
        }
    }
}

// Property 11: App Layer Knows Only Façades
// **Validates: Requirements 5.5**
//
// Test that 6.apps/editor imports only façade adapters or public API from 5.editor
proptest! {
    #![proptest_config(ProptestConfig::with_cases(10))]

    #[test]
    fn property_app_layer_knows_only_facades(
        _seed in 0u64..100u64
    ) {
        let workspace_root = get_workspace_root();
        let apps_path = workspace_root.join("6.apps/editor");

        if !apps_path.exists() {
            // Skip if the apps directory doesn't exist yet
            return Ok(());
        }

        // Scan all Rust files in 6.apps/editor
        for entry in WalkDir::new(&apps_path)
            .follow_links(false)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("rs") {
                let imports = extract_imports(path)
                    .expect(&format!("Failed to extract imports from {}", path.display()));

                // Check each import from 5.editor
                for import in imports {
                    // Only check imports that reference the editor layer
                    if import.contains("editor_state_containers")
                        || import.contains("l9.")
                        || import.contains("terrain_authoring")
                        || import.contains("material_authoring")
                        || import.contains("environment_authoring")
                        || import.contains("audio_authoring")
                    {
                        // Verify it's a façade or public API, not internal implementation
                        prop_assert!(
                            is_facade_or_public_api(&import),
                            "File '{}' imports internal implementation '{}' from 5.editor layer. \
                             6.apps should only import façades or public API.",
                            path.display(),
                            import
                        );
                    }
                }
            }
        }
    }
}

// Unit tests for specific scenarios

#[test]
fn test_is_in_editor_layer() {
    assert!(is_in_editor_layer(
        "5.editor/editor-state-containers/src/lib.rs"
    ));
    assert!(is_in_editor_layer(
        "/home/user/project/5.editor/l9.2-terrain/src/lib.rs"
    ));
    assert!(!is_in_editor_layer("6.apps/editor/src/main.rs"));
    assert!(!is_in_editor_layer(
        "7.quality/suites/repo_hygiene/src/lib.rs"
    ));
}

#[test]
fn test_is_in_apps_layer() {
    assert!(is_in_apps_layer("6.apps/editor/src/main.rs"));
    assert!(is_in_apps_layer(
        "/home/user/project/6.apps/editor/src/app_state.rs"
    ));
    assert!(!is_in_apps_layer(
        "5.editor/editor-state-containers/src/lib.rs"
    ));
    assert!(!is_in_apps_layer(
        "7.quality/suites/repo_hygiene/src/lib.rs"
    ));
}

#[test]
fn test_is_facade_or_public_api() {
    // Façades and public API (should return true)
    assert!(is_facade_or_public_api(
        "editor_state_containers::ProjectService"
    ));
    assert!(is_facade_or_public_api(
        "editor_state_containers::api::ProjectOwner"
    ));
    assert!(is_facade_or_public_api(
        "terrain_authoring::TerrainAuthoringService"
    ));

    // Internal implementation (should return false)
    assert!(!is_facade_or_public_api(
        "editor_state_containers::owners::project_owner"
    ));
    assert!(!is_facade_or_public_api(
        "editor_state_containers::queries::project_queries"
    ));
    assert!(!is_facade_or_public_api(
        "editor_state_containers::cache::rebuildable_caches"
    ));
    assert!(!is_facade_or_public_api(
        "editor_state_containers::persistence::project_persistence_view"
    ));
    assert!(!is_facade_or_public_api(
        "editor_state_containers::model::project_types"
    ));
}

#[test]
fn test_extract_imports() {
    let temp_dir = std::env::temp_dir();
    let test_file = temp_dir.join("test_imports.rs");

    let content = r#"
use std::collections::HashMap;
use editor_state_containers::ProjectService;
use editor_state_containers::owners::project_owner::ProjectOwner;
use terrain_authoring::TerrainAuthoringService;

pub struct MyStruct {
    service: ProjectService,
}
"#;

    fs::write(&test_file, content).expect("Failed to write test file");

    let imports = extract_imports(&test_file).expect("Failed to extract imports");

    assert_eq!(imports.len(), 4);
    assert!(imports.contains(&"std::collections::HashMap".to_string()));
    assert!(imports.contains(&"editor_state_containers::ProjectService".to_string()));
    assert!(imports
        .contains(&"editor_state_containers::owners::project_owner::ProjectOwner".to_string()));
    assert!(imports.contains(&"terrain_authoring::TerrainAuthoringService".to_string()));

    // Clean up
    let _ = fs::remove_file(&test_file);
}

#[test]
fn test_find_owner_services() {
    let workspace_root = get_workspace_root();
    let services = find_owner_services(&workspace_root).expect("Failed to find owner services");

    // We should find at least some services
    // (The exact number depends on implementation progress)
    assert!(
        !services.is_empty(),
        "Should find at least one owner service"
    );

    // Print found services for debugging
    for (service_name, file_path) in &services {
        println!("Found service: {} at {}", service_name, file_path);
    }
}

#[test]
fn test_no_owner_services_in_apps_layer() {
    let workspace_root = get_workspace_root();
    let services = find_owner_services(&workspace_root).expect("Failed to find owner services");

    for (service_name, file_path) in services {
        assert!(
            !is_in_apps_layer(&file_path),
            "Domain authoring service '{}' should not be in 6.apps/* layer, found at '{}'",
            service_name,
            file_path
        );
    }
}

#[test]
fn test_apps_layer_does_not_import_internal_modules() {
    let workspace_root = get_workspace_root();
    let apps_path = workspace_root.join("6.apps/editor");

    if !apps_path.exists() {
        // Skip if the apps directory doesn't exist yet
        return;
    }

    let mut violations = Vec::new();

    // Scan all Rust files in 6.apps/editor
    for entry in WalkDir::new(&apps_path)
        .follow_links(false)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("rs") {
            if let Ok(imports) = extract_imports(path) {
                for import in imports {
                    // Check for internal module imports
                    if (import.contains("editor_state_containers") || import.contains("l9."))
                        && !is_facade_or_public_api(&import)
                    {
                        violations.push(format!(
                            "File '{}' imports internal module '{}'",
                            path.display(),
                            import
                        ));
                    }
                }
            }
        }
    }

    if !violations.is_empty() {
        println!("Found {} layer separation violations:", violations.len());
        for violation in &violations {
            println!("  - {}", violation);
        }
    }

    assert!(
        violations.is_empty(),
        "Found {} layer separation violations. 6.apps should only import façades or public API from 5.editor",
        violations.len()
    );
}
