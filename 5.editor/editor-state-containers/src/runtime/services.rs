//! Editor services

use crate::*;

pub struct EditorServices {
    pub world_session: WorldSessionService,
    pub terrain_authoring: TerrainAuthoringService,
    pub material_authoring: MaterialAuthoringService,
    pub audio_authoring: AudioAuthoringService,
    pub environment_authoring: EnvironmentAuthoringService,
    pub runtime_mode: RuntimeModeService,
    pub diagnostics: DiagnosticsService,
}

impl EditorServices {
    pub fn new(
        project_state: std::sync::Arc<std::sync::Mutex<ProjectState>>,
        _workspace_state: std::sync::Arc<std::sync::Mutex<WorkspaceState>>,
        world_state: Option<std::sync::Arc<std::sync::Mutex<WorldState>>>,
        diagnostics_state: std::sync::Arc<std::sync::Mutex<DiagnosticsState>>,
        event_bus: std::sync::Arc<dyn EventBus>,
    ) -> Result<Self, String> {
        let world_state = world_state.ok_or("World state required")?;

        Ok(Self {
            world_session: WorldSessionService::new(
                world_state.clone(),
                project_state.clone(),
                event_bus.clone(),
            ),
            terrain_authoring: TerrainAuthoringService::new(world_state.clone(), event_bus.clone()),
            material_authoring: MaterialAuthoringService::new(
                project_state.clone(),
                event_bus.clone(),
            ),
            audio_authoring: AudioAuthoringService::new(world_state.clone(), event_bus.clone()),
            environment_authoring: EnvironmentAuthoringService::new(
                world_state.clone(),
                event_bus.clone(),
            ),
            runtime_mode: RuntimeModeService::new(
                world_state.clone(),
                diagnostics_state.clone(),
                event_bus.clone(),
            ),
            diagnostics: DiagnosticsService::new(diagnostics_state.clone(), event_bus.clone()),
        })
    }
}

pub struct EditorHost {
    pub state_system: std::sync::Arc<StateContainerSystem>,
    pub query_layer: QueryLayer,
    pub cache_layer: std::sync::Arc<std::sync::Mutex<CacheLayer>>,
    pub services: EditorServices,
    initialized: bool,
}

impl EditorHost {
    pub fn new(
        state_system: std::sync::Arc<StateContainerSystem>,
        query_layer: QueryLayer,
        cache_layer: std::sync::Arc<std::sync::Mutex<CacheLayer>>,
        services: EditorServices,
    ) -> Self {
        Self {
            state_system,
            query_layer,
            cache_layer,
            services,
            initialized: false,
        }
    }

    pub fn initialize(&mut self) -> Result<(), String> {
        if self.initialized {
            return Err("EditorHost already initialized".to_string());
        }
        self.initialized = true;
        Ok(())
    }

    pub fn shutdown(&mut self) -> Result<(), String> {
        if !self.initialized {
            return Err("EditorHost not initialized".to_string());
        }
        self.initialized = false;
        Ok(())
    }

    pub fn is_initialized(&self) -> bool {
        self.initialized
    }

    pub fn validate_state_ownership(&self) -> Result<(), String> {
        self.state_system
            .validate_ownership_uniqueness()
            .map_err(|violation| violation.to_user_message())
    }

    pub fn query(&self) -> &QueryLayer {
        &self.query_layer
    }

    pub fn services(&self) -> &EditorServices {
        &self.services
    }

    pub fn services_mut(&mut self) -> &mut EditorServices {
        &mut self.services
    }
}

pub trait EventBus: Send + Sync {
    fn publish(&self, event: &str);
    fn subscribe(&self, event_type: &str, callback: Box<dyn Fn(&str) + Send + Sync>);
}

type EventSubscriber = Box<dyn Fn(&str) + Send + Sync>;
type EventSubscribers = std::collections::HashMap<String, Vec<EventSubscriber>>;

#[derive(Default)]
pub struct BasicEventBus {
    // The event bus is shared through Arc<dyn EventBus>, so subscription updates
    // must be synchronized inside the concrete implementation.
    subscribers: std::sync::Mutex<EventSubscribers>,
}

impl BasicEventBus {
    pub fn new() -> Self {
        Self {
            subscribers: std::sync::Mutex::new(std::collections::HashMap::new()),
        }
    }
}

impl EventBus for BasicEventBus {
    fn publish(&self, event: &str) {
        if let Some(subscribers) = self
            .subscribers
            .lock()
            .expect("event bus subscribers lock poisoned")
            .get(event)
        {
            for callback in subscribers {
                callback(event);
            }
        }
    }

    fn subscribe(&self, event_type: &str, callback: Box<dyn Fn(&str) + Send + Sync>) {
        self.subscribers
            .lock()
            .expect("event bus subscribers lock poisoned")
            .entry(event_type.to_string())
            .or_default()
            .push(callback);
    }
}
