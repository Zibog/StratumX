//! Editor Product Structure

use super::types::*;
use super::bootstrap_types::*;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EditorProduct {
    pub title: String,
    pub project_name: String,
    pub tooling: ToolingRuntime,
    pub viewport: ViewportSystem,
    pub outliner: OutlinerSystem,
    pub content_browser: ContentBrowserSystem,
    pub inspector: InspectorSystem,
    pub tool_context: ToolContextSystem,
    pub overlay_and_gizmo: OverlayAndGizmoSystem,
    pub workspace_layout: WorkspaceLayoutSystem,
    pub interaction_routing: InteractionRoutingSystem,
    pub assistant_surface: AssistantSurface,
    pub diagnostics_surface: DiagnosticsSurface,
    pub build_release_surface: BuildReleaseSurface,
    pub world_suite: WorldAuthoringSuite,
    pub scene_suite: SceneEntityAuthoringSuite,
    pub terrain_suite: TerrainLandscapeAuthoringSuite,
    pub material_suite: MaterialLookdevAuthoringSuite,
    pub destruction_suite: DestructionFractureAuthoringSuite,
    pub simulation_suite: SimulationAiAuthoringSuite,
    pub weather_suite: WeatherEnvironmentAuthoringSuite,
    pub animation_suite: AnimationCinematicsAuthoringSuite,
    pub audio_suite: AudioVoiceAuthoringSuite,
    pub ui_suite: UiHudAuthoringSuite,
    pub quest_suite: QuestEventLogicAuthoringSuite,
    pub build_suite: BuildValidationReleaseSuite,
    pub project_bootstrap_service: ProjectBootstrapService,
    pub import_export_service: ImportExportPipelineService,
    pub graph_authoring_service: GraphAuthoringService,
    pub automation_service: AutomationAndBatchService,
    pub script_service: ScriptAndHotReloadService,
    pub plugin_host: PluginAndExtensionHost,
    pub template_service: TemplatePresetAndScaffoldService,
    pub package_service: PackageMarketAndDependencyService,
    pub collaboration_surface: CollaborationSessionSurface,
    pub review_surface: ReviewAnnotationSurface,
    pub approval_surface: AssetGateAndApprovalSurface,
    pub playtest_surface: PlaytestAndCaptureOperations,
    pub production_surface: ProductionDashboardAndTraceability,
    pub learning_surface: LearningOnboardingAndHelpSurface,
}

impl EditorProduct {
    pub fn refresh_from_tooling(&mut self) -> Result<(), ToolingError> {
        Ok(())
    }
}
