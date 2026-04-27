//! Terrain service affected-chunk computation helpers.
//!
//! Extracted from terrain_service.rs to keep files under 200 lines.

use super::terrain_service::TerrainDataSource;
use super::terrain_types::RegionID;

/// Compute which chunks are affected by a brush at a given position/radius.
pub fn compute_affected_chunks(
    source: &dyn TerrainDataSource,
    position: [f32; 3],
    radius: f32,
) -> Vec<RegionID> {
    let Some(chunk_grid) = source.terrain_chunk_grid() else {
        return vec![RegionID::all()];
    };
    let Some(world_size) = source.terrain_world_size() else {
        return vec![RegionID::all()];
    };
    let Some(resolution) = source.terrain_resolution() else {
        return vec![RegionID::all()];
    };

    let chunk_size_x = world_size[0] / chunk_grid[0] as f32;
    let chunk_size_y = world_size[1] / chunk_grid[1] as f32;
    let center_chunk_x = (position[0] / chunk_size_x).clamp(0.0, chunk_grid[0] as f32 - 1.0) as u32;
    let center_chunk_y = (position[1] / chunk_size_y).clamp(0.0, chunk_grid[1] as f32 - 1.0) as u32;
    let radius_chunks_x = (radius / chunk_size_x).ceil() as u32;
    let radius_chunks_y = (radius / chunk_size_y).ceil() as u32;

    let mut affected = Vec::new();
    let min_cx = center_chunk_x.saturating_sub(radius_chunks_x);
    let max_cx = (center_chunk_x + radius_chunks_x).min(chunk_grid[0] - 1);
    let min_cy = center_chunk_y.saturating_sub(radius_chunks_y);
    let max_cy = (center_chunk_y + radius_chunks_y).min(chunk_grid[1] - 1);

    for cy in min_cy..=max_cy {
        for cx in min_cx..=max_cx {
            affected.push(RegionID {
                chunk_x: cx,
                chunk_y: cy,
                resolution: resolution[0] / chunk_grid[0],
            });
        }
    }

    if affected.is_empty() {
        vec![RegionID::all()]
    } else {
        affected
    }
}

/// Compute all chunks for a full-layer configuration change.
pub fn compute_all_chunks(source: &dyn TerrainDataSource) -> Vec<RegionID> {
    let Some(chunk_grid) = source.terrain_chunk_grid() else {
        return vec![RegionID::all()];
    };
    let Some(resolution) = source.terrain_resolution() else {
        return vec![RegionID::all()];
    };
    (0..chunk_grid[1])
        .flat_map(|cy| {
            (0..chunk_grid[0]).map(move |cx| RegionID {
                chunk_x: cx,
                chunk_y: cy,
                resolution: resolution[0] / chunk_grid[0],
            })
        })
        .collect()
}
