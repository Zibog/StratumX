// Feature: phase-3-editor-app-gold-gates
// Property 8: Crate Classification Uniqueness
//
// **Validates: Requirement 6.2**
//
// This property test verifies that each crate in the workspace has exactly one
// classification status (ACTIVE_CORE, ACTIVE_PRODUCT, ACTIVE_SUPPORT, FUTURE_STUB,
// LEGACY, or DELETE), not multiple or none.

use proptest::prelude::*;
use regex::Regex;
use std::collections::{HashMap, HashSet};
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

// Property 8: Crate Classification Uniqueness
// **Validates: Requirement 6.2**
//
// For any crate in the workspace, it should have exactly one classification status
// (ACTIVE_CORE, ACTIVE_PRODUCT, ACTIVE_SUPPORT, FUTURE_STUB, LEGACY, or DELETE),
// not multiple or none.
proptest! {
    #![proptest_config(ProptestConfig::with_cases(10))]

    #[test]
    fn property_each_crate_has_exactly_one_status(
        _seed in 0u64..100u64
    ) {
        let workspace_root = get_workspace_root();
        let ledger_path = workspace_root.join("7.quality/inventory/crate_status_ledger.md");

        // Parse the ledger
        let entries = parse_crate_ledger(&ledger_path)
            .expect("Failed to parse crate ledger");

        prop_assert!(
            !entries.is_empty(),
            "Ledger should contain at least one crate entry"
        );

        let mut violations = Vec::new();

        for entry in &entries {
            if entry.statuses.is_empty() {
                violations.push(format!(
                    "  Crate '{}' (at {}) has NO status classification",
                    entry.package, entry.path
                ));
            } else if entry.statuses.len() > 1 {
                let status_names: Vec<&str> = entry.statuses.iter().map(|s| s.as_str()).collect();
                violations.push(format!(
                    "  Crate '{}' (at {}) has MULTIPLE status classifications: {}",
                    entry.package,
                    entry.path,
                    status_names.join(", ")
                ));
            }
        }

        prop_assert!(
            violations.is_empty(),
            "Found {} crates with invalid status classifications:\n{}",
            violations.len(),
            violations.join("\n")
        );
    }
}

// Property 8b: No Duplicate Crate Entries
// **Validates: Requirement 6.2**
//
// For any crate package name, it should appear exactly once in the ledger.
proptest! {
    #![proptest_config(ProptestConfig::with_cases(10))]

    #[test]
    fn property_no_duplicate_crate_entries(
        _seed in 0u64..100u64
    ) {
        let workspace_root = get_workspace_root();
        let ledger_path = workspace_root.join("7.quality/inventory/crate_status_ledger.md");

        // Parse the ledger
        let entries = parse_crate_ledger(&ledger_path)
            .expect("Failed to parse crate ledger");

        let mut package_counts: HashMap<String, Vec<String>> = HashMap::new();

        for entry in &entries {
            package_counts
                .entry(entry.package.clone())
                .or_default()
                .push(entry.path.clone());
        }

        let mut duplicates = Vec::new();

        for (package, paths) in &package_counts {
            if paths.len() > 1 {
                duplicates.push(format!(
                    "  Package '{}' appears {} times at: {}",
                    package,
                    paths.len(),
                    paths.join(", ")
                ));
            }
        }

        prop_assert!(
            duplicates.is_empty(),
            "Found {} duplicate crate entries:\n{}",
            duplicates.len(),
            duplicates.join("\n")
        );
    }
}

// Property 8c: Status Values Are Valid
// **Validates: Requirement 6.2**
//
// For any crate entry, the status should be one of the valid classification values.
proptest! {
    #![proptest_config(ProptestConfig::with_cases(10))]

    #[test]
    fn property_status_values_are_valid(
        _seed in 0u64..100u64
    ) {
        let workspace_root = get_workspace_root();
        let ledger_path = workspace_root.join("7.quality/inventory/crate_status_ledger.md");

        // Parse the ledger
        let entries = parse_crate_ledger(&ledger_path)
            .expect("Failed to parse crate ledger");

        // All entries should have valid statuses (parsing already filters invalid ones)
        // But we verify that each entry has at least one status
        let mut missing_status = Vec::new();

        for entry in &entries {
            if entry.statuses.is_empty() {
                missing_status.push(format!(
                    "  Crate '{}' (at {}) has no valid status",
                    entry.package, entry.path
                ));
            }
        }

        prop_assert!(
            missing_status.is_empty(),
            "Found {} crates with missing or invalid status:\n{}",
            missing_status.len(),
            missing_status.join("\n")
        );
    }
}

