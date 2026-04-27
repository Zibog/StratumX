//! Command Type Definitions

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CommandId(Uuid);

impl CommandId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for CommandId {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ActionId(u64);

impl ActionId {
    pub fn from_command_type(ct: &CommandType) -> Self {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        let mut h = DefaultHasher::new();
        match ct {
            CommandType::SculptTerrain => "terrain.sculpt".hash(&mut h),
            CommandType::PaintTerrain => "terrain.paint".hash(&mut h),
            CommandType::ImportHeightmap => "terrain.import_heightmap".hash(&mut h),
            CommandType::CreateMaterial => "material.create".hash(&mut h),
            CommandType::AssignTexture => "material.assign_texture".hash(&mut h),
            CommandType::SelectEntity => "selection.select_entity".hash(&mut h),
            CommandType::DeselectAll => "selection.deselect_all".hash(&mut h),
            CommandType::TranslateEntity => "transform.translate".hash(&mut h),
            CommandType::RotateEntity => "transform.rotate".hash(&mut h),
            CommandType::ScaleEntity => "transform.scale".hash(&mut h),
            CommandType::SaveProject => "project.save".hash(&mut h),
            CommandType::LoadProject => "project.load".hash(&mut h),
            CommandType::Custom(n) => format!("custom.{}", n).hash(&mut h),
        };
        Self(h.finish())
    }

    pub fn as_u64(&self) -> u64 {
        self.0
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CommandType {
    SculptTerrain,
    PaintTerrain,
    ImportHeightmap,
    CreateMaterial,
    AssignTexture,
    SelectEntity,
    DeselectAll,
    TranslateEntity,
    RotateEntity,
    ScaleEntity,
    SaveProject,
    LoadProject,
    Custom(String),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommandParameters {
    params: HashMap<String, String>,
}

impl CommandParameters {
    pub fn new() -> Self {
        Self {
            params: HashMap::new(),
        }
    }

    pub fn with_param(mut self, k: impl Into<String>, v: impl Into<String>) -> Self {
        self.params.insert(k.into(), v.into());
        self
    }

    pub fn get(&self, k: &str) -> Option<&str> {
        self.params.get(k).map(|s| s.as_str())
    }
}

impl Default for CommandParameters {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Command {
    pub id: CommandId,
    pub command_type: CommandType,
    pub parameters: CommandParameters,
}

impl Command {
    pub fn new(ct: CommandType) -> Self {
        Self {
            id: CommandId::new(),
            command_type: ct,
            parameters: CommandParameters::new(),
        }
    }

    pub fn with_parameters(ct: CommandType, p: CommandParameters) -> Self {
        Self {
            id: CommandId::new(),
            command_type: ct,
            parameters: p,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UndoData {
    data: HashMap<String, String>,
}

impl UndoData {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }
}

impl Default for UndoData {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExecutedCommand {
    pub command: Command,
    pub result: CommandResult,
    pub undo_data: Option<UndoData>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommandResult {
    pub command_id: CommandId,
    pub success: bool,
    pub result_data: Option<String>,
}

impl CommandResult {
    pub fn success(cid: CommandId) -> Self {
        Self {
            command_id: cid,
            success: true,
            result_data: None,
        }
    }

    pub fn success_with_data(cid: CommandId, d: impl Into<String>) -> Self {
        Self {
            command_id: cid,
            success: true,
            result_data: Some(d.into()),
        }
    }

    pub fn failure(cid: CommandId) -> Self {
        Self {
            command_id: cid,
            success: false,
            result_data: None,
        }
    }
}
