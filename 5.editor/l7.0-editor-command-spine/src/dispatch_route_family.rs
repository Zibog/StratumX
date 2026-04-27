use crate::ActionId;

/// Route families inferred from canonical action ids.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DispatchRouteFamily {
    WorldLifecycle,
    TerrainAuthoring,
    EnvironmentAuthoring,
    ProjectLifecycle,
    RuntimeControl,
    PanelChrome,
    DiagnosticsProof,
    BuildRelease,
    MaterialAuthoring,
    MaterialBinding,
    MaterialInspection,
    AudioAuthoring,
    AudioZone,
    AudioPreview,
    Unknown,
}

impl DispatchRouteFamily {
    /// Classifies an action id into its concrete route family.
    pub fn classify(action_id: &ActionId) -> Self {
        let action = action_id.as_str();

        if action.starts_with("world.") {
            Self::WorldLifecycle
        } else if action.starts_with("terrain.") {
            Self::TerrainAuthoring
        } else if action.starts_with("environment.") || action.starts_with("sky.") {
            Self::EnvironmentAuthoring
        } else if action.starts_with("project.") {
            Self::ProjectLifecycle
        } else if action.starts_with("runtime.") {
            Self::RuntimeControl
        } else if action.starts_with("panel.") {
            Self::PanelChrome
        } else if action.starts_with("diagnostics.") || action.starts_with("proof.") {
            Self::DiagnosticsProof
        } else if action.starts_with("build.") || action.starts_with("release.") {
            Self::BuildRelease
        } else if action.starts_with("material.") && action.contains("author") {
            Self::MaterialAuthoring
        } else if action.starts_with("material.") && action.contains("bind") {
            Self::MaterialBinding
        } else if action.starts_with("material.") && action.contains("inspect") {
            Self::MaterialInspection
        } else if action.starts_with("audio.") && action.contains("author") {
            Self::AudioAuthoring
        } else if action.starts_with("audio.")
            && (action.contains("configure") || action.contains("zone"))
        {
            Self::AudioZone
        } else if action.starts_with("audio.") && action.contains("preview") {
            Self::AudioPreview
        } else {
            Self::Unknown
        }
    }
}
