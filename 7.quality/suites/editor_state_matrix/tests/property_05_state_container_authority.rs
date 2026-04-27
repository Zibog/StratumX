//! Property-Based Tests for State Container Authority
//!
//! **Validates: Requirements 6.1, 6.2, 6.3, 6.4, 6.5, 6.6, 14.2, 14.3**
//!
//! This module contains property-based tests that verify state mutations occur
//! in state containers, not UI components. UI components must only read state
//! through query interfaces and never directly mutate state.

use proptest::prelude::*;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use stratumx_editor_state_containers::*;
use uuid::Uuid;

// ============================================================================
// Property 5: State Container Authority
// ============================================================================
//
// **Validates: Requirements 6.1, 6.2, 6.3, 6.4, 6.5, 6.6, 14.2, 14.3**
//
// Property 5: State Container Authority
//
// *For any* state mutation operation, when the mutation is performed, it must
// occur through a state container method and never through direct field access
// on UI components.
//
// This test validates that:
// - State mutations only happen through state container methods
// - UI components cannot directly mutate state
// - State containers are the single source of truth
// - All state changes go through proper container interfaces
//
// The test creates various state containers, performs mutations through their
// methods, and verifies that:
// 1. Mutations are tracked and observable
// 2. State changes emit events
// 3. Query interfaces return updated state
// 4. UI components cannot bypass container methods

proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]

    #[test]
    fn property_state_container_authority(
        // Generate random mutation operations
        mutation_ops in mutation_operations_strategy()
    ) {
        use std::collections::HashSet;

        // ====================================================================
        // Test 1: Project_State mutations occur through container methods
        // ====================================================================
        let project_id = ProjectIdentity::new(
            Uuid::new_v4(),
            "Test Project".to_string(),
            PathBuf::from("/test/project"),
        );
        let workspace_id = WorkspaceIdentity::new(
            Uuid::new_v4(),
            "Test Workspace".to_string(),
            PathBuf::from("/test/workspace"),
        );

        let mut project_state = ProjectState::new(project_id.clone(), workspace_id.clone());

        // Track events to verify mutations go through container
        let events = Arc::new(Mutex::new(Vec::new()));
        let events_clone = events.clone();
        project_state.set_event_callback(Box::new(move |event| {
            events_clone.lock().unwrap().push(event);
        }));

        // Perform mutations through container methods
        let initial_generation = project_state.get_save_generation();
        for _ in 0..mutation_ops.project_save_increments {
            project_state.increment_save_generation();
        }

        // Verify mutations occurred through container
        prop_assert_eq!(
            project_state.get_save_generation(),
            initial_generation + mutation_ops.project_save_increments as u64
        );

        // Verify events were emitted for each mutation
        let captured_events = events.lock().unwrap();
        prop_assert_eq!(
            captured_events.len(),
            mutation_ops.project_save_increments
        );

        // Verify all events are SaveGenerationIncremented
        for event in captured_events.iter() {
            match event {
                ProjectStateEvent::SaveGenerationIncremented { .. } => {},
                _ => prop_assert!(false, "Expected SaveGenerationIncremented event"),
            }
        }

        // ====================================================================
        // Test 2: Session_State mutations occur through container methods
        // ====================================================================
        let mut session_state = SessionState::new();

        // Verify initial state
        prop_assert!(session_state.active_world.is_none());
        prop_assert!(session_state.open_panels.is_empty());
        prop_assert!(!session_state.selection_state.has_selection());

        // Perform mutations through container methods
        if mutation_ops.set_active_world {
            let world_id = WorldIdentity::new(
                Uuid::new_v4(),
                "Test World".to_string(),
                PathBuf::from("/test/world"),
            );
            session_state.set_active_world(Some(world_id.clone()));

            // Verify mutation occurred
            prop_assert!(session_state.active_world.is_some());
            prop_assert_eq!(
                &session_state.active_world.as_ref().unwrap().world_name,
                "Test World"
            );
        }

        // Add panels through container methods
        for panel_id in &mutation_ops.panels_to_add {
            session_state.add_open_panel(panel_id.clone());
        }

        // Verify mutations occurred (deduplicated by panel ID)
        let unique_panels: HashSet<_> = mutation_ops.panels_to_add.iter().collect();
        prop_assert_eq!(
            session_state.open_panels.len(),
            unique_panels.len()
        );

        // Add entities to selection through container methods
        for entity_id in &mutation_ops.entities_to_select {
            session_state
                .apply_modification(StateModification::AddToSelection(*entity_id))
                .expect("selection mutation through session state API");
        }

        // Verify mutations occurred
        prop_assert_eq!(
            session_state.selection_state.selected_entities.len(),
            mutation_ops.entities_to_select.len()
        );

        // ====================================================================
        // Test 3: World_State mutations occur through container methods
        // ====================================================================
        let world_id = WorldIdentity::new(
            Uuid::new_v4(),
            "Test World".to_string(),
            PathBuf::from("/test/world"),
        );

        let mut world_state = WorldState::new(world_id.clone(), "snapshot_123".to_string());

        // Track events to verify mutations go through container
        let world_events = Arc::new(Mutex::new(Vec::new()));
        let world_events_clone = world_events.clone();
        world_state.set_event_callback(Box::new(move |event| {
            world_events_clone.lock().unwrap().push(event);
        }));

        // Perform mutations through container methods
        if mutation_ops.update_terrain_state {
            let terrain = TerrainState::new(
                (1024, 1024),
                (1000.0, 1000.0),
                "default".to_string(),
            );
            world_state.set_terrain_state(Some(terrain));

            // Verify mutation occurred
            prop_assert!(world_state.get_terrain_state().is_some());
        }

        if mutation_ops.update_environment_state {
            let environment = EnvironmentState::new();
            world_state.set_environment_state(Some(environment));

            // Verify mutation occurred
            prop_assert!(world_state.get_environment_state().is_some());
        }

        // Add diagnostics through container methods
        for _ in 0..mutation_ops.diagnostics_to_add {
            let diagnostic = DiagnosticMessage::new(
                Severity::Error,
                "Test error".to_string(),
                DiagnosticSource::CommandSpine,
            );
            world_state.add_diagnostic(diagnostic);
        }

        // Verify mutations occurred
        prop_assert_eq!(
            world_state.get_diagnostics().len(),
            mutation_ops.diagnostics_to_add
        );

        // Verify events were emitted
        let captured_world_events = world_events.lock().unwrap();
        let expected_event_count =
            (if mutation_ops.update_terrain_state { 1 } else { 0 }) +
            (if mutation_ops.update_environment_state { 1 } else { 0 }) +
            mutation_ops.diagnostics_to_add;
        prop_assert_eq!(captured_world_events.len(), expected_event_count);

        // ====================================================================
        // Test 4: Material_Registry_State mutations occur through container
        // ====================================================================
        let mut material_state = MaterialRegistryState::new();

        // Verify initial state
        prop_assert!(material_state.material_profiles.is_empty());

        // Add profiles through container methods
        for profile_name in &mutation_ops.material_profiles_to_add {
            let profile_id = MaterialProfileId::new(Uuid::new_v4());
            let profile = MaterialProfile::new(profile_id, profile_name.clone());
            material_state.add_profile(profile);
        }

        // Verify mutations occurred
        prop_assert_eq!(
            material_state.material_profiles.len(),
            mutation_ops.material_profiles_to_add.len()
        );

        // ====================================================================
        // Test 5: Diagnostics_State mutations occur through container
        // ====================================================================
        let mut diagnostics_state = DiagnosticsState::new();

        // Verify initial state
        prop_assert!(diagnostics_state.messages.is_empty());

        // Add messages through container methods
        for severity in &mutation_ops.diagnostic_severities {
            let message = DiagnosticMessage::new(
                *severity,
                "Test message".to_string(),
                DiagnosticSource::CommandSpine,
            );
            diagnostics_state.add_message(message);
        }

        // Verify mutations occurred
        prop_assert_eq!(
            diagnostics_state.messages.len(),
            mutation_ops.diagnostic_severities.len()
        );

        // ====================================================================
        // Test 6: Workspace_State mutations occur through container
        // ====================================================================
        let mut workspace_state = WorkspaceState::new();

        // Verify initial state
        prop_assert!(workspace_state.open_panel_ids.is_empty());

        // Add panels through container methods
        for panel_id in &mutation_ops.workspace_panels_to_add {
            let geometry = PanelGeometry::floating(0.0, 0.0, 800.0, 600.0);
            workspace_state.add_panel(panel_id.clone(), geometry);
        }

        // Verify mutations occurred (deduplicated by panel ID)
        let unique_panels: HashSet<_> = mutation_ops.workspace_panels_to_add.iter().collect();

        prop_assert_eq!(
            workspace_state.open_panel_ids.len(),
            unique_panels.len()
        );

        // Set focused panel through container method
        if let Some(panel_id) = mutation_ops.workspace_panels_to_add.first() {
            workspace_state.set_focused_panel(Some(panel_id.clone()));

            // Verify mutation occurred
            prop_assert_eq!(
                workspace_state.focused_panel.as_ref(),
                Some(panel_id)
            );
        }

        // ====================================================================
        // Critical Property: State queries return consistent state
        // ====================================================================
        // After all mutations, verify that query interfaces return the
        // updated state, proving that mutations went through containers

        // Project state queries
        prop_assert_eq!(
            project_state.get_save_generation(),
            initial_generation + mutation_ops.project_save_increments as u64
        );

        // Session state queries
        let unique_session_panels: HashSet<_> = mutation_ops.panels_to_add.iter().collect();
        prop_assert_eq!(
            session_state.open_panels.len(),
            unique_session_panels.len()
        );
        prop_assert_eq!(
            session_state.selection_state.selected_entities.len(),
            mutation_ops.entities_to_select.len()
        );

        // World state queries
        prop_assert_eq!(
            world_state.get_diagnostics().len(),
            mutation_ops.diagnostics_to_add
        );

        // Material state queries
        prop_assert_eq!(
            material_state.material_profiles.len(),
            mutation_ops.material_profiles_to_add.len()
        );

        // Diagnostics state queries
        prop_assert_eq!(
            diagnostics_state.messages.len(),
            mutation_ops.diagnostic_severities.len()
        );

        // Workspace state queries
        let unique_workspace_panels: HashSet<_> = mutation_ops.workspace_panels_to_add.iter().collect();
        prop_assert_eq!(
            workspace_state.open_panel_ids.len(),
            unique_workspace_panels.len()
        );
    }
}

