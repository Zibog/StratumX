//! Desktop-facing diagnostics panel state.

use std::collections::BTreeMap;

#[derive(Default, PartialEq, Eq, Clone, Copy)]
pub enum DiagnosticsTab {
    #[default]
    System,
    Coverage,
    Traces,
    Proof,
}

#[derive(Default)]
pub struct DiagnosticsState {
    pub visual_coverage: f32,
    pub runtime_coverage: f32,
    pub missing_coverage: Vec<MissingCoverage>,
    pub trace_chains: Vec<TraceChain>,
    pub freeze_ready: bool,
    pub proof_blockers: Vec<ProofBlocker>,
    pub recovery_anchors: Vec<super::RecoveryAnchor>,
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

pub type RecoverySnapshot = BTreeMap<String, String>;
