use crate::{
    ConnectionHandle, NetworkFailureReason, NetworkLaneId, NetworkSessionId, PacketDescriptor,
    PacketId, PacketLane,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetPacketEnvelope {
    pub connection: ConnectionHandle,
    pub descriptor: PacketDescriptor,
    pub lane: PacketLane,
    pub payload: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransportPacket {
    pub packet_id: PacketId,
    pub session_id: NetworkSessionId,
    pub session_epoch: u64,
    pub lane_id: NetworkLaneId,
    pub sequence: u64,
    pub payload_digest: u64,
    pub timestamp_ms: u64,
}

impl TransportPacket {
    /// Validate packet identity.
    pub fn validate(&self) -> Result<(), NetworkFailureReason> {
        if self.session_id.0 == 0 {
            return Err(NetworkFailureReason::InvalidSession);
        }
        if self.session_epoch == 0 {
            return Err(NetworkFailureReason::StaleSessionEpoch);
        }
        if self.lane_id.0 > 2 {
            return Err(NetworkFailureReason::InvalidLane);
        }
        if self.sequence == 0 {
            return Err(NetworkFailureReason::OutOfOrderPacket);
        }
        Ok(())
    }
}
