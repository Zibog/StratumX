// Feature: phase-3-editor-app-gold-gates
// Property 6: Workspace Package Integration
//
// **Validates: Requirements 7.12, 11.3**
//
// This property test verifies that all packages in the repository are members
// of the workspace, and no packages exist outside the workspace.

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

/// Find all Cargo.toml files in the workspace
fn find_all_cargo_tomls(workspace_root: &Path) -> Vec<PathBuf> {
    let mut cargo_files = Vec::new();

    for entry in WalkDir::new(workspace_root)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.file_name() == "Cargo.toml"
                && !e.path().to_string_lossy().contains("target")
                && !e.path().to_string_lossy().contains(".git")
        })
    {
        cargo_files.push(entry.path().to_path_buf());
    }

    cargo_files
}

/// Parse workspace members from root Cargo.toml
fn parse_workspace_members(workspace_root: &Path) -> Result<HashSet<PathBuf>, String> {
    let root_cargo = workspace_root.join("Cargo.toml");

    if !root_cargo.exists() {
        return Err("Root Cargo.toml not found".to_string());
    }

    let content = fs::read_to_string(&root_cargo)
        .map_err(|e| format!("Failed to read root Cargo.toml: {}", e))?;

    let mut members = HashSet::new();
    let mut in_workspace = false;
    let mut in_members = false;

    for line in content.lines() {
        let trimmed = line.trim();

        if trimmed.starts_with("[workspace]") {
            in_workspace = true;
            continue;
        }

        if in_workspace && trimmed.starts_with("members") {
            in_members = true;
            continue;
        }

        if in_members {
            if trimmed.starts_with("]") {
                in_members = false;
                in_workspace = false;
                continue;
            }

            // Parse member path from line like: "2.engine/l-0.05-world-region",
            if let Some(member_path) = trimmed
                .trim_start_matches('"')
                .trim_end_matches('"')
                .trim_end_matches(',')
                .split('"')
                .next()
            {
                if !member_path.is_empty() && member_path != "members" && member_path != "=" {
                    members.insert(workspace_root.join(member_path));
                }
            }
        }

        // Stop if we hit another section
        if in_workspace && trimmed.starts_with('[') && !trimmed.starts_with("[workspace") {
            in_workspace = false;
            in_members = false;
        }
    }

    Ok(members)
}

/// Check if a Cargo.toml is a package (has [package] section)
fn is_package_cargo_toml(cargo_path: &Path) -> bool {
    if let Ok(content) = fs::read_to_string(cargo_path) {
        content
            .lines()
            .any(|line| line.trim().starts_with("[package]"))
    } else {
        false
    }
}

// Property 6: Workspace Package Integration
// **Validates: Requirements 7.12, 11.3**
//
// For any package in the repository, it should be a member of the workspace.
proptest! {
    #![proptest_config(ProptestConfig::with_cases(10))]

    #[test]
    fn property_all_packages_in_workspace(
        _seed in 0u64..100u64
    ) {
        let workspace_root = get_workspace_root();

        // Parse workspace members
        let workspace_members = parse_workspace_members(&workspace_root)
            .expect("Failed to parse workspace members");

        prop_assert!(
            !workspace_members.is_empty(),
            "Workspace should have at least some members"
        );

        // Find all Cargo.toml files
        let all_cargo_files = find_all_cargo_tomls(&workspace_root);

        prop_assert!(
            !all_cargo_files.is_empty(),
            "Should find at least some Cargo.toml files"
        );

        let mut orphan_packages = Vec::new();

        for cargo_path in all_cargo_files {
            // Skip the root Cargo.toml
            if cargo_path == workspace_root.join("Cargo.toml") {
                continue;
            }

            // Check if this is a package (has [package] section)
            if !is_package_cargo_toml(&cargo_path) {
                continue;
            }

            // Get the package directory
            let package_dir = cargo_path.parent().expect("Cargo.toml should have parent");

            // Check if this package is in workspace members
            let is_member = workspace_members.iter().any(|member| {
                member.canonicalize().ok() == package_dir.canonicalize().ok()
            });

            if !is_member {
                let relative_path = cargo_path.strip_prefix(&workspace_root)
                    .unwrap_or(&cargo_path);
                orphan_packages.push(relative_path.display().to_string());
            }
        }

        prop_assert!(
            orphan_packages.is_empty(),
            "Found {} orphan packages not in workspace:\n{}",
            orphan_packages.len(),
            orphan_packages
                .iter()
                .map(|path| format!("  {}", path))
                .collect::<Vec<_>>()
                .join("\n")
        );
    }
}

