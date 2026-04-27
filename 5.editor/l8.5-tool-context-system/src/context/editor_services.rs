//! Editor Services — domain service container.

use std::sync::Arc;

#[derive(Debug, Clone)]
pub enum ServiceEvent {
    WorldOpened { world_id: String },
    WorldClosed,
    WorldSaved,
    RuntimeStateChanged { mode: String },
    TerrainChanged { tool: String, position: [f32; 2] },
    EnvironmentChanged { scope: String, change_type: String },
    DiagnosticsUpdated { scope: String, change_type: String },
    ViewportChanged { scope: String, change_type: String },
}

pub trait EventBusInterface: Send + Sync {
    fn emit_service_event(&self, event: ServiceEvent, source: &str) -> bool;
}

macro_rules! define_service {
    ($name:ident) => {
        pub struct $name { _event_bus: Option<Arc<dyn EventBusInterface>> }
        impl $name {
            pub fn new() -> Self { Self { _event_bus: None } }
            pub fn new_with_event_bus<E>(event_bus: Arc<E>) -> Self where E: EventBusInterface + 'static { Self { _event_bus: Some(event_bus) } }
            pub fn initialize(&mut self) -> Result<(), String> { Ok(()) }
            pub fn shutdown(&mut self) -> Result<(), String> { Ok(()) }
        }
        impl Default for $name { fn default() -> Self { Self::new() } }
    };
}

define_service!(WorldLifecycleService);
define_service!(RuntimeControlService);
define_service!(TerrainService);
define_service!(EnvironmentService);
define_service!(DiagnosticsService);
define_service!(ViewportService);

pub struct EditorServices {
    pub world_lifecycle: WorldLifecycleService,
    pub runtime_control: RuntimeControlService,
    pub terrain: TerrainService,
    pub environment: EnvironmentService,
    pub diagnostics: DiagnosticsService,
    pub viewport: ViewportService,
}

impl EditorServices {
    pub fn initialize_with_event_bus<E>(event_bus: Arc<E>) -> Self where E: EventBusInterface + 'static {
        Self {
            world_lifecycle: WorldLifecycleService::new_with_event_bus(event_bus.clone()),
            runtime_control: RuntimeControlService::new_with_event_bus(event_bus.clone()),
            terrain: TerrainService::new_with_event_bus(event_bus.clone()),
            environment: EnvironmentService::new_with_event_bus(event_bus.clone()),
            diagnostics: DiagnosticsService::new_with_event_bus(event_bus.clone()),
            viewport: ViewportService::new_with_event_bus(event_bus),
        }
    }
    pub fn initialize() -> Self {
        Self {
            world_lifecycle: WorldLifecycleService::new(),
            runtime_control: RuntimeControlService::new(),
            terrain: TerrainService::new(),
            environment: EnvironmentService::new(),
            diagnostics: DiagnosticsService::new(),
            viewport: ViewportService::new(),
        }
    }
}

impl Default for EditorServices { fn default() -> Self { Self::initialize() } }
