//! Editor Shell Library — main library file.

pub use serde::{Deserialize, Serialize};
pub use stratumx_tooling_l6_0_tool_session::{ObjectHandle, ToolingError, ToolingRuntime};
pub use stratumx_tooling_l6_1_command_envelopes::CommandLifecycleState;

// Split editor shell types into submodules
mod editor_shell_types;

// Re-export all types from editor_shell_types for backward compatibility
pub use editor_shell_types::{
    AnimationCinematicsAuthoringSuite, AssetGateAndApprovalSurface, AssistantSurface,
    AudioVoiceAuthoringSuite, AutomationAndBatchService, BuildReleaseSurface,
    BuildValidationReleaseSuite, CollaborationSessionSurface, ContentBrowserSystem,
    DestructionFractureAuthoringSuite, DiagnosticsSurface, GraphAuthoringService,
    ImportExportPipelineService, InspectorSystem, InteractionRoutingSystem,
    LearningOnboardingAndHelpSurface, MaterialLookdevAuthoringSuite, OutlinerSystem,
    OverlayAndGizmoSystem, PackageMarketAndDependencyService, PlaytestAndCaptureOperations,
    PluginAndExtensionHost, ProductionDashboardAndTraceability, ProjectBootstrapService,
    QuestEventLogicAuthoringSuite, ReviewAnnotationSurface, SceneEntityAuthoringSuite,
    ScriptAndHotReloadService, SimulationAiAuthoringSuite, TemplatePresetAndScaffoldService,
    TerrainLandscapeAuthoringSuite, ToolContextSystem, UiHudAuthoringSuite, ViewportSystem,
    WeatherEnvironmentAuthoringSuite, WorkspaceLayoutSystem, WorldAuthoringSuite,
};

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
