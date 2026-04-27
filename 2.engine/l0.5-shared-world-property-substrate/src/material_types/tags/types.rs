use super::enums::{
    AcousticSurfaceClass, BuoyancyResponse, FireResponse, FractureMode, MaterialBehaviorFlags,
    MechanicalClass, PropertyDomain, SegmentState, SegmentationMode, StructuralResponse,
};
use super::ids::{MaterialArchetypeId, MaterialId, MaterialStackId, ResponseProfileId};
use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MaterialArchetype {
    pub id: MaterialArchetypeId,
    pub label: String,
    pub behavior: MaterialBehaviorFlags,
    pub mechanical_class: MechanicalClass,
    pub fracture_mode: FractureMode,
    pub acoustic_class: AcousticSurfaceClass,
    pub fire_response: FireResponse,
    pub buoyancy: BuoyancyResponse,
    pub structural: StructuralResponse,
    pub segmentation: SegmentationMode,
    pub density_kg_m3: f32,
    pub hardness_mohs: f32,
    pub fracture_energy_j_m2: f32,
    pub bond_strength_mpa: f32,
    pub thickness_mm: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MaterialLayer {
    pub archetype_id: MaterialArchetypeId,
    pub thickness_mm: f32,
    pub coverage: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MaterialStack {
    pub id: MaterialStackId,
    pub label: String,
    pub layers: Vec<MaterialLayer>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LayerDamageState {
    pub layer_index: u8,
    pub integrity: f32,
    pub accumulated_energy_j: f32,
    pub segment_states: Vec<SegmentState>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DamageMemory {
    pub stack_id: MaterialStackId,
    pub layer_damage: Vec<LayerDamageState>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImpactResponseInput {
    pub entry_energy_j: f32,
    pub incidence_angle_deg: f32,
    pub projectile_diameter_mm: f32,
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LayerImpactResponse {
    pub layer_index: u8,
    pub energy_absorbed_j: f32,
    pub energy_transmitted_j: f32,
    pub cracked_segments: Vec<u16>,
    pub released_segments: Vec<u16>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImpactResponseSummary {
    pub layer_responses: Vec<LayerImpactResponse>,
    pub total_absorbed_j: f32,
    pub penetrated: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MaterialConfig {
    pub fallback_descriptor: MaterialDescriptor,
    pub default_reaction: ReactionRow,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MaterialDescriptor {
    pub material_id: MaterialId,
    pub label: String,
    pub property_domains: Vec<PropertyDomain>,
    pub response_profile: ResponseProfileId,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReactionRow {
    pub response_profile: ResponseProfileId,
    pub coefficients: [u16; 4],
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MaterialLookupResult {
    pub descriptor: MaterialDescriptor,
    pub reaction: ReactionRow,
    pub used_fallback: bool,
}
