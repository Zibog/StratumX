//! Editor Project Bootstrap Service

pub use serde::{Deserialize, Serialize};
pub use serde_json;
pub use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::Path,
    sync::Arc,
};

pub use stratumx_tooling::{
    ApprovalClass, BudgetClass, BuildArtifact, CommandOrigin, ObjectClass, ObjectHandle,
    ReleasePackage, ToolCommand, ToolObject, ToolSnapshot, ToolingError, ToolingRuntime,
};
pub use stratumx_tooling_l6_0_tool_session as stratumx_tooling;

pub use stratumx_editor_l8_0_editor_shell::{
    AnimationCinematicsAuthoringSuite, AssetGateAndApprovalSurface, AssistantSurface,
    AudioVoiceAuthoringSuite, AutomationAndBatchService, BuildReleaseSurface,
    BuildValidationReleaseSuite, CollaborationSessionSurface, DestructionFractureAuthoringSuite,
    DiagnosticsSurface, GraphAuthoringService, ImportExportPipelineService,
    LearningOnboardingAndHelpSurface, MaterialLookdevAuthoringSuite,
    PackageMarketAndDependencyService, PlaytestAndCaptureOperations, PluginAndExtensionHost,
    ProductionDashboardAndTraceability,
    ProjectBootstrapService as EditorShellProjectBootstrapService, QuestEventLogicAuthoringSuite,
    ReviewAnnotationSurface, SceneEntityAuthoringSuite, ScriptAndHotReloadService,
    SimulationAiAuthoringSuite, TemplatePresetAndScaffoldService, TerrainLandscapeAuthoringSuite,
    UiHudAuthoringSuite, WeatherEnvironmentAuthoringSuite, WorldAuthoringSuite,
};

mod product;

pub use product::EditorProduct;

mod bootstrap_types;
mod export_document_runtime;
mod import_document_runtime;
mod persistence_runtime;
mod reference_bootstrap_runtime;
mod reference_shell_runtime;

mod types;
pub use types::*;

#[cfg(feature = "desktop")]
#[path = "../desktop/mod.rs"]
pub mod desktop;

pub use bootstrap_types::*;
