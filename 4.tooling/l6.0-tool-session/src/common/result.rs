use crate::common::context::ToolSessionContext;
use crate::common::publication::{EditorPublication, PublicationChange};
use stratumx_tooling_l6_1_command_envelopes::RouteMetadata;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommandVerdict {
    Accepted,
    Success,
    RetryableFailure,
    TerminalFailure,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LifecycleState {
    Cataloged,
    AdapterCutover,
    NeedsOwnerWiring,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiagnosticsRecord {
    pub kind: String,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArtifactRef {
    pub family: String,
    pub button_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommandResult {
    pub verdict: CommandVerdict,
    pub publications: Vec<EditorPublication>,
    pub focus_target: Option<String>,
    pub retry_target: Option<String>,
    pub recovery_anchor: Option<String>,
    pub denial: Option<String>,
    pub diagnostics: Vec<DiagnosticsRecord>,
    pub artifact_ref: Option<ArtifactRef>,
    pub next_legal_action: Option<String>,
    pub lifecycle_state: LifecycleState,
}

impl CommandResult {
    pub fn from_route_metadata(route: &RouteMetadata, ctx: &mut ToolSessionContext) -> Self {
        ctx.executed_buttons.push(route.button_id.clone());
        ctx.focus_history.push(route.focus_success.clone());
        ctx.diagnostics_notes.push(route.diagnostics_kind.clone());

        Self {
            verdict: CommandVerdict::Success,
            publications: route
                .publication_kinds
                .iter()
                .filter_map(|kind| publication_from_kind(kind, route))
                .collect(),
            focus_target: Some(route.focus_success.clone()),
            retry_target: Some(route.focus_retry.clone()),
            recovery_anchor: Some(route.recovery_anchor.clone()),
            denial: None,
            diagnostics: vec![DiagnosticsRecord {
                kind: route.diagnostics_kind.clone(),
                message: format!("route {} executed", route.button_id),
            }],
            artifact_ref: (route.evidence_family != "none").then(|| ArtifactRef {
                family: route.evidence_family.clone(),
                button_id: route.button_id.clone(),
            }),
            next_legal_action: Some(route.action_id.clone()),
            lifecycle_state: match route.status.as_str() {
                "adapter_cutover" => LifecycleState::AdapterCutover,
                "needs_owner_wiring" => LifecycleState::NeedsOwnerWiring,
                _ => LifecycleState::Cataloged,
            },
        }
    }
}

fn publication_from_kind(kind: &str, route: &RouteMetadata) -> Option<EditorPublication> {
    let change = PublicationChange {
        button_id: route.button_id.clone(),
        owner_state: route.owner_state.clone(),
    };
    match kind {
        "ShellChanged" => Some(EditorPublication::ShellChanged(change)),
        "ProjectChanged" => Some(EditorPublication::ProjectChanged(change)),
        "WorldChanged" => Some(EditorPublication::WorldChanged(change)),
        "TerrainChanged" => Some(EditorPublication::TerrainChanged(change)),
        "MaterialChanged" => Some(EditorPublication::MaterialChanged(change)),
        "EnvironmentChanged" => Some(EditorPublication::EnvironmentChanged(change)),
        "AudioChanged" => Some(EditorPublication::AudioChanged(change)),
        "RuntimeChanged" => Some(EditorPublication::RuntimeChanged(change)),
        "ValidationChanged" => Some(EditorPublication::ValidationChanged(change)),
        "DiagnosticsChanged" => Some(EditorPublication::DiagnosticsChanged(change)),
        "EvidenceChanged" => Some(EditorPublication::EvidenceChanged(change)),
        "BuildChanged" => Some(EditorPublication::BuildChanged(change)),
        _ => None,
    }
}
