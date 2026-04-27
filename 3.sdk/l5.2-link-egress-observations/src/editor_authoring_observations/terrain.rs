use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthoringTerrainPatchDto {
    pub patch_id: u32,
    pub label: String,
    pub position: [f32; 3],
    pub size: [f32; 2],
    pub surface_regions: Vec<SurfaceRegionDto>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SurfaceRegionDto {
    pub region_id: u32,
    pub center: [f32; 3],
    pub radius: f32,
    pub stack_id: u16,
}
