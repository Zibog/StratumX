//! Unit Tests for Editor State Containers
//!
//! These tests verify specific examples and edge cases.

use std::path::PathBuf;
use stratumx_editor_l8_5_tool_context_system::{
    PanelId, SelectionMode, SessionState, StateModification, ToolMode, ValidationErrorCode,
    WorldIdentity,
};
use uuid::Uuid;

// ============================================================================
// Empty State Tests (Requirements 1.4)
// ============================================================================

#[test]
fn test_empty_state_creation() {
    let state = SessionState::new();

    assert!(state.get_active_world().is_none());
    assert!(state.get_open_panels().is_empty());
    assert!(state.get_focused_panel().is_none());
    assert!(!state.get_selection_state().has_selection());
    assert_eq!(state.get_tool_mode(), ToolMode::Select);
    assert!(state.get_recently_opened_worlds().is_empty());
}

#[test]
fn test_empty_state_serialization() {
    let state = SessionState::new();

    let serialized = state.serialize().expect("Serialization should succeed");
    let deserialized =
        SessionState::deserialize(&serialized).expect("Deserialization should succeed");

    assert_eq!(state.get_active_world(), deserialized.get_active_world());
    assert_eq!(state.get_tool_mode(), deserialized.get_tool_mode());
}

#[test]
fn test_default_state_equals_new_state() {
    let new_state = SessionState::new();
    let default_state = SessionState::default();

    assert_eq!(
        new_state.get_active_world(),
        default_state.get_active_world()
    );
    assert_eq!(new_state.get_tool_mode(), default_state.get_tool_mode());
    assert_eq!(new_state.get_open_panels(), default_state.get_open_panels());
}

// ============================================================================
// Invalid Modification Tests (Requirements 1.4)
// ============================================================================

#[test]
fn test_nil_entity_id_rejected() {
    let state = SessionState::new();
    let modification = StateModification::AddToSelection(vec![Uuid::nil()]);

    let result = state.validate_modification(&modification);
    assert!(result.is_err());

    let error = result.unwrap_err();
    assert_eq!(error.code, ValidationErrorCode::InvalidEntityId);
    assert!(error.message.contains("nil"));
}

#[test]
fn test_empty_panel_id_rejected() {
    let state = SessionState::new();
    let modification = StateModification::AddOpenPanel(PanelId(String::new()));

    let result = state.validate_modification(&modification);
    assert!(result.is_err());

    let error = result.unwrap_err();
    assert_eq!(error.code, ValidationErrorCode::InvalidPanelId);
    assert!(error.message.contains("empty"));
}

#[test]
fn test_nil_world_id_rejected() {
    let state = SessionState::new();
    let world = WorldIdentity {
        world_id: Uuid::nil(),
        world_name: "test".to_string(),
    };
    let modification = StateModification::SetActiveWorld(Some(world));

    let result = state.validate_modification(&modification);
    assert!(result.is_err());

    let error = result.unwrap_err();
    assert_eq!(error.code, ValidationErrorCode::InvalidWorldIdentity);
}

#[test]
fn test_empty_world_name_rejected() {
    let state = SessionState::new();
    let world = WorldIdentity {
        world_id: Uuid::new_v4(),
        world_name: String::new(),
    };
    let modification = StateModification::SetActiveWorld(Some(world));

    let result = state.validate_modification(&modification);
    assert!(result.is_err());

    let error = result.unwrap_err();
    assert_eq!(error.code, ValidationErrorCode::InvalidWorldIdentity);
    assert!(error.message.contains("empty"));
}

#[test]
fn test_multiple_nil_entities_in_selection_rejected() {
    let state = SessionState::new();
    let entities = vec![Uuid::nil(), Uuid::nil(), Uuid::nil()];
    let modification = StateModification::SetSelection(entities);

    let result = state.validate_modification(&modification);
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().code,
        ValidationErrorCode::InvalidEntityId
    );
}

