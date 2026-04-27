//! SDK Ingress Controls — control toggles and ingress controls.

use engine_handle_refs::{ObjectHandle, RuntimeHandle, SessionHandle};
use legality_gates::LegalityGateId;
use sdk_compat::versions::CompatDomain;
use sdk_compat::Capability;
use serde::{Deserialize, Serialize};

pub const CANONICAL_LEVEL: &str = "l5.1-link-ingress-controls";

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct IngressControlEnvelopeId(pub u64);

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum IngressControlKind {
    SetLabel { label: String },
    SetField { key: String, value: String },
    ClearField { key: String },
    AddTag { tag: String },
    RemoveTag { tag: String },
    RetireObject,
    RestoreObject,
    RefreshSnapshot,
    WorldControl { domain: CompatDomain, payload: WorldControlPayload },
    TerrainControl { payload: TerrainControlPayload },
    MaterialControl { payload: MaterialControlPayload },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum WorldControlPayload {
    Open { world_id: String, mode: String },
    Close { world_id: String },
    Save { world_id: String },
    Load { world_id: String, snapshot_id: String },
    Bind { world_id: String, binding_id: String },
    Unbind { world_id: String },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TerrainControlPayload {
    Deform { patch_id: String, delta: Vec<f32> },
    Paint { patch_id: String, material_stack_id: String },
    Query { patch_id: String },
    List,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MaterialControlPayload {
    CreateArchetype { name: String, properties: Vec<(String, String)> },
    CreateStack { archetype_id: String, layers: Vec<MaterialLayerSpec> },
    Assign { entity_id: String, stack_id: String },
    Query { archetype_id: Option<String> },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MaterialLayerSpec { pub archetype_id: String, pub thickness_mm: f32, pub coverage: f32 }

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BridgeControl {
    pub ingress_control_envelope_id: IngressControlEnvelopeId,
    pub control_kind: IngressControlKind,
    pub target_object_handle: Option<ObjectHandle>,
    pub target_runtime_handle: RuntimeHandle,
    pub source_session_handle: SessionHandle,
    pub submission_order_key: u64,
    pub legality_gate_id: LegalityGateId,
}

impl BridgeControl {
    pub fn new(ingress_control_envelope_id: IngressControlEnvelopeId, control_kind: IngressControlKind, target_runtime_handle: RuntimeHandle, source_session_handle: SessionHandle, submission_order_key: u64, legality_gate_id: LegalityGateId) -> Self {
        Self { ingress_control_envelope_id, control_kind, target_object_handle: None, target_runtime_handle, source_session_handle, submission_order_key, legality_gate_id }
    }
    pub fn with_object_target(mut self, object: ObjectHandle) -> Self { self.target_object_handle = Some(object); self }
    pub fn set_label(envelope_id: IngressControlEnvelopeId, runtime: RuntimeHandle, session: SessionHandle, order_key: u64, gate_id: LegalityGateId, label: String) -> Self {
        Self::new(envelope_id, IngressControlKind::SetLabel { label }, runtime, session, order_key, gate_id)
    }
    pub fn set_field(envelope_id: IngressControlEnvelopeId, runtime: RuntimeHandle, session: SessionHandle, order_key: u64, gate_id: LegalityGateId, key: String, value: String) -> Self {
        Self::new(envelope_id, IngressControlKind::SetField { key, value }, runtime, session, order_key, gate_id)
    }
    pub fn retire_object(envelope_id: IngressControlEnvelopeId, runtime: RuntimeHandle, session: SessionHandle, order_key: u64, gate_id: LegalityGateId, object: ObjectHandle) -> Self {
        Self::new(envelope_id, IngressControlKind::RetireObject, runtime, session, order_key, gate_id).with_object_target(object)
    }
    pub fn world_open(envelope_id: IngressControlEnvelopeId, runtime: RuntimeHandle, session: SessionHandle, order_key: u64, gate_id: LegalityGateId, world_id: String, mode: String) -> Self {
        Self::new(envelope_id, IngressControlKind::WorldControl { domain: CompatDomain::World, payload: WorldControlPayload::Open { world_id, mode } }, runtime, session, order_key, gate_id)
    }
}

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
