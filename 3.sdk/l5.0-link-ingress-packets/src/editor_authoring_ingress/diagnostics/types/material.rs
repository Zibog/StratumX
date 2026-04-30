use super::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MaterialArchetypeInput {
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
pub struct MaterialLayerInput {
    pub archetype_id: u16,
    pub thickness_mm: f32,
    pub coverage: f32,
    pub bond_strength: f32,
    pub segmentation_mode: String,
    pub segment_size_mm: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MaterialCommand {
    CreateArchetype {
        input: MaterialArchetypeInput,
    },
    ListArchetypes,
    CreateStack {
        label: String,
    },
    StackAddLayer {
        stack_id: u16,
        layer: MaterialLayerInput,
    },
    StackRemoveLayer {
        stack_id: u16,
        layer_index: u8,
    },
    AssignStackToEntitySlot {
        entity_id: u32,
        slot_id: u8,
        stack_id: u16,
    },
    GetStack {
        stack_id: u16,
    },
    ListStacks,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MaterialWorldCommand {
    SetBarrelWater { liters: f32 },
    GetBarrelWater,
    SetBarrelLeak { active: bool },
    IgniteFireObject,
    ExtinguishFireObject,
    SetFireObjectWetness { wetness_percent: f32 },
    GetFireObjectState,
    GetSmokeParticleCount,
    UpdateMaterialWorld { delta_time: f32 },
}
