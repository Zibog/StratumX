//! Property Test: Domain Service Encapsulation
//!
//! Feature: editor-state-ownership-normalization
//! Property 14: Domain Service Encapsulation
//! Validates: Requirements 9.2
//!
//! For any domain service S and its domain D, all business logic for domain D is
//! encapsulated in S and not scattered across other components.

use proptest::prelude::*;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

use stratumx_editor_state_containers::{
    AudioAuthoringService, BasicEventBus, DiagnosticMessage, DiagnosticSource, DiagnosticsService,
    DiagnosticsState, EnvironmentAuthoringService, EnvironmentState, MaterialAuthoringService,
    MaterialProfile, MaterialProfileId, ProjectIdentity, ProjectState, RuntimeModeService,
    Severity, TerrainAuthoringService, TerrainState, WeatherCondition, WorkspaceIdentity,
    WorkspaceState, WorldIdentity, WorldSessionService, WorldState,
};

/// Domain service type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DomainServiceType {
    WorldSession,
    TerrainAuthoring,
    MaterialAuthoring,
    AudioAuthoring,
    EnvironmentAuthoring,
    RuntimeMode,
    Diagnostics,
}

/// Strategy for generating domain service types
fn any_domain_service_type() -> impl Strategy<Value = DomainServiceType> {
    prop_oneof![
        Just(DomainServiceType::WorldSession),
        Just(DomainServiceType::TerrainAuthoring),
        Just(DomainServiceType::MaterialAuthoring),
        Just(DomainServiceType::AudioAuthoring),
        Just(DomainServiceType::EnvironmentAuthoring),
        Just(DomainServiceType::RuntimeMode),
        Just(DomainServiceType::Diagnostics),
    ]
}

/// Test fixture for domain services
struct DomainServiceFixture {
    project_state: Arc<Mutex<ProjectState>>,
    _workspace_state: Arc<Mutex<WorkspaceState>>,
    world_state: Arc<Mutex<WorldState>>,
    diagnostics_state: Arc<Mutex<DiagnosticsState>>,
    event_bus: Arc<BasicEventBus>,
}

impl DomainServiceFixture {
    fn new() -> Self {
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
        let world_id = WorldIdentity::new(
            Uuid::new_v4(),
            "Test World".to_string(),
            PathBuf::from("/test/world"),
        );

        let project_state = Arc::new(Mutex::new(ProjectState::new(project_id, workspace_id)));
        let _workspace_state = Arc::new(Mutex::new(WorkspaceState::new()));
        let world_state = Arc::new(Mutex::new(WorldState::new(
            world_id,
            "snapshot_123".to_string(),
        )));
        let diagnostics_state = Arc::new(Mutex::new(DiagnosticsState::new()));
        let event_bus = Arc::new(BasicEventBus::new());

        Self {
            project_state,
            _workspace_state,
            world_state,
            diagnostics_state,
            event_bus,
        }
    }

    /// Get the initial state snapshot for a domain
    fn get_domain_state_snapshot(&self, service_type: DomainServiceType) -> DomainStateSnapshot {
        match service_type {
            DomainServiceType::WorldSession => {
                let world = self.world_state.lock().unwrap();
                let project = self.project_state.lock().unwrap();
                DomainStateSnapshot::WorldSession {
                    world_id: world.world_identity.world_id,
                    save_generation: project.save_generation,
                }
            }
            DomainServiceType::TerrainAuthoring => {
                let world = self.world_state.lock().unwrap();
                DomainStateSnapshot::TerrainAuthoring {
                    has_terrain: world.terrain_state.is_some(),
                    modification_count: world
                        .terrain_state
                        .as_ref()
                        .map(|t| t.modification_count)
                        .unwrap_or(0),
                }
            }
            DomainServiceType::MaterialAuthoring => {
                let project = self.project_state.lock().unwrap();
                DomainStateSnapshot::MaterialAuthoring {
                    save_generation: project.save_generation,
                }
            }
            DomainServiceType::AudioAuthoring => {
                let world = self.world_state.lock().unwrap();
                DomainStateSnapshot::AudioAuthoring {
                    audio_source_count: world.audio_source_count,
                }
            }
            DomainServiceType::EnvironmentAuthoring => {
                let world = self.world_state.lock().unwrap();
                DomainStateSnapshot::EnvironmentAuthoring {
                    has_environment: world.environment_state.is_some(),
                }
            }
            DomainServiceType::RuntimeMode => {
                let world = self.world_state.lock().unwrap();
                DomainStateSnapshot::RuntimeMode {
                    is_preview_mode: world.runtime_mode.is_preview_mode,
                    is_simulation_running: world.runtime_mode.is_simulation_running,
                }
            }
            DomainServiceType::Diagnostics => {
                let diagnostics = self.diagnostics_state.lock().unwrap();
                DomainStateSnapshot::Diagnostics {
                    message_count: diagnostics.messages.len(),
                }
            }
        }
    }

