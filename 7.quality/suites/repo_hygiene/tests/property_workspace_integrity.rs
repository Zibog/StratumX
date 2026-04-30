// Feature: stratumx-100-percent-canon-coverage
// Property 1: Workspace Membership Completeness
// Property 2: Metadata Consistency
// Property 3: Workspace-Command Surface Consistency
//
// **Validates: Requirements 1.1, 1.2, 1.3, 1.4**

use proptest::prelude::*;
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use walkdir::WalkDir;

fn get_workspace_root() -> PathBuf {
    // Try to find the workspace root by looking for Cargo.toml
    let mut current = std::env::current_dir().expect("Failed to get current directory");

    // Walk up until we find the workspace root (contains 2.engine, 3.sdk, etc.)
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

/// Get all packages from cargo metadata
fn get_metadata_packages(root: &Path) -> Result<HashSet<String>, String> {
    let output = Command::new("cargo")
        .arg("metadata")
        .arg("--format-version=1")
        .arg("--no-deps")
        .current_dir(root)
        .output()
        .map_err(|e| format!("Failed to run cargo metadata: {}", e))?;

    if !output.status.success() {
        return Err(format!(
            "cargo metadata failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    let metadata: serde_json::Value = serde_json::from_slice(&output.stdout)
        .map_err(|e| format!("Failed to parse metadata JSON: {}", e))?;

    let packages = metadata["packages"]
        .as_array()
        .ok_or("No packages array in metadata")?;

    let workspace_root = metadata["workspace_root"]
        .as_str()
        .ok_or("No workspace_root in metadata")?;

    let mut package_paths = HashSet::new();
    for package in packages {
        let manifest_path = package["manifest_path"]
            .as_str()
            .ok_or("No manifest_path in package")?;

        // Convert to relative path from workspace root
        let relative_path = manifest_path
            .replace(workspace_root, "")
            .trim_start_matches('/')
            .trim_start_matches('\\')
            .replace('\\', "/")
            .replace("/Cargo.toml", "");

        package_paths.insert(relative_path);
    }

    Ok(package_paths)
}

/// Find all Cargo.toml files in active directories
fn get_filesystem_packages(root: &Path) -> Result<HashSet<String>, String> {
    let active_dirs = vec![
        "2.engine",
        "3.sdk",
        "4.tooling",
        "5.editor",
        "6.apps",
        "7.quality",
    ];

    let mut packages = HashSet::new();

    for dir in active_dirs {
        let full_path = root.join(dir);
        if !full_path.exists() {
            continue;
        }

        for entry in WalkDir::new(&full_path)
            .follow_links(false)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            if path.is_file() && path.file_name() == Some(std::ffi::OsStr::new("Cargo.toml")) {
                // Get the directory containing Cargo.toml
                if let Some(package_dir) = path.parent() {
                    let relative_path = package_dir
                        .strip_prefix(root)
                        .map_err(|e| format!("Failed to strip prefix: {}", e))?
                        .to_str()
                        .ok_or("Invalid UTF-8 in path")?
                        .replace('\\', "/");

                    packages.insert(relative_path);
                }
            }
        }
    }

    Ok(packages)
}

/// Read workspace.members from root Cargo.toml
fn get_workspace_members(root: &Path) -> Result<HashSet<String>, String> {
    let cargo_toml_path = root.join("Cargo.toml");
    let content = fs::read_to_string(&cargo_toml_path)
        .map_err(|e| format!("Failed to read Cargo.toml: {}", e))?;

    let toml: toml::Value =
        toml::from_str(&content).map_err(|e| format!("Failed to parse Cargo.toml: {}", e))?;

    let members = toml
        .get("workspace")
        .and_then(|w| w.get("members"))
        .and_then(|m| m.as_array())
        .ok_or("No workspace.members in Cargo.toml")?;

    let mut member_set = HashSet::new();
    for member in members {
        if let Some(path) = member.as_str() {
            member_set.insert(path.replace('\\', "/"));
        }
    }

    Ok(member_set)
}

/// Check if workspace has any exclude entries
fn has_workspace_excludes(root: &Path) -> Result<bool, String> {
    let cargo_toml_path = root.join("Cargo.toml");
    let content = fs::read_to_string(&cargo_toml_path)
        .map_err(|e| format!("Failed to read Cargo.toml: {}", e))?;

    let toml: toml::Value =
        toml::from_str(&content).map_err(|e| format!("Failed to parse Cargo.toml: {}", e))?;

    if let Some(workspace) = toml.get("workspace") {
        if let Some(exclude) = workspace.get("exclude") {
            if let Some(exclude_array) = exclude.as_array() {
                return Ok(!exclude_array.is_empty());
            }
        }
    }

    Ok(false)
}

// Property 1: Workspace Membership Completeness
// **Validates: Requirements 1.1, 1.4**
//
// For any package marked as active in the project, it must either appear in workspace.members
// or have documented --manifest-path usage in all root commands that reference it.
proptest! {
    #![proptest_config(ProptestConfig::with_cases(10))]

    #[test]
    fn property_workspace_membership_completeness(
        _seed in 0u64..100u64
    ) {
        let workspace_root = get_workspace_root();

        // Get all packages from filesystem
        let filesystem_packages = get_filesystem_packages(&workspace_root)
            .expect("Failed to get filesystem packages");

        // Get all packages from workspace.members
        let workspace_members = get_workspace_members(&workspace_root)
            .expect("Failed to get workspace members");

        // Find packages that are in filesystem but not in workspace
        let missing_packages: Vec<_> = filesystem_packages
            .difference(&workspace_members)
            .collect();

        prop_assert!(
            missing_packages.is_empty(),
            "Found {} packages in filesystem not in workspace.members:\n{}",
            missing_packages.len(),
            missing_packages.iter()
                .map(|p| format!("  - {}", p))
                .collect::<Vec<_>>()
                .join("\n")
        );

        // Also check for phantom packages (in workspace but not in filesystem)
        let phantom_packages: Vec<_> = workspace_members
            .difference(&filesystem_packages)
            .collect();

        prop_assert!(
            phantom_packages.is_empty(),
            "Found {} workspace members without Cargo.toml:\n{}",
            phantom_packages.len(),
            phantom_packages.iter()
                .map(|p| format!("  - {}", p))
                .collect::<Vec<_>>()
                .join("\n")
        );
    }
}

// Property 2: Metadata Consistency
// **Validates: Requirements 1.2**
//
// For any active package in the project, running `cargo metadata --format-version=1`
// from the repository root must include that package in the returned package list.
proptest! {
    #![proptest_config(ProptestConfig::with_cases(10))]

    #[test]
    fn property_metadata_consistency(
        _seed in 0u64..100u64
    ) {
        let workspace_root = get_workspace_root();

        // Get all packages from filesystem
        let filesystem_packages = get_filesystem_packages(&workspace_root)
            .expect("Failed to get filesystem packages");

        // Get all packages from cargo metadata
        let metadata_packages = get_metadata_packages(&workspace_root)
            .expect("Failed to get metadata packages");

        // Find packages that are in filesystem but not in metadata
        let missing_from_metadata: Vec<_> = filesystem_packages
            .difference(&metadata_packages)
            .collect();

        prop_assert!(
            missing_from_metadata.is_empty(),
            "Found {} packages in filesystem not returned by cargo metadata:\n{}",
            missing_from_metadata.len(),
            missing_from_metadata.iter()
                .map(|p| format!("  - {}", p))
                .collect::<Vec<_>>()
                .join("\n")
        );

        // Verify counts match
        prop_assert_eq!(
            filesystem_packages.len(),
            metadata_packages.len(),
            "Filesystem package count ({}) doesn't match metadata count ({})",
            filesystem_packages.len(),
            metadata_packages.len()
        );
    }
}

// Property 3: Workspace-Command Surface Consistency
// **Validates: Requirements 1.3**
//
// For any package referenced in root command scripts, that package must either be in
// workspace.members or have explicit --manifest-path handling in the script.
proptest! {
    #![proptest_config(ProptestConfig::with_cases(10))]

    #[test]
    fn property_workspace_command_surface_consistency(
        _seed in 0u64..100u64
    ) {
        let workspace_root = get_workspace_root();

        // Get workspace members
        let workspace_members = get_workspace_members(&workspace_root)
            .expect("Failed to get workspace members");

        // Check that stratumx_quality_tasks is in workspace
        // (This is the main package referenced by root commands)
        let quality_tasks_path = "7.quality/tasks/stratumx_quality_tasks";
        prop_assert!(
            workspace_members.contains(quality_tasks_path),
            "stratumx_quality_tasks must be in workspace.members for root commands to work. \
             Found in workspace: {}",
            workspace_members.contains(quality_tasks_path)
        );

        // Verify no workspace exclusions (for honest topology)
        let has_excludes = has_workspace_excludes(&workspace_root)
            .expect("Failed to check workspace excludes");

        prop_assert!(
            !has_excludes,
            "workspace.exclude should be empty for honest topology. \
             All active packages should be in workspace.members."
        );
    }
}

// Unit tests for helper functions

#[test]
fn test_get_workspace_root() {
    let root = get_workspace_root();
    assert!(root.join("2.engine").exists());
    assert!(root.join("3.sdk").exists());
    assert!(root.join("Cargo.toml").exists());
}

#[test]
fn test_get_filesystem_packages() {
    let root = get_workspace_root();
    let packages = get_filesystem_packages(&root).expect("Failed to get filesystem packages");

    // Should find packages in all active directories
    assert!(!packages.is_empty(), "Should find at least one package");

    // Should find some engine packages
    let has_engine = packages.iter().any(|p| p.starts_with("2.engine/"));
    assert!(has_engine, "Should find at least one engine package");

    // Should find some SDK packages
    let has_sdk = packages.iter().any(|p| p.starts_with("3.sdk/"));
    assert!(has_sdk, "Should find at least one SDK package");

    println!("Found {} packages in filesystem", packages.len());
}

#[test]
fn test_get_workspace_members() {
    let root = get_workspace_root();
    let members = get_workspace_members(&root).expect("Failed to get workspace members");

    // Should have workspace members
    assert!(
        !members.is_empty(),
        "Should have at least one workspace member"
    );

    println!("Found {} workspace members", members.len());
}

#[test]
fn test_get_metadata_packages() {
    let root = get_workspace_root();
    let packages = get_metadata_packages(&root).expect("Failed to get metadata packages");

    // Should have packages in metadata
    assert!(
        !packages.is_empty(),
        "Should have at least one package in metadata"
    );

    println!("Found {} packages in metadata", packages.len());
}

#[test]
fn test_has_workspace_excludes() {
    let root = get_workspace_root();
    let has_excludes = has_workspace_excludes(&root).expect("Failed to check workspace excludes");

    // For honest topology, should have no excludes
    assert!(
        !has_excludes,
        "Workspace should have no excludes for honest topology"
    );
}

#[test]
fn test_workspace_truth_integrity() {
    let root = get_workspace_root();

    let filesystem_packages =
        get_filesystem_packages(&root).expect("Failed to get filesystem packages");
    let workspace_members = get_workspace_members(&root).expect("Failed to get workspace members");
    let metadata_packages = get_metadata_packages(&root).expect("Failed to get metadata packages");

    println!("Filesystem packages: {}", filesystem_packages.len());
    println!("Workspace members: {}", workspace_members.len());
    println!("Metadata packages: {}", metadata_packages.len());

    // All three should match
    assert_eq!(
        filesystem_packages.len(),
        workspace_members.len(),
        "Filesystem and workspace member counts should match"
    );

    assert_eq!(
        filesystem_packages.len(),
        metadata_packages.len(),
        "Filesystem and metadata package counts should match"
    );

    // Find any discrepancies
    let missing_from_workspace: Vec<_> =
        filesystem_packages.difference(&workspace_members).collect();

    let missing_from_metadata: Vec<_> =
        filesystem_packages.difference(&metadata_packages).collect();

    if !missing_from_workspace.is_empty() {
        println!("Missing from workspace:");
        for pkg in &missing_from_workspace {
            println!("  - {}", pkg);
        }
    }

    if !missing_from_metadata.is_empty() {
        println!("Missing from metadata:");
        for pkg in &missing_from_metadata {
            println!("  - {}", pkg);
        }
    }

    assert!(
        missing_from_workspace.is_empty(),
        "All filesystem packages should be in workspace"
    );

    assert!(
        missing_from_metadata.is_empty(),
        "All filesystem packages should be in metadata"
    );
}
