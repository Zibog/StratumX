// Feature: phase-3-editor-app-gold-gates
// Property 11: Delete Crate Removal
//
// **Validates: Requirement 6.6**
//
// This property test verifies that no crates classified as DELETE remain in the
// workspace. Any crate marked for deletion should be removed from the filesystem.

use proptest::prelude::*;
use regex::Regex;
use std::collections::HashMap;
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

#[derive(Debug, Clone, PartialEq, Eq)]
enum CrateStatus {
    ActiveCore,
    ActiveProduct,
    ActiveSupport,
    FutureStub,
    Legacy,
    Delete,
}

impl CrateStatus {
    fn from_str(s: &str) -> Option<Self> {
        match s.trim() {
            "ACTIVE_CORE" => Some(CrateStatus::ActiveCore),
            "ACTIVE_PRODUCT" => Some(CrateStatus::ActiveProduct),
            "ACTIVE_SUPPORT" => Some(CrateStatus::ActiveSupport),
            "FUTURE_STUB" => Some(CrateStatus::FutureStub),
            "LEGACY" => Some(CrateStatus::Legacy),
            "DELETE" => Some(CrateStatus::Delete),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
struct CrateEntry {
    #[allow(dead_code)]
    layer: String,
    path: String,
    package: String,
    statuses: Vec<CrateStatus>,
}

/// Parse the crate_status_ledger.md file
fn parse_crate_ledger(ledger_path: &Path) -> Result<Vec<CrateEntry>, String> {
    if !ledger_path.exists() {
        return Err(format!("Ledger file not found: {}", ledger_path.display()));
    }

    let content =
        fs::read_to_string(ledger_path).map_err(|e| format!("Failed to read ledger: {}", e))?;

    let mut entries = Vec::new();
    let mut in_table = false;

    // Regex to parse table rows: | Layer | Path | Package | Status | ... |
    let row_regex = Regex::new(r"^\|\s*([^|]+)\s*\|\s*([^|]+)\s*\|\s*([^|]+)\s*\|\s*([^|]+)\s*\|")
        .map_err(|e| format!("Failed to compile regex: {}", e))?;

    for line in content.lines() {
        let trimmed = line.trim();

        // Detect table header for full inventory
        if trimmed.starts_with("| Layer |") && trimmed.contains("| Status |") {
            in_table = true;
            continue;
        }

        // Skip separator line
        if trimmed.starts_with("|---") || trimmed.starts_with("| ---") {
            in_table = false;
            continue;
        }

        // Parse table rows
        if in_table {
            if trimmed.is_empty() || !trimmed.starts_with('|') {
                in_table = false;
                continue;
            }

            if let Some(captures) = row_regex.captures(trimmed) {
                let layer = captures
                    .get(1)
                    .map(|m| m.as_str().trim().to_string())
                    .unwrap_or_default();
                let path = captures
                    .get(2)
                    .map(|m| m.as_str().trim().to_string())
                    .unwrap_or_default();
                let package = captures
                    .get(3)
                    .map(|m| m.as_str().trim().to_string())
                    .unwrap_or_default();
                let status_str = captures
                    .get(4)
                    .map(|m| m.as_str().trim().to_string())
                    .unwrap_or_default();

                // Parse status - could be multiple if someone made a mistake
                let mut statuses = Vec::new();
                for status_part in status_str.split(',') {
                    if let Some(status) = CrateStatus::from_str(status_part.trim()) {
                        statuses.push(status);
                    }
                }

                // If no valid status found, skip this entry (might be header or invalid)
                if !statuses.is_empty() && !package.is_empty() {
                    entries.push(CrateEntry {
                        layer,
                        path,
                        package,
                        statuses,
                    });
                }
            }
        }
    }

    Ok(entries)
}

/// Find all Cargo.toml files in the workspace
fn find_workspace_crates(workspace_root: &Path) -> Vec<PathBuf> {
    let mut crates = Vec::new();
    let search_dirs = vec![
        "2.engine",
        "3.sdk",
        "4.tooling",
        "5.editor",
        "6.apps",
        "7.quality",
    ];

    for dir in search_dirs {
        let full_path = workspace_root.join(dir);
        if full_path.exists() {
            for entry in WalkDir::new(&full_path).into_iter().filter_map(|e| e.ok()) {
                if entry.file_name() == "Cargo.toml" {
                    // Skip target directories
                    if !entry.path().to_string_lossy().contains("/target/")
                        && !entry.path().to_string_lossy().contains("\\target\\")
                    {
                        crates.push(entry.path().to_path_buf());
                    }
                }
            }
        }
    }

    crates
}

/// Extract package name from Cargo.toml
fn get_package_name(cargo_toml_path: &Path) -> Option<String> {
    let content = fs::read_to_string(cargo_toml_path).ok()?;

    // Simple regex to extract package name
    let name_regex = Regex::new(r#"name\s*=\s*"([^"]+)""#).ok()?;

    if let Some(captures) = name_regex.captures(&content) {
        return captures.get(1).map(|m| m.as_str().to_string());
    }

    None
}

// Property 11: Delete Crate Removal
// **Validates: Requirement 6.6**
//
// For any crate classified as DELETE, if it still exists in the workspace,
// validation should fail.
proptest! {
    #![proptest_config(ProptestConfig::with_cases(10))]

    #[test]
    fn property_no_delete_crates_in_workspace(
        _seed in 0u64..100u64
    ) {
        let workspace_root = get_workspace_root();
        let ledger_path = workspace_root.join("7.quality/inventory/crate_status_ledger.md");

        // Parse the ledger to find DELETE-classified crates
        let entries = parse_crate_ledger(&ledger_path)
            .expect("Failed to parse crate ledger");

        // Find all crates marked as DELETE
        let delete_crates: Vec<&CrateEntry> = entries
            .iter()
            .filter(|entry| entry.statuses.iter().any(|s| matches!(s, CrateStatus::Delete)))
            .collect();

        // Find all actual crates in the workspace
        let workspace_crates = find_workspace_crates(&workspace_root);

        // Build a map of package names to paths
        let mut workspace_packages: HashMap<String, PathBuf> = HashMap::new();
        for crate_path in &workspace_crates {
            if let Some(package_name) = get_package_name(crate_path) {
                workspace_packages.insert(package_name, crate_path.clone());
            }
        }

        // Check if any DELETE-classified crates still exist
        let mut violations = Vec::new();

        for delete_entry in &delete_crates {
            if workspace_packages.contains_key(&delete_entry.package) {
                let actual_path = workspace_packages.get(&delete_entry.package).unwrap();
                violations.push(format!(
                    "  Crate '{}' is classified as DELETE but still exists at: {}",
                    delete_entry.package,
                    actual_path.display()
                ));
            }
        }

        prop_assert!(
            violations.is_empty(),
            "Found {} DELETE-classified crates still in workspace:\n{}",
            violations.len(),
            violations.join("\n")
        );
    }
}

// Property 11b: DELETE Crates Not in Workspace Members
// **Validates: Requirement 6.6**
//
// For any crate classified as DELETE, it should not be listed in workspace members.
proptest! {
    #![proptest_config(ProptestConfig::with_cases(10))]

    #[test]
    fn property_delete_crates_not_in_workspace_members(
        _seed in 0u64..100u64
    ) {
        let workspace_root = get_workspace_root();
        let ledger_path = workspace_root.join("7.quality/inventory/crate_status_ledger.md");

        // Parse the ledger to find DELETE-classified crates
        let entries = parse_crate_ledger(&ledger_path)
            .expect("Failed to parse crate ledger");

        // Find all crates marked as DELETE
        let delete_crates: Vec<&CrateEntry> = entries
            .iter()
            .filter(|entry| entry.statuses.iter().any(|s| matches!(s, CrateStatus::Delete)))
            .collect();

        // Get workspace members from cargo metadata
        let output = std::process::Command::new("cargo")
            .arg("metadata")
            .arg("--format-version=1")
            .arg("--no-deps")
            .current_dir(&workspace_root)
            .output()
            .expect("Failed to run cargo metadata");

        prop_assert!(
            output.status.success(),
            "cargo metadata command failed"
        );

        let metadata_str = String::from_utf8_lossy(&output.stdout);
        let metadata: serde_json::Value = serde_json::from_str(&metadata_str)
            .expect("Failed to parse cargo metadata");

        let workspace_members: Vec<String> = metadata["packages"]
            .as_array()
            .expect("packages should be an array")
            .iter()
            .filter_map(|pkg| pkg["name"].as_str().map(|s| s.to_string()))
            .collect();

        // Check if any DELETE-classified crates are in workspace members
        let mut violations = Vec::new();

        for delete_entry in &delete_crates {
            if workspace_members.contains(&delete_entry.package) {
                violations.push(format!(
                    "  Crate '{}' is classified as DELETE but is still a workspace member",
                    delete_entry.package
                ));
            }
        }

        prop_assert!(
            violations.is_empty(),
            "Found {} DELETE-classified crates still in workspace members:\n{}",
            violations.len(),
            violations.join("\n")
        );
    }
}

// Unit tests for helper functions

#[test]
fn test_find_workspace_crates() {
    let workspace_root = get_workspace_root();
    let crates = find_workspace_crates(&workspace_root);

    // Should find at least some crates
    assert!(!crates.is_empty(), "Should find at least one crate");

    println!("Found {} crates in workspace", crates.len());

    // All paths should end with Cargo.toml
    for crate_path in &crates {
        assert!(
            crate_path.ends_with("Cargo.toml"),
            "Path should end with Cargo.toml: {}",
            crate_path.display()
        );
    }

    println!("✓ All crate paths are valid");
}

#[test]
fn test_get_package_name() {
    let workspace_root = get_workspace_root();
    let crates = find_workspace_crates(&workspace_root);

    let mut package_names = Vec::new();

    for crate_path in crates.iter().take(5) {
        if let Some(name) = get_package_name(crate_path) {
            package_names.push(name);
        }
    }

    assert!(
        !package_names.is_empty(),
        "Should extract at least one package name"
    );

    println!("Sample package names: {:?}", package_names);
    println!("✓ Package name extraction works");
}

#[test]
fn test_no_delete_crates_exist() {
    let workspace_root = get_workspace_root();
    let ledger_path = workspace_root.join("7.quality/inventory/crate_status_ledger.md");

    // Parse the ledger
    let entries = parse_crate_ledger(&ledger_path).expect("Failed to parse crate ledger");

    // Find all crates marked as DELETE
    let delete_crates: Vec<&CrateEntry> = entries
        .iter()
        .filter(|entry| {
            entry
                .statuses
                .iter()
                .any(|s| matches!(s, CrateStatus::Delete))
        })
        .collect();

    if delete_crates.is_empty() {
        println!("✓ No crates are classified as DELETE (good!)");
        return;
    }

    println!("Found {} crates classified as DELETE:", delete_crates.len());
    for entry in &delete_crates {
        println!("  - {} at {}", entry.package, entry.path);
    }

    // Find all actual crates in the workspace
    let workspace_crates = find_workspace_crates(&workspace_root);

    // Build a map of package names to paths
    let mut workspace_packages: HashMap<String, PathBuf> = HashMap::new();
    for crate_path in &workspace_crates {
        if let Some(package_name) = get_package_name(crate_path) {
            workspace_packages.insert(package_name, crate_path.clone());
        }
    }

    // Check if any DELETE-classified crates still exist
    let mut violations = Vec::new();

    for delete_entry in &delete_crates {
        if workspace_packages.contains_key(&delete_entry.package) {
            let actual_path = workspace_packages.get(&delete_entry.package).unwrap();
            violations.push(format!(
                "Crate '{}' is classified as DELETE but still exists at: {}",
                delete_entry.package,
                actual_path.display()
            ));
        }
    }

    assert!(
        violations.is_empty(),
        "Found DELETE-classified crates still in workspace:\n{}",
        violations.join("\n")
    );

    println!("✓ All DELETE-classified crates have been removed from workspace");
}

#[test]
fn test_delete_crates_not_in_cargo_metadata() {
    let workspace_root = get_workspace_root();
    let ledger_path = workspace_root.join("7.quality/inventory/crate_status_ledger.md");

    // Parse the ledger
    let entries = parse_crate_ledger(&ledger_path).expect("Failed to parse crate ledger");

    // Find all crates marked as DELETE
    let delete_crates: Vec<&CrateEntry> = entries
        .iter()
        .filter(|entry| {
            entry
                .statuses
                .iter()
                .any(|s| matches!(s, CrateStatus::Delete))
        })
        .collect();

    if delete_crates.is_empty() {
        println!("✓ No crates are classified as DELETE");
        return;
    }

    // Get workspace members from cargo metadata
    let output = std::process::Command::new("cargo")
        .arg("metadata")
        .arg("--format-version=1")
        .arg("--no-deps")
        .current_dir(&workspace_root)
        .output()
        .expect("Failed to run cargo metadata");

    assert!(output.status.success(), "cargo metadata command failed");

    let metadata_str = String::from_utf8_lossy(&output.stdout);
    let metadata: serde_json::Value =
        serde_json::from_str(&metadata_str).expect("Failed to parse cargo metadata");

    let workspace_members: Vec<String> = metadata["packages"]
        .as_array()
        .expect("packages should be an array")
        .iter()
        .filter_map(|pkg| pkg["name"].as_str().map(|s| s.to_string()))
        .collect();

    // Check if any DELETE-classified crates are in workspace members
    let mut violations = Vec::new();

    for delete_entry in &delete_crates {
        if workspace_members.contains(&delete_entry.package) {
            violations.push(format!(
                "Crate '{}' is classified as DELETE but is still a workspace member",
                delete_entry.package
            ));
        }
    }

    assert!(
        violations.is_empty(),
        "Found DELETE-classified crates in workspace members:\n{}",
        violations.join("\n")
    );

    println!("✓ No DELETE-classified crates are workspace members");
}