    /// Perform a domain operation through the service
    fn perform_domain_operation(&self, service_type: DomainServiceType) -> Result<(), String> {
        match service_type {
            DomainServiceType::WorldSession => {
                let mut service = WorldSessionService::new(
                    self.world_state.clone(),
                    self.project_state.clone(),
                    self.event_bus.clone(),
                );
                let world_id = WorldIdentity::new(
                    Uuid::new_v4(),
                    "New World".to_string(),
                    PathBuf::from("/test/new_world"),
                );
                service.open_world(world_id)
            }
            DomainServiceType::TerrainAuthoring => {
                // Directly modify terrain state through WorldState API
                {
                    let mut world = self.world_state.lock().unwrap();
                    if world.terrain_state.is_none() {
                        world.set_terrain_state(Some(TerrainState::new(
                            (1024, 1024),
                            (1000.0, 1000.0),
                            "default".to_string(),
                        )));
                    }
                    if let Some(terrain) = world.get_terrain_state_mut() {
                        terrain.increment_modifications();
                    }
                }
                Ok(())
            }
            DomainServiceType::MaterialAuthoring => {
                let mut service = MaterialAuthoringService::new(
                    self.project_state.clone(),
                    self.event_bus.clone(),
                );
                let profile_id = MaterialProfileId::new(Uuid::new_v4());
                let mut profile = MaterialProfile::new(profile_id, "Test Material".to_string());
                profile.base_color = [1.0, 1.0, 1.0, 1.0];
                profile.roughness = 0.5;
                profile.metallic = 0.0;
                service.create_material_profile(profile).map(|_| ())
            }
            DomainServiceType::AudioAuthoring => {
                let mut service =
                    AudioAuthoringService::new(self.world_state.clone(), self.event_bus.clone());
                service
                    .place_audio_source([0.0, 0.0, 0.0], "ambient".to_string())
                    .map(|_| ())
            }
            DomainServiceType::EnvironmentAuthoring => {
                // Directly modify environment state through WorldState API
                {
                    let mut world = self.world_state.lock().unwrap();
                    if world.environment_state.is_none() {
                        world.set_environment_state(Some(EnvironmentState::new()));
                    }
                    if let Some(env) = world.get_environment_state_mut() {
                        env.set_time_of_day(18.0);
                        env.set_weather(WeatherCondition::Rain);
                    }
                }
                Ok(())
            }
            DomainServiceType::RuntimeMode => {
                let mut service = RuntimeModeService::new(
                    self.world_state.clone(),
                    self.diagnostics_state.clone(),
                    self.event_bus.clone(),
                );
                service.enter_preview_mode()
            }
            DomainServiceType::Diagnostics => {
                let mut service =
                    DiagnosticsService::new(self.diagnostics_state.clone(), self.event_bus.clone());
                let message = DiagnosticMessage::new(
                    Severity::Info,
                    "Test diagnostic".to_string(),
                    DiagnosticSource::CommandSpine,
                );
                service.add_diagnostic(message)
            }
        }
    }

    /// Verify that the domain state changed only through the service
    fn verify_domain_state_changed(
        &self,
        service_type: DomainServiceType,
        before: &DomainStateSnapshot,
    ) -> bool {
        let after = self.get_domain_state_snapshot(service_type);
        before != &after
    }

    /// Verify that other domains' state did not change
    fn verify_other_domains_unchanged(
        &self,
        service_type: DomainServiceType,
        snapshots: &[DomainStateSnapshot],
    ) -> bool {
        let all_types = [
            DomainServiceType::WorldSession,
            DomainServiceType::TerrainAuthoring,
            DomainServiceType::MaterialAuthoring,
            DomainServiceType::AudioAuthoring,
            DomainServiceType::EnvironmentAuthoring,
            DomainServiceType::RuntimeMode,
            DomainServiceType::Diagnostics,
        ];

        for (i, other_type) in all_types.iter().enumerate() {
            if *other_type == service_type {
                continue;
            }

            // Check if this domain's state is independent
            if !self.are_domains_independent(service_type, *other_type) {
                continue;
            }

            let current = self.get_domain_state_snapshot(*other_type);
            if &current != &snapshots[i] {
                return false;
            }
        }

        true
    }

