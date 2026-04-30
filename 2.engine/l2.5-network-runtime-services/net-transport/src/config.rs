use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetTransportConfig {
    pub max_packet_size_bytes: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetTransportMetrics {
    pub queued_packets: usize,
    pub queued_bytes: usize,
}
