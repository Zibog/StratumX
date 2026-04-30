// Feature: phase-3-editor-app-gold-gates
// Property 10: Active Core Crate Integration
//
// **Validates: Requirement 6.4**
//
// This property test verifies that for any crate classified as ACTIVE_CORE,
// if it is orphaned (not properly integrated into the workspace), validation
// should fail.

use proptest::prelude::*;
use regex::Regex;
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

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

    fn as_str(&self) -> &'static str {
        match self {
            CrateStatus::ActiveCore => "ACTIVE_CORE",
            CrateStatus::ActiveProduct => "ACTIVE_PRODUCT",
            CrateStatus::ActiveSupport => "ACTIVE_SUPPORT",
            CrateStatus::FutureStub => "FUTURE_STUB",
            CrateStatus::Legacy => "LEGACY",
            CrateStatus::Delete => "DELETE",
        }
    }
}

#[derive(Debug, Clone)]
struct CrateEntry {
    path: String,
    package: String,
    status: CrateStatus,
    product_integrated: bool,
}

/// Parse the crate_status_ledger.md file
fn parse_crate_ledger(ledger_path: &Path) -> Result<Vec<CrateEntry>, String> {
    if !ledger_path.exists() {
        return Err(format!("Ledger file not found: {}", ledger_path.display()));
    }

    let content =
        fs::read_to_string(ledger_path).map_err(|e| format!("Failed to read ledger: {}", e))?;

    let mut entries = Vec::new();
    let mut in_full_inventory = false;

    // Regex to parse full inventory table rows: | Layer | Path | Package | Status | Product integrated | Action |
    let row_regex = Regex::new(
        r"^\|\s*([^|]+)\s*\|\s*([^|]+)\s*\|\s*([^|]+)\s*\|\s*([^|]+)\s*\|\s*([^|]+)\s*\|",
    )
    .map_err(|e| format!("Failed to compile regex: {}", e))?;

    for line in content.lines() {
        let trimmed = line.trim();

        // Detect full inventory table header
        if trimmed.starts_with("| Layer |")
            && trimmed.contains("| Status |")
            && trimmed.contains("| Product integrated |")
        {
            in_full_inventory = true;
            continue;
        }

        // Skip separator line
        if trimmed.starts_with("|---") || trimmed.starts_with("| ---") {
            continue;
        }

        // Parse full inventory table rows
        if in_full_inventory {
            if trimmed.is_empty() || !trimmed.starts_with('|') {
                in_full_inventory = false;
                continue;
            }

            if let Some(captures) = row_regex.captures(trimmed) {
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
                let integrated_str = captures
                    .get(5)
                    .map(|m| m.as_str().trim().to_string())
                    .unwrap_or_default();

                // Parse status
                if let Some(status) = CrateStatus::from_str(&status_str) {
                    let product_integrated = integrated_str == "yes";

                    if !package.is_empty() {
                        entries.push(CrateEntry {
                            path,
                            package,
                            status,
                            product_integrated,
                        });
                    }
                }
            }
        }
    }

    Ok(entries)
}

/// Parse workspace members from Cargo.toml
fn parse_workspace_members(cargo_toml_path: &Path) -> Result<HashSet<String>, String> {
    if !cargo_toml_path.exists() {
        return Err(format!(
            "Cargo.toml not found: {}",
            cargo_toml_path.display()
        ));
    }

    let content = fs::read_to_string(cargo_toml_path)
        .map_err(|e| format!("Failed to read Cargo.toml: {}", e))?;

    let mut members = HashSet::new();
    let mut in_members = false;

    for line in content.lines() {
        let trimmed = line.trim();

        if trimmed.starts_with("members = [") {
            in_members = true;
            continue;
        }

        if in_members {
            if trimmed == "]" {
                break;
            }

            // Extract path from quoted string
            if let Some(start) = trimmed.find('"') {
                if let Some(end) = trimmed[start + 1..].find('"') {
                    let member_path = &trimmed[start + 1..start + 1 + end];
                    members.insert(member_path.to_string());
                }
            }
        }
    }

    Ok(members)
}

/// Check if a crate path exists in the workspace members
fn is_crate_in_workspace(crate_path: &str, workspace_members: &HashSet<String>) -> bool {
    workspace_members.contains(crate_path)
}

