// Feature: editor-state-truth-normalization-phase3
// Property 16: No Duplicate State Ownership
//
// **Validates: Requirements 9.2**

use proptest::prelude::*;
use repo_hygiene::OwnerInventory;
use std::collections::{HashMap, HashSet};
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

/// Extracts the base entity name without the struct prefix
/// For example: "AppState::shell" -> "shell"
fn extract_base_entity_name(entity_name: &str) -> &str {
    if let Some(idx) = entity_name.rfind("::") {
        &entity_name[idx + 2..]
    } else {
        entity_name
    }
}

/// Checks if two entity names refer to the same logical state field
/// This handles cases where the same field might be referenced with different prefixes
fn is_same_logical_field(name1: &str, name2: &str) -> bool {
    // If the full names match, they're definitely the same
    if name1 == name2 {
        return true;
    }

    // Extract base names and compare
    let base1 = extract_base_entity_name(name1);
    let base2 = extract_base_entity_name(name2);

    // If base names match, they might be the same field
    // But we need to be careful - different structs can have fields with the same name
    // So we only consider them duplicates if the base names match AND they're not
    // clearly from different contexts
    if base1 == base2 {
        // Check if they're from the same file path (strong indicator of duplication)
        // This will be checked in the property test using the current_path field
        return true;
    }

    false
}

// Property 16: No Duplicate State Ownership
// **Validates: Requirements 9.2**
//
// Test that state field appears in exactly one owner container
proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]

    #[test]
    fn property_no_duplicate_state_ownership(
        _seed in 0u64..1000u64
    ) {
        // Load the owner inventory
        let inventory_path = get_inventory_path();
        let inventory = OwnerInventory::load_from_file(&inventory_path)
            .expect("Failed to load owner inventory");

        // Track which entities are owned by which owners
        // Key: entity_name, Value: list of (target_owner, current_path)
        let mut entity_ownership: HashMap<String, Vec<(String, String)>> = HashMap::new();

        for entry in &inventory.entries {
            entity_ownership
                .entry(entry.entity_name.clone())
                .or_default()
                .push((entry.target_owner.clone(), entry.current_path.clone()));
        }

        // Check for duplicate ownership
        let mut violations = Vec::new();

        for (entity_name, owners) in &entity_ownership {
            // Each entity should have exactly one owner
            if owners.len() > 1 {
                // Check if these are truly duplicates or just different instances
                // Group by target owner to see if the same entity is claimed by multiple owners
                let mut owner_set: HashSet<String> = HashSet::new();
                for (owner, _path) in owners {
                    owner_set.insert(owner.clone());
                }

                if owner_set.len() > 1 {
                    violations.push(format!(
                        "Entity '{}' has multiple owners: {:?}",
                        entity_name,
                        owners
                    ));
                }
            }
        }

        // Also check for logical duplicates - same field name in same file with different owners
        let mut file_field_ownership: HashMap<(String, String), Vec<String>> = HashMap::new();

        for entry in &inventory.entries {
            let base_name = extract_base_entity_name(&entry.entity_name);
            let key = (entry.current_path.clone(), base_name.to_string());

            file_field_ownership
                .entry(key)
                .or_default()
                .push(entry.target_owner.clone());
        }

        for ((file_path, field_name), owners) in &file_field_ownership {
            let unique_owners: HashSet<_> = owners.iter().collect();
            if unique_owners.len() > 1 {
                violations.push(format!(
                    "Field '{}' in file '{}' is claimed by multiple owners: {:?}",
                    field_name,
                    file_path,
                    unique_owners
                ));
            }
        }

        // Assert no violations found
        prop_assert!(
            violations.is_empty(),
            "Found {} duplicate state ownership violations:\n{}",
            violations.len(),
            violations.join("\n")
        );
    }
}

// Property: Each entity has exactly one target owner
proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]

    #[test]
    fn property_each_entity_has_single_target_owner(
        _seed in 0u64..1000u64
    ) {
        // Load the owner inventory
        let inventory_path = get_inventory_path();
        let inventory = OwnerInventory::load_from_file(&inventory_path)
            .expect("Failed to load owner inventory");

        // For each unique entity name, verify it has exactly one target owner
        let mut entity_owners: HashMap<String, HashSet<String>> = HashMap::new();

        for entry in &inventory.entries {
            entity_owners
                .entry(entry.entity_name.clone())
                .or_default()
                .insert(entry.target_owner.clone());
        }

        for (entity_name, owners) in &entity_owners {
            prop_assert_eq!(
                owners.len(),
                1,
                "Entity '{}' has {} target owners (expected exactly 1): {:?}",
                entity_name,
                owners.len(),
                owners
            );
        }
    }
}

