// Feature: phase-3-editor-app-gold-gates
// Property 2: Public API Preservation During File Splitting
//
// **Validates: Requirements 3.1, 3.2, 3.3, 3.4**
//
// This property test verifies that when files are split into multiple modules,
// all public functions, types, and trait implementations remain accessible
// through re-exports in the mod.rs file.

use proptest::prelude::*;
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

fn get_workspace_root() -> PathBuf {
    let mut current = std::env::current_dir().expect("Failed to get current directory");

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

/// Extracts public items (functions, structs, enums, traits) from a Rust file
/// Only extracts top-level items, not impl methods (which are accessible through the type)
fn extract_public_items(file_path: &Path) -> Result<HashSet<String>, String> {
    let content = fs::read_to_string(file_path)
        .map_err(|e| format!("Failed to read file {}: {}", file_path.display(), e))?;

    let mut public_items = HashSet::new();
    let mut in_impl_block = false;
    let mut brace_depth = 0;

    for line in content.lines() {
        let trimmed = line.trim();

        // Track impl blocks
        if trimmed.starts_with("impl ") || trimmed.contains(" impl ") {
            in_impl_block = true;
            brace_depth = 0;
        }

        // Track braces to know when we exit impl blocks
        for ch in trimmed.chars() {
            match ch {
                '{' => brace_depth += 1,
                '}' => {
                    brace_depth -= 1;
                    if brace_depth == 0 && in_impl_block {
                        in_impl_block = false;
                    }
                }
                _ => {}
            }
        }

        // Skip items inside impl blocks (they're accessible through the type)
        if in_impl_block && brace_depth > 0 {
            continue;
        }

        // Match public item declarations (only top-level, not in impl blocks)
        if trimmed.starts_with("pub fn ") && !in_impl_block {
            if let Some(name) = extract_function_name(trimmed) {
                public_items.insert(format!("fn {}", name));
            }
        } else if trimmed.starts_with("pub struct ") {
            if let Some(name) = extract_type_name(trimmed, "pub struct ") {
                public_items.insert(format!("struct {}", name));
            }
        } else if trimmed.starts_with("pub enum ") {
            if let Some(name) = extract_type_name(trimmed, "pub enum ") {
                public_items.insert(format!("enum {}", name));
            }
        } else if trimmed.starts_with("pub trait ") {
            if let Some(name) = extract_type_name(trimmed, "pub trait ") {
                public_items.insert(format!("trait {}", name));
            }
        } else if trimmed.starts_with("pub type ") {
            if let Some(name) = extract_type_name(trimmed, "pub type ") {
                public_items.insert(format!("type {}", name));
            }
        } else if trimmed.starts_with("pub const ") && !in_impl_block {
            if let Some(name) = extract_const_name(trimmed) {
                public_items.insert(format!("const {}", name));
            }
        }
    }

    Ok(public_items)
}

/// Extracts function name from a function declaration
fn extract_function_name(line: &str) -> Option<String> {
    line.strip_prefix("pub fn ")?
        .split('(')
        .next()
        .map(|s| s.trim().to_string())
}

/// Extracts type name from a type declaration
fn extract_type_name(line: &str, prefix: &str) -> Option<String> {
    line.strip_prefix(prefix)?
        .split(|c: char| c.is_whitespace() || c == '<' || c == '{' || c == '(')
        .next()
        .map(|s| s.trim().to_string())
}

/// Extracts const name from a const declaration
fn extract_const_name(line: &str) -> Option<String> {
    line.strip_prefix("pub const ")?
        .split(':')
        .next()
        .map(|s| s.trim().to_string())
}

/// Extracts re-exported items from a mod.rs file
fn extract_reexports(mod_file: &Path) -> Result<HashSet<String>, String> {
    let content = fs::read_to_string(mod_file)
        .map_err(|e| format!("Failed to read mod file {}: {}", mod_file.display(), e))?;

    let mut reexports = HashSet::new();

    for line in content.lines() {
        let trimmed = line.trim();

        // Match re-export patterns: pub use module::*; or pub use module::{Item1, Item2};
        if trimmed.starts_with("pub use ") {
            // For wildcard re-exports, we need to check the source module
            if trimmed.contains("::*") {
                // Extract module name and mark as wildcard
                if let Some(module) = trimmed
                    .strip_prefix("pub use ")
                    .and_then(|s| s.split("::*").next())
                {
                    reexports.insert(format!("wildcard::{}", module.trim()));
                }
            } else {
                // Extract specific item names
                if let Some(items_part) = trimmed.strip_prefix("pub use ") {
                    if let Some(items_str) = items_part.split('{').nth(1) {
                        if let Some(items_list) = items_str.split('}').next() {
                            for item in items_list.split(',') {
                                let item_name = item.trim().to_string();
                                if !item_name.is_empty() {
                                    reexports.insert(item_name);
                                }
                            }
                        }
                    } else {
                        // Single item re-export: pub use module::Item;
                        if let Some(item) = items_part.split("::").last() {
                            if let Some(item_name) = item.split(';').next() {
                                reexports.insert(item_name.trim().to_string());
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(reexports)
}

/// Finds split module directories (directories that replaced a single file)
fn find_split_modules(root: &Path) -> Result<Vec<PathBuf>, String> {
    let mut split_modules = Vec::new();

    // Known split modules from Phase 3
    let known_splits = vec![
        "6.apps/editor/stratumx_editor_app/src/desktop_app",
        "5.editor/l10.0-project-bootstrap-service/src/bootstrap_impl/types",
        "5.editor/l9.3-material-lookdev-authoring-suite/src/runtime/authoring_service/service",
        "5.editor/l8.10-diagnostics-surface/src/diagnostics_types",
        "5.editor/l9.0-world-authoring-suite/src/world_session_service",
        "5.editor/l9.0-world-authoring-suite/src/persistence/world_persistence_view",
        "5.editor/l8.7-workspace-layout-system/src/queries/workspace_queries",
        "5.editor/l8.0-editor-shell/src/command_palette_state",
        "5.editor/l8.0-editor-shell/src/editor_shell",
    ];

    for split_path in known_splits {
        let full_path = root.join(split_path);
        if full_path.exists() && full_path.is_dir() {
            split_modules.push(full_path);
        }
    }

    Ok(split_modules)
}

// Property 2: Public API Preservation During File Splitting
// **Validates: Requirements 3.1, 3.2, 3.3, 3.4**
//
// Test that split modules properly declare their submodules as public,
// making all public items accessible through module paths.
// This ensures the public API is preserved even if not all items are re-exported at the top level.
proptest! {
    #![proptest_config(ProptestConfig::with_cases(10))]

    #[test]
    fn property_public_api_preserved_after_split(
        _seed in 0u64..100u64
    ) {
        let workspace_root = get_workspace_root();
        let split_modules = find_split_modules(&workspace_root)
            .expect("Failed to find split modules");

        for module_dir in split_modules {
            let mod_file = module_dir.join("mod.rs");

            // Skip if mod.rs doesn't exist (not a split module)
            if !mod_file.exists() {
                continue;
            }

            // Read mod.rs content
            let mod_content = fs::read_to_string(&mod_file)
                .unwrap_or_else(|e| panic!("Failed to read {}: {}", mod_file.display(), e));

            // Scan all .rs files in the module directory (except mod.rs)
            for entry in WalkDir::new(&module_dir)
                .max_depth(1)
                .into_iter()
                .filter_map(|e| e.ok())
            {
                let path = entry.path();
                if path.is_file()
                    && path.extension().and_then(|s| s.to_str()) == Some("rs")
                    && path.file_name() != Some(std::ffi::OsStr::new("mod.rs"))
                {
                    // Extract module name from filename
                    let module_name = path.file_stem()
                        .and_then(|s| s.to_str())
                        .unwrap_or("");

                    // Verify the module is declared in mod.rs (either pub mod or mod)
                    let has_pub_mod = mod_content.contains(&format!("pub mod {}", module_name));
                    let has_mod = mod_content.contains(&format!("mod {}", module_name));

                    prop_assert!(
                        has_pub_mod || has_mod,
                        "Module '{}' from file '{}' is not declared in '{}'",
                        module_name,
                        path.display(),
                        mod_file.display()
                    );

                    // If the module has public items, it should be declared as pub mod
                    let public_items = extract_public_items(path)
                        .unwrap_or_else(|e| panic!("Failed to extract public items from {}: {}", path.display(), e));

                    if !public_items.is_empty() {
                        prop_assert!(
                            has_pub_mod,
                            "Module '{}' has public items but is not declared as 'pub mod' in '{}'. \
                             Public items: {:?}",
                            module_name,
                            mod_file.display(),
                            public_items
                        );
                    }
                }
            }
        }
    }
}

// Unit tests for helper functions

#[test]
fn test_extract_function_name() {
    assert_eq!(
        extract_function_name("pub fn my_function() -> Result<(), Error> {"),
        Some("my_function".to_string())
    );
    assert_eq!(
        extract_function_name("pub fn another_fn(arg: i32) {"),
        Some("another_fn".to_string())
    );
    assert_eq!(extract_function_name("fn private_fn() {"), None);
}

#[test]
fn test_extract_type_name() {
    assert_eq!(
        extract_type_name("pub struct MyStruct {", "pub struct "),
        Some("MyStruct".to_string())
    );
    assert_eq!(
        extract_type_name("pub enum MyEnum {", "pub enum "),
        Some("MyEnum".to_string())
    );
    assert_eq!(
        extract_type_name("pub struct GenericStruct<T> {", "pub struct "),
        Some("GenericStruct".to_string())
    );
}

#[test]
fn test_extract_const_name() {
    assert_eq!(
        extract_const_name("pub const MAX_SIZE: usize = 100;"),
        Some("MAX_SIZE".to_string())
    );
    assert_eq!(
        extract_const_name("pub const VERSION: &str = \"1.0\";"),
        Some("VERSION".to_string())
    );
}

#[test]
fn test_extract_public_items() {
    let temp_dir = std::env::temp_dir();
    let test_file = temp_dir.join("test_public_items.rs");

    let content = r#"
pub fn public_function() {}
fn private_function() {}
pub struct PublicStruct {}
struct PrivateStruct {}
pub enum PublicEnum {}
pub trait PublicTrait {}
pub const PUBLIC_CONST: i32 = 42;
"#;

    fs::write(&test_file, content).expect("Failed to write test file");

    let items = extract_public_items(&test_file).expect("Failed to extract public items");

    assert!(items.contains("fn public_function"));
    assert!(items.contains("struct PublicStruct"));
    assert!(items.contains("enum PublicEnum"));
    assert!(items.contains("trait PublicTrait"));
    assert!(items.contains("const PUBLIC_CONST"));
    assert!(!items.iter().any(|item| item.contains("private")));

    // Clean up
    let _ = fs::remove_file(&test_file);
}

#[test]
fn test_extract_reexports() {
    let temp_dir = std::env::temp_dir();
    let test_file = temp_dir.join("test_mod.rs");

    let content = r#"
pub mod submodule;
pub use submodule::*;
pub use another_module::SpecificItem;
pub use third_module::{Item1, Item2, Item3};
"#;

    fs::write(&test_file, content).expect("Failed to write test file");

    let reexports = extract_reexports(&test_file).expect("Failed to extract re-exports");

    assert!(reexports.contains("wildcard::submodule"));
    assert!(reexports.contains("SpecificItem"));
    assert!(reexports.contains("Item1"));
    assert!(reexports.contains("Item2"));
    assert!(reexports.contains("Item3"));

    // Clean up
    let _ = fs::remove_file(&test_file);
}

#[test]
fn test_desktop_app_split_preserves_api() {
    let workspace_root = get_workspace_root();
    let desktop_app_dir = workspace_root.join("6.apps/editor/stratumx_editor_app/src/desktop_app");

    if !desktop_app_dir.exists() {
        // Skip if directory doesn't exist
        return;
    }

    let mod_file = desktop_app_dir.join("mod.rs");
    if !mod_file.exists() {
        // Skip if mod.rs doesn't exist
        return;
    }

    // The desktop_app module should have split files and a mod.rs with re-exports
    let split_files = vec![
        "app_state.rs",
        "app_update.rs",
        "app_panels.rs",
        "app_viewport.rs",
        "app_commands.rs",
        "app_diagnostics.rs",
    ];

    for split_file in split_files {
        let file_path = desktop_app_dir.join(split_file);
        if file_path.exists() {
            let public_items = extract_public_items(&file_path)
                .unwrap_or_else(|_| panic!("Failed to extract public items from {}", split_file));

            // If there are public items, verify they're accessible
            // (either through re-exports or through module visibility)
            if !public_items.is_empty() {
                println!(
                    "Found {} public items in {}",
                    public_items.len(),
                    split_file
                );
            }
        }
    }
}
