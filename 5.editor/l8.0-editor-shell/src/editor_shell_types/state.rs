//! Editor shell state types.

use serde::{Deserialize, Serialize};
use stratumx_tooling_l6_0_tool_session::ObjectHandle;

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
