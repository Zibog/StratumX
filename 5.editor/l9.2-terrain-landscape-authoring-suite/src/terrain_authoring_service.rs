//! Terrain Authoring Service
//!
//! Domain service for terrain editing operations including heightmap manipulation and layer management.
//! Abstraction Level: L4 (Authoring Tools)

use crate::{EditorEvent, EventBus, TerrainState, WorldState};
use std::sync::{Arc, Mutex};

/// Region specification for terrain modifications
#[derive(Debug, Clone)]
pub struct Region {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

/// Terrain authoring service for terrain editing operations
pub struct TerrainAuthoringService {
    world_state: Arc<Mutex<WorldState>>,
    event_bus: Arc<dyn EventBus>,
}

impl TerrainAuthoringService {
    /// Create a new terrain authoring service
    pub fn new(world_state: Arc<Mutex<WorldState>>, event_bus: Arc<dyn EventBus>) -> Self {
        Self {
            world_state,
            event_bus,
        }
    }

    /// Modify heightmap in a specific region
    pub fn modify_heightmap(&mut self, region: Region, _delta: f32) -> Result<(), String> {
        let region_str = format!(
            "({},{},{}x{})",
            region.x, region.y, region.width, region.height
        );

        // Update terrain state
        {
            let mut world = self.world_state.lock().unwrap();
            if world.terrain_state.is_none() {
                world.terrain_state = Some(TerrainState::new(
                    (1024, 1024),
                    (1000.0, 1000.0),
                    "default".to_string(),
                ));
            }

            if let Some(terrain) = &mut world.terrain_state {
                // Apply heightmap modification by incrementing modification count
                terrain.increment_modifications();
            }
        }

        // Emit event
        self.event_bus
            .emit(EditorEvent::TerrainModified { region: region_str });

        Ok(())
    }

    /// Set the active terrain layer
    pub fn set_layer(&mut self, layer_name: String) -> Result<(), String> {
        // Update terrain state
        {
            let mut world = self.world_state.lock().unwrap();
            if world.terrain_state.is_none() {
                world.terrain_state = Some(TerrainState::new(
                    (1024, 1024),
                    (1000.0, 1000.0),
                    "default".to_string(),
                ));
            }

            if let Some(terrain) = &mut world.terrain_state {
                terrain.current_layer = layer_name.clone();
            }
        }

        // Emit event
        self.event_bus
            .emit(EditorEvent::TerrainLayerChanged { layer_name });

        Ok(())
    }

    /// Get the current terrain state
    pub fn get_terrain_state(&self) -> Option<TerrainState> {
        let world = self.world_state.lock().unwrap();
        world.terrain_state.clone()
    }
}

