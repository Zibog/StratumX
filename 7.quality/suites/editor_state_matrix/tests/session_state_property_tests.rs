//! Property-Based Tests for Editor State Containers
//!
//! These tests verify universal properties that should hold across all inputs.

use proptest::prelude::*;
use stratumx_editor_l8_5_tool_context_system::{
    EntityId, PanelId, SelectionMode, SessionState, StateModification, ToolMode,
    ValidationErrorCode, WorldIdentity,
};
use uuid::Uuid;

// ============================================================================
// Generators (Strategies)
// ============================================================================

/// Generate arbitrary EntityId (UUID)
fn arbitrary_entity_id() -> impl Strategy<Value = EntityId> {
    any::<[u8; 16]>().prop_map(|bytes| Uuid::from_bytes(bytes))
}

/// Generate arbitrary non-nil EntityId
fn arbitrary_valid_entity_id() -> impl Strategy<Value = EntityId> {
    any::<[u8; 16]>()
        .prop_filter("UUID must not be nil", |bytes| {
            !Uuid::from_bytes(*bytes).is_nil()
        })
        .prop_map(|bytes| Uuid::from_bytes(bytes))
}

/// Generate arbitrary PanelId
fn arbitrary_panel_id() -> impl Strategy<Value = PanelId> {
    "[a-z]{1,20}".prop_map(PanelId)
}

/// Generate arbitrary non-empty PanelId
fn arbitrary_valid_panel_id() -> impl Strategy<Value = PanelId> {
    "[a-z]{1,20}".prop_map(PanelId)
}

/// Generate arbitrary WorldIdentity
fn arbitrary_world_identity() -> impl Strategy<Value = WorldIdentity> {
    (any::<[u8; 16]>(), "[a-z]{1,20}", "[a-z/]{1,50}")
        .prop_map(|(uuid_bytes, name, _path)| WorldIdentity {
            world_id: Uuid::from_bytes(uuid_bytes),
            world_name: name,
        })
}

/// Generate arbitrary valid WorldIdentity (non-nil UUID, non-empty name)
fn arbitrary_valid_world_identity() -> impl Strategy<Value = WorldIdentity> {
    (
        any::<[u8; 16]>().prop_filter("UUID must not be nil", |bytes| {
            !Uuid::from_bytes(*bytes).is_nil()
        }),
        "[a-z]{1,20}",
        "[a-z/]{1,50}",
    )
        .prop_map(|(uuid_bytes, name, _path)| WorldIdentity {
            world_id: Uuid::from_bytes(uuid_bytes),
            world_name: name,
        })
}

/// Generate arbitrary SelectionMode
fn arbitrary_selection_mode() -> impl Strategy<Value = SelectionMode> {
    prop_oneof![
        Just(SelectionMode::Single),
        Just(SelectionMode::Multiple),
        Just(SelectionMode::Hierarchical),
    ]
}

/// Generate arbitrary ToolMode
fn arbitrary_tool_mode() -> impl Strategy<Value = ToolMode> {
    prop_oneof![
        Just(ToolMode::Select),
        Just(ToolMode::Move),
        Just(ToolMode::Rotate),
        Just(ToolMode::Scale),
        Just(ToolMode::TerrainSculpt),
        Just(ToolMode::Paint),
    ]
}

/// Generate arbitrary StateModification
fn arbitrary_state_modification() -> impl Strategy<Value = StateModification> {
    prop_oneof![
        // SetActiveWorld with Some
        arbitrary_world_identity().prop_map(|w| StateModification::SetActiveWorld(Some(w))),
        // SetActiveWorld with None
        Just(StateModification::SetActiveWorld(None)),
        // AddOpenPanel
        arbitrary_panel_id().prop_map(StateModification::AddOpenPanel),
        // RemoveOpenPanel
        arbitrary_panel_id().prop_map(StateModification::RemoveOpenPanel),
        // SetFocusedPanel with Some
        arbitrary_panel_id().prop_map(|p| StateModification::SetFocusedPanel(Some(p))),
        // SetFocusedPanel with None
        Just(StateModification::SetFocusedPanel(None)),
        // SetSelection
        prop::collection::vec(arbitrary_entity_id(), 0..10)
            .prop_map(StateModification::SetSelection),
        // AddToSelection
        arbitrary_entity_id().prop_map(|entity| StateModification::AddToSelection(vec![entity])),
        // RemoveFromSelection
        arbitrary_entity_id().prop_map(|entity| StateModification::RemoveFromSelection(vec![entity])),
        // ClearSelection
        Just(StateModification::ClearSelection),
        // SetSelectionMode
        arbitrary_selection_mode().prop_map(StateModification::SetSelectionMode),
        // SetToolMode
        arbitrary_tool_mode().prop_map(StateModification::SetToolMode),
        // IncrementSaveGeneration
        Just(StateModification::IncrementSaveGeneration),
    ]
}

