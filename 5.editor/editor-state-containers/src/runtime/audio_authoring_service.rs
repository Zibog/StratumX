//! Audio authoring service

use crate::*;

pub struct AudioAuthoringService {
    world_state: std::sync::Arc<std::sync::Mutex<WorldState>>,
    event_bus: std::sync::Arc<dyn EventBus>,
}

impl AudioAuthoringService {
    pub fn new(
        world_state: std::sync::Arc<std::sync::Mutex<WorldState>>,
        event_bus: std::sync::Arc<dyn EventBus>,
    ) -> Self {
        Self {
            world_state,
            event_bus,
        }
    }

    pub fn place_audio_source(
        &mut self,
        _position: [f32; 3],
        _label: String,
    ) -> Result<(), String> {
        self.world_state.lock().unwrap().audio_source_count += 1;
        self.event_bus.publish("audio.source.placed");
        Ok(())
    }
}
