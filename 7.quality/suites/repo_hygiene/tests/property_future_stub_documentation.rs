// Feature: phase-3-editor-app-gold-gates
// Property 9: Future Stub Documentation
//
// **Validates: Requirement 6.3**
//
// This property test verifies that for any crate classified as FUTURE_STUB,
// there should be documentation explaining its intended purpose in the crate
// status ledger.

use proptest::prelude::*;
use regex::Regex;
use std::collections::HashMap;
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
}

#[derive(Debug, Clone)]
struct CrateEntry {
    path: String,
    package: String,
    status: CrateStatus,
    reason: String,
    must_not_pretend: String,
}

/// Parse the crate_status_ledger.md file
fn parse_crate_ledger(ledger_path: &Path) -> Result<Vec<CrateEntry>, String> {
    if !ledger_path.exists() {
        return Err(format!("Ledger file not found: {}", ledger_path.display()));
    }

    let content =
        fs::read_to_string(ledger_path).map_err(|e| format!("Failed to read ledger: {}", e))?;

    let mut entries = Vec::new();
    let mut in_orphan_table = false;
    let mut in_full_inventory = false;

    // Regex to parse orphan/future surfaces table: | Crate | Status | Reason | Must not pretend |
    let orphan_row_regex =
        Regex::new(r"^\|\s*([^|]+)\s*\|\s*([^|]+)\s*\|\s*([^|]+)\s*\|\s*([^|]+)\s*\|")
            .map_err(|e| format!("Failed to compile orphan regex: {}", e))?;

    // Regex to parse full inventory table: | Layer | Path | Package | Status | Product integrated | Action |
    let inventory_row_regex = Regex::new(
        r"^\|\s*([^|]+)\s*\|\s*([^|]+)\s*\|\s*([^|]+)\s*\|\s*([^|]+)\s*\|\s*([^|]+)\s*\|\s*([^|]+)\s*\|",
    )
    .map_err(|e| format!("Failed to compile inventory regex: {}", e))?;

    for line in content.lines() {
        let trimmed = line.trim();

        // Detect orphan/future surfaces table header
        if trimmed.starts_with("| Crate |") && trimmed.contains("| Must not pretend |") {
            in_orphan_table = true;
            in_full_inventory = false;
            continue;
        }

        // Detect full inventory table header
        if trimmed.starts_with("| Layer |")
            && trimmed.contains("| Package |")
            && trimmed.contains("| Status |")
        {
            in_full_inventory = true;
            in_orphan_table = false;
            continue;
        }

        // Skip separator line
        if trimmed.starts_with("|---") || trimmed.starts_with("| ---") {
            continue;
        }

        // Parse orphan/future surfaces table rows
        if in_orphan_table {
            if trimmed.is_empty() || !trimmed.starts_with('|') {
                in_orphan_table = false;
                continue;
            }

            if let Some(captures) = orphan_row_regex.captures(trimmed) {
                let path = captures
                    .get(1)
                    .map(|m| m.as_str().trim().to_string())
                    .unwrap_or_default();
                let status_str = captures
                    .get(2)
                    .map(|m| m.as_str().trim().to_string())
                    .unwrap_or_default();
                let reason = captures
                    .get(3)
                    .map(|m| m.as_str().trim().to_string())
                    .unwrap_or_default();
                let must_not_pretend = captures
                    .get(4)
                    .map(|m| m.as_str().trim().to_string())
                    .unwrap_or_default();

                if let Some(status) = CrateStatus::from_str(&status_str) {
                    // Extract package name from path (last component)
                    let package = path.split('/').next_back().unwrap_or(&path).to_string();

                    entries.push(CrateEntry {
                        path: path.clone(),
                        package,
                        status,
                        reason,
                        must_not_pretend,
                    });
                }
            }
        }

        // Parse full inventory table rows
        if in_full_inventory {
            if trimmed.is_empty() || !trimmed.starts_with('|') {
                in_full_inventory = false;
                continue;
            }

            if let Some(captures) = inventory_row_regex.captures(trimmed) {
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

                if let Some(status) = CrateStatus::from_str(&status_str) {
                    // Only add if it's a FUTURE_STUB and not already in entries
                    if status == CrateStatus::FutureStub
                        && !entries.iter().any(|e| e.package == package)
                    {
                        entries.push(CrateEntry {
                            path: path.clone(),
                            package,
                            status,
                            reason: String::new(),
                            must_not_pretend: String::new(),
                        });
                    }
                }
            }
        }
    }

    Ok(entries)
}

