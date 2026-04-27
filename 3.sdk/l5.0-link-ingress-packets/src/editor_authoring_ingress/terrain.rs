use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TerrainPatchInput {
    pub position: [f32; 3],
    pub size: [f32; 2],
    pub label: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SurfacePaintInput {
    pub center: [f32; 3],
    pub radius: f32,
    pub stack_id: u16,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TerrainCommand {
    CreatePatch {
        input: TerrainPatchInput,
    },
    ListPatches,
    PaintSurfaceStack {
        target_entity_id: u32,
        paint: SurfacePaintInput,
    },
    GetPatchDetails {
        patch_id: u32,
    },
}
