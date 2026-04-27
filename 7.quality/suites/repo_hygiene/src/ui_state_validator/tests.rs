use super::*;
use crate::{OwnerInventoryEntry, StateClassification, StateField};

fn create_test_inventory() -> OwnerInventory {
    OwnerInventory {
        version: "1.0".to_string(),
        entries: vec![
            OwnerInventoryEntry {
                entity_name: "hover_state".to_string(),
                current_path: "6.apps/editor/src/desktop_app/state/ui_transient_state.rs"
                    .to_string(),
                current_owner: "UiTransientState".to_string(),
                target_owner: "UiTransientState".to_string(),
                classification: StateClassification::Transient,
                can_mutate: vec!["UI".to_string()],
                can_read: vec!["UI".to_string()],
                publishes_changes: None,
                rebuilds_cache: None,
            },
            OwnerInventoryEntry {
                entity_name: "project_identity".to_string(),
                current_path: "5.editor/editor-state-containers/src/owners/project_owner.rs"
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
                entity_name: "material_registry".to_string(),
                current_path: "5.editor/l9.3-material-authoring/src/material_authoring_service.rs"
                    .to_string(),
                current_owner: "MaterialAuthoringService".to_string(),
                target_owner: "MaterialAuthoringService".to_string(),
                classification: StateClassification::Persistable,
                can_mutate: vec!["MaterialAuthoringService".to_string()],
                can_read: vec!["QueryLayer".to_string()],
                publishes_changes: Some("EventBus".to_string()),
                rebuilds_cache: Some("MaterialRegistryCache".to_string()),
            },
        ],
    }
}

fn create_test_codebase_state() -> CodebaseState {
    CodebaseState {
        state_fields: vec![StateField {
            name: "hover_state".to_string(),
            file_path: "6.apps/editor/src/desktop_app/state/ui_transient_state.rs".to_string(),
            owner_type: "UiTransientState".to_string(),
            field_type: "Option<HoverState>".to_string(),
        }],
    }
}

#[test]
fn test_validator_detects_business_truth_in_app_state() {
    let inventory = create_test_inventory();
    let mut codebase_state = create_test_codebase_state();
    codebase_state.state_fields.push(StateField {
        name: "project_identity".to_string(),
        file_path: "6.apps/editor/src/desktop_app/app_state.rs".to_string(),
        owner_type: "EditorAppRuntime".to_string(),
        field_type: "ProjectIdentity".to_string(),
    });

    let validator = UiStateClassificationValidator::new(inventory);
    let violations = validator.validate(&codebase_state).unwrap_err();

    assert_eq!(violations.len(), 1);
    assert_eq!(
        violations[0].violation_type,
        UiStateViolationType::BusinessTruthInAppState
    );
    assert_eq!(violations[0].field_name, "project_identity");
}

#[test]
fn test_validator_detects_non_transient_in_panel() {
    let inventory = create_test_inventory();
    let mut codebase_state = create_test_codebase_state();
    codebase_state.state_fields.push(StateField {
        name: "material_registry".to_string(),
        file_path: "6.apps/editor/src/panels/material_panel.rs".to_string(),
        owner_type: "MaterialPanel".to_string(),
        field_type: "MaterialRegistry".to_string(),
    });

    let validator = UiStateClassificationValidator::new(inventory);
    let violations = validator.validate(&codebase_state).unwrap_err();

    assert!(violations
        .iter()
        .any(|v| v.violation_type == UiStateViolationType::NonTransientInPanel));
}

#[test]
fn test_validator_detects_business_logic_in_editor_host() {
    let inventory = create_test_inventory();
    let mut codebase_state = create_test_codebase_state();
    codebase_state.state_fields.push(StateField {
        name: "create_material_impl".to_string(),
        file_path: "6.apps/editor/src/editor_host/mod.rs".to_string(),
        owner_type: "EditorHost".to_string(),
        field_type: "fn create_material(&mut self, profile: MaterialProfile)".to_string(),
    });

    let validator = UiStateClassificationValidator::new(inventory);
    let violations = validator.validate(&codebase_state).unwrap_err();

    assert!(violations
        .iter()
        .any(|v| v.violation_type == UiStateViolationType::EditorHostBusinessLogic));
}

#[test]
fn test_validator_allows_service_references_and_transient_state() {
    let inventory = create_test_inventory();
    let mut codebase_state = create_test_codebase_state();
    codebase_state.state_fields.push(StateField {
        name: "material_service".to_string(),
        file_path: "6.apps/editor/src/editor_host/mod.rs".to_string(),
        owner_type: "EditorHost".to_string(),
        field_type: "MaterialAuthoringService".to_string(),
    });

    let validator = UiStateClassificationValidator::new(inventory);
    assert!(validator.validate(&codebase_state).is_ok());
}
