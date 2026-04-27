//! Diagnostics view state and lightweight read models.

use std::collections::BTreeMap;

#[derive(Default, PartialEq, Eq, Clone, Copy)]
pub enum DiagnosticsTab {
    #[default]
    System,
    Coverage,
    Traces,
    Proof,
}

#[derive(Debug, Clone)]
pub struct RecoveryAnchor {
    pub id: String,
    pub label: String,
    pub timestamp: String,
    pub snapshot: BTreeMap<String, String>,
}

#[derive(Default)]
pub struct DiagnosticsState {
    pub visual_coverage: f32,
    pub runtime_coverage: f32,
    pub missing_coverage: Vec<MissingCoverage>,
    pub trace_chains: Vec<TraceChain>,
    pub freeze_ready: bool,
    pub proof_blockers: Vec<ProofBlocker>,
    pub recovery_anchors: Vec<RecoveryAnchor>,
    pub selected_anchor: Option<String>,
}

#[derive(Debug, Clone)]
pub struct MissingCoverage {
    pub branch: String,
    pub missing_bindings: String,
}

#[derive(Debug, Clone)]
pub struct TraceChain {
    pub route_id: String,
    pub result: String,
    pub denial_family: String,
    pub recovery_target: String,
}

#[derive(Debug, Clone)]
pub struct ProofBlocker {
    pub description: String,
}
