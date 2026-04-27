//! Terrain Types
//!
//! Type definitions for terrain service.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BrushType {
    Raise,
    Lower,
    Smooth,
    Flatten,
}

/// Identifies a terrain chunk region.
///
/// **PHASE 6 REMEDIATED**: Now carries explicit chunk coordinates for accurate
/// affected region tracking instead of a raw u64 hash.
#[derive(Debug, Clone)]
pub struct RegionID {
    pub chunk_x: u32,
    pub chunk_y: u32,
    pub resolution: u32,
}

impl RegionID {
    pub fn new(chunk_x: u32, chunk_y: u32, resolution: u32) -> Self {
        Self {
            chunk_x,
            chunk_y,
            resolution,
        }
    }

    pub fn from_position(position: [f32; 3]) -> Self {
        let x = (position[0] / 64.0).floor() as u32;
        let z = (position[2] / 64.0).floor() as u32;
        Self {
            chunk_x: x,
            chunk_y: z,
            resolution: 64,
        }
    }

    pub fn all() -> Self {
        Self {
            chunk_x: u32::MAX,
            chunk_y: u32::MAX,
            resolution: u32::MAX,
        }
    }
}

#[derive(Debug, Clone)]
pub struct LayerConfiguration {
    pub layers: Vec<LayerDefinition>,
}

#[derive(Debug, Clone)]
pub struct LayerDefinition {
    pub material_id: String,
    pub blend_mode: BlendMode,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlendMode {
    Replace,
    Add,
    Multiply,
}

#[derive(Debug, Clone)]
pub enum TerrainChangeType {
    Sculpt,
    Paint,
    LayerConfig,
}

#[derive(Debug, Clone)]
pub struct TerrainChangedEvent {
    pub affected_regions: Vec<RegionID>,
    pub change_type: TerrainChangeType,
}