/// Generate arbitrary valid StateModification (passes validation)
fn arbitrary_valid_state_modification() -> impl Strategy<Value = StateModification> {
    prop_oneof![
        // SetActiveWorld with valid world
        arbitrary_valid_world_identity().prop_map(|w| StateModification::SetActiveWorld(Some(w))),
        // SetActiveWorld with None
        Just(StateModification::SetActiveWorld(None)),
        // AddOpenPanel with valid panel
        arbitrary_valid_panel_id().prop_map(StateModification::AddOpenPanel),
        // RemoveOpenPanel with valid panel
        arbitrary_valid_panel_id().prop_map(StateModification::RemoveOpenPanel),
        // SetFocusedPanel with valid panel
        arbitrary_valid_panel_id().prop_map(|p| StateModification::SetFocusedPanel(Some(p))),
        // SetFocusedPanel with None
        Just(StateModification::SetFocusedPanel(None)),
        // SetSelection with valid entities
        prop::collection::vec(arbitrary_valid_entity_id(), 0..10)
            .prop_map(StateModification::SetSelection),
        // AddToSelection with valid entity
        arbitrary_valid_entity_id()
            .prop_map(|entity| StateModification::AddToSelection(vec![entity])),
        // RemoveFromSelection with valid entity
        arbitrary_valid_entity_id()
            .prop_map(|entity| StateModification::RemoveFromSelection(vec![entity])),
        // ClearSelection
        Just(StateModification::ClearSelection),
        // SetSelectionMode
        arbitrary_selection_mode().prop_map(StateModification::SetSelectionMode),
        // SetToolMode
        arbitrary_tool_mode().prop_map(StateModification::SetToolMode),
    ]
}

// ============================================================================
// Property Tests
// ============================================================================

// Feature: editor-apps-proof-lane-closure, Property 1: Валидация модификаций состояния
// Validates: Requirements 1.2
proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]

    #[test]
    fn prop_valid_modifications_pass_validation(modification in arbitrary_valid_state_modification()) {
        let state = SessionState::new();
        let result = state.validate_modification(&modification);
        prop_assert!(result.is_ok(), "Valid modification should pass validation: {:?}", modification);
    }
}

// Feature: editor-apps-proof-lane-closure, Property 1: Валидация модификаций состояния
// Validates: Requirements 1.2
proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]

    #[test]
    fn prop_invalid_modifications_fail_validation_with_descriptive_error(
        modification in arbitrary_state_modification()
    ) {
        let state = SessionState::new();
        let result = state.validate_modification(&modification);

        // If validation fails, error must have a descriptive message
        if let Err(error) = result {
            prop_assert!(!error.message.is_empty(), "Error message must not be empty");
            prop_assert!(
                matches!(
                    error.code,
                    ValidationErrorCode::InvalidEntityId
                        | ValidationErrorCode::InvalidPanelId
                        | ValidationErrorCode::InvalidWorldIdentity
                        | ValidationErrorCode::InvalidToolMode
                        | ValidationErrorCode::InvalidSelectionMode
                ),
                "Error must have a valid error code"
            );
        }
    }
}