// Property 10: Active Core Crate Integration
// **Validates: Requirement 6.4**
//
// For any crate classified as ACTIVE_CORE, if it is orphaned (not properly
// integrated into the workspace), validation should fail.
proptest! {
    #![proptest_config(ProptestConfig::with_cases(10))]

    #[test]
    fn property_active_core_crates_not_orphaned(
        _seed in 0u64..100u64
    ) {
        let workspace_root = get_workspace_root();
        let ledger_path = workspace_root.join("7.quality/inventory/crate_status_ledger.md");
        let cargo_toml_path = workspace_root.join("Cargo.toml");

        // Parse the ledger
        let entries = parse_crate_ledger(&ledger_path)
            .expect("Failed to parse crate ledger");

        // Parse workspace members
        let workspace_members = parse_workspace_members(&cargo_toml_path)
            .expect("Failed to parse workspace members");

        prop_assert!(
            !entries.is_empty(),
            "Ledger should contain at least one crate entry"
        );

        prop_assert!(
            !workspace_members.is_empty(),
            "Workspace should have at least one member"
        );

        // Find all ACTIVE_CORE crates
        let active_core_crates: Vec<&CrateEntry> = entries
            .iter()
            .filter(|e| e.status == CrateStatus::ActiveCore)
            .collect();

        prop_assert!(
            !active_core_crates.is_empty(),
            "Should have at least one ACTIVE_CORE crate"
        );

        let mut orphaned_crates = Vec::new();

        for entry in &active_core_crates {
            if !is_crate_in_workspace(&entry.path, &workspace_members) {
                orphaned_crates.push(format!(
                    "  ACTIVE_CORE crate '{}' (at {}) is ORPHANED - not in workspace members",
                    entry.package, entry.path
                ));
            }
        }

        prop_assert!(
            orphaned_crates.is_empty(),
            "Found {} orphaned ACTIVE_CORE crates (VALIDATION FAILURE):\n{}",
            orphaned_crates.len(),
            orphaned_crates.join("\n")
        );
    }
}

// Property 10b: Active Core Crates Are Product Integrated
// **Validates: Requirement 6.4**
//
// For any crate classified as ACTIVE_CORE, it should be marked as product
// integrated in the ledger.
proptest! {
    #![proptest_config(ProptestConfig::with_cases(10))]

    #[test]
    fn property_active_core_crates_product_integrated(
        _seed in 0u64..100u64
    ) {
        let workspace_root = get_workspace_root();
        let ledger_path = workspace_root.join("7.quality/inventory/crate_status_ledger.md");

        // Parse the ledger
        let entries = parse_crate_ledger(&ledger_path)
            .expect("Failed to parse crate ledger");

        // Find all ACTIVE_CORE crates
        let active_core_crates: Vec<&CrateEntry> = entries
            .iter()
            .filter(|e| e.status == CrateStatus::ActiveCore)
            .collect();

        let mut not_integrated = Vec::new();

        for entry in &active_core_crates {
            if !entry.product_integrated {
                not_integrated.push(format!(
                    "  ACTIVE_CORE crate '{}' (at {}) is marked as NOT product integrated",
                    entry.package, entry.path
                ));
            }
        }

        prop_assert!(
            not_integrated.is_empty(),
            "Found {} ACTIVE_CORE crates not marked as product integrated:\n{}",
            not_integrated.len(),
            not_integrated.join("\n")
        );
    }
}

// Property 10c: Active Product Crates Not Orphaned
// **Validates: Requirement 6.5**
//
// For any crate classified as ACTIVE_PRODUCT, it should not be orphaned unless
// intentionally consumed by host/root.
proptest! {
    #![proptest_config(ProptestConfig::with_cases(10))]

    #[test]
    fn property_active_product_crates_not_orphaned(
        _seed in 0u64..100u64
    ) {
        let workspace_root = get_workspace_root();
        let ledger_path = workspace_root.join("7.quality/inventory/crate_status_ledger.md");
        let cargo_toml_path = workspace_root.join("Cargo.toml");

        // Parse the ledger
        let entries = parse_crate_ledger(&ledger_path)
            .expect("Failed to parse crate ledger");

        // Parse workspace members
        let workspace_members = parse_workspace_members(&cargo_toml_path)
            .expect("Failed to parse workspace members");

        // Find all ACTIVE_PRODUCT crates
        let active_product_crates: Vec<&CrateEntry> = entries
            .iter()
            .filter(|e| e.status == CrateStatus::ActiveProduct)
            .collect();

        let mut orphaned_crates = Vec::new();

        for entry in &active_product_crates {
            if !is_crate_in_workspace(&entry.path, &workspace_members) {
                orphaned_crates.push(format!(
                    "  ACTIVE_PRODUCT crate '{}' (at {}) is ORPHANED - not in workspace members",
                    entry.package, entry.path
                ));
            }
        }

        prop_assert!(
            orphaned_crates.is_empty(),
            "Found {} orphaned ACTIVE_PRODUCT crates (VALIDATION FAILURE):\n{}",
            orphaned_crates.len(),
            orphaned_crates.join("\n")
        );
    }
}

// Unit tests for helper functions

#[test]
fn test_parse_crate_ledger() {
    let workspace_root = get_workspace_root();
    let ledger_path = workspace_root.join("7.quality/inventory/crate_status_ledger.md");

    let entries = parse_crate_ledger(&ledger_path).expect("Failed to parse crate ledger");

    // Should have at least some crate entries
    assert!(!entries.is_empty(), "Ledger should have crate entries");

    println!("Found {} crate entries in ledger", entries.len());

    // Count ACTIVE_CORE crates
    let active_core_count = entries
        .iter()
        .filter(|e| e.status == CrateStatus::ActiveCore)
        .count();
    println!("Found {} ACTIVE_CORE crates", active_core_count);

    assert!(
        active_core_count > 0,
        "Should have at least one ACTIVE_CORE crate"
    );
}

