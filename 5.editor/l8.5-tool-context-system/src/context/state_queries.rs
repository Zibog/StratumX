//! Editor host read accessors for the active session.

use engine_world::WorldState;

use crate::context::session::EditorSession;

/// Trait for accessing terrain grid data from the editor host.
pub trait TerrainDataSource {
    fn terrain_resolution(&self) -> Option<[u32; 2]>;
    fn terrain_world_size(&self) -> Option<[f32; 2]>;
    fn terrain_chunk_grid(&self) -> Option<[u32; 2]>;
}

/// Read accessors for the active session.
pub struct StateQueries;

impl StateQueries {
    pub fn get_world_state(session: &Option<EditorSession>) -> Option<&WorldState> {
        session.as_ref().map(|s| &s.world)
    }

    pub fn get_world_state_mut(session: &mut Option<EditorSession>) -> Option<&mut WorldState> {
        session.as_mut().map(|s| &mut s.world)
    }

    pub fn world_label(session: &Option<EditorSession>) -> Option<&str> {
        session.as_ref().map(|s| s.world_label.as_str())
    }
}

/// Helper implementation of TerrainDataSource for session-based queries.
pub struct SessionTerrainSource;

impl TerrainDataSource for SessionTerrainSource {
    fn terrain_resolution(&self) -> Option<[u32; 2]> {
        None
    }

    fn terrain_world_size(&self) -> Option<[f32; 2]> {
        None
    }

    fn terrain_chunk_grid(&self) -> Option<[u32; 2]> {
        None
    }
}

/// Trait extension for session-based terrain data source.
pub trait SessionTerrainQueries {
    fn terrain_resolution(&self) -> Option<[u32; 2]>;
    fn terrain_world_size(&self) -> Option<[f32; 2]>;
    fn terrain_chunk_grid(&self) -> Option<[u32; 2]>;
}

impl SessionTerrainQueries for EditorSession {
    fn terrain_resolution(&self) -> Option<[u32; 2]> {
        self.world
            .vertical_slice_scene()
            .map(|scene| scene.terrain.resolution)
    }

    fn terrain_world_size(&self) -> Option<[f32; 2]> {
        self.world
            .vertical_slice_scene()
            .map(|scene| scene.terrain.world_size)
    }

    fn terrain_chunk_grid(&self) -> Option<[u32; 2]> {
        self.world
            .vertical_slice_scene()
            .map(|scene| scene.terrain.chunk_grid)
    }
}