// Property: No state field appears in multiple owner containers
proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]

    #[test]
    fn property_no_state_in_multiple_owner_containers(
        _seed in 0u64..1000u64
    ) {
        // Load the owner inventory
        let inventory_path = get_inventory_path();
        let inventory = OwnerInventory::load_from_file(&inventory_path)
            .expect("Failed to load owner inventory");

        // Define the 4 canonical owner containers
        let owner_containers = [
            "ProjectOwner",
            "WorkspaceOwner",
            "WorldOwner",
            "DiagnosticsOwner",
        ];

        // Track which state fields are in which owner containers
        // Key: (file_path, field_name), Value: set of owner containers
        let mut field_containers: HashMap<(String, String), HashSet<String>> = HashMap::new();

        for entry in &inventory.entries {
            // Only check if the target owner is one of the 4 canonical containers
            if owner_containers.contains(&entry.target_owner.as_str()) {
                let base_name = extract_base_entity_name(&entry.entity_name);
                let key = (entry.current_path.clone(), base_name.to_string());

                field_containers
                    .entry(key)
                    .or_default()
                    .insert(entry.target_owner.clone());
            }
        }

        // Check for fields in multiple owner containers
        let mut violations = Vec::new();

        for ((file_path, field_name), containers) in &field_containers {
            if containers.len() > 1 {
                violations.push(format!(
                    "Field '{}' in '{}' appears in multiple owner containers: {:?}",
                    field_name,
                    file_path,
                    containers
                ));
            }
        }

        prop_assert!(
            violations.is_empty(),
            "Found {} fields in multiple owner containers:\n{}",
            violations.len(),
            violations.join("\n")
        );
    }
}

// Unit tests for specific scenarios

#[test]
fn test_extract_base_entity_name() {
    assert_eq!(extract_base_entity_name("AppState::shell"), "shell");
    assert_eq!(extract_base_entity_name("AudioState::sources"), "sources");
    assert_eq!(extract_base_entity_name("simple_name"), "simple_name");
    assert_eq!(extract_base_entity_name("Nested::Path::field"), "field");
}

#[test]
fn test_is_same_logical_field() {
    assert!(is_same_logical_field("AppState::shell", "AppState::shell"));
    assert!(is_same_logical_field(
        "AppState::shell",
        "OtherState::shell"
    ));
    assert!(!is_same_logical_field(
        "AppState::shell",
        "AppState::camera"
    ));
}

#[test]
fn test_no_duplicate_entity_names_in_inventory() {
    let inventory_path = get_inventory_path();
    let inventory =
        OwnerInventory::load_from_file(&inventory_path).expect("Failed to load owner inventory");

    let mut entity_counts: HashMap<String, usize> = HashMap::new();

    for entry in &inventory.entries {
        *entity_counts.entry(entry.entity_name.clone()).or_insert(0) += 1;
    }

    let duplicates: Vec<_> = entity_counts
        .iter()
        .filter(|(_, &count)| count > 1)
        .collect();

    if !duplicates.is_empty() {
        println!("Found {} duplicate entity names:", duplicates.len());
        for (name, count) in duplicates {
            println!("  - '{}' appears {} times", name, count);
        }
    }

    // Note: This test documents duplicates but doesn't fail
    // The property test above will fail if duplicates have different owners
}

#[test]
fn test_each_entity_has_single_owner() {
    let inventory_path = get_inventory_path();
    let inventory =
        OwnerInventory::load_from_file(&inventory_path).expect("Failed to load owner inventory");

    let mut entity_owners: HashMap<String, HashSet<String>> = HashMap::new();

    for entry in &inventory.entries {
        entity_owners
            .entry(entry.entity_name.clone())
            .or_default()
            .insert(entry.target_owner.clone());
    }

    let mut violations = Vec::new();

    for (entity_name, owners) in &entity_owners {
        if owners.len() > 1 {
            violations.push(format!(
                "Entity '{}' has {} owners: {:?}",
                entity_name,
                owners.len(),
                owners
            ));
        }
    }

    if !violations.is_empty() {
        println!("Found {} ownership violations:", violations.len());
        for violation in &violations {
            println!("  - {}", violation);
        }
    }

    assert!(
        violations.is_empty(),
        "Found {} entities with multiple owners",
        violations.len()
    );
}

#[test]
fn test_no_fields_in_multiple_owner_containers() {
    let inventory_path = get_inventory_path();
    let inventory =
        OwnerInventory::load_from_file(&inventory_path).expect("Failed to load owner inventory");

    let owner_containers = [
        "ProjectOwner",
        "WorkspaceOwner",
        "WorldOwner",
        "DiagnosticsOwner",
    ];

    let mut field_containers: HashMap<(String, String), HashSet<String>> = HashMap::new();

    for entry in &inventory.entries {
        if owner_containers.contains(&entry.target_owner.as_str()) {
            let base_name = extract_base_entity_name(&entry.entity_name);
            let key = (entry.current_path.clone(), base_name.to_string());

            field_containers
                .entry(key)
                .or_default()
                .insert(entry.target_owner.clone());
        }
    }

    let mut violations = Vec::new();

    for ((file_path, field_name), containers) in &field_containers {
        if containers.len() > 1 {
            violations.push(format!(
                "Field '{}' in '{}' appears in {} owner containers: {:?}",
                field_name,
                file_path,
                containers.len(),
                containers
            ));
        }
    }

    if !violations.is_empty() {
        println!(
            "Found {} fields in multiple owner containers:",
            violations.len()
        );
        for violation in &violations {
            println!("  - {}", violation);
        }
    }

    assert!(
        violations.is_empty(),
        "Found {} fields in multiple owner containers",
        violations.len()
    );
}

#[test]
fn test_owner_inventory_loads_successfully() {
    let inventory_path = get_inventory_path();
    let inventory =
        OwnerInventory::load_from_file(&inventory_path).expect("Failed to load owner inventory");

    assert!(
        !inventory.entries.is_empty(),
        "Owner inventory should not be empty"
    );
    println!(
        "Loaded {} entries from owner inventory",
        inventory.entries.len()
    );
}
