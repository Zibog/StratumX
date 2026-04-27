pub use serde::{Deserialize, Serialize};
pub use serde_json;
pub use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::Path,
    sync::Arc,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct PlaytestAndCaptureOperations {
    pub last_capture_label: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct EditorProduct {
    pub playtest_surface: PlaytestAndCaptureOperations,
}

impl EditorProduct {
    pub fn run_playtest_capture(&mut self, label: impl Into<String>) {
        self.playtest_surface.last_capture_label = Some(label.into());
    }
}
