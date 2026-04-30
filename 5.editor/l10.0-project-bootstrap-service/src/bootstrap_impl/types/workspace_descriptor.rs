use crate::*;

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
