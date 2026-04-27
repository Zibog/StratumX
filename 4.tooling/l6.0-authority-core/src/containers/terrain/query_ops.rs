// Terrain Authority Container - Query operations

use super::types::{BrushState, TerrainAuthorityContainer, TerrainChunk};

impl TerrainAuthorityContainer {
    /// Query terrain chunks (read-only)
    pub fn query_chunks(&self) -> &[TerrainChunk] {
        &self.registry.chunks
    }

    /// Get brush state
    pub fn brush_state(&self) -> &BrushState {
        &self.brush_state
    }

    /// Get mutable brush state (internal use)
    pub fn brush_state_mut(&mut self) -> &mut BrushState {
        &mut self.brush_state
    }
}
