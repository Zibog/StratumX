use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BranchCoverage {
    pub physical_complete: bool,
    pub visual_complete: bool,
    pub acoustic_complete: bool,
    pub light_complete: bool,
    pub runtime_complete: bool,
    pub persistence_complete: bool,
    pub proof_complete: bool,
    pub missing_bindings: Vec<String>,
    pub invalid_combinations: Vec<String>,
}
