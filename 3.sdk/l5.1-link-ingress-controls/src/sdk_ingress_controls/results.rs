use super::{Capability, Deserialize, IngressControlEnvelopeId, LegalityGateId, Serialize};

/// Typed control rejection payload.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ControlRejection {
    pub envelope_id: IngressControlEnvelopeId,
    pub reason: ControlRejectionReason,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ControlRejectionReason {
    Illegal { gate_id: LegalityGateId },
    MissingCapability { required: Capability },
    StaleOrderKey { expected: u64, got: u64 },
    InvalidTarget { handle_kind: String },
    PayloadTooLarge { size: usize, max: usize },
}

/// Control submission result for successful application.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ControlApplied {
    pub envelope_id: IngressControlEnvelopeId,
    pub applied_at_tick: u64,
    pub side_effects: Vec<String>,
}
