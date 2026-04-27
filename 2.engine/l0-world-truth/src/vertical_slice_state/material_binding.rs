use super::identity::{EntityId, MaterialStackId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EntityMaterialBinding {
    pub entity_id: EntityId,
    pub stack_id: MaterialStackId,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LayerDamageState {
    pub layer_index: u8,
    pub integrity: f32,
    pub accumulated_energy_j: f32,
    pub cracked_segments: Vec<u16>,
    pub released_segments: Vec<u16>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EntityDamageMemory {
    pub entity_id: EntityId,
    pub stack_id: MaterialStackId,
    pub layer_damage: Vec<LayerDamageState>,
}
