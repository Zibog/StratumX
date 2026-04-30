//! Editor shell panel and surface types.

use serde::{Deserialize, Serialize};
use stratumx_tooling_l6_0_tool_session::ObjectHandle;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct WorldAuthoringSuite {
    pub world_roots: Vec<ObjectHandle>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct SceneEntityAuthoringSuite {
    pub scene_entities: Vec<ObjectHandle>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct TerrainLandscapeAuthoringSuite {
    pub terrains: Vec<ObjectHandle>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct MaterialLookdevAuthoringSuite {
    pub materials: Vec<ObjectHandle>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct DestructionFractureAuthoringSuite {
    pub fracture_targets: Vec<ObjectHandle>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct SimulationAiAuthoringSuite {
    pub logic_nodes: Vec<ObjectHandle>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct WeatherEnvironmentAuthoringSuite {
    pub environment_nodes: Vec<ObjectHandle>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct AnimationCinematicsAuthoringSuite {
    pub timelines: Vec<ObjectHandle>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct AudioVoiceAuthoringSuite {
    pub audio_nodes: Vec<ObjectHandle>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct UiHudAuthoringSuite {
    pub widgets: Vec<ObjectHandle>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct QuestEventLogicAuthoringSuite {
    pub quest_nodes: Vec<ObjectHandle>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct BuildValidationReleaseSuite {
    pub validation_runs: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct CollaborationSessionSurface {
    pub reviewers: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ReviewAnnotationSurface {
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct AssetGateAndApprovalSurface {
    pub pending_assets: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct PlaytestAndCaptureOperations {
    pub last_capture_label: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ProductionDashboardAndTraceability {
    pub counters: std::collections::BTreeMap<String, u64>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct LearningOnboardingAndHelpSurface {
    pub quick_start_steps: Vec<String>,
}
