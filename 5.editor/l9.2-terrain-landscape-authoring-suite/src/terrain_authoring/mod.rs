// Terrain Authoring - модульная структура

pub mod dirty_regions;
pub mod flatten;
pub mod holes;
pub mod import;
pub mod paint;
pub mod sculpt;
pub mod service;
pub mod sync;
pub mod types;

pub use service::TerrainAuthoringService;
pub use types::*;

pub struct TerrainAuthoring {
    pub(crate) dirty_chunks: Vec<(u32, u32)>,
}

impl TerrainAuthoring {
    pub fn new() -> Self {
        Self {
            dirty_chunks: Vec::new(),
        }
    }

    pub fn get_dirty_chunks(&self) -> &[(u32, u32)] {
        &self.dirty_chunks
    }

    pub fn clear_dirty_chunks(&mut self) {
        self.dirty_chunks.clear();
    }

    pub fn mark_chunk_dirty(&mut self, chunk_x: u32, chunk_y: u32) {
        if !self.dirty_chunks.contains(&(chunk_x, chunk_y)) {
            self.dirty_chunks.push((chunk_x, chunk_y));
        }
    }

    pub fn sculpt_raise(
        &mut self,
        _world: &mut engine_world::WorldState,
        _center: [f32; 2],
        _radius: f32,
        _strength: f32,
    ) -> Result<(), String> {
        // Deferred: preserve the current no-op behavior until terrain sculpt raise is wired.
        Ok(())
    }

    pub fn sculpt_lower(
        &mut self,
        _world: &mut engine_world::WorldState,
        _center: [f32; 2],
        _radius: f32,
        _strength: f32,
    ) -> Result<(), String> {
        // Deferred: preserve the current no-op behavior until terrain sculpt lower is wired.
        Ok(())
    }

    pub fn sculpt_smooth(
        &mut self,
        _world: &mut engine_world::WorldState,
        _center: [f32; 2],
        _radius: f32,
        _strength: f32,
    ) -> Result<(), String> {
        // Deferred: preserve the current no-op behavior until terrain sculpt smooth is wired.
        Ok(())
    }

    pub fn paint_material(
        &mut self,
        _world: &mut engine_world::WorldState,
        _center: [f32; 2],
        _radius: f32,
        _layer_index: u16,
        _strength: f32,
    ) -> Result<(), String> {
        // Deferred: preserve the current no-op behavior until terrain material paint is wired.
        Ok(())
    }
}

impl Default for TerrainAuthoring {
    fn default() -> Self {
        Self::new()
    }
}
