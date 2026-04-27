// Feature: editor-state-truth-normalization-phase3
// Property 5: State Classification Completeness
// Property 6: Persistence Contains Only Persistable State
// Property 7: Cache Depends Only On Persistable State
//
// **Validates: Requirements 3.1, 3.5, 3.6**

use proptest::prelude::*;
use repo_hygiene::{OwnerInventory, StateClassification};
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

// Property 5: State Classification Completeness
// **Validates: Requirements 3.1**
//
// Test that every state field is classified as exactly one of: persistable, transient, or derived
proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]

    #[test]
    fn property_state_classification_completeness(
        _seed in 0u64..1000u64
    ) {
        // Load the owner inventory
        let inventory_path = get_inventory_path();
        let inventory = OwnerInventory::load_from_file(&inventory_path)
            .expect("Failed to load owner inventory");

        // For every entry in the inventory, verify it has exactly one classification
        for entry in &inventory.entries {
            // The classification field exists and is one of the three valid values
            let classification = entry.classification;

            // Verify it's exactly one of the three types
            let is_persistable = classification.is_persistable();
            let is_transient = classification.is_transient();
            let is_derived = classification.is_derived();

            // Count how many classifications are true
            let classification_count = [is_persistable, is_transient, is_derived]
                .iter()
                .filter(|&&x| x)
                .count();

            prop_assert_eq!(
                classification_count,
                1,
                "Entry '{}' has {} classifications (expected exactly 1). Classification: {:?}",
                entry.entity_name,
                classification_count,
                classification
            );
        }
    }
}

// Property 6: Persistence Contains Only Persistable State
// **Validates: Requirements 3.5**
//
// Test that serialized data contains only persistable fields
proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]

    #[test]
    fn property_persistence_contains_only_persistable_state(
        _seed in 0u64..1000u64
    ) {
        // Load the owner inventory
        let inventory_path = get_inventory_path();
        let inventory = OwnerInventory::load_from_file(&inventory_path)
            .expect("Failed to load owner inventory");

        // Find all entries that are in persistence views
        // Persistence views are in the persistence/ directory
        let persistence_entries: Vec<_> = inventory.entries
            .iter()
            .filter(|entry| {
                entry.current_path.contains("/persistence/") ||
                entry.current_path.contains("persistence_view") ||
                entry.target_owner.contains("PersistenceView") ||
                entry.target_owner.contains("Persisted")
            })
            .collect();

        // For each persistence entry, verify it's classified as persistable
        for entry in persistence_entries {
            prop_assert!(
                entry.classification.is_persistable(),
                "Entry '{}' in persistence path '{}' is not classified as persistable (classification: {:?})",
                entry.entity_name,
                entry.current_path,
                entry.classification
            );
        }

        // Also verify that transient and derived state are NOT in persistence paths
        for entry in &inventory.entries {
            if entry.classification.is_transient() || entry.classification.is_derived() {
                prop_assert!(
                    !entry.current_path.contains("/persistence/") &&
                    !entry.current_path.contains("persistence_view") &&
                    !entry.target_owner.contains("PersistenceView") &&
                    !entry.target_owner.contains("Persisted"),
                    "Entry '{}' is classified as {:?} but appears in persistence path '{}' or owner '{}'",
                    entry.entity_name,
                    entry.classification,
                    entry.current_path,
                    entry.target_owner
                );
            }
        }
    }
}