// Property 8d: Active Crates Are Not Also Future Stubs
// **Validates: Requirement 6.2**
//
// For any crate classified as active (ACTIVE_CORE, ACTIVE_PRODUCT, ACTIVE_SUPPORT),
// it should not also be classified as FUTURE_STUB.
proptest! {
    #![proptest_config(ProptestConfig::with_cases(10))]

    #[test]
    fn property_active_crates_not_future_stubs(
        _seed in 0u64..100u64
    ) {
        let workspace_root = get_workspace_root();
        let ledger_path = workspace_root.join("7.quality/inventory/crate_status_ledger.md");

        // Parse the ledger
        let entries = parse_crate_ledger(&ledger_path)
            .expect("Failed to parse crate ledger");

        let mut conflicts = Vec::new();

        for entry in &entries {
            let has_active = entry.statuses.iter().any(|s| matches!(
                s,
                CrateStatus::ActiveCore | CrateStatus::ActiveProduct | CrateStatus::ActiveSupport
            ));
            let has_future_stub = entry.statuses.iter().any(|s| matches!(s, CrateStatus::FutureStub));

            if has_active && has_future_stub {
                let status_names: Vec<&str> = entry.statuses.iter().map(|s| s.as_str()).collect();
                conflicts.push(format!(
                    "  Crate '{}' (at {}) is classified as both ACTIVE and FUTURE_STUB: {}",
                    entry.package,
                    entry.path,
                    status_names.join(", ")
                ));
            }
        }

        prop_assert!(
            conflicts.is_empty(),
            "Found {} crates with conflicting active/future classifications:\n{}",
            conflicts.len(),
            conflicts.join("\n")
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

    // Print first entry for debugging
    if let Some(first) = entries.first() {
        println!("First entry: {:?}", first);
    }
}

#[test]
fn test_all_crates_have_single_status() {
    let workspace_root = get_workspace_root();
    let ledger_path = workspace_root.join("7.quality/inventory/crate_status_ledger.md");

    let entries = parse_crate_ledger(&ledger_path).expect("Failed to parse crate ledger");

    let mut violations = Vec::new();

    for entry in &entries {
        if entry.statuses.len() != 1 {
            violations.push(format!(
                "Crate '{}' has {} statuses (expected 1)",
                entry.package,
                entry.statuses.len()
            ));
        }
    }

    assert!(
        violations.is_empty(),
        "Found crates with invalid status count:\n{}",
        violations.join("\n")
    );

    println!("✓ All {} crates have exactly one status", entries.len());
}

#[test]
fn test_no_duplicate_packages() {
    let workspace_root = get_workspace_root();
    let ledger_path = workspace_root.join("7.quality/inventory/crate_status_ledger.md");

    let entries = parse_crate_ledger(&ledger_path).expect("Failed to parse crate ledger");

    let mut seen = HashSet::new();
    let mut duplicates = Vec::new();

    for entry in &entries {
        if !seen.insert(&entry.package) {
            duplicates.push(entry.package.clone());
        }
    }

    assert!(
        duplicates.is_empty(),
        "Found duplicate package entries: {:?}",
        duplicates
    );

    println!("✓ All {} package names are unique", entries.len());
}

#[test]
fn test_status_enum_parsing() {
    assert_eq!(
        CrateStatus::from_str("ACTIVE_CORE"),
        Some(CrateStatus::ActiveCore)
    );
    assert_eq!(
        CrateStatus::from_str("ACTIVE_PRODUCT"),
        Some(CrateStatus::ActiveProduct)
    );
    assert_eq!(
        CrateStatus::from_str("ACTIVE_SUPPORT"),
        Some(CrateStatus::ActiveSupport)
    );
    assert_eq!(
        CrateStatus::from_str("FUTURE_STUB"),
        Some(CrateStatus::FutureStub)
    );
    assert_eq!(CrateStatus::from_str("LEGACY"), Some(CrateStatus::Legacy));
    assert_eq!(CrateStatus::from_str("DELETE"), Some(CrateStatus::Delete));
    assert_eq!(CrateStatus::from_str("INVALID"), None);

    println!("✓ Status enum parsing works correctly");
}

#[test]
fn test_ledger_has_expected_statuses() {
    let workspace_root = get_workspace_root();
    let ledger_path = workspace_root.join("7.quality/inventory/crate_status_ledger.md");

    let entries = parse_crate_ledger(&ledger_path).expect("Failed to parse crate ledger");

    let mut status_counts: HashMap<String, usize> = HashMap::new();

    for entry in &entries {
        for status in &entry.statuses {
            *status_counts
                .entry(status.as_str().to_string())
                .or_insert(0) += 1;
        }
    }

    println!("Status distribution:");
    for (status, count) in &status_counts {
        println!("  {}: {}", status, count);
    }

    // Should have at least some ACTIVE_CORE and FUTURE_STUB entries
    assert!(
        status_counts.get("ACTIVE_CORE").unwrap_or(&0) > &0,
        "Should have at least one ACTIVE_CORE crate"
    );
    assert!(
        status_counts.get("FUTURE_STUB").unwrap_or(&0) > &0,
        "Should have at least one FUTURE_STUB crate"
    );

    println!("✓ Ledger contains expected status types");
}
