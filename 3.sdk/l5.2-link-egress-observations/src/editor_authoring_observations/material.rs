use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MaterialSlotDto {
    pub slot_id: u8,
    pub slot_name: String,
    pub stack_id: Option<u16>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MaterialArchetypeDto {
    pub archetype_id: u16,
    pub label: String,
    pub mechanical_class: String,
    pub hardness_mohs: f32,
    pub brittleness: f32,
    pub density_kg_m3: f32,
    pub fracture_mode: String,
    pub penetration_resistance: f32,
    pub ricochet_bias: f32,
    pub debris_profile: String,
    pub dust_amount: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthoringMaterialLayerDto {
    pub archetype_id: u16,
    pub archetype_label: String,
    pub thickness_mm: f32,
    pub coverage: f32,
    pub bond_strength: f32,
    pub segmentation_mode: String,
    pub segment_size_mm: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthoringMaterialStackDto {
    pub stack_id: u16,
    pub label: String,
    pub layers: Vec<AuthoringMaterialLayerDto>,
    pub total_thickness_mm: f32,
}
