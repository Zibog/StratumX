// Feature: phase-3-editor-app-gold-gates
// Property 7: Ledger Domain Completeness
//
// **Validates: Requirements 4.9, 4.10**
//
// This property test verifies that all domain entries in the CODE_VS_CANON_STATUS_LEDGER.md
// have percentage values for docs readiness and code readiness, and have next implementation
// steps documented.

use proptest::prelude::*;
use regex::Regex;
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

#[derive(Debug, Clone)]
struct DomainEntry {
    domain_name: String,
    docs_readiness: Option<u8>,
    code_readiness: Option<u8>,
    status: String,
    next_steps: String,
}

/// Parse the CODE_VS_CANON_STATUS_LEDGER.md file
fn parse_ledger(ledger_path: &Path) -> Result<Vec<DomainEntry>, String> {
    if !ledger_path.exists() {
        return Err(format!("Ledger file not found: {}", ledger_path.display()));
    }

    let content =
        fs::read_to_string(ledger_path).map_err(|e| format!("Failed to read ledger: {}", e))?;

    let mut entries = Vec::new();
    let mut in_table = false;

    // Regex to parse table rows: | Domain | XX% | YY% | status | next steps |
    let row_regex = Regex::new(
        r"^\|\s*([^|]+)\s*\|\s*(\d+)%\s*\|\s*(\d+)%\s*\|\s*([^|]+)\s*\|\s*([^|]+)\s*\|$",
    )
    .map_err(|e| format!("Failed to compile regex: {}", e))?;

    for line in content.lines() {
        let trimmed = line.trim();

        // Detect table header
        if trimmed.starts_with("| Domain |") {
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
                let domain_name = captures
                    .get(1)
                    .map(|m| m.as_str().trim().to_string())
                    .unwrap_or_default();
                let docs_readiness = captures.get(2).and_then(|m| m.as_str().parse::<u8>().ok());
                let code_readiness = captures.get(3).and_then(|m| m.as_str().parse::<u8>().ok());
                let status = captures
                    .get(4)
                    .map(|m| m.as_str().trim().to_string())
                    .unwrap_or_default();
                let next_steps = captures
                    .get(5)
                    .map(|m| m.as_str().trim().to_string())
                    .unwrap_or_default();

                entries.push(DomainEntry {
                    domain_name,
                    docs_readiness,
                    code_readiness,
                    status,
                    next_steps,
                });
            }
        }
    }

    Ok(entries)
}

// Property 7: Ledger Domain Completeness
// **Validates: Requirements 4.9, 4.10**
//
// For any domain entry in the CODE_VS_CANON_STATUS_LEDGER.md, it should have
// percentage values for docs readiness and code readiness, and should have
// next implementation steps documented.
proptest! {
    #![proptest_config(ProptestConfig::with_cases(10))]

    #[test]
    fn property_ledger_domains_have_complete_information(
        _seed in 0u64..100u64
    ) {
        let workspace_root = get_workspace_root();
        let ledger_path = workspace_root.join("1.docs/developer_docs/CODE_VS_CANON_STATUS_LEDGER.md");

        // Parse the ledger
        let entries = parse_ledger(&ledger_path)
            .expect("Failed to parse ledger");

        prop_assert!(
            !entries.is_empty(),
            "Ledger should contain at least one domain entry"
        );

        let mut incomplete_entries = Vec::new();

        for entry in &entries {
            let mut issues = Vec::new();

            // Check docs readiness percentage
            if entry.docs_readiness.is_none() {
                issues.push("missing docs readiness percentage");
            } else if let Some(pct) = entry.docs_readiness {
                if pct > 100 {
                    issues.push("docs readiness percentage exceeds 100");
                }
            }

            // Check code readiness percentage
            if entry.code_readiness.is_none() {
                issues.push("missing code readiness percentage");
            } else if let Some(pct) = entry.code_readiness {
                if pct > 100 {
                    issues.push("code readiness percentage exceeds 100");
                }
            }

            // Check status is not empty
            if entry.status.is_empty() {
                issues.push("missing status");
            }

            // Check next steps are documented
            if entry.next_steps.is_empty() {
                issues.push("missing next implementation steps");
            }

            if !issues.is_empty() {
                incomplete_entries.push(format!(
                    "  Domain '{}': {}",
                    entry.domain_name,
                    issues.join(", ")
                ));
            }
        }

        prop_assert!(
            incomplete_entries.is_empty(),
            "Found {} domain entries with incomplete information:\n{}",
            incomplete_entries.len(),
            incomplete_entries.join("\n")
        );
    }
}