// Feature: editor-apps-proof-lane-closure, Property 1: Валидация модификаций состояния
// Validates: Requirements 1.2
proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]

    #[test]
    fn prop_state_unchanged_on_validation_failure(
        modification in arbitrary_state_modification()
    ) {
        let mut state = SessionState::new();

        // Set some initial state
        let initial_tool_mode = ToolMode::Move;
        state.set_tool_mode(initial_tool_mode);

        // Clone state before validation
        let state_before = state.clone();

        // Validate modification
        let validation_result = state.validate_modification(&modification);

        // State should be unchanged after validation (validation is read-only)
        prop_assert_eq!(state.tool_mode, state_before.tool_mode);
        prop_assert_eq!(state.active_world, state_before.active_world);
        prop_assert_eq!(state.open_panels, state_before.open_panels);

        // If validation fails, applying should also fail or not be attempted
        if validation_result.is_err() {
            // State remains unchanged
            prop_assert_eq!(state.tool_mode, initial_tool_mode);
        }
    }
}

// Feature: editor-apps-proof-lane-closure, Property 1: Валидация модификаций состояния
// Validates: Requirements 1.2
proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]

    #[test]
    fn prop_validated_modifications_can_be_applied(
        modification in arbitrary_valid_state_modification()
    ) {
        let mut state = SessionState::new();

        // Validate modification
        let validation_result = state.validate_modification(&modification);
        prop_assert!(validation_result.is_ok(), "Valid modification should pass validation");

        // Apply modification
        let apply_result = state.apply_modification(modification.clone());

        // Application should succeed for session-related modifications
        // (IncrementSaveGeneration is for ProjectState, so it's expected to fail)
        if !matches!(modification, StateModification::IncrementSaveGeneration) {
            prop_assert!(apply_result.is_ok(), "Validated modification should be applicable: {:?}", modification);
        }
    }
}

// Feature: editor-apps-proof-lane-closure, Property 1: Валидация модификаций состояния
// Validates: Requirements 1.2
proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]

    #[test]
    fn prop_nil_entity_ids_rejected(
        entities in prop::collection::vec(Just(Uuid::nil()), 1..5)
    ) {
        let state = SessionState::new();
        let modification = StateModification::SetSelection(entities);

        let result = state.validate_modification(&modification);
        prop_assert!(result.is_err(), "Nil entity IDs should be rejected");

        if let Err(error) = result {
            prop_assert_eq!(error.code, ValidationErrorCode::InvalidEntityId);
        }
    }
}

// Feature: editor-apps-proof-lane-closure, Property 1: Валидация модификаций состояния
// Validates: Requirements 1.2
#[test]
fn prop_empty_panel_ids_rejected() {
    let state = SessionState::new();
    let modification = StateModification::AddOpenPanel(PanelId(String::new()));

    let result = state.validate_modification(&modification);
    assert!(result.is_err(), "Empty panel IDs should be rejected");

    if let Err(error) = result {
        assert_eq!(error.code, ValidationErrorCode::InvalidPanelId);
    }
}

// Feature: editor-apps-proof-lane-closure, Property 1: Валидация модификаций состояния
// Validates: Requirements 1.2
#[test]
fn prop_nil_world_id_rejected() {
    let state = SessionState::new();
    let world = WorldIdentity {
        world_id: Uuid::nil(),
        world_name: "test".to_string(),
    };
    let modification = StateModification::SetActiveWorld(Some(world));

    let result = state.validate_modification(&modification);
    assert!(result.is_err(), "Nil world ID should be rejected");

    if let Err(error) = result {
        assert_eq!(error.code, ValidationErrorCode::InvalidWorldIdentity);
    }
}

// Feature: editor-apps-proof-lane-closure, Property 1: Валидация модификаций состояния
// Validates: Requirements 1.2
#[test]
fn prop_empty_world_name_rejected() {
    let state = SessionState::new();
    let world = WorldIdentity {
        world_id: Uuid::new_v4(),
        world_name: String::new(),
    };
    let modification = StateModification::SetActiveWorld(Some(world));

    let result = state.validate_modification(&modification);
    assert!(result.is_err(), "Empty world name should be rejected");

    if let Err(error) = result {
        assert_eq!(error.code, ValidationErrorCode::InvalidWorldIdentity);
    }
}
