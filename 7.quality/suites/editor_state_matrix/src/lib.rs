#[cfg(test)]
mod tests {
    use repo_hygiene::{
        CacheRebuildabilityValidator, CodebaseState, OwnerInventory, OwnerInventoryEntry,
        OwnershipCompletenessValidator, PersistenceSeparationValidator, StateClassification,
        StateField, UiStateClassificationValidator,
    };
    use stratumx_repo_hygiene_support::workspace_root_from_manifest_dir;

    fn fixture_inventory() -> OwnerInventory {
        OwnerInventory {
            version: "phase5".to_string(),
            entries: vec![
                OwnerInventoryEntry {
                    entity_name: "project_identity".to_string(),
                    current_path: "5.editor/editor-state-containers/src/owners/project_owner.rs".to_string(),
                    current_owner: "ProjectOwner".to_string(),
                    target_owner: "ProjectOwner".to_string(),
                    classification: StateClassification::Persistable,
                    can_mutate: vec!["ProjectBootstrapService".to_string()],
                    can_read: vec!["ProjectQueries".to_string()],
                    publishes_changes: Some("ProjectChanged".to_string()),
                    rebuilds_cache: None,
                },
                OwnerInventoryEntry {
                    entity_name: "material_registry_cache".to_string(),
                    current_path: "5.editor/editor-state-containers/src/cache/material_cache.rs".to_string(),
                    current_owner: "MaterialCache".to_string(),
                    target_owner: "MaterialCache".to_string(),
                    classification: StateClassification::Derived,
                    can_mutate: vec!["MaterialCache".to_string()],
                    can_read: vec!["MaterialQueries".to_string()],
                    publishes_changes: None,
                    rebuilds_cache: Some("MaterialRegistryCache".to_string()),
                },
                OwnerInventoryEntry {
                    entity_name: "hover_state".to_string(),
                    current_path: "6.apps/editor/stratumx_editor_app/src/desktop_app/state/ui_state.rs".to_string(),
                    current_owner: "UiTransientState".to_string(),
                    target_owner: "UiTransientState".to_string(),
                    classification: StateClassification::Transient,
                    can_mutate: vec!["EditorShell".to_string()],
                    can_read: vec!["EditorShell".to_string()],
                    publishes_changes: None,
                    rebuilds_cache: None,
                },
                OwnerInventoryEntry {
                    entity_name: "project_identity_view".to_string(),
                    current_path: "5.editor/editor-state-containers/src/persistence/project_persistence_view.rs".to_string(),
                    current_owner: "ProjectPersistenceView".to_string(),
                    target_owner: "ProjectPersistenceView".to_string(),
                    classification: StateClassification::Persistable,
                    can_mutate: vec!["ProjectPersistence".to_string()],
                    can_read: vec!["ProjectPersistence".to_string()],
                    publishes_changes: None,
                    rebuilds_cache: None,
                },
            ],
        }
    }

    fn fixture_state() -> CodebaseState {
        CodebaseState {
            state_fields: vec![
                StateField {
                    name: "project_identity".to_string(),
                    file_path: "5.editor/editor-state-containers/src/owners/project_owner.rs".to_string(),
                    owner_type: "ProjectOwner".to_string(),
                    field_type: "ProjectIdentity".to_string(),
                },
                StateField {
                    name: "project_identity_view".to_string(),
                    file_path: "5.editor/editor-state-containers/src/persistence/project_persistence_view.rs".to_string(),
                    owner_type: "ProjectPersistenceView".to_string(),
                    field_type: "ProjectIdentity".to_string(),
                },
                StateField {
                    name: "material_registry_cache".to_string(),
                    file_path: "5.editor/editor-state-containers/src/cache/material_cache.rs".to_string(),
                    owner_type: "MaterialRegistryCache".to_string(),
                    field_type: "impl RebuildableCache<Key, Value>".to_string(),
                },
                StateField {
                    name: "hover_state".to_string(),
                    file_path: "6.apps/editor/stratumx_editor_app/src/desktop_app/state/ui_state.rs".to_string(),
                    owner_type: "UiTransientState".to_string(),
                    field_type: "Option<HoverState>".to_string(),
                },
            ],
        }
    }

    #[test]
    fn state_matrix_fixture_respects_owner_persistence_cache_and_ui_laws() {
        let inventory = fixture_inventory();
        let state = fixture_state();

        OwnershipCompletenessValidator::new(inventory.clone())
            .validate(&state)
            .expect("fixture ownership should be valid");
        PersistenceSeparationValidator::new(inventory.clone())
            .validate(&state)
            .expect("fixture persistence split should be valid");
        CacheRebuildabilityValidator::new()
            .validate(&state)
            .expect("fixture caches should be rebuildable");
        UiStateClassificationValidator::new(inventory)
            .validate(&state)
            .expect("fixture ui state should be transient-only");
    }

    #[test]
    fn repo_scan_produces_real_state_inventory_input() {
        let repo_root = workspace_root_from_manifest_dir(env!("CARGO_MANIFEST_DIR"));
        let codebase = CodebaseState::scan_from_directory(&repo_root).expect("repo scan");
        assert!(!codebase.state_fields.is_empty());
    }
}
