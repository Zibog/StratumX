pub use serde::{Deserialize, Serialize};
pub use serde_json;
pub use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::Path,
    sync::Arc,
};

pub use stratumx_tooling::{
    CommandOrigin, ObjectClass, ObjectHandle, ToolCommand, ToolingError, ToolingRuntime,
};
pub use stratumx_tooling_l6_0_tool_session as stratumx_tooling;

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorldAuthoringSuite {
    pub world_roots: Vec<ObjectHandle>,
}
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct SceneEntityAuthoringSuite {
    pub scene_entities: Vec<ObjectHandle>,
}
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct TerrainLandscapeAuthoringSuite {
    pub terrains: Vec<ObjectHandle>,
}
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct MaterialLookdevAuthoringSuite {
    pub materials: Vec<ObjectHandle>,
}
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct SimulationAiAuthoringSuite {
    pub logic_nodes: Vec<ObjectHandle>,
}
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct UiHudAuthoringSuite {
    pub widgets: Vec<ObjectHandle>,
}
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct QuestEventLogicAuthoringSuite {
    pub quest_nodes: Vec<ObjectHandle>,
}

pub use template_types::*;

#[derive(Debug, Clone, Default)]
pub struct EditorProduct {
    pub tooling: ToolingRuntime,
    pub world_suite: WorldAuthoringSuite,
    pub scene_suite: SceneEntityAuthoringSuite,
    pub terrain_suite: TerrainLandscapeAuthoringSuite,
    pub material_suite: MaterialLookdevAuthoringSuite,
    pub simulation_suite: SimulationAiAuthoringSuite,
    pub ui_suite: UiHudAuthoringSuite,
    pub quest_suite: QuestEventLogicAuthoringSuite,
    pub viewport_selected: Vec<ObjectHandle>,
}

impl EditorProduct {
    pub fn select_object(&mut self, handle: ObjectHandle) -> Result<(), ToolingError> {
        self.viewport_selected = vec![handle];
        Ok(())
    }
    pub fn refresh_from_tooling(&mut self) -> Result<(), ToolingError> {
        Ok(())
    }
}

mod object_factory_runtime;
mod template_catalog_runtime;
mod template_types;
