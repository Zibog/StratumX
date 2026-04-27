use super::evaluation::HardwareFloorResult;
use crate::tool_evidence_capture::{
    BaselineRegistry, CertificationEngine, CompareEngine, EvidenceBundle,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FreezeBlocker {
    NoBaseline,
    NoCompareDigest,
    NoRecoverPath,
    NoHardwareFloorResult,
    UndocumentedFallback,
    CertificationBlocked,
    MissingArtifacts,
}

impl FreezeBlocker {
    pub fn description(&self) -> &str {
        match self {
            FreezeBlocker::NoBaseline => "No baseline exists for this pack",
            FreezeBlocker::NoCompareDigest => "No compare digest generated",
            FreezeBlocker::NoRecoverPath => "No recovery path defined for failures",
            FreezeBlocker::NoHardwareFloorResult => "No hardware floor measurement exists",
            FreezeBlocker::UndocumentedFallback => "Undocumented fallback behavior detected",
            FreezeBlocker::CertificationBlocked => "Certification is blocked",
            FreezeBlocker::MissingArtifacts => "Required artifacts are missing",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FreezeGateResult {
    pub pack_id: String,
    pub can_freeze: bool,
    pub blockers: Vec<FreezeBlocker>,
    pub warnings: Vec<String>,
}

pub struct FreezeGateEngine {
    hardware_floor_results: std::collections::HashMap<String, HardwareFloorResult>,
}

impl Default for FreezeGateEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl FreezeGateEngine {
    pub fn new() -> Self {
        Self {
            hardware_floor_results: std::collections::HashMap::new(),
        }
    }

    pub fn register_hardware_floor_result(&mut self, pack_id: String, result: HardwareFloorResult) {
        self.hardware_floor_results.insert(pack_id, result);
    }

    pub fn check_freeze_gate(
        &self,
        pack_id: &str,
        baseline_registry: &BaselineRegistry,
        compare_engine: &CompareEngine,
        cert_engine: &CertificationEngine,
        evidence: &EvidenceBundle,
    ) -> FreezeGateResult {
        let mut blockers = Vec::new();
        let mut warnings = Vec::new();

        // Gate 1: No baseline -> no freeze
        if baseline_registry.get_baseline(pack_id).is_none() {
            blockers.push(FreezeBlocker::NoBaseline);
        }

        // Gate 2: No compare digest -> no freeze
        if compare_engine.get_triplet(pack_id).is_none() {
            blockers.push(FreezeBlocker::NoCompareDigest);
        }

        // Gate 3: No recover path -> no freeze
        if let Some(triplet) = compare_engine.get_triplet(pack_id) {
            if triplet.failed_ref.is_some() && triplet.recovery_ref.is_none() {
                blockers.push(FreezeBlocker::NoRecoverPath);
            }
        }

        // Gate 4: No hardware floor result -> no freeze
        if !self.hardware_floor_results.contains_key(pack_id) {
            blockers.push(FreezeBlocker::NoHardwareFloorResult);
        }

        // Gate 5: Certification blocked -> no freeze
        if !cert_engine.is_freeze_ready(pack_id) {
            blockers.push(FreezeBlocker::CertificationBlocked);
        }

        // Gate 6: Missing artifacts -> no freeze
        if evidence.artifacts.is_empty() {
            blockers.push(FreezeBlocker::MissingArtifacts);
        }

        // Warnings (not blockers)
        if evidence.operator_signoff.is_none() {
            warnings.push("No operator signoff recorded".to_string());
        }

        FreezeGateResult {
            pack_id: pack_id.to_string(),
            can_freeze: blockers.is_empty(),
            blockers,
            warnings,
        }
    }
}