// ============================================================================
// Edge Cases (Requirements 1.4)
// ============================================================================

#[test]
fn test_set_active_world_to_none() {
    let mut state = SessionState::new();

    // First set a world
    let world = WorldIdentity {
        world_id: Uuid::new_v4(),
        world_name: "test".to_string(),
    };
    let modification = StateModification::SetActiveWorld(Some(world.clone()));
    assert!(state.validate_modification(&modification).is_ok());
    assert!(state.apply_modification(modification).is_ok());
    assert!(state.get_active_world().is_some());

    // Now set to None
    let modification = StateModification::SetActiveWorld(None);
    assert!(state.validate_modification(&modification).is_ok());
    assert!(state.apply_modification(modification).is_ok());
    assert!(state.get_active_world().is_none());
}

#[test]
fn test_clear_empty_selection() {
    let mut state = SessionState::new();
    assert!(!state.get_selection_state().has_selection());

    let modification = StateModification::ClearSelection;
    assert!(state.validate_modification(&modification).is_ok());
    assert!(state.apply_modification(modification).is_ok());
    assert!(!state.get_selection_state().has_selection());
}

#[test]
fn test_remove_entity_not_in_selection() {
    let mut state = SessionState::new();
    let entity = Uuid::new_v4();

    let modification = StateModification::RemoveFromSelection(vec![entity]);
    assert!(state.validate_modification(&modification).is_ok());
    assert!(state.apply_modification(modification).is_ok());
    assert!(!state.get_selection_state().has_selection());
}

#[test]
fn test_add_duplicate_panel() {
    let mut state = SessionState::new();
    let panel_id = PanelId("viewport".to_string());

    // Add panel first time
    let modification = StateModification::AddOpenPanel(panel_id.clone());
    assert!(state.validate_modification(&modification).is_ok());
    assert!(state.apply_modification(modification).is_ok());
    assert_eq!(state.get_open_panels().len(), 1);

    // Add same panel again
    let modification = StateModification::AddOpenPanel(panel_id.clone());
    assert!(state.validate_modification(&modification).is_ok());
    assert!(state.apply_modification(modification).is_ok());
    // Should still be 1 (no duplicates)
    assert_eq!(state.get_open_panels().len(), 1);
}

#[test]
fn test_remove_panel_not_open() {
    let mut state = SessionState::new();
    let panel_id = PanelId("viewport".to_string());

    let modification = StateModification::RemoveOpenPanel(panel_id);
    assert!(state.validate_modification(&modification).is_ok());
    assert!(state.apply_modification(modification).is_ok());
    assert!(state.get_open_panels().is_empty());
}

#[test]
fn test_large_selection() {
    let mut state = SessionState::new();

    // Create a large selection (100 entities)
    let entities: Vec<Uuid> = (0..100).map(|_| Uuid::new_v4()).collect();
    let modification = StateModification::SetSelection(entities.clone());

    assert!(state.validate_modification(&modification).is_ok());
    assert!(state.apply_modification(modification).is_ok());
    assert_eq!(state.get_selection_state().selection_count(), 100);
}

#[test]
fn test_many_open_panels() {
    let mut state = SessionState::new();

    // Open 20 panels
    for i in 0..20 {
        let panel_id = PanelId(format!("panel_{}", i));
        let modification = StateModification::AddOpenPanel(panel_id);
        assert!(state.validate_modification(&modification).is_ok());
        assert!(state.apply_modification(modification).is_ok());
    }

    assert_eq!(state.get_open_panels().len(), 20);
}

#[test]
fn test_recently_opened_worlds_limit() {
    let mut state = SessionState::new();

    // Add 15 worlds
    for i in 0..15 {
        state.add_recently_opened_world(PathBuf::from(format!("/world{}", i)));
    }

    // Should only keep last 10
    assert_eq!(state.get_recently_opened_worlds().len(), 10);
}

