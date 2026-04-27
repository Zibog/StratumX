//! Chunk fixture for testing

use engine_world::TerrainChunk;

/// Creates a test terrain chunk
pub fn create_test_chunk() -> TerrainChunk {
    TerrainChunk {
        chunk_x: 0,
        chunk_y: 0,
        data_file: None,
        loaded: false,
        dirty: false,
        mesh_built: false,
    }
}

/// Creates a chunk at specific coordinates
pub fn create_chunk_at(x: u32, y: u32) -> TerrainChunk {
    TerrainChunk {
        chunk_x: x,
        chunk_y: y,
        data_file: None,
        loaded: false,
        dirty: false,
        mesh_built: false,
    }
}
