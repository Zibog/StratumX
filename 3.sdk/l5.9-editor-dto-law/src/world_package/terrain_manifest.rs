use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TerrainManifest {
    pub terrain_id: Uuid,
    pub origin: [f32; 3],
    pub world_size: [f32; 2],
    pub resolution: [u32; 2],
    pub chunk_grid: [u32; 2],
    pub chunk_size: u32,
    pub height_range: [f32; 2],
    pub material_layers: Vec<MaterialLayer>,
    pub chunks: Vec<ChunkDescriptor>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MaterialLayer {
    pub layer_id: u16,
    pub material_family: String,
    pub density_kg_m3: f32,

    pub albedo_texture_ref: Option<String>,
    pub normal_texture_ref: Option<String>,
    pub orm_texture_ref: Option<String>,

    pub uv_scale: [f32; 2],
    pub base_tint: [f32; 4],
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChunkDescriptor {
    pub chunk_x: u32,
    pub chunk_y: u32,
    pub data_file: String,
    pub revision: u64,
}
