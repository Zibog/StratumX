// Compare Engine - triplet comparison system

use super::capture::ArtifactRef;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CompareTriplet {
    pub baseline_ref: String,
    pub failed_ref: Option<String>,
    pub recovery_ref: Option<String>,
    pub compare_mode: CompareMode,
    pub diff_artifacts: Vec<ArtifactRef>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CompareMode {
    Exact,
    Threshold,
    Visual,
    Semantic,
}

#[derive(Debug, Clone)]
pub struct CompareEngine {
    triplets: HashMap<String, CompareTriplet>,
}

impl Default for CompareEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl CompareEngine {
    pub fn new() -> Self {
        Self {
            triplets: HashMap::new(),
        }
    }

    pub fn create_triplet(
        &mut self,
        pack_id: String,
        baseline_ref: String,
        compare_mode: CompareMode,
    ) -> String {
        let triplet = CompareTriplet {
            baseline_ref,
            failed_ref: None,
            recovery_ref: None,
            compare_mode,
            diff_artifacts: Vec::new(),
        };
        self.triplets.insert(pack_id.clone(), triplet);
        pack_id
    }

    pub fn add_failed_run(&mut self, pack_id: &str, failed_ref: String) {
        if let Some(triplet) = self.triplets.get_mut(pack_id) {
            triplet.failed_ref = Some(failed_ref);
        }
    }

    pub fn add_recovery_run(&mut self, pack_id: &str, recovery_ref: String) {
        if let Some(triplet) = self.triplets.get_mut(pack_id) {
            triplet.recovery_ref = Some(recovery_ref);
        }
    }

    pub fn get_triplet(&self, pack_id: &str) -> Option<&CompareTriplet> {
        self.triplets.get(pack_id)
    }

    pub fn execute_compare(&mut self, pack_id: &str) -> Result<CompareResult, String> {
        let triplet = self
            .triplets
            .get(pack_id)
            .ok_or_else(|| format!("No triplet for pack {}", pack_id))?;

        let baseline_exists = !triplet.baseline_ref.is_empty();
        let has_failed = triplet.failed_ref.is_some();
        let has_recovery = triplet.recovery_ref.is_some();

        let verdict = if !baseline_exists {
            CompareVerdict::NoBaseline
        } else if !has_failed {
            CompareVerdict::Pass
        } else if has_recovery {
            CompareVerdict::Recovered
        } else {
            CompareVerdict::Failed
        };

        Ok(CompareResult {
            pack_id: pack_id.to_string(),
            verdict,
            diff_summary: format!("Compare mode: {:?}", triplet.compare_mode),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CompareResult {
    pub pack_id: String,
    pub verdict: CompareVerdict,
    pub diff_summary: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CompareVerdict {
    Pass,
    Failed,
    Recovered,
    NoBaseline,
}
