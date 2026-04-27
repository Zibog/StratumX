//! Material Authoring Service Types

use crate::model::{BranchCoverage, TextureSlot};
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct CheapnessReport {
    pub passes: bool,
    pub estimated_texture_memory_bytes: u64,
    pub texture_count: usize,
    pub issues: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct MaterialDiagnostics {
    pub coverage: BranchCoverage,
    pub cheapness: CheapnessReport,
    pub textures: BTreeMap<TextureSlot, String>,
}
