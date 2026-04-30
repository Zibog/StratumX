use super::{
    Deserialize, IngressControlEnvelopeId, IngressControlKind, LegalityGateId, ObjectHandle,
    RuntimeHandle, Serialize, SessionHandle, WorldControlPayload,
};
use sdk_compat::versions::CompatDomain;

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
    pub fn new(
        ingress_control_envelope_id: IngressControlEnvelopeId,
        control_kind: IngressControlKind,
        target_runtime_handle: RuntimeHandle,
        source_session_handle: SessionHandle,
        submission_order_key: u64,
        legality_gate_id: LegalityGateId,
    ) -> Self {
        Self {
            ingress_control_envelope_id,
            control_kind,
            target_object_handle: None,
            target_runtime_handle,
            source_session_handle,
            submission_order_key,
            legality_gate_id,
        }
    }

    pub fn with_object_target(mut self, object: ObjectHandle) -> Self {
        self.target_object_handle = Some(object);
        self
    }

    pub fn set_label(
        envelope_id: IngressControlEnvelopeId,
        runtime: RuntimeHandle,
        session: SessionHandle,
        order_key: u64,
        gate_id: LegalityGateId,
        label: String,
    ) -> Self {
        Self::new(
            envelope_id,
            IngressControlKind::SetLabel { label },
            runtime,
            session,
            order_key,
            gate_id,
        )
    }

    pub fn set_field(
        envelope_id: IngressControlEnvelopeId,
        runtime: RuntimeHandle,
        session: SessionHandle,
        order_key: u64,
        gate_id: LegalityGateId,
        key: String,
        value: String,
    ) -> Self {
        Self::new(
            envelope_id,
            IngressControlKind::SetField { key, value },
            runtime,
            session,
            order_key,
            gate_id,
        )
    }

    pub fn retire_object(
        envelope_id: IngressControlEnvelopeId,
        runtime: RuntimeHandle,
        session: SessionHandle,
        order_key: u64,
        gate_id: LegalityGateId,
        object: ObjectHandle,
    ) -> Self {
        Self::new(
            envelope_id,
            IngressControlKind::RetireObject,
            runtime,
            session,
            order_key,
            gate_id,
        )
        .with_object_target(object)
    }

    pub fn world_open(
        envelope_id: IngressControlEnvelopeId,
        runtime: RuntimeHandle,
        session: SessionHandle,
        order_key: u64,
        gate_id: LegalityGateId,
        world_id: String,
        mode: String,
    ) -> Self {
        Self::new(
            envelope_id,
            IngressControlKind::WorldControl {
                domain: CompatDomain::World,
                payload: WorldControlPayload::Open { world_id, mode },
            },
            runtime,
            session,
            order_key,
            gate_id,
        )
    }
}
