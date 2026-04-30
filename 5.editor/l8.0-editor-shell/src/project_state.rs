//! Project State - Minimal project state holder
//!
//! Per canon: shell holds state, tooling manages lifecycle.

use std::path::PathBuf;

#[derive(Default)]
pub struct ProjectState {
    pub project_id: Option<String>,
    pub project_path: Option<PathBuf>,
}

impl ProjectState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_open(&self) -> bool {
        self.project_id.is_some()
    }
}
