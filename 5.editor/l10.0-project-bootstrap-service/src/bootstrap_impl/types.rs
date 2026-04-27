//! Editor Project Bootstrap Types

pub use serde::{Deserialize, Serialize};
pub use std::collections::{BTreeMap, BTreeSet};
pub use stratumx_tooling::{
    BuildArtifact, ObjectHandle, ReleasePackage, ToolObject, ToolingError, ToolingRuntime,
};
pub use stratumx_tooling_l6_0_tool_session as stratumx_tooling;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ViewportProjection {
    Perspective,
    Orthographic,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ViewportSystem {
    pub active_viewport: u32,
    pub projection: ViewportProjection,
    pub selected: Vec<ObjectHandle>,
    pub camera_bookmark: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OutlinerSystem {
    pub items: Vec<String>,
    pub expanded_labels: BTreeSet<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContentBrowserSystem {
    pub visible_assets: Vec<String>,
    pub active_path: String,
    pub search_query: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InspectorField {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InspectorSystem {
    pub selected: Option<ObjectHandle>,
    pub label: Option<String>,
    pub fields: Vec<InspectorField>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ToolMode {
    Select,
    Translate,
    Rotate,
    Scale,
    Paint,
    Script,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ToolContextSystem {
    pub active_mode: ToolMode,
    pub active_suite: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OverlayAndGizmoSystem {
    pub gizmos_enabled: bool,
    pub overlay_labels: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EditorPanel {
    Viewport,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceLayoutSystem {
    pub open_panels: Vec<EditorPanel>,
    pub focused_panel: EditorPanel,
}

pub fn anchored_panels() -> Vec<EditorPanel> {
    vec![EditorPanel::Viewport]
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InteractionRoutingSystem {
    pub command_palette_open: bool,
    pub pointer_captured: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AssistantSurface {
    pub goal_draft: String,
    pub staged_proposal: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiagnosticsSurface {
    pub diagnostics: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BuildReleaseSurface {
    pub last_build: Option<BuildArtifact>,
    pub last_release: Option<ReleasePackage>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorldAuthoringSuite {
    pub world_roots: Vec<ObjectHandle>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SceneEntityAuthoringSuite {
    pub scene_entities: Vec<ObjectHandle>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TerrainLandscapeAuthoringSuite {
    pub terrains: Vec<ObjectHandle>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MaterialLookdevAuthoringSuite {
    pub materials: Vec<ObjectHandle>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DestructionFractureAuthoringSuite {
    pub fracture_targets: Vec<ObjectHandle>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SimulationAiAuthoringSuite {
    pub logic_nodes: Vec<ObjectHandle>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WeatherEnvironmentAuthoringSuite {
    pub environment_nodes: Vec<ObjectHandle>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AnimationCinematicsAuthoringSuite {
    pub timelines: Vec<ObjectHandle>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AudioVoiceAuthoringSuite {
    pub audio_nodes: Vec<ObjectHandle>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UiHudAuthoringSuite {
    pub widgets: Vec<ObjectHandle>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QuestEventLogicAuthoringSuite {
    pub quest_nodes: Vec<ObjectHandle>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BuildValidationReleaseSuite {
    pub validation_runs: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImportExportPipelineService {
    pub import_queue_depth: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GraphAuthoringService {
    pub graph_count: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AutomationAndBatchService {
    pub active_batches: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScriptAndHotReloadService {
    pub reload_count: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PluginAndExtensionHost {
    pub registered_plugins: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TemplatePresetAndScaffoldService {
    pub template_count: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PackageMarketAndDependencyService {
    pub installed_packages: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CollaborationSessionSurface {
    pub reviewers: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewAnnotationSurface {
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AssetGateAndApprovalSurface {
    pub pending_assets: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlaytestAndCaptureOperations {
    pub last_capture_label: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProductionDashboardAndTraceability {
    pub counters: BTreeMap<String, u64>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LearningOnboardingAndHelpSurface {
    pub quick_start_steps: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EditorProjectDocument {
    pub title: String,
    pub project_name: String,
    pub viewport: ViewportSystem,
    pub tool_context: ToolContextSystem,
    pub workspace_layout: WorkspaceLayoutSystem,
    pub interaction_routing: InteractionRoutingSystem,
    pub plugin_host: PluginAndExtensionHost,
    pub package_service: PackageMarketAndDependencyService,
    pub collaboration_surface: CollaborationSessionSurface,
    pub review_surface: ReviewAnnotationSurface,
    pub approval_surface: AssetGateAndApprovalSurface,
    pub playtest_surface: PlaytestAndCaptureOperations,
    pub production_surface: ProductionDashboardAndTraceability,
    pub learning_surface: LearningOnboardingAndHelpSurface,
    pub assistant_surface: AssistantSurface,
    pub objects: Vec<ToolObject>,
}
