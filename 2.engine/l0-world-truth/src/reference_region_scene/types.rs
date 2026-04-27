use engine_material::{CombustibleMaterial, TerrainMaterialType};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TerrainPatch {
    pub position: [f32; 3],
    pub size: [f32; 2],
    pub material_type: TerrainMaterialType,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum StructureType {
    Tree,
    WoodenBuilding,
    BrickWall,
    Tunnel,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Structure {
    pub id: u64,
    pub structure_type: StructureType,
    pub position: [f32; 3],
    pub rotation: f32,
    pub destructible: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Container {
    pub id: u64,
    pub position: [f32; 3],
    pub capacity_liters: f32,
    pub height_m: f32,
    pub cross_section_m2: f32,
    pub has_leak: bool,
    pub leak_position: [f32; 3],
    pub leak_diameter_mm: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FireSource {
    pub id: u64,
    pub position: [f32; 3],
    pub material: CombustibleMaterial,
    pub ignited: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Door {
    pub id: u64,
    pub position: [f32; 3],
    pub handle_position: [f32; 3],
    pub opened: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WeatherConfig {
    pub storm_center: [f32; 2],
    pub storm_radius_km: f32,
    pub storm_intensity: f32,
    pub storm_velocity: [f32; 2],
    pub rainfall_intensity: f32,
}
