use serde::{Deserialize, Serialize};

use super::material::MaterialSlotDto;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransformDto {
    pub position: [f32; 3],
    pub rotation: [f32; 4],
    pub scale: [f32; 3],
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EntityDto {
    pub entity_id: u32,
    pub label: String,
    pub entity_type: String,
    pub transform: TransformDto,
    pub asset_id: Option<u32>,
    pub material_slots: Vec<MaterialSlotDto>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthoringSceneDto {
    pub scene_name: String,
    pub entities: Vec<EntityDto>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EntityDetailsDto {
    pub entity_id: u32,
    pub label: String,
    pub entity_type: String,
    pub transform: TransformDto,
    pub asset_id: Option<u32>,
    pub material_slots: Vec<MaterialSlotDto>,
    pub components: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActorPresetDto {
    pub preset_id: u16,
    pub label: String,
    pub actor_type: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActorDto {
    pub actor_id: u32,
    pub preset_id: u16,
    pub transform: TransformDto,
    pub attached_weapon: Option<u16>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssetDto {
    pub asset_id: u32,
    pub label: String,
    pub asset_type: String,
    pub path: String,
}
