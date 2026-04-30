use super::identity::{EntityId, MaterialStackId, WeaponProfileId};
use super::terrain_state::TerrainPatchState;
use engine_material::SkyWeatherState;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WallState {
    pub entity_id: EntityId,
    pub position: [f32; 3],
    pub dimensions: [f32; 3],
    pub stack_id: MaterialStackId,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WeaponState {
    pub entity_id: EntityId,
    pub profile_id: WeaponProfileId,
    pub position: [f32; 3],
    pub aim_direction: [f32; 3],
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CameraState {
    pub position: [f32; 3],
    pub look_at: [f32; 3],
    pub fov_deg: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProofRegionScene {
    pub scene_name: String,
    pub terrain: TerrainPatchState,
    pub wall: WallState,
    pub weapon: WeaponState,
    pub camera: CameraState,
    pub sky: SkyWeatherState,
    pub sky_bundle_path: Option<String>,
}
