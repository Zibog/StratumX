//! Runtime Control Service — play/pause/step operations.

pub struct RuntimeControlService {
    event_bus: Option<std::sync::Arc<dyn EventBusInterface>>,
}

impl RuntimeControlService {
    pub fn new() -> Self {
        Self { event_bus: None }
    }
    pub fn new_with_event_bus<E>(event_bus: std::sync::Arc<E>) -> Self
    where
        E: EventBusInterface + 'static,
    {
        Self {
            event_bus: Some(event_bus),
        }
    }
    pub fn initialize(&mut self) -> Result<(), String> {
        Ok(())
    }
    pub fn shutdown(&mut self) -> Result<(), String> {
        Ok(())
    }
    pub fn tick(&self, _delta_time: f32) -> Result<(), String> {
        Ok(())
    }

    pub fn play(
        &self,
        world_open: bool,
        runtime_available: bool,
    ) -> Result<RuntimeStateChangedEvent, String> {
        if !world_open {
            return Err("Cannot enter play mode: no world open".into());
        }
        if !runtime_available {
            return Err("Cannot enter play mode: runtime kernel unavailable".into());
        }
        if let Some(ref bus) = self.event_bus {
            bus.emit_service_event(
                ServiceEvent::RuntimeStateChanged {
                    mode: "Play".into(),
                },
                "RuntimeControlService",
            );
        }
        Ok(RuntimeStateChangedEvent {
            state: RuntimeState::Playing,
        })
    }

    pub fn pause(&self, runtime_active: bool) -> Result<RuntimeStateChangedEvent, String> {
        if !runtime_active {
            return Err("Cannot pause: runtime not active".into());
        }
        if let Some(ref bus) = self.event_bus {
            bus.emit_service_event(
                ServiceEvent::RuntimeStateChanged {
                    mode: "Paused".into(),
                },
                "RuntimeControlService",
            );
        }
        Ok(RuntimeStateChangedEvent {
            state: RuntimeState::Paused,
        })
    }

    pub fn step(&self, runtime_paused: bool) -> Result<RuntimeStateChangedEvent, String> {
        if !runtime_paused {
            return Err("Cannot step: runtime must be paused".into());
        }
        Ok(RuntimeStateChangedEvent {
            state: RuntimeState::Paused,
        })
    }
}

impl Default for RuntimeControlService {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RuntimeState {
    Stopped,
    Playing,
    Paused,
    Simulating,
}

#[derive(Debug, Clone)]
pub struct RuntimeStateChangedEvent {
    pub state: RuntimeState,
}

pub trait EventBusInterface: Send + Sync {
    fn emit_service_event(&self, event: ServiceEvent, source: &str);
}

#[derive(Debug, Clone)]
pub enum ServiceEvent {
    RuntimeStateChanged { mode: String },
}
