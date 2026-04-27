use crate::{CodebaseState, OwnerInventory, StateClassification};
use std::collections::{HashMap, HashSet};

/// Validator for ownership completeness and correctness
pub struct OwnershipCompletenessValidator {
    inventory: OwnerInventory,
}

impl OwnershipCompletenessValidator {
    pub fn new(inventory: OwnerInventory) -> Self {
        Self { inventory }
    }

    /// Validate ownership completeness and correctness
    pub fn validate(&self, codebase_state: &CodebaseState) -> Result<(), Vec<OwnershipViolation>> {
        let mut violations = Vec::new();

        // Check that every field has an owner
        if let Err(mut v) = self.check_every_field_has_owner(codebase_state) {
            violations.append(&mut v);
        }

        // Check for duplicate ownership
        if let Err(mut v) = self.check_no_duplicate_ownership() {
            violations.append(&mut v);
        }

        // Check that domain truth lives in services, not UI
        if let Err(mut v) = self.check_domain_truth_in_services(codebase_state) {
            violations.append(&mut v);
        }

        if violations.is_empty() {
            Ok(())
        } else {
            Err(violations)
        }
    }

    /// Check that every state field in the codebase has an owner in the inventory
    fn check_every_field_has_owner(
        &self,
        codebase_state: &CodebaseState,
    ) -> Result<(), Vec<OwnershipViolation>> {
        let mut violations = Vec::new();

        for field in &codebase_state.state_fields {
            if self.inventory.find_entry(&field.name).is_none() {
                violations.push(OwnershipViolation {
                    entity_name: field.name.clone(),
                    file_path: field.file_path.clone(),
                    violation_type: ViolationType::NoOwner,
                    message: format!(
                        "State field '{}' in {} has no owner in Owner_Inventory",
                        field.name, field.file_path
                    ),
                });
            }
        }

        if violations.is_empty() {
            Ok(())
        } else {
            Err(violations)
        }
    }

    /// Check that no state field has multiple owners
    fn check_no_duplicate_ownership(&self) -> Result<(), Vec<OwnershipViolation>> {
        let mut violations = Vec::new();
        let mut entity_owners: HashMap<String, Vec<String>> = HashMap::new();

        // Group entries by entity name to detect duplicates
        for entry in &self.inventory.entries {
            entity_owners
                .entry(entry.entity_name.clone())
                .or_insert_with(Vec::new)
                .push(entry.target_owner.clone());
        }

        // Check for entities with multiple owners
        for (entity_name, owners) in entity_owners {
            if owners.len() > 1 {
                let unique_owners: HashSet<_> = owners.iter().collect();
                if unique_owners.len() > 1 {
                    violations.push(OwnershipViolation {
                        entity_name: entity_name.clone(),
                        file_path: String::new(),
                        violation_type: ViolationType::MultipleOwners,
                        message: format!(
                            "State field '{}' has multiple owners: {}",
                            entity_name,
                            owners.join(", ")
                        ),
                    });
                }
            }
        }

        if violations.is_empty() {
            Ok(())
        } else {
            Err(violations)
        }
    }

    /// Check that domain truth lives in services (5.editor/l9.*), not UI (6.apps/editor)
    fn check_domain_truth_in_services(
        &self,
        codebase_state: &CodebaseState,
    ) -> Result<(), Vec<OwnershipViolation>> {
        let mut violations = Vec::new();

        // Define domain truth patterns that should NOT be in UI
        let domain_truth_patterns = vec![
            "registry",
            "manifest",
            "bindings",
            "coverage",
            "chunk_dirtiness",
            "layer_bindings",
            "sky_profile_binding",
            "weather_regime_binding",
            "cloud_profile_binding",
            "source_registry",
            "sound_profile_bindings",
        ];

        for field in &codebase_state.state_fields {
            // Check if field is in UI layer (6.apps/editor)
            let is_in_ui = field.file_path.contains("6.apps/editor");

            // Check if field name suggests domain truth
            let is_domain_truth = domain_truth_patterns
                .iter()
                .any(|pattern| field.name.contains(pattern));

            // Check inventory classification
            let is_persistable_or_derived = self
                .inventory
                .find_entry(&field.name)
                .map(|entry| {
                    matches!(
                        entry.classification,
                        StateClassification::Persistable | StateClassification::Derived
                    )
                })
                .unwrap_or(false);

            if is_in_ui && is_domain_truth && is_persistable_or_derived {
                violations.push(OwnershipViolation {
                    entity_name: field.name.clone(),
                    file_path: field.file_path.clone(),
                    violation_type: ViolationType::DomainTruthInUI,
                    message: format!(
                        "Domain truth field '{}' found in UI layer at {}. Domain truth should live in services (5.editor/l9.*)",
                        field.name, field.file_path
                    ),
                });
            }
        }

        if violations.is_empty() {
            Ok(())
        } else {
            Err(violations)
        }
    }
}

/// Represents a violation of ownership rules
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OwnershipViolation {
    pub entity_name: String,
    pub file_path: String,
    pub violation_type: ViolationType,
    pub message: String,
}