#[test]
fn test_parse_workspace_members() {
    let workspace_root = get_workspace_root();
    let cargo_toml_path = workspace_root.join("Cargo.toml");

    let members =
        parse_workspace_members(&cargo_toml_path).expect("Failed to parse workspace members");

    assert!(!members.is_empty(), "Workspace should have members");

    println!("Found {} workspace members", members.len());

    // Check for some expected members
    assert!(
        members.contains("2.engine/l-1-foundation"),
        "Should contain engine foundation"
    );
    assert!(
        members.contains("5.editor/l7.0-editor-command-spine"),
        "Should contain editor command spine"
    );
}

#[test]
fn test_all_active_core_in_workspace() {
    let workspace_root = get_workspace_root();
    let ledger_path = workspace_root.join("7.quality/inventory/crate_status_ledger.md");
    let cargo_toml_path = workspace_root.join("Cargo.toml");

    let entries = parse_crate_ledger(&ledger_path).expect("Failed to parse crate ledger");
    let workspace_members =
        parse_workspace_members(&cargo_toml_path).expect("Failed to parse workspace members");

    let active_core_crates: Vec<&CrateEntry> = entries
        .iter()
        .filter(|e| e.status == CrateStatus::ActiveCore)
        .collect();

    let mut orphaned = Vec::new();

    for entry in &active_core_crates {
        if !is_crate_in_workspace(&entry.path, &workspace_members) {
            orphaned.push(format!("{} ({})", entry.package, entry.path));
        }
    }

    assert!(
        orphaned.is_empty(),
        "Found orphaned ACTIVE_CORE crates:\n{}",
        orphaned.join("\n")
    );

    println!(
        "✓ All {} ACTIVE_CORE crates are in workspace",
        active_core_crates.len()
    );
}

#[test]
fn test_all_active_product_in_workspace() {
    let workspace_root = get_workspace_root();
    let ledger_path = workspace_root.join("7.quality/inventory/crate_status_ledger.md");
    let cargo_toml_path = workspace_root.join("Cargo.toml");

    let entries = parse_crate_ledger(&ledger_path).expect("Failed to parse crate ledger");
    let workspace_members =
        parse_workspace_members(&cargo_toml_path).expect("Failed to parse workspace members");

    let active_product_crates: Vec<&CrateEntry> = entries
        .iter()
        .filter(|e| e.status == CrateStatus::ActiveProduct)
        .collect();

    let mut orphaned = Vec::new();

    for entry in &active_product_crates {
        if !is_crate_in_workspace(&entry.path, &workspace_members) {
            orphaned.push(format!("{} ({})", entry.package, entry.path));
        }
    }

    assert!(
        orphaned.is_empty(),
        "Found orphaned ACTIVE_PRODUCT crates:\n{}",
        orphaned.join("\n")
    );

    println!(
        "✓ All {} ACTIVE_PRODUCT crates are in workspace",
        active_product_crates.len()
    );
}

#[test]
fn test_active_core_product_integrated() {
    let workspace_root = get_workspace_root();
    let ledger_path = workspace_root.join("7.quality/inventory/crate_status_ledger.md");

    let entries = parse_crate_ledger(&ledger_path).expect("Failed to parse crate ledger");

    let active_core_crates: Vec<&CrateEntry> = entries
        .iter()
        .filter(|e| e.status == CrateStatus::ActiveCore)
        .collect();

    let mut not_integrated = Vec::new();

    for entry in &active_core_crates {
        if !entry.product_integrated {
            not_integrated.push(format!("{} ({})", entry.package, entry.path));
        }
    }

    assert!(
        not_integrated.is_empty(),
        "Found ACTIVE_CORE crates not marked as product integrated:\n{}",
        not_integrated.join("\n")
    );

    println!(
        "✓ All {} ACTIVE_CORE crates are product integrated",
        active_core_crates.len()
    );
}

#[test]
fn test_crate_status_distribution() {
    let workspace_root = get_workspace_root();
    let ledger_path = workspace_root.join("7.quality/inventory/crate_status_ledger.md");

    let entries = parse_crate_ledger(&ledger_path).expect("Failed to parse crate ledger");

    let mut status_counts = std::collections::HashMap::new();

    for entry in &entries {
        *status_counts.entry(entry.status.as_str()).or_insert(0) += 1;
    }

    println!("Crate status distribution:");
    for (status, count) in &status_counts {
        println!("  {}: {}", status, count);
    }

    // Verify we have the expected status types
    assert!(
        status_counts.get("ACTIVE_CORE").unwrap_or(&0) > &0,
        "Should have ACTIVE_CORE crates"
    );
    assert!(
        status_counts.get("ACTIVE_PRODUCT").unwrap_or(&0) > &0,
        "Should have ACTIVE_PRODUCT crates"
    );
    assert!(
        status_counts.get("ACTIVE_SUPPORT").unwrap_or(&0) > &0,
        "Should have ACTIVE_SUPPORT crates"
    );
    assert!(
        status_counts.get("FUTURE_STUB").unwrap_or(&0) > &0,
        "Should have FUTURE_STUB crates"
    );

    println!("✓ Ledger contains all expected status types");
}
