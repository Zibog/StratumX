//! Property-Based Tests for State Serialization
//!
//! These tests verify that serialization and deserialization work correctly.

use proptest::prelude::*;
use stratumx_editor_state_containers::{
    SessionState, SelectionMode, ToolMode, WorldIdentity,
    EntityId, PanelId,
};
use uuid::Uuid;
use std::path::PathBuf;

// ============================================================================
// Generators (Strategies)
// ============================================================================

/// Generate arbitrary EntityId (UUID)
fn arbitrary_entity_id() -> impl Strategy<Value = EntityId> {
    any::<[u8; 16]>().prop_map(|bytes| Uuid::from_bytes(bytes))
}

/// Generate arbitrary PanelId
fn arbitrary_panel_id() -> impl Strategy<Value = PanelId> {
    "[a-z]{1,20}".prop_map(PanelId)
}

/// Generate arbitrary WorldIdentity
fn arbitrary_world_identity() -> impl Strategy<Value = WorldIdentity> {
    (
        any::<[u8; 16]>(),
        "[a-z]{1,20}",
        "[a-z/]{1,50}",
    ).prop_map(|(uuid_bytes, name, path)| {
        WorldIdentity {
            world_id: Uuid::from_bytes(uuid_bytes),
            world_name: name,
            world_path: PathBuf::from(path),
        }
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

/// Generate arbitrary SessionState
fn arbitrary_session_state() -> impl Strategy<Value = SessionState> {
    (
        prop::option::of(arbitrary_world_identity()),
        prop::collection::vec(arbitrary_panel_id(), 0..5),
        prop::option::of(arbitrary_panel_id()),
        prop::collection::vec(arbitrary_entity_id(), 0..10),
        arbitrary_selection_mode(),
        arbitrary_tool_mode(),
        prop::collection::vec("[a-z/]{1,50}", 0..10),
    ).prop_map(|(active_world, open_panels, focused_panel, selected_entities, selection_mode, tool_mode, recent_paths)| {
        let mut state = SessionState::new();
        state.active_world = active_world;
        state.open_panels = open_panels;
        state.focused_panel = focused_panel;
        state.selection_state.selected_entities = selected_entities;
        state.selection_state.selection_mode = selection_mode;
        state.tool_mode = tool_mode;
        state.recently_opened_worlds = recent_paths.into_iter().map(PathBuf::from).collect();
        state
    })
}

// ============================================================================
// Property Tests
// ============================================================================

// Feature: editor-apps-proof-lane-closure, Property 2: Сериализация состояния (round-trip)
// Validates: Requirements 1.3
proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]
    
    #[test]
    fn prop_session_state_serialization_roundtrip(state in arbitrary_session_state()) {
        // Serialize
        let serialized = state.serialize();
        prop_assert!(serialized.is_ok(), "Serialization should succeed");
        
        let serialized_data = serialized.unwrap();
        prop_assert!(!serialized_data.is_empty(), "Serialized data should not be empty");
        
        // Deserialize
        let deserialized = SessionState::deserialize(&serialized_data);
        prop_assert!(deserialized.is_ok(), "Deserialization should succeed");
        
        let deserialized_state = deserialized.unwrap();
        
        // Verify equivalence
        prop_assert_eq!(state.active_world, deserialized_state.active_world);
        prop_assert_eq!(state.open_panels, deserialized_state.open_panels);
        prop_assert_eq!(state.focused_panel, deserialized_state.focused_panel);
        prop_assert_eq!(state.selection_state.selected_entities, deserialized_state.selection_state.selected_entities);
        prop_assert_eq!(state.selection_state.selection_mode, deserialized_state.selection_state.selection_mode);
        prop_assert_eq!(state.tool_mode, deserialized_state.tool_mode);
        prop_assert_eq!(state.recently_opened_worlds, deserialized_state.recently_opened_worlds);
    }
}

// Feature: editor-apps-proof-lane-closure, Property 2: Сериализация состояния (round-trip)
// Validates: Requirements 1.3
proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]
    
    #[test]
    fn prop_double_roundtrip_produces_same_result(state in arbitrary_session_state()) {
        // First round-trip
        let serialized1 = state.serialize().unwrap();
        let deserialized1 = SessionState::deserialize(&serialized1).unwrap();
        
        // Second round-trip
        let serialized2 = deserialized1.serialize().unwrap();
        let deserialized2 = SessionState::deserialize(&serialized2).unwrap();
        
        // Both deserialized states should be equivalent
        prop_assert_eq!(deserialized1.active_world, deserialized2.active_world);
        prop_assert_eq!(deserialized1.open_panels, deserialized2.open_panels);
        prop_assert_eq!(deserialized1.focused_panel, deserialized2.focused_panel);
        prop_assert_eq!(deserialized1.selection_state.selected_entities, deserialized2.selection_state.selected_entities);
        prop_assert_eq!(deserialized1.selection_state.selection_mode, deserialized2.selection_state.selection_mode);
        prop_assert_eq!(deserialized1.tool_mode, deserialized2.tool_mode);
        prop_assert_eq!(deserialized1.recently_opened_worlds, deserialized2.recently_opened_worlds);
    }
}

// Feature: editor-apps-proof-lane-closure, Property 2: Сериализация состояния (round-trip)
// Validates: Requirements 1.3
proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]
    
    #[test]
    fn prop_serialized_data_is_deterministic(state in arbitrary_session_state()) {
        // Serialize twice
        let serialized1 = state.serialize().unwrap();
        let serialized2 = state.serialize().unwrap();
        
        // Both serializations should produce identical bytes
        prop_assert_eq!(serialized1, serialized2, "Serialization should be deterministic");
    }
}

// Feature: editor-apps-proof-lane-closure, Property 2: Сериализация состояния (round-trip)
// Validates: Requirements 1.3
#[test]
fn prop_invalid_data_fails_deserialization() {
    // Invalid binary data
    let invalid_data = vec![0xFF, 0xFF, 0xFF, 0xFF];
    
    let result = SessionState::deserialize(&invalid_data);
    assert!(result.is_err(), "Invalid data should fail deserialization");
}

// Feature: editor-apps-proof-lane-closure, Property 2: Сериализация состояния (round-trip)
// Validates: Requirements 1.3
#[test]
fn prop_empty_data_fails_deserialization() {
    let empty_data: Vec<u8> = vec![];
    
    let result = SessionState::deserialize(&empty_data);
    assert!(result.is_err(), "Empty data should fail deserialization");
}

// Feature: editor-apps-proof-lane-closure, Property 2: Сериализация состояния (round-trip)
// Validates: Requirements 1.3
#[test]
fn prop_default_state_serialization() {
    let state = SessionState::default();
    
    // Serialize
    let serialized = state.serialize();
    assert!(serialized.is_ok(), "Default state should serialize successfully");
    
    // Deserialize
    let deserialized = SessionState::deserialize(&serialized.unwrap());
    assert!(deserialized.is_ok(), "Default state should deserialize successfully");
    
    let deserialized_state = deserialized.unwrap();
    assert_eq!(state.active_world, deserialized_state.active_world);
    assert_eq!(state.tool_mode, deserialized_state.tool_mode);
}