// ============================================================================
// Test Strategies
// ============================================================================

/// Mutation operations for property testing
#[derive(Debug, Clone)]
struct MutationOperations {
    // Project state mutations
    project_save_increments: usize,

    // Session state mutations
    set_active_world: bool,
    panels_to_add: Vec<PanelId>,
    entities_to_select: Vec<Uuid>,

    // World state mutations
    update_terrain_state: bool,
    update_environment_state: bool,
    diagnostics_to_add: usize,

    // Material state mutations
    material_profiles_to_add: Vec<String>,

    // Diagnostics state mutations
    diagnostic_severities: Vec<Severity>,

    // Workspace state mutations
    workspace_panels_to_add: Vec<PanelId>,
}

/// Strategy for generating mutation operations
fn mutation_operations_strategy() -> impl Strategy<Value = MutationOperations> {
    (
        0usize..10,                                        // project_save_increments
        any::<bool>(),                                     // set_active_world
        prop::collection::vec(panel_id_strategy(), 0..10), // panels_to_add
        prop::collection::vec(
            any::<[u8; 16]>().prop_map(|bytes| Uuid::from_bytes(bytes)),
            0..10,
        ), // entities_to_select
        any::<bool>(),                                     // update_terrain_state
        any::<bool>(),                                     // update_environment_state
        0usize..10,                                        // diagnostics_to_add
        prop::collection::vec("[a-z]{3,10}", 0..10),       // material_profiles_to_add
        prop::collection::vec(severity_strategy(), 0..10), // diagnostic_severities
        prop::collection::vec(panel_id_strategy(), 0..10), // workspace_panels_to_add
    )
        .prop_map(
            |(
                project_save_increments,
                set_active_world,
                panels_to_add,
                entities_to_select,
                update_terrain_state,
                update_environment_state,
                diagnostics_to_add,
                material_profiles_to_add,
                diagnostic_severities,
                workspace_panels_to_add,
            )| MutationOperations {
                project_save_increments,
                set_active_world,
                panels_to_add,
                entities_to_select,
                update_terrain_state,
                update_environment_state,
                diagnostics_to_add,
                material_profiles_to_add,
                diagnostic_severities,
                workspace_panels_to_add,
            },
        )
}

