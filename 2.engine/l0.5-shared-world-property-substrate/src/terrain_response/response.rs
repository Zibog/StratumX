use super::crater::CraterMorphology;
use super::debris::{generate_debris, DebrisParticle};
use super::terrain::TerrainMaterialType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TerrainBlastResponse {
    pub crater: CraterMorphology,
    pub debris: Vec<DebrisParticle>,
    pub dust_cloud_radius_m: f32,
}

impl TerrainBlastResponse {
    pub fn from_explosion(
        position: [f32; 3],
        energy_j: f32,
        terrain_type: TerrainMaterialType,
    ) -> Self {
        let crater = CraterMorphology::from_blast(position, energy_j, terrain_type);
        let debris = generate_debris(
            position,
            energy_j,
            terrain_type.density_kg_m3(),
            crater.debris_count,
            crater.radius_m,
            crater.ejecta_radius_m,
            crater.ejecta_volume_m3,
        );
        let dust_cloud_radius_m = crater.ejecta_radius_m * 1.5;
        Self {
            crater,
            debris,
            dust_cloud_radius_m,
        }
    }
}
