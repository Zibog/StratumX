use super::terrain::TerrainMaterialType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CraterMorphology {
    pub center_position: [f32; 3],
    pub radius_m: f32,
    pub depth_m: f32,
    pub rim_height_m: f32,
    pub ejecta_radius_m: f32,
    pub ejecta_volume_m3: f32,
    pub debris_count: u32,
}

impl CraterMorphology {
    pub fn from_blast(
        position: [f32; 3],
        energy_j: f32,
        terrain_type: TerrainMaterialType,
    ) -> Self {
        let cohesion = terrain_type.cohesion_kpa();
        let ejecta_coeff = terrain_type.ejecta_coefficient();
        let base_radius = (energy_j / (cohesion * 1000.0)).powf(1.0 / 3.0);
        let radius_m = base_radius.clamp(0.1, 10.0);
        let depth_ratio = match terrain_type {
            TerrainMaterialType::Dirt | TerrainMaterialType::Mud => 0.3,
            TerrainMaterialType::Sand => 0.25,
            TerrainMaterialType::Gravel => 0.35,
            TerrainMaterialType::Asphalt => 0.15,
            TerrainMaterialType::Concrete => 0.1,
            TerrainMaterialType::Grass => 0.28,
            TerrainMaterialType::Snow => 0.4,
            TerrainMaterialType::Ice => 0.2,
        };
        let depth_m = radius_m * depth_ratio;
        let rim_height_m = depth_m * 0.2;
        let ejecta_radius_m = radius_m * (2.0 + ejecta_coeff);
        let crater_volume = std::f32::consts::PI * radius_m * radius_m * depth_m / 3.0;
        let ejecta_volume_m3 = crater_volume * ejecta_coeff;
        let debris_count = (ejecta_volume_m3 * 10.0) as u32;
        Self {
            center_position: position,
            radius_m,
            depth_m,
            rim_height_m,
            ejecta_radius_m,
            ejecta_volume_m3,
            debris_count,
        }
    }
}
