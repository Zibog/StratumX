use super::{CompatDomain, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum IngressControlKind {
    SetLabel {
        label: String,
    },
    SetField {
        key: String,
        value: String,
    },
    ClearField {
        key: String,
    },
    AddTag {
        tag: String,
    },
    RemoveTag {
        tag: String,
    },
    RetireObject,
    RestoreObject,
    RefreshSnapshot,
    WorldControl {
        domain: CompatDomain,
        payload: WorldControlPayload,
    },
    TerrainControl {
        payload: TerrainControlPayload,
    },
    MaterialControl {
        payload: MaterialControlPayload,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum WorldControlPayload {
    Open {
        world_id: String,
        mode: String,
    },
    Close {
        world_id: String,
    },
    Save {
        world_id: String,
    },
    Load {
        world_id: String,
        snapshot_id: String,
    },
    Bind {
        world_id: String,
        binding_id: String,
    },
    Unbind {
        world_id: String,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TerrainControlPayload {
    Deform {
        patch_id: String,
        delta: Vec<f32>,
    },
    Paint {
        patch_id: String,
        material_stack_id: String,
    },
    Query {
        patch_id: String,
    },
    List,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MaterialControlPayload {
    CreateArchetype {
        name: String,
        properties: Vec<(String, String)>,
    },
    CreateStack {
        archetype_id: String,
        layers: Vec<MaterialLayerSpec>,
    },
    Assign {
        entity_id: String,
        stack_id: String,
    },
    Query {
        archetype_id: Option<String>,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MaterialLayerSpec {
    pub archetype_id: String,
    pub thickness_mm: f32,
    pub coverage: f32,
}
