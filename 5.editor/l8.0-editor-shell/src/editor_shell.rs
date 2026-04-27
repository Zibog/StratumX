//! Editor Shell Library — main library file.

pub use serde::{Deserialize, Serialize};
pub use stratumx_tooling_l6_0_tool_session::{ObjectHandle, ToolingError, ToolingRuntime};
pub use stratumx_tooling_l6_1_command_envelopes::CommandLifecycleState;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ViewportSystem {
    pub active_viewport: u32,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct OutlinerSystem {
    pub items: Vec<String>,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ContentBrowserSystem {
    pub visible_assets: Vec<String>,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct InspectorSystem {
    pub selected: Option<ObjectHandle>,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ToolContextSystem {
    pub active_mode: String,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct OverlayAndGizmoSystem {
    pub gizmos_enabled: bool,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct WorkspaceLayoutSystem {
    pub open_panels: Vec<String>,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct InteractionRoutingSystem {
    pub command_palette_open: bool,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct AssistantSurface {
    pub goal_draft: String,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct DiagnosticsSurface {
    pub diagnostics: Vec<String>,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct BuildReleaseSurface {
    pub last_build: Option<String>,
}
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
pub struct ProjectBootstrapService {
    pub template_count: u64,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ImportExportPipelineService {
    pub import_queue_depth: u64,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct GraphAuthoringService {
    pub graph_count: u64,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct AutomationAndBatchService {
    pub active_batches: u64,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ScriptAndHotReloadService {
    pub reload_count: u64,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct PluginAndExtensionHost {
    pub registered_plugins: Vec<String>,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct TemplatePresetAndScaffoldService {
    pub template_count: u64,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct PackageMarketAndDependencyService {
    pub installed_packages: Vec<String>,
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

pub mod command_palette_state;
pub mod docking_manager;
pub mod editor_product_model;
pub mod file_dialogs;
pub mod layout_persistence;
pub mod menu_tree;
pub mod overview_queries;
pub mod panel_catalog;
pub mod product_relay;
pub mod project_dialogs;
pub mod project_document;
pub mod project_state;
pub mod runtime_entry;
pub mod shell_core;
pub mod shell_runtime;
pub mod stage_strip;
pub mod status_bar;
pub mod suite_overview;

pub mod command_spine;
pub mod world_lifecycle;

pub mod panel_registry;
pub mod panel_trait;
pub mod product_shell;

pub use command_palette_state::{
    filter_command_palette, CommandPaletteEntry, CommandPaletteState, COMMAND_PALETTE_ENTRIES,
};
pub use docking_manager::DockingManager;
pub use file_dialogs::{show_file_dialog, FileDialogKind};
pub use layout_persistence::{LayoutState, PanelState};
pub use menu_tree::{MenuEntry, MenuSection, FILE_MENU, MENU_SECTIONS, PANEL_MENU, RUNTIME_MENU};
pub use panel_registry::{
    DockPosition, DockingConfig, PanelDefinition, PanelDependency, PanelGeometry, PanelLifecycle,
    PanelRegistry, SplitConfig, SplitDirection, WorkspaceState,
};
pub use panel_trait::{ActionContext, Panel, PanelEvent, PanelFactory, PanelId};
pub use product_relay::ProductRelay;
pub use product_shell::ProductShell;
pub use project_dialogs::{
    DialogValidation, NewProjectDialogState, OpenWorldDialogState, ProjectDialogKind,
    ProjectDialogsState, ValidationTone,
};
pub use shell_runtime::{default_generated_artifacts_root, ShellRuntime};
pub use stage_strip::{StageDefinition, StageStrip, WorkspaceStage, STAGE_DEFINITIONS};
pub use status_bar::GeneratedQualitySummary;
pub use world_lifecycle::WorldLifecycleManager;

#[cfg(feature = "desktop")]
pub mod shell_extensions;
