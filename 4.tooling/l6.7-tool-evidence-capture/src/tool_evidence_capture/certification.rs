// Certification Engine - pack certification system

use super::compare::{CompareResult, CompareVerdict};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CertificationResult {
    pub pack_id: String,
    pub scenario_id: String,
    pub verdict: CertificationVerdict,
    pub blockers: Vec<String>,
    pub evidence_bundle_ref: String,
    pub compare_result_ref: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CertificationVerdict {
    Certified,
    ConditionalPass,
    Blocked,
}

#[derive(Debug, Clone)]
pub struct CertificationEngine {
    results: HashMap<String, CertificationResult>,
}

impl Default for CertificationEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl CertificationEngine {
    pub fn new() -> Self {
        Self {
            results: HashMap::new(),
        }
    }

    pub fn certify_pack(
        &mut self,
        pack_id: String,
        scenario_id: String,
        evidence_bundle: &super::capture::EvidenceBundle,
        compare_result: &CompareResult,
    ) -> CertificationResult {
        let mut blockers = Vec::new();

        if matches!(
            evidence_bundle.verdict,
            super::capture::EvidenceVerdict::Red
        ) {
            blockers.push("Evidence verdict is RED".to_string());
        }

        if matches!(compare_result.verdict, CompareVerdict::Failed) {
            blockers.push("Compare failed".to_string());
        }

        if matches!(compare_result.verdict, CompareVerdict::NoBaseline) {
            blockers.push("No baseline exists".to_string());
        }

        let verdict = if blockers.is_empty() {
            CertificationVerdict::Certified
        } else {
            CertificationVerdict::Blocked
        };

        let result = CertificationResult {
            pack_id: pack_id.clone(),
            scenario_id,
            verdict,
            blockers,
            evidence_bundle_ref: evidence_bundle.bundle_id.clone(),
            compare_result_ref: compare_result.pack_id.clone(),
            timestamp: evidence_bundle.timestamp,
        };

        self.results.insert(pack_id, result.clone());
        result
    }

    pub fn get_certification(&self, pack_id: &str) -> Option<&CertificationResult> {
        self.results.get(pack_id)
    }

    pub fn is_freeze_ready(&self, pack_id: &str) -> bool {
        if let Some(cert) = self.results.get(pack_id) {
            matches!(cert.verdict, CertificationVerdict::Certified)
        } else {
            false
        }
    }
}
