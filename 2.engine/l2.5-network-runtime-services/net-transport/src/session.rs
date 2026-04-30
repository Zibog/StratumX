use crate::{ConnectionHandle, NetworkSessionId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SessionState {
    Closed,
    Opening,
    Active,
    Closing,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransportSession {
    pub connection: ConnectionHandle,
    pub session_id: NetworkSessionId,
    pub session_epoch: u64,
    pub state: SessionState,
}
