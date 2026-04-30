//! FUTURE_STUB.
//!
//! This crate is present as a canonical future surface.
//! It is not part of the active product spine yet.
//! It must not be counted as product-complete.

pub use serde::{Deserialize, Serialize};
pub use serde_json;
pub use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::Path,
    sync::Arc,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum EditorPanel {
    Viewport,
    Outliner,
    Inspector,
    ContentBrowser,
    Diagnostics,
    BuildRelease,
    Assistant,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceLayoutSystem {
    pub open_panels: BTreeSet<EditorPanel>,
    pub focused_panel: EditorPanel,
}

#[derive(Debug, Clone)]
pub struct EditorProduct {
    pub workspace_layout: WorkspaceLayoutSystem,
}

impl Default for EditorProduct {
    fn default() -> Self {
        Self {
            workspace_layout: WorkspaceLayoutSystem {
                open_panels: anchored_panels(),
                focused_panel: EditorPanel::Viewport,
            },
        }
    }
}

pub fn anchored_panels() -> BTreeSet<EditorPanel> {
    BTreeSet::from([
        EditorPanel::Viewport,
        EditorPanel::Outliner,
        EditorPanel::Inspector,
        EditorPanel::ContentBrowser,
    ])
}

impl EditorProduct {
    pub fn anchored_panels(&self) -> BTreeSet<EditorPanel> {
        self.workspace_layout.open_panels.clone()
    }

    pub fn toggle_panel(&mut self, panel: EditorPanel) {
        if self.workspace_layout.open_panels.contains(&panel) {
            self.workspace_layout.open_panels.remove(&panel);
        } else {
            self.workspace_layout.open_panels.insert(panel);
        }
    }
}

#[cfg(feature = "desktop")]
pub mod app;

#[cfg(feature = "desktop")]
pub mod owners;

#[cfg(feature = "desktop")]
pub mod queries;
