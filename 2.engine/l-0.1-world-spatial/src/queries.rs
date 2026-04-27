//! Query interfaces for spatial analysis.

use crate::types::*;

/// Query interface for spatial address analysis and transformation.
pub struct SpatialQuery;

impl SpatialQuery {
    /// Check if two chunks are in the same region.
    pub fn same_region(a: ChunkAddress, b: ChunkAddress) -> bool {
        a.region == b.region
    }

    /// Get the region containing a chunk.
    pub fn chunk_region(chunk: ChunkAddress) -> RegionAddress {
        chunk.region
    }

    /// Get the slab layer of a chunk.
    pub fn chunk_slab(chunk: ChunkAddress) -> i32 {
        chunk.region.slab_z
    }

    /// Check if chunks share the same slab.
    pub fn same_slab(a: ChunkAddress, b: ChunkAddress) -> bool {
        a.region.slab_z == b.region.slab_z
    }

    /// Get grid position of chunk within its region.
    pub fn chunk_grid_position(chunk: ChunkAddress) -> (i32, i32) {
        let region_min_x = chunk.region.x * REGION_EDGE_CHUNKS;
        let region_min_y = chunk.region.y * REGION_EDGE_CHUNKS;
        (chunk.chunk_x - region_min_x, chunk.chunk_y - region_min_y)
    }

    /// Check if coordinate is valid for transformations.
    pub fn is_valid_coordinate(coord: WorldCoordinate) -> bool {
        coord.meters.x.is_finite() && coord.meters.y.is_finite() && coord.meters.z.is_finite()
    }

    /// Calculate Manhattan distance between two chunks (ignoring z).
    pub fn horizontal_distance(a: ChunkAddress, b: ChunkAddress) -> i32 {
        (a.chunk_x - b.chunk_x).abs() + (a.chunk_y - b.chunk_y).abs()
    }

    /// Calculate Chebyshev distance between two chunks (grid-based).
    pub fn chebyshev_distance(a: ChunkAddress, b: ChunkAddress) -> i32 {
        ((a.chunk_x - b.chunk_x).abs()).max((a.chunk_y - b.chunk_y).abs())
    }
}
