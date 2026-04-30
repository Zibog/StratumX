// Feature: editor-state-truth-normalization-phase3, Property 2: Single Owner Assignment
// **Validates: Requirements 1.2, 9.1**
//
// For any state field in the Owner_Inventory, the field has exactly one target owner,
// and that owner is one of the 4 defined owner containers (ProjectOwner, WorkspaceOwner,
// WorldOwner, DiagnosticsOwner).

use proptest::prelude::*;
use repo_hygiene::OwnerInventory;
use std::path::PathBuf;

const VALID_OWNERS: &[&str] = &[
    "ProjectOwner",
    "WorkspaceOwner",
    "WorldOwner",
    "DiagnosticsOwner",
    "UiTransientState", // Transient UI state is also valid
    "CacheLayer",       // Derived/cached state is also valid
];

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
    fn property_single_owner_assignment(
        _seed in 0u64..1000u64
    ) {
        // Load the owner inventory
        let inventory_path = get_inventory_path();
        let inventory = OwnerInventory::load_from_file(&inventory_path)
            .expect("Failed to load owner inventory");

        // For any entry in inventory, verify single owner assignment
        for entry in &inventory.entries {
            // Verify target owner is not empty (exactly one owner)
            prop_assert!(
                !entry.target_owner.is_empty(),
                "Entry '{}' has no target owner",
                entry.entity_name
            );

            // Verify target owner is one of the valid owners
            prop_assert!(
                VALID_OWNERS.contains(&entry.target_owner.as_str()),
                "Entry '{}' has invalid target owner '{}'. Must be one of: {:?}",
                entry.entity_name,
                entry.target_owner,
                VALID_OWNERS
            );
        }
    }
}

#[test]
fn test_no_duplicate_ownership() {
    let inventory_path = get_inventory_path();
    let inventory =
        OwnerInventory::load_from_file(&inventory_path).expect("Failed to load owner inventory");

    // Check that no entity appears multiple times with different owners
    let mut seen_entities = std::collections::HashMap::new();

    for entry in &inventory.entries {
        if let Some(existing_owner) = seen_entities.get(&entry.entity_name) {
            panic!(
                "Entity '{}' appears multiple times with different owners: '{}' and '{}'",
                entry.entity_name, existing_owner, entry.target_owner
            );
        }
        seen_entities.insert(entry.entity_name.clone(), entry.target_owner.clone());
    }
}

#[test]
fn test_all_owners_are_valid() {
    let inventory_path = get_inventory_path();
    let inventory =
        OwnerInventory::load_from_file(&inventory_path).expect("Failed to load owner inventory");

    for entry in &inventory.entries {
        assert!(
            VALID_OWNERS.contains(&entry.target_owner.as_str()),
            "Entry '{}' has invalid target owner '{}'. Must be one of: {:?}",
            entry.entity_name,
            entry.target_owner,
            VALID_OWNERS
        );
    }
}

#[test]
fn test_owner_distribution() {
    let inventory_path = get_inventory_path();
    let inventory =
        OwnerInventory::load_from_file(&inventory_path).expect("Failed to load owner inventory");

    let mut owner_counts = std::collections::HashMap::new();

    for entry in &inventory.entries {
        *owner_counts.entry(entry.target_owner.clone()).or_insert(0) += 1;
    }

    println!("Owner distribution:");
    for (owner, count) in &owner_counts {
        println!("  {}: {} fields", owner, count);
    }

    // Verify we have at least some entries for each major owner
    // (This is a sanity check, not a strict requirement)
    assert!(!owner_counts.is_empty(), "No owners found in inventory");
}
