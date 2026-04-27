use super::*;
use crate::{OwnerInventoryEntry, StateClassification, StateField};

fn create_test_inventory() -> OwnerInventory {
    OwnerInventory {
        version: "1.0".to_string(),
        entries: vec![
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
                entity_name: "event_bus".to_string(),
                current_path: "5.editor/editor-state-containers/src/owners/project_owner.rs"
                    .to_string(),
                current_owner: "ProjectOwner".to_string(),
                target_owner: "ProjectOwner".to_string(),
                classification: StateClassification::Transient,
                can_mutate: vec!["ProjectOwner".to_string()],
                can_read: vec!["ProjectOwner".to_string()],
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
                file_path: "5.editor/editor-state-containers/src/owners/project_owner.rs"
                    .to_string(),
                owner_type: "ProjectOwner".to_string(),
                field_type: "ProjectIdentity".to_string(),
            },
            StateField {
                name: "project_identity".to_string(),
                file_path:
                    "5.editor/editor-state-containers/src/persistence/project_persistence_view.rs"
                        .to_string(),
                owner_type: "ProjectPersistenceView".to_string(),
                field_type: "ProjectIdentity".to_string(),
            },
        ],
    }
}

#[test]
fn test_validator_detects_runtime_field_in_persistence() {
    let inventory = create_test_inventory();
    let mut codebase_state = create_test_codebase_state();
    codebase_state.state_fields.push(StateField {
        name: "event_bus".to_string(),
        file_path: "5.editor/editor-state-containers/src/persistence/project_persistence_view.rs"
            .to_string(),
        owner_type: "ProjectPersistenceView".to_string(),
        field_type: "Arc<dyn EventBus>".to_string(),
    });

    let validator = PersistenceSeparationValidator::new(inventory);
    let violations = validator.validate(&codebase_state).unwrap_err();

    assert!(violations
        .iter()
        .any(|v| v.violation_type == PersistenceViolationType::RuntimeFieldInPersistence));
}

#[test]
fn test_validator_detects_fake_defaults() {
    let inventory = create_test_inventory();
    let mut codebase_state = create_test_codebase_state();
    codebase_state.state_fields.push(StateField {
        name: "project_identity".to_string(),
        file_path: "5.editor/editor-state-containers/src/persistence/project_persistence_view.rs"
            .to_string(),
        owner_type: "ProjectPersistenceView".to_string(),
        field_type: "Option<ProjectIdentity>.unwrap_or_default()".to_string(),
    });

    let validator = PersistenceSeparationValidator::new(inventory);
    let violations = validator.validate(&codebase_state).unwrap_err();

    assert!(violations
        .iter()
        .any(|v| v.violation_type == PersistenceViolationType::FakeDefaultsInPersistence));
}

#[test]
fn test_validator_detects_wrong_directory() {
    let inventory = create_test_inventory();
    let mut codebase_state = create_test_codebase_state();
    codebase_state.state_fields.push(StateField {
        name: "project_data".to_string(),
        file_path: "5.editor/editor-state-containers/src/owners/project_owner.rs".to_string(),
        owner_type: "ProjectPersistenceView".to_string(),
        field_type: "ProjectData".to_string(),
    });

    let validator = PersistenceSeparationValidator::new(inventory);
    let violations = validator.validate(&codebase_state).unwrap_err();

    assert!(violations
        .iter()
        .any(|v| v.violation_type == PersistenceViolationType::WrongDirectory));
}

#[test]
fn test_validator_passes_with_valid_persistence() {
    let validator = PersistenceSeparationValidator::new(create_test_inventory());
    assert!(validator.validate(&create_test_codebase_state()).is_ok());
}