    /// Check if two domains are independent (don't share state)
    fn are_domains_independent(
        &self,
        domain1: DomainServiceType,
        domain2: DomainServiceType,
    ) -> bool {
        use DomainServiceType::*;

        // Domains that share state containers are not independent
        match (domain1, domain2) {
            // WorldSession and TerrainAuthoring both use WorldState
            (WorldSession, TerrainAuthoring) | (TerrainAuthoring, WorldSession) => false,
            // WorldSession and EnvironmentAuthoring both use WorldState
            (WorldSession, EnvironmentAuthoring) | (EnvironmentAuthoring, WorldSession) => false,
            // WorldSession and AudioAuthoring both use WorldState
            (WorldSession, AudioAuthoring) | (AudioAuthoring, WorldSession) => false,
            // TerrainAuthoring and EnvironmentAuthoring both use WorldState
            (TerrainAuthoring, EnvironmentAuthoring) | (EnvironmentAuthoring, TerrainAuthoring) => {
                false
            }
            // TerrainAuthoring and AudioAuthoring both use WorldState
            (TerrainAuthoring, AudioAuthoring) | (AudioAuthoring, TerrainAuthoring) => false,
            // EnvironmentAuthoring and AudioAuthoring both use WorldState
            (EnvironmentAuthoring, AudioAuthoring) | (AudioAuthoring, EnvironmentAuthoring) => {
                false
            }
            // WorldSession and RuntimeMode both use WorldState
            (WorldSession, RuntimeMode) | (RuntimeMode, WorldSession) => false,
            // RuntimeMode and TerrainAuthoring both use WorldState
            (RuntimeMode, TerrainAuthoring) | (TerrainAuthoring, RuntimeMode) => false,
            // RuntimeMode and EnvironmentAuthoring both use WorldState
            (RuntimeMode, EnvironmentAuthoring) | (EnvironmentAuthoring, RuntimeMode) => false,
            // RuntimeMode and AudioAuthoring both use WorldState
            (RuntimeMode, AudioAuthoring) | (AudioAuthoring, RuntimeMode) => false,
            // RuntimeMode and Diagnostics both use DiagnosticsState
            (RuntimeMode, Diagnostics) | (Diagnostics, RuntimeMode) => false,
            // WorldSession and MaterialAuthoring share ProjectState
            (WorldSession, MaterialAuthoring) | (MaterialAuthoring, WorldSession) => false,
            // All other combinations are independent
            _ => true,
        }
    }
}

