//! Terrain authoring service

use crate::runtime::state_types::WorldState;
use crate::runtime::EventBus;
use std::sync::{Arc, Mutex};

pub struct TerrainAuthoringService {
    _world_state: Arc<Mutex<WorldState>>,
    _event_bus: Arc<dyn EventBus>,
}

impl TerrainAuthoringService {
    pub fn new(world_state: Arc<Mutex<WorldState>>, event_bus: Arc<dyn EventBus>) -> Self {
        Self {
            _world_state: world_state,
            _event_bus: event_bus,
        }
    }
}
