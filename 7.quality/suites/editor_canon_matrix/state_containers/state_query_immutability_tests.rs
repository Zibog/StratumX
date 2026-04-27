//! Property-Based Tests for State Query Immutability
//!
//! **Validates: Requirements 14.3, 14.4**
//!
//! This module contains property-based tests that verify state queries return
//! immutable references or copied values, never mutable references that allow
//! UI components to mutate shared state.

#[cfg(test)]
mod tests {
    use crate::{
        DiagnosticMessage, DiagnosticSource, DiagnosticsState, MaterialProfile,
        MaterialProfileId, MaterialRegistryState, AudioRegistryState, AudioSource,
        AudioSourceId, AudioSourceType, PanelId, SelectionState, SessionState,
        Severity, StateQueries, WorldIdentity,
    };
    use proptest::prelude::*;
    use std::path::PathBuf;
    use uuid::Uuid;

    // ========================================================================
    // Property 18: State Query Immutability
    // ========================================================================
    //
    // **Validates: Requirements 14.3, 14.4**
    //
    // Property 18: State Query Immutability
    //
    // *For any* state query from UI components, the query must return immutable
    // references or copied values, never mutable references that allow UI
    // components to mutate shared state.
    //
    // This test validates that:
    // - State queries return immutable references (&T) or copied values
    // - UI components cannot mutate state through query interfaces
    // - All query methods enforce immutability
    //
    // The test creates a mock state container, performs various queries,
    // and verifies that the returned values cannot be used to mutate the
    // underlying state.

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(100))]

        #[test]
        fn property_state_query_immutability(
            // Generate random state configurations
            state_config in state_configuration_strategy()
        ) {
            // Create a mock state container with the generated configuration
            let queries = create_mock_state_queries(state_config.clone());
            
            // ================================================================
            // Test 1: get_active_world returns immutable reference
            // ================================================================
            if let Some(world) = queries.get_active_world() {
                // Verify we got an immutable reference
                // The type system enforces this - we cannot call mutable methods
                let _world_id = &world.world_id;
                let _world_name = &world.world_name;
                let _world_path = &world.world_path;
                
                // Verify the world identity matches the original
                prop_assert_eq!(world.world_id, state_config.world_identity.as_ref().unwrap().world_id);
            }
            
            // ================================================================
            // Test 2: get_selection returns immutable reference
            // ================================================================
            let selection = queries.get_selection();
            // Verify we got an immutable reference
            let _entities = &selection.selected_entities;
            let _mode = &selection.selection_mode;
            
            // Verify selection matches original
            prop_assert_eq!(
                selection.selected_entities.len(),
                state_config.selection_entities.len()
            );
            
            // ================================================================
            // Test 3: get_focused_panel returns immutable reference to Option
            // ================================================================
            if let Some(focused) = queries.get_focused_panel() {
                // Verify we got an immutable reference
                let _panel_id_str = &focused.0;
                
                // Verify focused panel matches original
                prop_assert_eq!(focused, state_config.focused_panel.as_ref().unwrap());
            }
            
            // ================================================================
            // Test 4: get_open_panels returns immutable slice
            // ================================================================
            let open_panels = queries.get_open_panels();
            // Verify we got an immutable slice
            let _panel_count = open_panels.len();
            for panel in open_panels {
                let _panel_id = &panel.0;
            }
            
            // Verify open panels match original
            prop_assert_eq!(open_panels.len(), state_config.open_panels.len());
            
            // ================================================================
            // Test 5: get_material_profiles returns Vec of immutable references
            // ================================================================
            let profiles = queries.get_material_profiles();
            // Verify we got immutable references
            for profile in &profiles {
                let _profile_id = &profile.profile_id;
                let _profile_name = &profile.profile_name;
                let _base_color = &profile.base_color;
            }
            
            // Verify profiles match original
            prop_assert_eq!(profiles.len(), state_config.material_profiles.len());
            
            // ================================================================
            // Test 6: get_audio_sources returns Vec of immutable references
            // ================================================================
            let audio_sources = queries.get_audio_sources();
            // Verify we got immutable references
            for source in &audio_sources {
                let _source_id = &source.source_id;
                let _source_name = &source.source_name;
                let _source_type = &source.source_type;
            }
            
            // Verify audio sources match original
            prop_assert_eq!(audio_sources.len(), state_config.audio_sources.len());
            
            // ================================================================
            // Test 7: get_diagnostics returns immutable slice
            // ================================================================
            let diagnostics = queries.get_diagnostics();
            // Verify we got an immutable slice
            for diagnostic in diagnostics {
                let _severity = &diagnostic.severity;
                let _message = &diagnostic.message;
                let _source = &diagnostic.source;
            }
            
            // Verify diagnostics match original
            prop_assert_eq!(diagnostics.len(), state_config.diagnostic_messages.len());
            
            // ================================================================
            // Test 8: Boolean queries return copied values (not references)
            // ================================================================
            let has_project = queries.has_project();
            let has_world = queries.has_active_world();
            let has_selection = queries.has_selection();
            
            // These are bool values (Copy type), not references
            // Verify they match the original state
            prop_assert_eq!(has_project, state_config.has_project);
            prop_assert_eq!(has_world, state_config.world_identity.is_some());
            prop_assert_eq!(has_selection, !state_config.selection_entities.is_empty());
            
            // ================================================================
            // Critical Property: State remains unchanged after queries
            // ================================================================
            // Query the state again and verify it hasn't changed
            let selection_after = queries.get_selection();
            prop_assert_eq!(
                selection_after.selected_entities.len(),
                state_config.selection_entities.len()
            );
            
            let open_panels_after = queries.get_open_panels();
            prop_assert_eq!(open_panels_after.len(), state_config.open_panels.len());
            
            let profiles_after = queries.get_material_profiles();
            prop_assert_eq!(profiles_after.len(), state_config.material_profiles.len());
            
            let audio_sources_after = queries.get_audio_sources();
            prop_assert_eq!(audio_sources_after.len(), state_config.audio_sources.len());
            
            let diagnostics_after = queries.get_diagnostics();
            prop_assert_eq!(diagnostics_after.len(), state_config.diagnostic_messages.len());
        }
    }

    // ========================================================================
    // Test Strategies
    // ========================================================================

    /// State configuration for property testing
    #[derive(Debug, Clone)]
    struct StateConfiguration {
        has_project: bool,
        world_identity: Option<WorldIdentity>,
        selection_entities: Vec<Uuid>,
        focused_panel: Option<PanelId>,
        open_panels: Vec<PanelId>,
        material_profiles: Vec<MaterialProfile>,
        audio_sources: Vec<AudioSource>,
        diagnostic_messages: Vec<DiagnosticMessage>,
    }

    /// Strategy for generating state configurations
    fn state_configuration_strategy() -> impl Strategy<Value = StateConfiguration> {
        (
            any::<bool>(),
            prop::option::of(world_identity_strategy()),
            prop::collection::vec(any::<[u8; 16]>().prop_map(|bytes| Uuid::from_bytes(bytes)), 0..10),
            prop::option::of(panel_id_strategy()),
            prop::collection::vec(panel_id_strategy(), 0..10),
            prop::collection::vec(material_profile_strategy(), 0..10),
            prop::collection::vec(audio_source_strategy(), 0..10),
            prop::collection::vec(diagnostic_message_strategy(), 0..10),
        )
            .prop_map(
                |(
                    has_project,
                    world_identity,
                    selection_entities,
                    focused_panel,
                    open_panels,
                    material_profiles,
                    audio_sources,
                    diagnostic_messages,
                )| StateConfiguration {
                    has_project,
                    world_identity,
                    selection_entities,
                    focused_panel,
                    open_panels,
                    material_profiles,
                    audio_sources,
                    diagnostic_messages,
                },
            )
    }

    /// Strategy for generating world identities
    fn world_identity_strategy() -> impl Strategy<Value = WorldIdentity> {
        (
            any::<[u8; 16]>().prop_map(|bytes| Uuid::from_bytes(bytes)),
            "[a-z]{3,10}",
            "[a-z/]{5,20}",
        )
            .prop_map(|(uuid, name, path)| {
                WorldIdentity::new(uuid, name, PathBuf::from(path))
            })
    }

    /// Strategy for generating panel IDs
    fn panel_id_strategy() -> impl Strategy<Value = PanelId> {
        prop_oneof![
            Just(PanelId("viewport".to_string())),
            Just(PanelId("outliner".to_string())),
            Just(PanelId("inspector".to_string())),
            Just(PanelId("material".to_string())),
            Just(PanelId("terrain".to_string())),
            Just(PanelId("diagnostics".to_string())),
        ]
    }

    /// Strategy for generating material profiles
    fn material_profile_strategy() -> impl Strategy<Value = MaterialProfile> {
        (
            any::<[u8; 16]>().prop_map(|bytes| MaterialProfileId::new(Uuid::from_bytes(bytes))),
            "[a-z]{3,10}",
        )
            .prop_map(|(profile_id, name)| MaterialProfile::new(profile_id, name))
    }

    /// Strategy for generating audio sources
    fn audio_source_strategy() -> impl Strategy<Value = AudioSource> {
        (
            any::<[u8; 16]>().prop_map(|bytes| AudioSourceId::new(Uuid::from_bytes(bytes))),
            "[a-z]{3,10}",
            audio_source_type_strategy(),
        )
            .prop_map(|(source_id, name, source_type)| {
                AudioSource::new(source_id, name, source_type)
            })
    }

    /// Strategy for generating audio source types
    fn audio_source_type_strategy() -> impl Strategy<Value = AudioSourceType> {
        prop_oneof![
            Just(AudioSourceType::Ambient),
            Just(AudioSourceType::Impact),
            Just(AudioSourceType::Continuous),
            Just(AudioSourceType::Trigger),
        ]
    }

    /// Strategy for generating diagnostic messages
    fn diagnostic_message_strategy() -> impl Strategy<Value = DiagnosticMessage> {
        (
            severity_strategy(),
            "[a-z ]{10,50}",
            diagnostic_source_strategy(),
        )
            .prop_map(|(severity, message, source)| DiagnosticMessage {
                severity,
                message,
                source,
                trace_id: None,
                artifact_ref: None,
                timestamp: 0,
            })
    }

    /// Strategy for generating severity levels
    fn severity_strategy() -> impl Strategy<Value = Severity> {
        prop_oneof![
            Just(Severity::Error),
            Just(Severity::Warning),
            Just(Severity::Info),
        ]
    }

    /// Strategy for generating diagnostic sources
    fn diagnostic_source_strategy() -> impl Strategy<Value = DiagnosticSource> {
        prop_oneof![
            Just(DiagnosticSource::CommandSpine),
            Just(DiagnosticSource::ToolingRoute("route.world.open.v1".to_string())),
            Just(DiagnosticSource::SdkPacket("packet.world.open".to_string())),
            Just(DiagnosticSource::EngineTruthOwner("engine/50".to_string())),
            Just(DiagnosticSource::EditorComponent("viewport".to_string())),
        ]
    }

    // ========================================================================
    // Mock State Queries Implementation
    // ========================================================================

    /// Mock state queries for testing
    struct MockStateQueries {
        session_state: SessionState,
        material_state: MaterialRegistryState,
        audio_state: AudioRegistryState,
        diagnostics_state: DiagnosticsState,
        has_project: bool,
    }

    impl StateQueries for MockStateQueries {
        fn get_active_world(&self) -> Option<&WorldIdentity> {
            self.session_state.active_world.as_ref()
        }

        fn get_selection(&self) -> &SelectionState {
            &self.session_state.selection_state
        }

        fn get_focused_panel(&self) -> Option<&PanelId> {
            self.session_state.focused_panel.as_ref()
        }

        fn get_open_panels(&self) -> &[PanelId] {
            &self.session_state.open_panels
        }

        fn get_material_profiles(&self) -> Vec<&MaterialProfile> {
            self.material_state.material_profiles.values().collect()
        }

        fn get_audio_sources(&self) -> Vec<&AudioSource> {
            self.audio_state.audio_sources.values().collect()
        }

        fn get_diagnostics(&self) -> &[DiagnosticMessage] {
            &self.diagnostics_state.messages
        }

        fn has_project(&self) -> bool {
            self.has_project
        }

        fn has_active_world(&self) -> bool {
            self.session_state.active_world.is_some()
        }

        fn has_selection(&self) -> bool {
            self.session_state.selection_state.has_selection()
        }
    }

    /// Creates a mock state queries instance from a state configuration
    fn create_mock_state_queries(config: StateConfiguration) -> MockStateQueries {
        let mut session_state = SessionState::new();
        
        // Set active world
        session_state.active_world = config.world_identity;
        
        // Set selection
        session_state.selection_state.selected_entities = config.selection_entities;
        
        // Set focused panel
        session_state.focused_panel = config.focused_panel;
        
        // Set open panels
        session_state.open_panels = config.open_panels;
        
        // Create material state
        let mut material_state = MaterialRegistryState::new();
        for profile in config.material_profiles {
            material_state.add_profile(profile);
        }
        
        // Create audio state
        let mut audio_state = AudioRegistryState::new();
        for source in config.audio_sources {
            audio_state.add_source(source);
        }
        
        // Create diagnostics state
        let mut diagnostics_state = DiagnosticsState::new();
        for message in config.diagnostic_messages {
            diagnostics_state.add_message(message);
        }
        
        MockStateQueries {
            session_state,
            material_state,
            audio_state,
            diagnostics_state,
            has_project: config.has_project,
        }
    }

    // ========================================================================
    // Unit Tests for State Query Immutability
    // ========================================================================

    #[test]
    fn test_get_active_world_returns_immutable_reference() {
        let mut session_state = SessionState::new();
        let world_id = WorldIdentity::new(
            Uuid::new_v4(),
            "Test World".to_string(),
            PathBuf::from("/test/world"),
        );
        session_state.set_active_world(Some(world_id.clone()));
        
        let queries = MockStateQueries {
            session_state,
            material_state: MaterialRegistryState::new(),
            audio_state: AudioRegistryState::new(),
            diagnostics_state: DiagnosticsState::new(),
            has_project: true,
        };
        
        // Get immutable reference
        let world_ref = queries.get_active_world();
        assert!(world_ref.is_some());
        
        // Verify we can read but not mutate
        let world = world_ref.unwrap();
        assert_eq!(world.world_name, "Test World");
        
        // The following would not compile (immutable reference):
        // world.world_name = "Modified".to_string();
    }

    #[test]
    fn test_get_selection_returns_immutable_reference() {
        let mut session_state = SessionState::new();
        let entity_id = Uuid::new_v4();
        session_state.selection_state.add_entity(entity_id);
        
        let queries = MockStateQueries {
            session_state,
            material_state: MaterialRegistryState::new(),
            audio_state: AudioRegistryState::new(),
            diagnostics_state: DiagnosticsState::new(),
            has_project: true,
        };
        
        // Get immutable reference
        let selection = queries.get_selection();
        assert!(selection.has_selection());
        assert_eq!(selection.selected_entities.len(), 1);
        
        // The following would not compile (immutable reference):
        // selection.selected_entities.push(Uuid::new_v4());
    }

    #[test]
    fn test_get_open_panels_returns_immutable_slice() {
        let mut session_state = SessionState::new();
        session_state.add_open_panel(PanelId("viewport".to_string()));
        session_state.add_open_panel(PanelId("inspector".to_string()));
        
        let queries = MockStateQueries {
            session_state,
            material_state: MaterialRegistryState::new(),
            audio_state: AudioRegistryState::new(),
            diagnostics_state: DiagnosticsState::new(),
            has_project: true,
        };
        
        // Get immutable slice
        let panels = queries.get_open_panels();
        assert_eq!(panels.len(), 2);
        
        // The following would not compile (immutable slice):
        // panels.push(PanelId("material".to_string()));
    }

    #[test]
    fn test_get_material_profiles_returns_immutable_references() {
        let mut material_state = MaterialRegistryState::new();
        let profile_id = MaterialProfileId::new(Uuid::new_v4());
        let profile = MaterialProfile::new(profile_id, "Test Profile".to_string());
        material_state.add_profile(profile);
        
        let queries = MockStateQueries {
            session_state: SessionState::new(),
            material_state,
            audio_state: AudioRegistryState::new(),
            diagnostics_state: DiagnosticsState::new(),
            has_project: true,
        };
        
        // Get immutable references
        let profiles = queries.get_material_profiles();
        assert_eq!(profiles.len(), 1);
        
        let profile = profiles[0];
        assert_eq!(profile.profile_name, "Test Profile");
        
        // The following would not compile (immutable reference):
        // profile.profile_name = "Modified".to_string();
    }

    #[test]
    fn test_get_diagnostics_returns_immutable_slice() {
        let mut diagnostics_state = DiagnosticsState::new();
        diagnostics_state.add_message(DiagnosticMessage {
            severity: Severity::Error,
            message: "Test error".to_string(),
            source: DiagnosticSource::CommandSpine,
            trace_id: None,
            artifact_ref: None,
            timestamp: 0,
        });
        
        let queries = MockStateQueries {
            session_state: SessionState::new(),
            material_state: MaterialRegistryState::new(),
            audio_state: AudioRegistryState::new(),
            diagnostics_state,
            has_project: true,
        };
        
        // Get immutable slice
        let diagnostics = queries.get_diagnostics();
        assert_eq!(diagnostics.len(), 1);
        
        // The following would not compile (immutable slice):
        // diagnostics.push(DiagnosticMessage { ... });
    }

    #[test]
    fn test_boolean_queries_return_copied_values() {
        let queries = MockStateQueries {
            session_state: SessionState::new(),
            material_state: MaterialRegistryState::new(),
            audio_state: AudioRegistryState::new(),
            diagnostics_state: DiagnosticsState::new(),
            has_project: true,
        };
        
        // Boolean queries return copied values (not references)
        let has_project = queries.has_project();
        let has_world = queries.has_active_world();
        let has_selection = queries.has_selection();
        
        // These are bool values, not &bool
        assert_eq!(has_project, true);
        assert_eq!(has_world, false);
        assert_eq!(has_selection, false);
    }

    #[test]
    fn test_state_unchanged_after_multiple_queries() {
        let mut session_state = SessionState::new();
        let entity_id = Uuid::new_v4();
        session_state.selection_state.add_entity(entity_id);
        
        let queries = MockStateQueries {
            session_state,
            material_state: MaterialRegistryState::new(),
            audio_state: AudioRegistryState::new(),
            diagnostics_state: DiagnosticsState::new(),
            has_project: true,
        };
        
        // Query multiple times
        let selection1 = queries.get_selection();
        let selection2 = queries.get_selection();
        let selection3 = queries.get_selection();
        
        // All queries return the same state
        assert_eq!(selection1.selected_entities.len(), 1);
        assert_eq!(selection2.selected_entities.len(), 1);
        assert_eq!(selection3.selected_entities.len(), 1);
        
        // Verify the entity ID is the same
        assert_eq!(selection1.selected_entities[0], entity_id);
        assert_eq!(selection2.selected_entities[0], entity_id);
        assert_eq!(selection3.selected_entities[0], entity_id);
    }

    #[test]
    fn test_get_audio_sources_returns_immutable_references() {
        let mut audio_state = AudioRegistryState::new();
        let source_id = AudioSourceId::new(Uuid::new_v4());
        let source = AudioSource::new(
            source_id.clone(),
            "Test Source".to_string(),
            AudioSourceType::Ambient,
        );
        audio_state.add_source(source);
        
        let queries = MockStateQueries {
            session_state: SessionState::new(),
            material_state: MaterialRegistryState::new(),
            audio_state,
            diagnostics_state: DiagnosticsState::new(),
            has_project: true,
        };
        
        // Get immutable references
        let sources = queries.get_audio_sources();
        assert_eq!(sources.len(), 1);
        
        let source = sources[0];
        assert_eq!(source.source_name, "Test Source");
        
        // The following would not compile (immutable reference):
        // source.source_name = "Modified".to_string();
    }
}