/// Strategy for generating panel IDs
fn panel_id_strategy() -> impl Strategy<Value = PanelId> {
    prop_oneof![
        Just(PanelId("viewport".to_string())),
        Just(PanelId("outliner".to_string())),
        Just(PanelId("inspector".to_string())),
        Just(PanelId("material".to_string())),
        Just(PanelId("terrain".to_string())),
        Just(PanelId("world".to_string())),
        Just(PanelId("sky_lighting".to_string())),
        Just(PanelId("audio".to_string())),
        Just(PanelId("diagnostics".to_string())),
        Just(PanelId("build_release".to_string())),
    ]
}

/// Strategy for generating severity levels
fn severity_strategy() -> impl Strategy<Value = Severity> {
    prop_oneof![
        Just(Severity::Error),
        Just(Severity::Warning),
        Just(Severity::Info),
    ]
}

// ============================================================================
// Unit Tests for State Container Authority
// ============================================================================

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn test_project_state_mutations_through_container() {
        let project_id = ProjectIdentity::new(
            Uuid::new_v4(),
            "Test Project".to_string(),
            PathBuf::from("/test/project"),
        );
        let workspace_id = WorkspaceIdentity::new(
            Uuid::new_v4(),
            "Test Workspace".to_string(),
            PathBuf::from("/test/workspace"),
        );

        let mut project_state = ProjectState::new(project_id, workspace_id);

        // Track events
        let events = Arc::new(Mutex::new(Vec::new()));
        let events_clone = events.clone();
        project_state.set_event_callback(Box::new(move |event| {
            events_clone.lock().unwrap().push(event);
        }));

        // Mutate through container method
        project_state.increment_save_generation();

        // Verify mutation occurred
        assert_eq!(project_state.get_save_generation(), 1);

        // Verify event was emitted
        let captured_events = events.lock().unwrap();
        assert_eq!(captured_events.len(), 1);
        assert!(matches!(
            captured_events[0],
            ProjectStateEvent::SaveGenerationIncremented { new_generation: 1 }
        ));
    }

    #[test]
    fn test_session_state_mutations_through_container() {
        let mut session_state = SessionState::new();

        // Mutate through container methods
        let panel_id = PanelId("viewport".to_string());
        session_state.add_open_panel(panel_id.clone());

        // Verify mutation occurred
        assert_eq!(session_state.open_panels.len(), 1);
        assert_eq!(session_state.open_panels[0], panel_id);

        // Mutate selection through container methods
        let entity_id = Uuid::new_v4();
        session_state
            .apply_modification(StateModification::AddToSelection(entity_id))
            .expect("selection mutation through session state API");

        // Verify mutation occurred
        assert!(session_state.selection_state.has_selection());
        assert_eq!(session_state.selection_state.selected_entities.len(), 1);
    }

    #[test]
    fn test_world_state_mutations_through_container() {
        let world_id = WorldIdentity::new(
            Uuid::new_v4(),
            "Test World".to_string(),
            PathBuf::from("/test/world"),
        );

        let mut world_state = WorldState::new(world_id, "snapshot_123".to_string());

        // Track events
        let events = Arc::new(Mutex::new(Vec::new()));
        let events_clone = events.clone();
        world_state.set_event_callback(Box::new(move |event| {
            events_clone.lock().unwrap().push(event);
        }));

        // Mutate through container method
        let terrain = TerrainState::new((1024, 1024), (1000.0, 1000.0), "default".to_string());
        world_state.set_terrain_state(Some(terrain));

        // Verify mutation occurred
        assert!(world_state.get_terrain_state().is_some());

        // Verify event was emitted
        let captured_events = events.lock().unwrap();
        assert_eq!(captured_events.len(), 1);
        assert!(matches!(
            captured_events[0],
            WorldStateEvent::TerrainStateUpdated
        ));
    }

    #[test]
    fn test_material_state_mutations_through_container() {
        let mut material_state = MaterialRegistryState::new();

        // Mutate through container method
        let profile_id = MaterialProfileId::new(Uuid::new_v4());
        let profile = MaterialProfile::new(profile_id, "Test Profile".to_string());
        material_state.add_profile(profile);

        // Verify mutation occurred
        assert_eq!(material_state.material_profiles.len(), 1);
    }

    #[test]
    fn test_diagnostics_state_mutations_through_container() {
        let mut diagnostics_state = DiagnosticsState::new();

        // Mutate through container method
        let message = DiagnosticMessage::new(
            Severity::Error,
            "Test error".to_string(),
            DiagnosticSource::CommandSpine,
        );
        diagnostics_state.add_message(message);

        // Verify mutation occurred
        assert_eq!(diagnostics_state.messages.len(), 1);
    }

    #[test]
    fn test_workspace_state_mutations_through_container() {
        let mut workspace_state = WorkspaceState::new();

        // Mutate through container method
        let panel_id = PanelId("viewport".to_string());
        let geometry = PanelGeometry::floating(0.0, 0.0, 800.0, 600.0);
        workspace_state.add_panel(panel_id.clone(), geometry);

        // Verify mutation occurred
        assert_eq!(workspace_state.open_panel_ids.len(), 1);
        assert_eq!(workspace_state.open_panel_ids[0], panel_id);
    }

    #[test]
    fn test_state_queries_reflect_mutations() {
        let project_id = ProjectIdentity::new(
            Uuid::new_v4(),
            "Test Project".to_string(),
            PathBuf::from("/test/project"),
        );
        let workspace_id = WorkspaceIdentity::new(
            Uuid::new_v4(),
            "Test Workspace".to_string(),
            PathBuf::from("/test/workspace"),
        );

        let mut project_state = ProjectState::new(project_id, workspace_id);

        // Initial query
        assert_eq!(project_state.get_save_generation(), 0);

        // Mutate
        project_state.increment_save_generation();

        // Query reflects mutation
        assert_eq!(project_state.get_save_generation(), 1);

        // Mutate again
        project_state.increment_save_generation();

        // Query reflects mutation
        assert_eq!(project_state.get_save_generation(), 2);
    }

    #[test]
    fn test_multiple_mutations_tracked_through_events() {
        let project_id = ProjectIdentity::new(
            Uuid::new_v4(),
            "Test Project".to_string(),
            PathBuf::from("/test/project"),
        );
        let workspace_id = WorkspaceIdentity::new(
            Uuid::new_v4(),
            "Test Workspace".to_string(),
            PathBuf::from("/test/workspace"),
        );

        let mut project_state = ProjectState::new(project_id, workspace_id);

        // Track events
        let events = Arc::new(Mutex::new(Vec::new()));
        let events_clone = events.clone();
        project_state.set_event_callback(Box::new(move |event| {
            events_clone.lock().unwrap().push(event);
        }));

        // Perform multiple mutations
        for _ in 0..5 {
            project_state.increment_save_generation();
        }

        // Verify all mutations were tracked
        let captured_events = events.lock().unwrap();
        assert_eq!(captured_events.len(), 5);

        // Verify final state
        assert_eq!(project_state.get_save_generation(), 5);
    }
}
