use super::terrain::{AuthoringTerrainPatchDto, SurfaceRegionDto};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TerrainObservation {
    TerrainPatchCreated {
        patch: AuthoringTerrainPatchDto,
    },
    TerrainPatchList {
        patches: Vec<AuthoringTerrainPatchDto>,
    },
    SurfacePainted {
        target_entity_id: u32,
        region: SurfaceRegionDto,
    },
    TerrainPatchDetails {
        patch: AuthoringTerrainPatchDto,
    },
}
