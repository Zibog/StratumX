#![deny(unused_imports)]
#![deny(unused_variables)]
#![deny(dead_code)]

pub use editor_authoring_ingress::*;
pub use vertical_slice_ingress::*;

pub mod packet_executor;
pub use packet_executor::*;

mod editor_authoring_ingress;
mod vertical_slice_ingress;

use engine_handle_refs::SessionHandle;
use legality_gates::transport_legality;
use sdk_compat::LegalityVerdict;
use sdk_compat::{BridgeVersion, CompatVersionId};
use transport_policies::{TransportPolicy, TransportPolicyId};

pub const CANONICAL_LEVEL: &str = "L5.0";

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PacketId(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PacketDecodeStatus {
    Pending,
    Accepted,
    Rejected,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgePacket {
    pub packet_id: PacketId,
    pub session_handle: SessionHandle,
    pub transport_policy_id: TransportPolicyId,
    pub transport_policy: TransportPolicy,
    pub compat_version_id: CompatVersionId,
    pub compat_version: BridgeVersion,
    pub payload_bytes: Vec<u8>,
    pub received_at_tick: u64,
    pub decode_status: PacketDecodeStatus,
}

pub fn packet_legality(packet: &BridgePacket) -> LegalityVerdict {
    transport_legality(
        &packet.transport_policy,
        packet.payload_bytes.len(),
        false,
        false,
    )
    .unwrap_or(LegalityVerdict::Illegal)
}
