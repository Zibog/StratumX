//! Editor Shell Library — main library file.

pub mod commands;
pub mod diagnostics;
pub mod layout;
pub mod panels;
pub mod state;

// Re-export all public items to preserve API compatibility
pub use commands::{
    AutomationAndBatchService, GraphAuthoringService, ImportExportPipelineService,
    PackageMarketAndDependencyService, PluginAndExtensionHost, ProjectBootstrapService,
    ScriptAndHotReloadService, TemplatePresetAndScaffoldService,
};
pub use layout::WorkspaceLayoutSystem;
pub use panels::{
    AnimationCinematicsAuthoringSuite, AssetGateAndApprovalSurface, AudioVoiceAuthoringSuite,
    BuildValidationReleaseSuite, CollaborationSessionSurface, DestructionFractureAuthoringSuite,
    LearningOnboardingAndHelpSurface, MaterialLookdevAuthoringSuite, PlaytestAndCaptureOperations,
    ProductionDashboardAndTraceability, QuestEventLogicAuthoringSuite, ReviewAnnotationSurface,
    SceneEntityAuthoringSuite, SimulationAiAuthoringSuite, TerrainLandscapeAuthoringSuite,
    UiHudAuthoringSuite, WeatherEnvironmentAuthoringSuite, WorldAuthoringSuite,
};
pub use state::{
    AssistantSurface, BuildReleaseSurface, ContentBrowserSystem, DiagnosticsSurface,
    InspectorSystem, InteractionRoutingSystem, OutlinerSystem, OverlayAndGizmoSystem,
    ToolContextSystem, ViewportSystem,
};
