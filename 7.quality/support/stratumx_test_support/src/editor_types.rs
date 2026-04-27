use crate::bridge_types::ObjectHandle;
use crate::tooling_types::{BuildArtifact, ReleasePackage};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum EditorPanel {
    Viewport,
    Outliner,
    Inspector,
    ContentBrowser,
    Assistant,
    BuildRelease,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ToolMode {
    Select,
    Translate,
    Move,
    Rotate,
    Scale,
    Paint,
    Script,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ViewportState {
    pub selected: Vec<ObjectHandle>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OutlinerState {
    pub items: Vec<(ObjectHandle, String, Option<ObjectHandle>)>,
    pub expanded: Vec<ObjectHandle>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InspectorState {
    pub inspected: Option<ObjectHandle>,
    pub selected: Option<ObjectHandle>,
    pub label: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InteractionRoutingState {
    pub active_tool: Option<String>,
    pub command_palette_open: bool,
    pub pointer_captured: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OverlayAndGizmoState {
    pub gizmo_visible: bool,
    pub gizmos_enabled: bool,
    pub overlay_labels: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AssistantSurfaceState {
    pub active_goal: Option<String>,
    pub staged_proposal: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DiagnosticsSurfaceState {
    pub error_count: usize,
    pub diagnostics: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BuildSuiteState {
    pub last_build_success: bool,
    pub validation_runs: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BuildReleaseSurfaceState {
    pub release_channel: String,
    pub last_build: Option<BuildArtifact>,
    pub last_release: Option<ReleasePackage>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProductionSurfaceState {
    pub production_ready: bool,
    pub counters: BTreeMap<String, usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ContentBrowserState {
    pub current_path: String,
    pub visible_assets: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PlaytestSurfaceState {
    pub session_active: bool,
    pub last_capture_label: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProjectBootstrapService {
    pub active_project: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PluginHostState {
    pub loaded_plugins: Vec<String>,
    pub registered_plugins: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TemplateServiceState {
    pub available_templates: Vec<String>,
    pub template_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WorkspaceLayoutState {
    pub layout_name: String,
    pub open_panels: Vec<EditorPanel>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolContextState {
    pub active_mode: ToolMode,
    pub context_stack: Vec<String>,
}

impl Default for ToolContextState {
    fn default() -> Self {
        Self {
            active_mode: ToolMode::Select,
            context_stack: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WorldSuiteState {
    pub world_roots: Vec<ObjectHandle>,
    pub world_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SceneSuiteState {
    pub scene_entities: Vec<ObjectHandle>,
    pub scene_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MaterialSuiteState {
    pub material_count: usize,
    pub materials: Vec<ObjectHandle>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct QuestSuiteState {
    pub quest_count: usize,
    pub quest_nodes: Vec<ObjectHandle>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SimulationSuiteState {
    pub simulation_active: bool,
    pub logic_nodes: Vec<ObjectHandle>,
}
