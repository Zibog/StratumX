use super::capture::EvidenceVerdict;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Baseline {
    pub baseline_id: String,
    pub pack_id: String,
    pub scenario_id: String,
    pub timestamp: u64,
    pub artifacts: Vec<super::capture::ArtifactRef>,
    pub protected: bool, // Cannot be replaced by red/orange runs
}

#[derive(Debug, Clone)]
pub struct BaselineRegistry {
    baselines: HashMap<String, Baseline>,
}

impl Default for BaselineRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl BaselineRegistry {
    pub fn new() -> Self {
        Self {
            baselines: HashMap::new(),
        }
    }

    pub fn register_baseline(&mut self, baseline: Baseline) {
        self.baselines.insert(baseline.pack_id.clone(), baseline);
    }

    pub fn get_baseline(&self, pack_id: &str) -> Option<&Baseline> {
        self.baselines.get(pack_id)
    }

    pub fn can_replace_baseline(&self, _pack_id: &str, verdict: &EvidenceVerdict) -> bool {
        match verdict {
            EvidenceVerdict::Green => true,
            EvidenceVerdict::Orange => {
                // Orange can only replace if explicitly allowed (waiver)
                false
            }
            EvidenceVerdict::Red => {
                // Red can never replace baseline
                false
            }
        }
    }

    pub fn update_baseline(&mut self, pack_id: &str, new_baseline: Baseline) -> Result<(), String> {
        if let Some(existing) = self.baselines.get(pack_id) {
            if existing.protected {
                return Err(format!("Baseline {} is protected", pack_id));
            }
        }
        self.baselines.insert(pack_id.to_string(), new_baseline);
        Ok(())
    }
}
