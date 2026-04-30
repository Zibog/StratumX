//! SDK Ingress Controls — control toggles and ingress controls.

use engine_handle_refs::{ObjectHandle, RuntimeHandle, SessionHandle};
use legality_gates::LegalityGateId;
use sdk_compat::versions::CompatDomain;
use sdk_compat::Capability;
use serde::{Deserialize, Serialize};

mod bridge_control;
mod ids;
mod payloads;
mod results;

pub const CANONICAL_LEVEL: &str = "l5.1-link-ingress-controls";

pub use bridge_control::BridgeControl;
pub use ids::IngressControlEnvelopeId;
pub use payloads::{
    IngressControlKind, MaterialControlPayload, MaterialLayerSpec, TerrainControlPayload,
    WorldControlPayload,
};
pub use results::{ControlApplied, ControlRejection, ControlRejectionReason};