/// Types of ownership violations
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ViolationType {
    /// State field has no owner in the inventory
    NoOwner,
    /// State field has multiple conflicting owners
    MultipleOwners,
    /// Domain truth found in UI layer instead of services
    DomainTruthInUI,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{OwnerInventoryEntry, StateField};

    fn create_test_inventory() -> OwnerInventory {
        OwnerInventory {
            version: "1.0".to_string(),
            entries: vec![
                OwnerInventoryEntry {
                    entity_name: "project_identity".to_string(),
                    current_path: "5.editor/editor-state-containers/src/project_owner.rs"
                        .to_string(),
                    current_owner: "ProjectOwner".to_string(),
                    target_owner: "ProjectOwner".to_string(),
                    classification: StateClassification::Persistable,
                    can_mutate: vec!["ProjectService".to_string()],
                    can_read: vec!["QueryLayer".to_string()],
                    publishes_changes: Some("EventBus".to_string()),
                    rebuilds_cache: None,
                },
                OwnerInventoryEntry {
                    entity_name: "hover_state".to_string(),
                    current_path: "6.apps/editor/src/ui_transient_state.rs".to_string(),
                    current_owner: "UiTransientState".to_string(),
                    target_owner: "UiTransientState".to_string(),
                    classification: StateClassification::Transient,
                    can_mutate: vec!["UI".to_string()],
                    can_read: vec!["UI".to_string()],
                    publishes_changes: None,
                    rebuilds_cache: None,
                },
            ],
        }
    }

    fn create_test_codebase_state() -> CodebaseState {
        CodebaseState {
            state_fields: vec![
                StateField {
                    name: "project_identity".to_string(),
                    file_path: "5.editor/editor-state-containers/src/project_owner.rs".to_string(),
                    owner_type: "ProjectOwner".to_string(),
                    field_type: "ProjectIdentity".to_string(),
                },
                StateField {
                    name: "hover_state".to_string(),
                    file_path: "6.apps/editor/src/ui_transient_state.rs".to_string(),
                    owner_type: "UiTransientState".to_string(),
                    field_type: "Option<HoverState>".to_string(),
                },
            ],
        }
    }

    #[test]
    fn test_validator_detects_no_owner() {
        let inventory = create_test_inventory();
        let mut codebase_state = create_test_codebase_state();

        // Add a field that's not in the inventory
        codebase_state.state_fields.push(StateField {
            name: "orphan_field".to_string(),
            file_path: "5.editor/some_file.rs".to_string(),
            owner_type: "SomeOwner".to_string(),
            field_type: "String".to_string(),
        });

        let validator = OwnershipCompletenessValidator::new(inventory);
        let result = validator.validate(&codebase_state);

        assert!(result.is_err());
        let violations = result.unwrap_err();
        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].violation_type, ViolationType::NoOwner);
        assert_eq!(violations[0].entity_name, "orphan_field");
    }

    #[test]
    fn test_validator_detects_multiple_owners() {
        let mut inventory = create_test_inventory();

        // Add duplicate entry with different owner
        inventory.entries.push(OwnerInventoryEntry {
            entity_name: "project_identity".to_string(),
            current_path: "6.apps/editor/src/app_state.rs".to_string(),
            current_owner: "AppState".to_string(),
            target_owner: "AppState".to_string(),
            classification: StateClassification::Persistable,
            can_mutate: vec!["AppState".to_string()],
            can_read: vec!["UI".to_string()],
            publishes_changes: None,
            rebuilds_cache: None,
        });

        let codebase_state = create_test_codebase_state();
        let validator = OwnershipCompletenessValidator::new(inventory);
        let result = validator.validate(&codebase_state);

        assert!(result.is_err());
        let violations = result.unwrap_err();
        assert!(violations
            .iter()
            .any(|v| v.violation_type == ViolationType::MultipleOwners));
    }

    #[test]
    fn test_validator_detects_domain_truth_in_ui() {
        let mut inventory = create_test_inventory();
        let mut codebase_state = create_test_codebase_state();

        // Add domain truth field in UI layer
        let domain_field = StateField {
            name: "material_registry".to_string(),
            file_path: "6.apps/editor/src/material_panel.rs".to_string(),
            owner_type: "MaterialPanel".to_string(),
            field_type: "MaterialRegistry".to_string(),
        };
        codebase_state.state_fields.push(domain_field);

        inventory.entries.push(OwnerInventoryEntry {
            entity_name: "material_registry".to_string(),
            current_path: "6.apps/editor/src/material_panel.rs".to_string(),
            current_owner: "MaterialPanel".to_string(),
            target_owner: "MaterialAuthoringService".to_string(),
            classification: StateClassification::Persistable,
            can_mutate: vec!["MaterialAuthoringService".to_string()],
            can_read: vec!["QueryLayer".to_string()],
            publishes_changes: Some("EventBus".to_string()),
            rebuilds_cache: Some("MaterialRegistryCache".to_string()),
        });

        let validator = OwnershipCompletenessValidator::new(inventory);
        let result = validator.validate(&codebase_state);

        assert!(result.is_err());
        let violations = result.unwrap_err();
        assert!(violations
            .iter()
            .any(|v| v.violation_type == ViolationType::DomainTruthInUI));
    }

    #[test]
    fn test_validator_passes_with_valid_ownership() {
        let inventory = create_test_inventory();
        let codebase_state = create_test_codebase_state();

        let validator = OwnershipCompletenessValidator::new(inventory);
        let result = validator.validate(&codebase_state);

        assert!(result.is_ok());
    }
}
