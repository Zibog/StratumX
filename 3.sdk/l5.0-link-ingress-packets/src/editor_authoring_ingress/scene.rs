use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Transform {
    pub position: [f32; 3],
    pub rotation: [f32; 4],
    pub scale: [f32; 3],
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            position: [0.0, 0.0, 0.0],
            rotation: [0.0, 0.0, 0.0, 1.0],
            scale: [1.0, 1.0, 1.0],
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SceneCommand {
    CreateEmpty {
        scene_name: String,
    },
    CreateEntityFromAsset {
        asset_id: u32,
        transform: Transform,
        label: String,
    },
    SetTransform {
        entity_id: u32,
        transform: Transform,
    },
    ListEntities,
    DeleteEntity {
        entity_id: u32,
    },
    GetEntityDetails {
        entity_id: u32,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ActorCommand {
    SpawnPreset {
        preset_id: u16,
        transform: Transform,
    },
    ListPresets,
    AttachWeapon {
        actor_id: u32,
        weapon_profile_id: u16,
    },
    SetActive {
        actor_id: u32,
    },
    GetActive,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AssetCommand {
    ImportStaticMesh { path: String, label: String },
    ImportTexture { path: String, label: String },
    List,
    GetDetails { asset_id: u32 },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BallisticsFireInput {
    pub actor_id: Option<u32>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BallisticsCommand {
    FireActiveActor { input: BallisticsFireInput },
}
