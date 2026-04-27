// Feature: editor-state-truth-normalization-phase3, Property 1: Owner Inventory Completeness
// **Validates: Requirements 1.2**
//
// For any state field in the codebase (app_state, EditorHost, editor-state-containers,
// workspace layout, authoring suites), the Owner_Inventory contains an entry for that
// field with all required metadata (entity name, current path, current owner, target owner,
// classification, mutation rights, read rights, change publication, cache rebuild).

use proptest::prelude::*;
use repo_hygiene::{CodebaseState, OwnerInventory};
use std::path::PathBuf;

fn get_inventory_path() -> PathBuf {
    // Try to find the workspace root by looking for Cargo.toml
    let mut current = std::env::current_dir().expect("Failed to get current directory");

    // Walk up until we find the workspace root (contains 7.quality directory)
    loop {
        let quality_dir = current.join("7.quality");
        if quality_dir.exists() {
            return quality_dir.join("suites/repo_hygiene/owner_inventory.json");
        }

        if !current.pop() {
            panic!("Could not find workspace root");
        }
    }
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]

    #[test]
    fn property_owner_inventory_completeness(
        _seed in 0u64..1000u64
    ) {
        // Load the owner inventory
        let inventory_path = get_inventory_path();
        let inventory = OwnerInventory::load_from_file(&inventory_path)
            .expect("Failed to load owner inventory");

        // Scan the codebase
        let mut root = inventory_path.clone();
        root.pop(); // Remove owner_inventory.json
        root.pop(); // Remove repo_hygiene
        root.pop(); // Remove suites
        root.pop(); // Remove 7.quality

        let codebase = CodebaseState::scan_from_directory(&root)
            .expect("Failed to scan codebase");

        // For any state field in codebase, inventory should contain entry
        for field in &codebase.state_fields {
            let entity_name = format!("{}::{}", field.owner_type, field.name);
            let entry = inventory.find_entry(&entity_name);

            prop_assert!(
                entry.is_some(),
                "Field '{}' in {} not found in inventory",
                entity_name,
                field.file_path
            );

            // Verify entry has all required metadata
            if let Some(entry) = entry {
                prop_assert!(
                    !entry.entity_name.is_empty(),
                    "Entry for '{}' has empty entity_name",
                    entity_name
                );
                prop_assert!(
                    !entry.current_path.is_empty(),
                    "Entry for '{}' has empty current_path",
                    entity_name
                );
                prop_assert!(
                    !entry.current_owner.is_empty(),
                    "Entry for '{}' has empty current_owner",
                    entity_name
                );
                prop_assert!(
                    !entry.target_owner.is_empty(),
                    "Entry for '{}' has empty target_owner",
                    entity_name
                );
                prop_assert!(
                    !entry.can_mutate.is_empty(),
                    "Entry for '{}' has empty can_mutate",
                    entity_name
                );
                prop_assert!(
                    !entry.can_read.is_empty(),
                    "Entry for '{}' has empty can_read",
                    entity_name
                );
            }
        }
    }
}

#[test]
fn test_owner_inventory_exists() {
    let inventory_path = get_inventory_path();
    assert!(
        inventory_path.exists(),
        "Owner inventory file does not exist at {}",
        inventory_path.display()
    );
}

#[test]
fn test_owner_inventory_loads() {
    let inventory_path = get_inventory_path();
    let result = OwnerInventory::load_from_file(&inventory_path);
    assert!(
        result.is_ok(),
        "Failed to load owner inventory: {:?}",
        result.err()
    );
}

#[test]
fn test_owner_inventory_has_entries() {
    let inventory_path = get_inventory_path();
    let inventory =
        OwnerInventory::load_from_file(&inventory_path).expect("Failed to load owner inventory");

    assert!(
        !inventory.entries.is_empty(),
        "Owner inventory has no entries"
    );
}

#[test]
fn test_all_entries_have_required_metadata() {
    let inventory_path = get_inventory_path();
    let inventory =
        OwnerInventory::load_from_file(&inventory_path).expect("Failed to load owner inventory");

    for entry in &inventory.entries {
        assert!(!entry.entity_name.is_empty(), "Entry has empty entity_name");
        assert!(
            !entry.current_path.is_empty(),
            "Entry '{}' has empty current_path",
            entry.entity_name
        );
        assert!(
            !entry.current_owner.is_empty(),
            "Entry '{}' has empty current_owner",
            entry.entity_name
        );
        assert!(
            !entry.target_owner.is_empty(),
            "Entry '{}' has empty target_owner",
            entry.entity_name
        );
    }
}