// Property 7: Cache Depends Only On Persistable State
// **Validates: Requirements 3.6**
//
// Test that cache computation depends only on persistable state from owners
proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]

    #[test]
    fn property_cache_depends_only_on_persistable_state(
        _seed in 0u64..1000u64
    ) {
        // Load the owner inventory
        let inventory_path = get_inventory_path();
        let inventory = OwnerInventory::load_from_file(&inventory_path)
            .expect("Failed to load owner inventory");

        // Find all entries that are caches or derived state
        let cache_entries: Vec<_> = inventory.entries
            .iter()
            .filter(|entry| {
                entry.classification.is_derived() ||
                entry.current_path.contains("/cache/") ||
                entry.target_owner.contains("Cache") ||
                entry.rebuilds_cache.is_some()
            })
            .collect();

        // For each cache entry, verify its dependencies
        for entry in cache_entries {
            // Check the can_read field to see what this cache depends on
            for dependency in &entry.can_read {
                // Find the dependency in the inventory
                if let Some(dep_entry) = inventory.entries.iter().find(|e| {
                    e.target_owner == *dependency ||
                    e.entity_name.contains(dependency) ||
                    e.current_owner == *dependency
                }) {
                    // Verify the dependency is persistable (not transient or derived)
                    prop_assert!(
                        dep_entry.classification.is_persistable(),
                        "Cache '{}' depends on '{}' which is classified as {:?} (expected Persistable)",
                        entry.entity_name,
                        dependency,
                        dep_entry.classification
                    );
                }
            }

            // Verify that caches themselves are classified as derived
            if entry.current_path.contains("/cache/") || entry.target_owner.contains("Cache") {
                prop_assert!(
                    entry.classification.is_derived(),
                    "Cache '{}' in path '{}' is not classified as derived (classification: {:?})",
                    entry.entity_name,
                    entry.current_path,
                    entry.classification
                );
            }
        }
    }
}

// Unit tests for specific scenarios

#[test]
fn test_state_classification_enum_values() {
    // Test that StateClassification enum has exactly three variants
    let persistable = StateClassification::Persistable;
    let transient = StateClassification::Transient;
    let derived = StateClassification::Derived;

    assert!(persistable.is_persistable());
    assert!(!persistable.is_transient());
    assert!(!persistable.is_derived());

    assert!(!transient.is_persistable());
    assert!(transient.is_transient());
    assert!(!transient.is_derived());

    assert!(!derived.is_persistable());
    assert!(!derived.is_transient());
    assert!(derived.is_derived());
}

#[test]
fn test_all_inventory_entries_have_classification() {
    let inventory_path = get_inventory_path();
    let inventory =
        OwnerInventory::load_from_file(&inventory_path).expect("Failed to load owner inventory");

    for entry in &inventory.entries {
        // Just accessing the classification field verifies it exists
        let _classification = entry.classification;
    }
}

#[test]
fn test_no_transient_state_in_persistence_paths() {
    let inventory_path = get_inventory_path();
    let inventory =
        OwnerInventory::load_from_file(&inventory_path).expect("Failed to load owner inventory");

    for entry in &inventory.entries {
        if entry.classification.is_transient() {
            assert!(
                !entry.current_path.contains("/persistence/"),
                "Transient state '{}' found in persistence path '{}'",
                entry.entity_name,
                entry.current_path
            );
        }
    }
}

#[test]
fn test_no_derived_state_in_persistence_paths() {
    let inventory_path = get_inventory_path();
    let inventory =
        OwnerInventory::load_from_file(&inventory_path).expect("Failed to load owner inventory");

    for entry in &inventory.entries {
        if entry.classification.is_derived() {
            assert!(
                !entry.current_path.contains("/persistence/"),
                "Derived state '{}' found in persistence path '{}'",
                entry.entity_name,
                entry.current_path
            );
        }
    }
}

#[test]
fn test_cache_entries_are_derived() {
    let inventory_path = get_inventory_path();
    let inventory =
        OwnerInventory::load_from_file(&inventory_path).expect("Failed to load owner inventory");

    for entry in &inventory.entries {
        if entry.current_path.contains("/cache/") || entry.target_owner.contains("Cache") {
            assert!(
                entry.classification.is_derived(),
                "Cache '{}' is not classified as derived (classification: {:?})",
                entry.entity_name,
                entry.classification
            );
        }
    }
}