// Property 9: Future Stub Documentation
// **Validates: Requirement 6.3**
//
// For any crate classified as FUTURE_STUB, there should be documentation
// explaining its intended purpose in the crate status ledger.
proptest! {
    #![proptest_config(ProptestConfig::with_cases(10))]

    #[test]
    fn property_future_stubs_have_documentation(
        _seed in 0u64..100u64
    ) {
        let workspace_root = get_workspace_root();
        let ledger_path = workspace_root.join("7.quality/inventory/crate_status_ledger.md");

        // Parse the ledger
        let entries = parse_crate_ledger(&ledger_path)
            .expect("Failed to parse crate ledger");

        // Filter to only FUTURE_STUB entries
        let future_stubs: Vec<&CrateEntry> = entries
            .iter()
            .filter(|e| e.status == CrateStatus::FutureStub)
            .collect();

        prop_assert!(
            !future_stubs.is_empty(),
            "Ledger should contain at least one FUTURE_STUB crate for this test to be meaningful"
        );

        let mut violations = Vec::new();

        for entry in &future_stubs {
            // Check if the entry has meaningful documentation
            // Documentation can be in either the "reason" or "must_not_pretend" field
            let has_reason = !entry.reason.is_empty() && entry.reason.len() > 10;
            let has_must_not_pretend = !entry.must_not_pretend.is_empty() && entry.must_not_pretend.len() > 10;

            if !has_reason && !has_must_not_pretend {
                violations.push(format!(
                    "  FUTURE_STUB crate '{}' (at {}) lacks documentation explaining its purpose",
                    entry.package, entry.path
                ));
            }
        }

        prop_assert!(
            violations.is_empty(),
            "Found {} FUTURE_STUB crates without proper documentation:\n{}",
            violations.len(),
            violations.join("\n")
        );
    }
}

// Property 9b: Future Stub Documentation Quality
// **Validates: Requirement 6.3**
//
// For any FUTURE_STUB crate, the documentation should be meaningful
// (not just placeholder text or empty strings).
proptest! {
    #![proptest_config(ProptestConfig::with_cases(10))]

    #[test]
    fn property_future_stub_documentation_is_meaningful(
        _seed in 0u64..100u64
    ) {
        let workspace_root = get_workspace_root();
        let ledger_path = workspace_root.join("7.quality/inventory/crate_status_ledger.md");

        // Parse the ledger
        let entries = parse_crate_ledger(&ledger_path)
            .expect("Failed to parse crate ledger");

        // Filter to only FUTURE_STUB entries
        let future_stubs: Vec<&CrateEntry> = entries
            .iter()
            .filter(|e| e.status == CrateStatus::FutureStub)
            .collect();

        let mut violations = Vec::new();

        // Common placeholder patterns that indicate low-quality documentation
        let placeholder_patterns = [
            "TODO",
            "TBD",
            "placeholder",
            "stub",
            "not implemented",
            "coming soon",
        ];

        for entry in &future_stubs {
            let combined_doc = format!("{} {}", entry.reason, entry.must_not_pretend).to_lowercase();

            // Check if documentation is too short (less than 20 characters total)
            if combined_doc.trim().len() < 20 {
                violations.push(format!(
                    "  FUTURE_STUB crate '{}' has insufficient documentation (too short)",
                    entry.package
                ));
                continue;
            }

            // Check if documentation is only placeholder text
            let is_only_placeholder = placeholder_patterns
                .iter()
                .any(|pattern| combined_doc.contains(pattern) && combined_doc.len() < 50);

            if is_only_placeholder {
                violations.push(format!(
                    "  FUTURE_STUB crate '{}' has only placeholder documentation",
                    entry.package
                ));
            }
        }

        prop_assert!(
            violations.is_empty(),
            "Found {} FUTURE_STUB crates with low-quality documentation:\n{}",
            violations.len(),
            violations.join("\n")
        );
    }
}