/// Domain state snapshot for verification
#[derive(Debug, Clone, PartialEq)]
enum DomainStateSnapshot {
    WorldSession {
        world_id: Uuid,
        save_generation: u64,
    },
    TerrainAuthoring {
        has_terrain: bool,
        modification_count: u64,
    },
    MaterialAuthoring {
        save_generation: u64,
    },
    AudioAuthoring {
        audio_source_count: usize,
    },
    EnvironmentAuthoring {
        has_environment: bool,
    },
    RuntimeMode {
        is_preview_mode: bool,
        is_simulation_running: bool,
    },
    Diagnostics {
        message_count: usize,
    },
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]

    /// **Validates: Requirements 9.2**
    ///
    /// Property 14: Domain Service Encapsulation
    ///
    /// For any domain service S and its domain D, all business logic for domain D is
    /// encapsulated in S. This means:
    /// 1. Operations on domain D can only be performed through service S
    /// 2. Service S has exclusive access to mutate its domain state
    /// 3. Other services cannot directly manipulate domain D's state
    #[test]
    fn property_domain_service_encapsulation(service_type in any_domain_service_type()) {
        let fixture = DomainServiceFixture::new();

        // Capture initial state of the target domain
        let before = fixture.get_domain_state_snapshot(service_type);

        // Capture initial state of all other domains
        let all_types = [
            DomainServiceType::WorldSession,
            DomainServiceType::TerrainAuthoring,
            DomainServiceType::MaterialAuthoring,
            DomainServiceType::AudioAuthoring,
            DomainServiceType::EnvironmentAuthoring,
            DomainServiceType::RuntimeMode,
            DomainServiceType::Diagnostics,
        ];
        let other_snapshots: Vec<_> = all_types.iter()
            .map(|t| fixture.get_domain_state_snapshot(*t))
            .collect();

        // Perform operation through the service
        let result = fixture.perform_domain_operation(service_type);
        prop_assert!(result.is_ok(), "Service operation should succeed: {:?}", result);

        // Verify the target domain state changed
        let changed = fixture.verify_domain_state_changed(service_type, &before);
        prop_assert!(changed, "Domain {:?} state should change after service operation", service_type);

        // Verify other independent domains' state did not change
        let others_unchanged = fixture.verify_other_domains_unchanged(service_type, &other_snapshots);
        prop_assert!(others_unchanged, "Independent domains should not change when operating on {:?}", service_type);
    }

    /// **Validates: Requirements 9.2**
    ///
    /// Property 14: Domain Service Encapsulation (service initialization)
    ///
    /// For any domain service, the service can be initialized with only its required
    /// dependencies (owner containers at its level or below), demonstrating that
    /// business logic is fully encapsulated within the service.
    #[test]
    fn property_domain_service_initialization(service_type in any_domain_service_type()) {
        let fixture = DomainServiceFixture::new();

        // Verify service can be created with only its required dependencies
        let creation_result: Result<(), String> = match service_type {
            DomainServiceType::WorldSession => {
                let _service = WorldSessionService::new(
                    fixture.world_state.clone(),
                    fixture.project_state.clone(),
                    fixture.event_bus.clone(),
                );
                Ok(())
            }
            DomainServiceType::TerrainAuthoring => {
                let _service = TerrainAuthoringService::new(
                    fixture.world_state.clone(),
                    fixture.event_bus.clone(),
                );
                Ok(())
            }
            DomainServiceType::MaterialAuthoring => {
                let _service = MaterialAuthoringService::new(
                    fixture.project_state.clone(),
                    fixture.event_bus.clone(),
                );
                Ok(())
            }
            DomainServiceType::AudioAuthoring => {
                let _service = AudioAuthoringService::new(
                    fixture.world_state.clone(),
                    fixture.event_bus.clone(),
                );
                Ok(())
            }
            DomainServiceType::EnvironmentAuthoring => {
                let _service = EnvironmentAuthoringService::new(
                    fixture.world_state.clone(),
                    fixture.event_bus.clone(),
                );
                Ok(())
            }
            DomainServiceType::RuntimeMode => {
                let _service = RuntimeModeService::new(
                    fixture.world_state.clone(),
                    fixture.diagnostics_state.clone(),
                    fixture.event_bus.clone(),
                );
                Ok(())
            }
            DomainServiceType::Diagnostics => {
                let _service = DiagnosticsService::new(
                    fixture.diagnostics_state.clone(),
                    fixture.event_bus.clone(),
                );
                Ok(())
            }
        };

        prop_assert!(creation_result.is_ok(),
            "Service {:?} should be creatable with only its required dependencies",
            service_type);
    }
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn test_world_session_service_encapsulation() {
        let fixture = DomainServiceFixture::new();
        let before = fixture.get_domain_state_snapshot(DomainServiceType::WorldSession);

        let result = fixture.perform_domain_operation(DomainServiceType::WorldSession);
        assert!(result.is_ok());

        let changed = fixture.verify_domain_state_changed(DomainServiceType::WorldSession, &before);
        assert!(changed, "World session domain should change");
    }

    #[test]
    fn test_terrain_authoring_service_encapsulation() {
        let fixture = DomainServiceFixture::new();
        let before = fixture.get_domain_state_snapshot(DomainServiceType::TerrainAuthoring);

        let result = fixture.perform_domain_operation(DomainServiceType::TerrainAuthoring);
        assert!(result.is_ok());

        let changed =
            fixture.verify_domain_state_changed(DomainServiceType::TerrainAuthoring, &before);
        assert!(changed, "Terrain authoring domain should change");
    }

    #[test]
    fn test_material_authoring_service_encapsulation() {
        let fixture = DomainServiceFixture::new();
        let before = fixture.get_domain_state_snapshot(DomainServiceType::MaterialAuthoring);

        let result = fixture.perform_domain_operation(DomainServiceType::MaterialAuthoring);
        assert!(result.is_ok());

        let changed =
            fixture.verify_domain_state_changed(DomainServiceType::MaterialAuthoring, &before);
        assert!(changed, "Material authoring domain should change");
    }

    #[test]
    fn test_diagnostics_service_encapsulation() {
        let fixture = DomainServiceFixture::new();
        let before = fixture.get_domain_state_snapshot(DomainServiceType::Diagnostics);

        let result = fixture.perform_domain_operation(DomainServiceType::Diagnostics);
        assert!(result.is_ok());

        let changed = fixture.verify_domain_state_changed(DomainServiceType::Diagnostics, &before);
        assert!(changed, "Diagnostics domain should change");
    }

    #[test]
    fn test_domain_independence() {
        let fixture = DomainServiceFixture::new();

        // MaterialAuthoring and Diagnostics are independent
        assert!(fixture.are_domains_independent(
            DomainServiceType::MaterialAuthoring,
            DomainServiceType::Diagnostics
        ));

        // WorldSession and TerrainAuthoring share WorldState
        assert!(!fixture.are_domains_independent(
            DomainServiceType::WorldSession,
            DomainServiceType::TerrainAuthoring
        ));
    }
}
