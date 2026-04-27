use crate::{
    AssetGateAndApprovalSurface, AssistantSurface, CollaborationSessionSurface,
    InteractionRoutingSystem, LearningOnboardingAndHelpSurface, PackageMarketAndDependencyService,
    PlaytestAndCaptureOperations, PluginAndExtensionHost, ProductionDashboardAndTraceability,
    ReviewAnnotationSurface, ToolContextSystem, ViewportSystem, WorkspaceLayoutSystem,
};
use serde::{Deserialize, Serialize};
use stratumx_tooling_l6_0_tool_session::ToolObject;

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
