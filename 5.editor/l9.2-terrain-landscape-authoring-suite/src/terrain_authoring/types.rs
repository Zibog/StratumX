//! Terrain Authoring Types
//!
//! Core types for terrain authoring operations.

use std::collections::HashMap;

/// Chunk identifier
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct ChunkId {
    pub x: u32,
    pub y: u32,
}

impl ChunkId {
    pub fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }
}

/// Layer identifier
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct LayerId(pub u16);

/// Layer profile (material family and properties)
#[derive(Debug, Clone)]
pub struct LayerProfile {
    pub material_family: String,
    pub density_kg_m3: f32,
}

/// Region for heightmap modification
#[derive(Debug, Clone)]
pub struct Region {
    pub min_x: f32,
    pub min_y: f32,
    pub max_x: f32,
    pub max_y: f32,
}

#[derive(Debug, Clone)]
pub enum SculptOperation {
    Raise {
        center: [f32; 2],
        radius: f32,
        strength: f32,
    },
    Lower {
        center: [f32; 2],
        radius: f32,
        strength: f32,
    },
    Smooth {
        center: [f32; 2],
        radius: f32,
        strength: f32,
    },
    Flatten {
        center: [f32; 2],
        radius: f32,
        strength: f32,
        target_height: f32,
    },
}

#[derive(Debug, Clone)]
pub struct PaintOperation {
    pub center: [f32; 2],
    pub radius: f32,
    pub layer_index: usize,
    pub strength: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HeightmapFormat {
    Raw,
    R16,
    Png,
    Tiff,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TerrainSyncReport {
    pub bound: bool,
    pub dirty_chunk_count: usize,
    pub gpu_sync_required: bool,
}

/// Layer bindings map
pub type LayerBindings = HashMap<LayerId, LayerProfile>;

/// Chunk dirtiness map
pub type ChunkDirtiness = HashMap<ChunkId, bool>;
