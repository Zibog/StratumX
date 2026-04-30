// Property 17: Ownership Validation Detects Violations
// Validates: Requirements 9.4, 12.4
//
// This property test verifies that the OwnershipCompletenessValidator correctly detects:
// 1. State fields with no owner in the inventory
// 2. State fields with multiple conflicting owners
// 3. Domain truth incorrectly placed in UI layer

use proptest::prelude::*;
use repo_hygiene::{
    CodebaseState, OwnerInventory, OwnerInventoryEntry, OwnershipCompletenessValidator,
    StateClassification, StateField, ViolationType,
};

// Strategy to generate valid state fields
fn state_field_strategy() -> impl Strategy<Value = StateField> {
    (
        "[a-z_]{3,15}",
        prop::bool::ANY,
        "[A-Z][a-zA-Z]{3,15}",
        "[A-Z][a-zA-Z]{3,15}",
    )
        .prop_map(|(name, is_ui, owner_type, field_type)| {
            let file_path = if is_ui {
                format!("6.apps/editor/src/{}.rs", name)
            } else {
                format!("5.editor/editor-state-containers/src/{}.rs", name)
            };
            StateField {
                name,
                file_path,
                owner_type,
                field_type,
            }
        })
}

// Strategy to generate inventory entries
fn inventory_entry_strategy() -> impl Strategy<Value = OwnerInventoryEntry> {
    (
        "[a-z_]{3,15}",
        prop::bool::ANY,
        "[A-Z][a-zA-Z]{3,15}",
        "[A-Z][a-zA-Z]{3,15}",
        prop::sample::select(vec![
            StateClassification::Persistable,
            StateClassification::Transient,
            StateClassification::Derived,
        ]),
    )
        .prop_map(
            |(name, is_ui, current_owner, target_owner, classification)| {
                let current_path = if is_ui {
                    format!("6.apps/editor/src/{}.rs", name)
                } else {
                    format!("5.editor/editor-state-containers/src/{}.rs", name)
                };
                OwnerInventoryEntry {
                    entity_name: name,
                    current_path,
                    current_owner,
                    target_owner,
                    classification,
                    can_mutate: vec!["Service".to_string()],
                    can_read: vec!["QueryLayer".to_string()],
                    publishes_changes: Some("EventBus".to_string()),
                    rebuilds_cache: None,
                }
            },
        )
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]

    /// Property: Validator detects state fields with no owner
    ///
    /// Given: A codebase with state fields and an inventory
    /// When: A state field exists in codebase but not in inventory
    /// Then: Validator detects NoOwner violation
    #[test]
    fn property_validator_detects_no_owner(
        inventory_entries in prop::collection::vec(inventory_entry_strategy(), 1..5),
        orphan_field in state_field_strategy(),
    ) {
        let inventory = OwnerInventory {
            version: "1.0".to_string(),
            entries: inventory_entries.clone(),
        };

        // Create codebase state with fields from inventory plus an orphan
        let mut codebase_state = CodebaseState {
            state_fields: inventory_entries
                .iter()
                .map(|entry| StateField {
                    name: entry.entity_name.clone(),
                    file_path: entry.current_path.clone(),
                    owner_type: entry.current_owner.clone(),
                    field_type: "String".to_string(),
                })
                .collect(),
        };

        // Ensure orphan field is not in inventory
        if inventory.find_entry(&orphan_field.name).is_none() {
            codebase_state.state_fields.push(orphan_field.clone());

            let validator = OwnershipCompletenessValidator::new(inventory);
            let result = validator.validate(&codebase_state);

            // Should detect violation
            prop_assert!(result.is_err());
            let violations = result.unwrap_err();
            let has_no_owner_violation = violations.iter().any(|v| {
                v.violation_type == ViolationType::NoOwner
                    && v.entity_name == orphan_field.name
            });
            prop_assert!(has_no_owner_violation);
        }
    }

    /// Property: Validator detects multiple owners for same field
    ///
    /// Given: An inventory with duplicate entries for the same field
    /// When: The entries have different target owners
    /// Then: Validator detects MultipleOwners violation
    #[test]
    fn property_validator_detects_multiple_owners(
        base_entry in inventory_entry_strategy(),
        different_owner in "[A-Z][a-zA-Z]{3,15}",
    ) {
        // Create inventory with duplicate entry having different owner
        let mut entries = vec![base_entry.clone()];

        // Only add duplicate if owner is actually different
        if different_owner != base_entry.target_owner {
            let duplicate_entry = OwnerInventoryEntry {
                entity_name: base_entry.entity_name.clone(),
                current_path: format!("different/path/{}.rs", base_entry.entity_name),
                current_owner: "DifferentOwner".to_string(),
                target_owner: different_owner,
                classification: base_entry.classification,
                can_mutate: vec!["Service".to_string()],
                can_read: vec!["QueryLayer".to_string()],
                publishes_changes: None,
                rebuilds_cache: None,
            };
            entries.push(duplicate_entry);

            let inventory = OwnerInventory {
                version: "1.0".to_string(),
                entries,
            };

            let codebase_state = CodebaseState {
                state_fields: vec![StateField {
                    name: base_entry.entity_name.clone(),
                    file_path: base_entry.current_path.clone(),
                    owner_type: base_entry.current_owner.clone(),
                    field_type: "String".to_string(),
                }],
            };

            let validator = OwnershipCompletenessValidator::new(inventory);
            let result = validator.validate(&codebase_state);

            // Should detect violation
            prop_assert!(result.is_err());
            let violations = result.unwrap_err();
            let has_multiple_owners_violation = violations.iter().any(|v| {
                v.violation_type == ViolationType::MultipleOwners
                    && v.entity_name == base_entry.entity_name
            });
            prop_assert!(has_multiple_owners_violation);
        }
    }

    /// Property: Validator detects domain truth in UI layer
    ///
    /// Given: A state field with domain truth pattern in UI layer
    /// When: Field is classified as persistable or derived
    /// Then: Validator detects DomainTruthInUI violation
    #[test]
    fn property_validator_detects_domain_truth_in_ui(
        domain_pattern in prop::sample::select(vec![
            "registry",
            "manifest",
            "bindings",
            "coverage",
        ]),
    ) {
        let field_name = format!("material_{}", domain_pattern);
        let ui_path = "6.apps/editor/src/material_panel.rs".to_string();

        let entry = OwnerInventoryEntry {
            entity_name: field_name.clone(),
            current_path: ui_path.clone(),
            current_owner: "MaterialPanel".to_string(),
            target_owner: "MaterialAuthoringService".to_string(),
            classification: StateClassification::Persistable,
            can_mutate: vec!["MaterialAuthoringService".to_string()],
            can_read: vec!["QueryLayer".to_string()],
            publishes_changes: Some("EventBus".to_string()),
            rebuilds_cache: Some("MaterialRegistryCache".to_string()),
        };

        let inventory = OwnerInventory {
            version: "1.0".to_string(),
            entries: vec![entry],
        };

        let codebase_state = CodebaseState {
            state_fields: vec![StateField {
                name: field_name.clone(),
                file_path: ui_path,
                owner_type: "MaterialPanel".to_string(),
                field_type: "MaterialRegistry".to_string(),
            }],
        };

        let validator = OwnershipCompletenessValidator::new(inventory);
        let result = validator.validate(&codebase_state);

        // Should detect violation
        prop_assert!(result.is_err());
        let violations = result.unwrap_err();
        let has_domain_truth_in_ui_violation = violations.iter().any(|v| {
            v.violation_type == ViolationType::DomainTruthInUI
                && v.entity_name == field_name
        });
        prop_assert!(has_domain_truth_in_ui_violation);
    }

    /// Property: Validator passes when all ownership rules are satisfied
    ///
    /// Given: A codebase and inventory with proper ownership
    /// When: All fields have single owners and domain truth is in services
    /// Then: Validator returns Ok
    #[test]
    fn property_validator_passes_with_valid_ownership(
        entries in prop::collection::vec(inventory_entry_strategy(), 1..10),
    ) {
        // Filter to ensure no duplicates and no domain truth in UI
        let mut seen_names = std::collections::HashSet::new();
        let valid_entries: Vec<_> = entries
            .into_iter()
            .filter(|entry| {
                // No duplicates
                if seen_names.contains(&entry.entity_name) {
                    return false;
                }
                seen_names.insert(entry.entity_name.clone());

                // No domain truth patterns in UI paths
                let is_ui = entry.current_path.contains("6.apps/editor");
                let has_domain_pattern = entry.entity_name.contains("registry")
                    || entry.entity_name.contains("manifest")
                    || entry.entity_name.contains("bindings")
                    || entry.entity_name.contains("coverage");
                let is_persistable_or_derived = matches!(
                    entry.classification,
                    StateClassification::Persistable | StateClassification::Derived
                );

                !(is_ui && has_domain_pattern && is_persistable_or_derived)
            })
            .collect();

        if !valid_entries.is_empty() {
            let inventory = OwnerInventory {
                version: "1.0".to_string(),
                entries: valid_entries.clone(),
            };

            let codebase_state = CodebaseState {
                state_fields: valid_entries
                    .iter()
                    .map(|entry| StateField {
                        name: entry.entity_name.clone(),
                        file_path: entry.current_path.clone(),
                        owner_type: entry.current_owner.clone(),
                        field_type: "String".to_string(),
                    })
                    .collect(),
            };

            let validator = OwnershipCompletenessValidator::new(inventory);
            let result = validator.validate(&codebase_state);

            // Should pass validation
            prop_assert!(result.is_ok());
        }
    }
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn test_no_owner_violation_detected() {
        let inventory = OwnerInventory {
            version: "1.0".to_string(),
            entries: vec![],
        };

        let codebase_state = CodebaseState {
            state_fields: vec![StateField {
                name: "orphan_field".to_string(),
                file_path: "5.editor/some_file.rs".to_string(),
                owner_type: "SomeOwner".to_string(),
                field_type: "String".to_string(),
            }],
        };

        let validator = OwnershipCompletenessValidator::new(inventory);
        let result = validator.validate(&codebase_state);

        assert!(result.is_err());
        let violations = result.unwrap_err();
        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].violation_type, ViolationType::NoOwner);
    }

    #[test]
    fn test_multiple_owners_violation_detected() {
        let inventory = OwnerInventory {
            version: "1.0".to_string(),
            entries: vec![
                OwnerInventoryEntry {
                    entity_name: "shared_field".to_string(),
                    current_path: "path1.rs".to_string(),
                    current_owner: "Owner1".to_string(),
                    target_owner: "Owner1".to_string(),
                    classification: StateClassification::Persistable,
                    can_mutate: vec![],
                    can_read: vec![],
                    publishes_changes: None,
                    rebuilds_cache: None,
                },
                OwnerInventoryEntry {
                    entity_name: "shared_field".to_string(),
                    current_path: "path2.rs".to_string(),
                    current_owner: "Owner2".to_string(),
                    target_owner: "Owner2".to_string(),
                    classification: StateClassification::Persistable,
                    can_mutate: vec![],
                    can_read: vec![],
                    publishes_changes: None,
                    rebuilds_cache: None,
                },
            ],
        };

        let codebase_state = CodebaseState {
            state_fields: vec![],
        };

        let validator = OwnershipCompletenessValidator::new(inventory);
        let result = validator.validate(&codebase_state);

        assert!(result.is_err());
        let violations = result.unwrap_err();
        assert!(violations
            .iter()
            .any(|v| v.violation_type == ViolationType::MultipleOwners));
    }

    #[test]
    fn test_domain_truth_in_ui_violation_detected() {
        let inventory = OwnerInventory {
            version: "1.0".to_string(),
            entries: vec![OwnerInventoryEntry {
                entity_name: "material_registry".to_string(),
                current_path: "6.apps/editor/src/material_panel.rs".to_string(),
                current_owner: "MaterialPanel".to_string(),
                target_owner: "MaterialAuthoringService".to_string(),
                classification: StateClassification::Persistable,
                can_mutate: vec![],
                can_read: vec![],
                publishes_changes: None,
                rebuilds_cache: None,
            }],
        };

        let codebase_state = CodebaseState {
            state_fields: vec![StateField {
                name: "material_registry".to_string(),
                file_path: "6.apps/editor/src/material_panel.rs".to_string(),
                owner_type: "MaterialPanel".to_string(),
                field_type: "MaterialRegistry".to_string(),
            }],
        };

        let validator = OwnershipCompletenessValidator::new(inventory);
        let result = validator.validate(&codebase_state);

        assert!(result.is_err());
        let violations = result.unwrap_err();
        assert!(violations
            .iter()
            .any(|v| v.violation_type == ViolationType::DomainTruthInUI));
    }
}