// Property 9c: Future Stub "Must Not Pretend" Documentation
// **Validates: Requirement 6.3**
//
// For any FUTURE_STUB crate in the orphan/future surfaces table,
// it should have a "Must not pretend" field explaining what it must not be reported as.
proptest! {
    #![proptest_config(ProptestConfig::with_cases(10))]

    #[test]
    fn property_future_stubs_have_must_not_pretend(
        _seed in 0u64..100u64
    ) {
        let workspace_root = get_workspace_root();
        let ledger_path = workspace_root.join("7.quality/inventory/crate_status_ledger.md");

        // Parse the ledger
        let entries = parse_crate_ledger(&ledger_path)
            .expect("Failed to parse crate ledger");

        // Filter to only FUTURE_STUB entries that have must_not_pretend field
        // (these come from the orphan/future surfaces table)
        let future_stubs_with_table: Vec<&CrateEntry> = entries
            .iter()
            .filter(|e| e.status == CrateStatus::FutureStub && !e.must_not_pretend.is_empty())
            .collect();

        let mut violations = Vec::new();

        for entry in &future_stubs_with_table {
            // Check if "Must not pretend" field contains meaningful content
            let must_not_pretend_lower = entry.must_not_pretend.to_lowercase();

            // Should contain phrases like "must not be reported as" or similar
            let has_meaningful_warning = must_not_pretend_lower.contains("must not")
                || must_not_pretend_lower.contains("production-ready")
                || must_not_pretend_lower.contains("product-complete");

            if !has_meaningful_warning || entry.must_not_pretend.len() < 20 {
                violations.push(format!(
                    "  FUTURE_STUB crate '{}' has insufficient 'Must not pretend' documentation",
                    entry.package
                ));
            }
        }

        prop_assert!(
            violations.is_empty(),
            "Found {} FUTURE_STUB crates with insufficient 'Must not pretend' documentation:\n{}",
            violations.len(),
            violations.join("\n")
        );
    }
}

// Unit tests for helper functions

#[test]
fn test_parse_future_stub_crates() {
    let workspace_root = get_workspace_root();
    let ledger_path = workspace_root.join("7.quality/inventory/crate_status_ledger.md");

    let entries = parse_crate_ledger(&ledger_path).expect("Failed to parse crate ledger");

    // Filter to FUTURE_STUB entries
    let future_stubs: Vec<&CrateEntry> = entries
        .iter()
        .filter(|e| e.status == CrateStatus::FutureStub)
        .collect();

    // Should have at least some FUTURE_STUB entries
    assert!(
        !future_stubs.is_empty(),
        "Ledger should have FUTURE_STUB entries"
    );

    println!("Found {} FUTURE_STUB crate entries", future_stubs.len());

    // Print first entry for debugging
    if let Some(first) = future_stubs.first() {
        println!("First FUTURE_STUB entry: {:?}", first);
    }
}

#[test]
fn test_all_future_stubs_have_documentation() {
    let workspace_root = get_workspace_root();
    let ledger_path = workspace_root.join("7.quality/inventory/crate_status_ledger.md");

    let entries = parse_crate_ledger(&ledger_path).expect("Failed to parse crate ledger");

    let future_stubs: Vec<&CrateEntry> = entries
        .iter()
        .filter(|e| e.status == CrateStatus::FutureStub)
        .collect();

    let mut violations = Vec::new();

    for entry in &future_stubs {
        let has_reason = !entry.reason.is_empty() && entry.reason.len() > 10;
        let has_must_not_pretend =
            !entry.must_not_pretend.is_empty() && entry.must_not_pretend.len() > 10;

        if !has_reason && !has_must_not_pretend {
            violations.push(format!(
                "FUTURE_STUB crate '{}' lacks documentation",
                entry.package
            ));
        }
    }

    assert!(
        violations.is_empty(),
        "Found FUTURE_STUB crates without documentation:\n{}",
        violations.join("\n")
    );

    println!(
        "✓ All {} FUTURE_STUB crates have documentation",
        future_stubs.len()
    );
}

#[test]
fn test_future_stub_documentation_quality() {
    let workspace_root = get_workspace_root();
    let ledger_path = workspace_root.join("7.quality/inventory/crate_status_ledger.md");

    let entries = parse_crate_ledger(&ledger_path).expect("Failed to parse crate ledger");

    let future_stubs: Vec<&CrateEntry> = entries
        .iter()
        .filter(|e| e.status == CrateStatus::FutureStub)
        .collect();

    let mut doc_lengths: HashMap<String, usize> = HashMap::new();

    for entry in &future_stubs {
        let combined_doc = format!("{} {}", entry.reason, entry.must_not_pretend);
        doc_lengths.insert(entry.package.clone(), combined_doc.trim().len());
    }

    println!("Documentation length distribution:");
    for (package, length) in &doc_lengths {
        println!("  {}: {} characters", package, length);
    }

    // All should have at least 20 characters of documentation
    let insufficient: Vec<_> = doc_lengths.iter().filter(|(_, &len)| len < 20).collect();

    assert!(
        insufficient.is_empty(),
        "Found FUTURE_STUB crates with insufficient documentation: {:?}",
        insufficient
    );

    println!("✓ All FUTURE_STUB crates have sufficient documentation");
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