// Property 6b: Workspace Members Exist
// **Validates: Requirements 7.12, 11.3**
//
// For any workspace member declared in root Cargo.toml, the package should exist.
proptest! {
    #![proptest_config(ProptestConfig::with_cases(10))]

    #[test]
    fn property_workspace_members_exist(
        _seed in 0u64..100u64
    ) {
        let workspace_root = get_workspace_root();

        // Parse workspace members
        let workspace_members = parse_workspace_members(&workspace_root)
            .expect("Failed to parse workspace members");

        let mut missing_members = Vec::new();

        for member_path in workspace_members {
            let cargo_path = member_path.join("Cargo.toml");

            if !cargo_path.exists() {
                let relative_path = member_path.strip_prefix(&workspace_root)
                    .unwrap_or(&member_path);
                missing_members.push(relative_path.display().to_string());
            }
        }

        prop_assert!(
            missing_members.is_empty(),
            "Found {} workspace members that don't exist:\n{}",
            missing_members.len(),
            missing_members
                .iter()
                .map(|path| format!("  {}", path))
                .collect::<Vec<_>>()
                .join("\n")
        );
    }
}

// Unit tests for helper functions

#[test]
fn test_find_all_cargo_tomls() {
    let workspace_root = get_workspace_root();
    let cargo_files = find_all_cargo_tomls(&workspace_root);

    // Should find at least the root Cargo.toml
    assert!(
        !cargo_files.is_empty(),
        "Should find at least root Cargo.toml"
    );

    // All files should be named Cargo.toml
    for file in &cargo_files {
        assert_eq!(
            file.file_name().and_then(|s| s.to_str()),
            Some("Cargo.toml"),
            "File should be named Cargo.toml: {}",
            file.display()
        );
    }

    // Should not include files in target or .git directories
    for file in &cargo_files {
        let path_str = file.to_string_lossy();
        assert!(
            !path_str.contains("target") && !path_str.contains(".git"),
            "Should not include target or .git files: {}",
            file.display()
        );
    }
}

#[test]
fn test_parse_workspace_members() {
    let workspace_root = get_workspace_root();
    let members =
        parse_workspace_members(&workspace_root).expect("Failed to parse workspace members");

    // Should have at least some members
    assert!(!members.is_empty(), "Workspace should have members");

    println!("Found {} workspace members", members.len());
}

#[test]
fn test_is_package_cargo_toml() {
    let workspace_root = get_workspace_root();

    // Root Cargo.toml should not be a package (it's a workspace)
    let root_cargo = workspace_root.join("Cargo.toml");
    if root_cargo.exists() {
        // Root might or might not have [package] - depends on workspace structure
        println!(
            "Root Cargo.toml is package: {}",
            is_package_cargo_toml(&root_cargo)
        );
    }

    // Find a package Cargo.toml
    let cargo_files = find_all_cargo_tomls(&workspace_root);
    let package_count = cargo_files
        .iter()
        .filter(|p| is_package_cargo_toml(p))
        .count();

    assert!(
        package_count > 0,
        "Should find at least one package Cargo.toml"
    );
    println!("Found {} package Cargo.toml files", package_count);
}

#[test]
fn test_workspace_layout_system_in_workspace() {
    let workspace_root = get_workspace_root();
    let workspace_members =
        parse_workspace_members(&workspace_root).expect("Failed to parse workspace members");

    // Check that l8.7-workspace-layout-system is in workspace
    let layout_system_path = workspace_root.join("5.editor/l8.7-workspace-layout-system");

    let is_member = workspace_members
        .iter()
        .any(|member| member.canonicalize().ok() == layout_system_path.canonicalize().ok());

    if layout_system_path.exists() {
        assert!(
            is_member,
            "l8.7-workspace-layout-system should be a workspace member"
        );
        println!("✓ l8.7-workspace-layout-system is properly integrated in workspace");
    }
}
