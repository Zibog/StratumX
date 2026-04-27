//! Editor Outliner System
//!
//! Role: Outliner hierarchy and scene graph presentation.
//! Owns: Outliner tree, selection-bound hierarchy, entity listing.

pub use serde::{Deserialize, Serialize};

#[cfg(feature = "desktop")]
pub mod outliner_panel;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct OutlinerNode {
    pub id: String,
    pub label: String,
    pub children: Vec<OutlinerNode>,
    pub expanded: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OutlinerSystem {
    pub root_nodes: Vec<OutlinerNode>,
    pub selected_ids: Vec<String>,
}

impl OutlinerSystem {
    pub fn new() -> Self {
        Self {
            root_nodes: Vec::new(),
            selected_ids: Vec::new(),
        }
    }
}
