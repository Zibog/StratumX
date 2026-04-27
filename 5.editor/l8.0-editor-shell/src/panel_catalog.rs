//! Panel Catalog - Declarative panel metadata
//!
//! Per canon: shell is declarative, not imperative.

pub struct PanelMetadata {
    pub id: &'static str,
    pub label: &'static str,
    pub default_visible: bool,
}

pub const PANEL_CATALOG: &[PanelMetadata] = &[
    PanelMetadata {
        id: "viewport",
        label: "Viewport",
        default_visible: true,
    },
    PanelMetadata {
        id: "outliner",
        label: "Outliner",
        default_visible: true,
    },
    PanelMetadata {
        id: "inspector",
        label: "Inspector",
        default_visible: true,
    },
    PanelMetadata {
        id: "content_browser",
        label: "Content Browser",
        default_visible: false,
    },
];