// Property 7b: Ledger Percentage Values Are Valid
// **Validates: Requirements 4.9**
//
// For any domain entry with percentage values, they should be in the valid range 0-100.
proptest! {
    #![proptest_config(ProptestConfig::with_cases(10))]

    #[test]
    fn property_ledger_percentages_are_valid(
        _seed in 0u64..100u64
    ) {
        let workspace_root = get_workspace_root();
        let ledger_path = workspace_root.join("1.docs/developer_docs/CODE_VS_CANON_STATUS_LEDGER.md");

        // Parse the ledger
        let entries = parse_ledger(&ledger_path)
            .expect("Failed to parse ledger");

        let mut invalid_entries = Vec::new();

        for entry in &entries {
            if let Some(docs_pct) = entry.docs_readiness {
                if docs_pct > 100 {
                    invalid_entries.push(format!(
                        "  Domain '{}': docs readiness {}% exceeds 100",
                        entry.domain_name, docs_pct
                    ));
                }
            }

            if let Some(code_pct) = entry.code_readiness {
                if code_pct > 100 {
                    invalid_entries.push(format!(
                        "  Domain '{}': code readiness {}% exceeds 100",
                        entry.domain_name, code_pct
                    ));
                }
            }
        }

        prop_assert!(
            invalid_entries.is_empty(),
            "Found {} domain entries with invalid percentage values:\n{}",
            invalid_entries.len(),
            invalid_entries.join("\n")
        );
    }
}

// Property 7c: Ledger Next Steps Are Actionable
// **Validates: Requirements 4.10**
//
// For any domain entry, the next implementation steps should be non-empty and actionable.
proptest! {
    #![proptest_config(ProptestConfig::with_cases(10))]

    #[test]
    fn property_ledger_next_steps_are_actionable(
        _seed in 0u64..100u64
    ) {
        let workspace_root = get_workspace_root();
        let ledger_path = workspace_root.join("1.docs/developer_docs/CODE_VS_CANON_STATUS_LEDGER.md");

        // Parse the ledger
        let entries = parse_ledger(&ledger_path)
            .expect("Failed to parse ledger");

        let mut vague_entries = Vec::new();

        for entry in &entries {
            // Check that next steps are not empty
            if entry.next_steps.is_empty() {
                vague_entries.push(format!(
                    "  Domain '{}': next steps are empty",
                    entry.domain_name
                ));
                continue;
            }

            // Check that next steps are not just placeholder text
            let next_steps_lower = entry.next_steps.to_lowercase();
            if next_steps_lower == "tbd"
                || next_steps_lower == "todo"
                || next_steps_lower == "n/a"
                || next_steps_lower == "none"
            {
                vague_entries.push(format!(
                    "  Domain '{}': next steps are placeholder text: '{}'",
                    entry.domain_name, entry.next_steps
                ));
            }
        }

        prop_assert!(
            vague_entries.is_empty(),
            "Found {} domain entries with vague or missing next steps:\n{}",
            vague_entries.len(),
            vague_entries.join("\n")
        );
    }
}

// Unit tests for helper functions

#[test]
fn test_parse_ledger() {
    let workspace_root = get_workspace_root();
    let ledger_path = workspace_root.join("1.docs/developer_docs/CODE_VS_CANON_STATUS_LEDGER.md");

    let entries = parse_ledger(&ledger_path).expect("Failed to parse ledger");

    // Should have at least some domain entries
    assert!(!entries.is_empty(), "Ledger should have domain entries");

    println!("Found {} domain entries in ledger", entries.len());

    // Print first entry for debugging
    if let Some(first) = entries.first() {
        println!("First entry: {:?}", first);
    }
}

#[test]
fn test_ledger_has_required_domains() {
    let workspace_root = get_workspace_root();
    let ledger_path = workspace_root.join("1.docs/developer_docs/CODE_VS_CANON_STATUS_LEDGER.md");

    let entries = parse_ledger(&ledger_path).expect("Failed to parse ledger");

    // Check for expected domains from requirements
    let expected_domains = vec![
        "Graphics",
        "Materials",
        "Asset",
        "Audio",
        "Netcode",
        "Editor",
    ];

    for expected in expected_domains {
        let found = entries.iter().any(|e| e.domain_name.contains(expected));
        assert!(
            found,
            "Ledger should contain domain related to '{}'",
            expected
        );
    }

    println!("✓ Ledger contains all expected domain categories");
}

#[test]
fn test_all_entries_have_percentages() {
    let workspace_root = get_workspace_root();
    let ledger_path = workspace_root.join("1.docs/developer_docs/CODE_VS_CANON_STATUS_LEDGER.md");

    let entries = parse_ledger(&ledger_path).expect("Failed to parse ledger");

    for entry in &entries {
        assert!(
            entry.docs_readiness.is_some(),
            "Domain '{}' should have docs readiness percentage",
            entry.domain_name
        );
        assert!(
            entry.code_readiness.is_some(),
            "Domain '{}' should have code readiness percentage",
            entry.domain_name
        );
    }

    println!(
        "✓ All {} domain entries have percentage values",
        entries.len()
    );
}

#[test]
fn test_all_entries_have_next_steps() {
    let workspace_root = get_workspace_root();
    let ledger_path = workspace_root.join("1.docs/developer_docs/CODE_VS_CANON_STATUS_LEDGER.md");

    let entries = parse_ledger(&ledger_path).expect("Failed to parse ledger");

    for entry in &entries {
        assert!(
            !entry.next_steps.is_empty(),
            "Domain '{}' should have next implementation steps",
            entry.domain_name
        );
    }

    println!(
        "✓ All {} domain entries have next implementation steps",
        entries.len()
    );
}
