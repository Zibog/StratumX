use super::identity::EntityId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TerrainLayerMaterialState {
    pub layer_id: u16,
    pub material_family: String,
    pub albedo_texture_ref: Option<String>,
    pub normal_texture_ref: Option<String>,
    pub orm_texture_ref: Option<String>,
    pub uv_scale: [f32; 2],
    pub base_tint: [f32; 4],
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TerrainPatchState {
    pub entity_id: EntityId,
    pub origin: [f32; 3],
    pub world_size: [f32; 2],
    pub resolution: [u32; 2],
    pub chunk_grid: [u32; 2],
    pub chunk_size: u32,
    pub height_source_ref: Option<String>,
    pub height_samples: Vec<f32>,
    pub material_layer_ids: Vec<u16>,
    pub layer_weights: Vec<[f32; 4]>,
    pub layer_materials: Vec<TerrainLayerMaterialState>,
    pub hole_mask: Option<Vec<u8>>,
    pub chunks: Vec<TerrainChunk>,
    pub dirty_regions: Vec<TerrainDirtyRegion>,
    pub mesh_revision: u64,
    pub collision_revision: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TerrainChunk {
    pub chunk_x: u32,
    pub chunk_y: u32,
    pub data_file: Option<String>,
    pub loaded: bool,
    pub dirty: bool,
    pub mesh_built: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TerrainDirtyRegion {
    pub chunk_x: u32,
    pub chunk_y: u32,
    pub revision: u64,
}
