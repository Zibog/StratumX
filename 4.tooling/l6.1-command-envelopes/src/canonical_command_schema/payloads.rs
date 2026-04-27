use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CommandPayload {
    Empty,
    Project(ProjectPayload),
    World(WorldPayload),
    Import(ImportPayload),
    Terrain(TerrainPayload),
    Material(MaterialPayload),
    Environment(EnvironmentPayload),
    Audio(AudioPayload),
    Shell(ShellPayload),
    Diagnostics(DiagnosticsPayload),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct ProjectPayload {
    pub project_name: Option<String>,
    pub project_path: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct WorldPayload {
    pub world_path: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct ImportPayload {
    pub source_path: Option<String>,
    pub selected_source_token: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct TerrainPayload {
    pub position: Option<[f32; 2]>,
    pub brush_radius: Option<f32>,
    pub brush_strength: Option<f32>,
    pub target_layer: Option<u32>,
    pub target_height: Option<f32>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct MaterialPayload {
    pub material_id: Option<String>,
    pub profile_name: Option<String>,
    pub binding_ref: Option<String>,
    pub preview_trigger: Option<String>,
    pub proof_mode: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct EnvironmentPayload {
    pub profile_ref: Option<String>,
    pub scalar_value: Option<f32>,
    pub regime: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct AudioPayload {
    pub resource_id: Option<String>,
    pub profile_ref: Option<String>,
    pub preview_mode: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct ShellPayload {
    pub panel_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct DiagnosticsPayload {
    pub scope: Option<String>,
}

impl From<ProjectPayload> for CommandPayload {
    fn from(value: ProjectPayload) -> Self {
        Self::Project(value)
    }
}

impl From<WorldPayload> for CommandPayload {
    fn from(value: WorldPayload) -> Self {
        Self::World(value)
    }
}

impl From<ImportPayload> for CommandPayload {
    fn from(value: ImportPayload) -> Self {
        Self::Import(value)
    }
}

impl From<TerrainPayload> for CommandPayload {
    fn from(value: TerrainPayload) -> Self {
        Self::Terrain(value)
    }
}

impl From<MaterialPayload> for CommandPayload {
    fn from(value: MaterialPayload) -> Self {
        Self::Material(value)
    }
}

impl From<EnvironmentPayload> for CommandPayload {
    fn from(value: EnvironmentPayload) -> Self {
        Self::Environment(value)
    }
}

impl From<AudioPayload> for CommandPayload {
    fn from(value: AudioPayload) -> Self {
        Self::Audio(value)
    }
}

impl From<ShellPayload> for CommandPayload {
    fn from(value: ShellPayload) -> Self {
        Self::Shell(value)
    }
}

impl From<DiagnosticsPayload> for CommandPayload {
    fn from(value: DiagnosticsPayload) -> Self {
        Self::Diagnostics(value)
    }
}

impl From<()> for CommandPayload {
    fn from(_: ()) -> Self {
        Self::Empty
    }
}
