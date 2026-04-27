//! Terrain Service
//!
//! Manages terrain sculpt/paint/layer operations through canonical command spine routing.
//! Queries terrain from index plane and emits events with affected regions.
//!
//! **PHASE 6 REMEDIATED — Truth Boundary:**
//! TerrainService now reads actual terrain data through a `TerrainDataSource` trait
//! instead of computing fake affected regions.
//!
//! **Requirements: 12.3, 12.8, 29.2, 29.3**

use super::event_bus_trait::{EventBusInterface, ServiceEvent};
use super::terrain_affected_chunks::{compute_affected_chunks, compute_all_chunks};
use super::terrain_types::*;
use std::sync::Arc;

/// Trait for reading terrain data — the truth boundary between service and world state.
pub trait TerrainDataSource {
    fn terrain_resolution(&self) -> Option<[u32; 2]>;
    fn terrain_world_size(&self) -> Option<[f32; 2]>;
    fn terrain_chunk_grid(&self) -> Option<[u32; 2]>;
}

/// Terrain service - handles terrain manipulation operations.
pub struct TerrainService {
    event_bus: Option<Arc<dyn EventBusInterface>>,
}

impl TerrainService {
    pub fn new() -> Self {
        Self { event_bus: None }
    }

    pub fn new_with_event_bus<E>(event_bus: Arc<E>) -> Self
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

    /// Sculpt terrain at position.
    /// **Canonical Route:** route.terrain.sculpt.v1
    pub fn sculpt(
        &self,
        terrain_source: &dyn TerrainDataSource,
        position: [f32; 3],
        radius: f32,
        strength: f32,
        brush_type: BrushType,
    ) -> Result<TerrainChangedEvent, String> {
        if radius <= 0.0 {
            return Err("Sculpt radius must be positive".to_string());
        }
        if !(0.0..=1.0).contains(&strength) {
            return Err("Sculpt strength must be in range [0, 1]".to_string());
        }
        let affected_regions = compute_affected_chunks(terrain_source, position, radius);
        let event = TerrainChangedEvent {
            affected_regions,
            change_type: TerrainChangeType::Sculpt,
        };
        if let Some(ref bus) = self.event_bus {
            bus.emit_service_event(
                ServiceEvent::TerrainChanged {
                    tool: format!("{:?}", brush_type),
                    position: [position[0], position[2]],
                },
                "TerrainService",
            );
        }
        Ok(event)
    }

    /// Paint terrain at position.
    /// **Canonical Route:** route.terrain.paint.v1
    pub fn paint(
        &self,
        terrain_source: &dyn TerrainDataSource,
        position: [f32; 3],
        radius: f32,
        _layer_index: u32,
        strength: f32,
    ) -> Result<TerrainChangedEvent, String> {
        if radius <= 0.0 {
            return Err("Paint radius must be positive".to_string());
        }
        if !(0.0..=1.0).contains(&strength) {
            return Err("Paint strength must be in range [0, 1]".to_string());
        }
        let affected_regions = compute_affected_chunks(terrain_source, position, radius);
        let event = TerrainChangedEvent {
            affected_regions,
            change_type: TerrainChangeType::Paint,
        };
        if let Some(ref bus) = self.event_bus {
            bus.emit_service_event(
                ServiceEvent::TerrainChanged {
                    tool: "Paint".to_string(),
                    position: [position[0], position[2]],
                },
                "TerrainService",
            );
        }
        Ok(event)
    }

    /// Configure terrain layers.
    /// **Canonical Route:** route.terrain.configure_layers.v1
    pub fn configure_layers(
        &self,
        terrain_source: &dyn TerrainDataSource,
        layer_config: LayerConfiguration,
    ) -> Result<TerrainChangedEvent, String> {
        if layer_config.layers.is_empty() {
            return Err("Layer configuration must have at least one layer".to_string());
        }
        let affected_regions = compute_all_chunks(terrain_source);
        let event = TerrainChangedEvent {
            affected_regions,
            change_type: TerrainChangeType::LayerConfig,
        };
        if let Some(ref bus) = self.event_bus {
            bus.emit_service_event(
                ServiceEvent::TerrainChanged {
                    tool: "LayerConfig".to_string(),
                    position: [0.0, 0.0],
                },
                "TerrainService",
            );
        }
        Ok(event)
    }
}

impl Default for TerrainService {
    fn default() -> Self {
        Self::new()
    }
}