#[test]
fn test_tool_mode_transitions() {
    let mut state = SessionState::new();
    assert_eq!(state.get_tool_mode(), ToolMode::Select);

    let modes = vec![
        ToolMode::Move,
        ToolMode::Rotate,
        ToolMode::Scale,
        ToolMode::TerrainSculpt,
        ToolMode::Paint,
        ToolMode::Select,
    ];

    for mode in modes {
        let modification = StateModification::SetToolMode(mode);
        assert!(state.validate_modification(&modification).is_ok());
        assert!(state.apply_modification(modification).is_ok());
        assert_eq!(state.get_tool_mode(), mode);
    }
}

#[test]
fn test_selection_mode_transitions() {
    let mut state = SessionState::new();
    assert_eq!(
        state.get_selection_state().get_selection_mode(),
        SelectionMode::Single
    );

    let modes = vec![
        SelectionMode::Multiple,
        SelectionMode::Hierarchical,
        SelectionMode::Single,
    ];

    for mode in modes {
        let modification = StateModification::SetSelectionMode(mode);
        assert!(state.validate_modification(&modification).is_ok());
        assert!(state.apply_modification(modification).is_ok());
        assert_eq!(state.get_selection_state().get_selection_mode(), mode);
    }
}

// ============================================================================
// Read-Only Access Tests (Requirements 1.5)
// ============================================================================

#[test]
fn test_read_only_access_methods() {
    let mut state = SessionState::new();

    // Set up some state
    let world = WorldIdentity {
        world_id: Uuid::new_v4(),
        world_name: "test".to_string(),
    };
    state
        .apply_modification(StateModification::SetActiveWorld(Some(world.clone())))
        .unwrap();
    state
        .apply_modification(StateModification::AddOpenPanel(PanelId(
            "viewport".to_string(),
        )))
        .unwrap();
    state
        .apply_modification(StateModification::SetToolMode(ToolMode::Move))
        .unwrap();

    // Test read-only access
    assert!(state.get_active_world().is_some());
    assert_eq!(state.get_open_panels().len(), 1);
    assert_eq!(state.get_tool_mode(), ToolMode::Move);

    // Verify we can't modify through read-only references
    // (This is enforced by the type system - the following would not compile:)
    // state.get_open_panels().push(PanelId("test".to_string()));
}

#[test]
fn test_selection_state_read_only_methods() {
    let mut state = SessionState::new();
    let entity = Uuid::new_v4();

    state
        .apply_modification(StateModification::AddToSelection(vec![entity]))
        .unwrap();

    let selection = state.get_selection_state();
    assert_eq!(selection.selection_count(), 1);
    assert!(selection.is_selected(&entity));
    assert!(selection.has_selection());
    assert_eq!(selection.get_selected_entities().len(), 1);
}

// ============================================================================
// Atomicity Tests (Requirements 1.2)
// ============================================================================

#[test]
fn test_state_unchanged_on_validation_failure() {
    let mut state = SessionState::new();

    // Set initial state
    state
        .apply_modification(StateModification::SetToolMode(ToolMode::Move))
        .unwrap();
    let initial_tool_mode = state.get_tool_mode();

    // Try to apply invalid modification
    let invalid_modification = StateModification::AddToSelection(vec![Uuid::nil()]);
    assert!(state.validate_modification(&invalid_modification).is_err());

    // State should be unchanged
    assert_eq!(state.get_tool_mode(), initial_tool_mode);
}

#[test]
fn test_validate_before_apply_pattern() {
    let mut state = SessionState::new();
    let entity = Uuid::new_v4();
    let modification = StateModification::AddToSelection(vec![entity]);

    // Validate first
    let validation_result = state.validate_modification(&modification);
    assert!(validation_result.is_ok());

    // Then apply
    let apply_result = state.apply_modification(modification);
    assert!(apply_result.is_ok());

    // Verify state changed
    assert!(state.get_selection_state().is_selected(&entity));
}
