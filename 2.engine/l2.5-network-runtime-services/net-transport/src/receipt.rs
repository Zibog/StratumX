use crate::{NetworkSessionId, PacketId};
use serde::{Deserialize, Serialize};

/// Delivery verdict for a packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DeliveryVerdict {
    Accepted,
    Duplicate,
    OutOfOrder,
    InvalidLane,
    InvalidSession,
    StaleSessionEpoch,
    Expired,
    Rejected,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct AckWindow {
    pub highest_sent_sequence: u64,
    pub highest_acked_sequence: u64,
    pub in_flight_packets: usize,
}

/// Packet delivery receipt.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PacketDeliveryReceipt {
    pub packet_id: PacketId,
    pub session_id: NetworkSessionId,
    pub session_epoch: u64,
    pub verdict: DeliveryVerdict,
    pub sequence: u64,
    pub ack_window: AckWindow,
}
