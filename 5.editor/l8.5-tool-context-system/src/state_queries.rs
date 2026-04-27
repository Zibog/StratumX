//! State Query Interface
//!
//! Provides read-only query interfaces for UI components to access state
//! without direct mutation capabilities.

use crate::{
    AudioSource, DiagnosticMessage, MaterialProfile, PanelId, SelectionState, WorldIdentity,
};

/// State query interface for read-only state access
///
/// UI components use this trait to access state without mutation capabilities.
/// All methods return immutable references or copied values.
pub trait StateQueries {
    /// Gets the active world identity
    fn get_active_world(&self) -> Option<&WorldIdentity>;

    /// Gets the current selection state
    fn get_selection(&self) -> &SelectionState;

    /// Gets the currently focused panel ID
    fn get_focused_panel(&self) -> Option<&PanelId>;

    /// Gets the list of open panel IDs
    fn get_open_panels(&self) -> &[PanelId];

    /// Gets all material profiles
    fn get_material_profiles(&self) -> Vec<&MaterialProfile>;

    /// Gets all audio sources
    fn get_audio_sources(&self) -> Vec<&AudioSource>;

    /// Gets all diagnostic messages
    fn get_diagnostics(&self) -> &[DiagnosticMessage];

    /// Returns true if a project is open
    fn has_project(&self) -> bool;

    /// Returns true if a world is open
    fn has_active_world(&self) -> bool;

    /// Returns true if any entities are selected
    fn has_selection(&self) -> bool;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{AudioRegistryState, DiagnosticsState, MaterialRegistryState, SessionState};
    use std::path::PathBuf;
    use uuid::Uuid;

    // Mock implementation for testing
    struct MockStateQueries {
        session_state: SessionState,
        material_state: MaterialRegistryState,
        audio_state: AudioRegistryState,
        diagnostics_state: DiagnosticsState,
        has_project: bool,
    }

    impl MockStateQueries {
        fn new() -> Self {
            Self {
                session_state: SessionState::new(),
                material_state: MaterialRegistryState::new(),
                audio_state: AudioRegistryState::new(),
                diagnostics_state: DiagnosticsState::new(),
                has_project: false,
            }
        }
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

    #[test]
    fn test_state_queries_trait() {
        let queries = MockStateQueries::new();

        assert!(!queries.has_project());
        assert!(!queries.has_active_world());
        assert!(!queries.has_selection());
        assert!(queries.get_active_world().is_none());
        assert!(queries.get_focused_panel().is_none());
        assert!(queries.get_open_panels().is_empty());
        assert!(queries.get_material_profiles().is_empty());
        assert!(queries.get_audio_sources().is_empty());
        assert!(queries.get_diagnostics().is_empty());
    }

    #[test]
    fn test_state_queries_with_data() {
        let mut queries = MockStateQueries::new();
        queries.has_project = true;

        // Add a world
        let world_id = WorldIdentity::new(
            Uuid::new_v4(),
            "Test World".to_string(),
            PathBuf::from("/test/world"),
        );
        queries.session_state.set_active_world(Some(world_id));

        // Add selection
        queries
            .session_state
            .selection_state
            .add_entity(Uuid::new_v4());

        assert!(queries.has_project());
        assert!(queries.has_active_world());
        assert!(queries.has_selection());
        assert!(queries.get_active_world().is_some());
    }

    #[test]
    fn test_get_audio_sources_returns_immutable_references() {
        use crate::{AudioSourceId, AudioSourceType};

        let mut queries = MockStateQueries::new();

        // Add audio sources
        let source_id = AudioSourceId::new(Uuid::new_v4());
        let source = AudioSource::new(
            source_id.clone(),
            "Test Source".to_string(),
            AudioSourceType::Ambient,
        );
        queries.audio_state.add_source(source);

        // Get immutable references
        let sources = queries.get_audio_sources();
        assert_eq!(sources.len(), 1);

        let source = sources[0];
        assert_eq!(source.source_name, "Test Source");

        // The following would not compile (immutable reference):
        // source.source_name = "Modified".to_string();
    }
}
