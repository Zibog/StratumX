use engine_core::{EngineCoreError, EngineCoreResult};
use engine_runtime::{ConnectionKey, RuntimeKernel};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetTransportConfig {
    pub max_packet_size_bytes: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct ConnectionHandle(pub u64);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PacketDescriptor {
    pub reliable: bool,
    pub ordered: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PacketLane {
    Control,
    State,
    Bulk,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransportSession {
    pub connection: ConnectionHandle,
    pub session_epoch: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetPacketEnvelope {
    pub connection: ConnectionHandle,
    pub descriptor: PacketDescriptor,
    pub lane: PacketLane,
    pub payload: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NetTransportMetrics {
    pub queued_packets: usize,
    pub queued_bytes: usize,
}

#[derive(Debug)]
pub struct NetTransportService {
    config: NetTransportConfig,
    sessions: BTreeMap<ConnectionHandle, TransportSession>,
}

impl NetTransportService {
    pub fn new(config: NetTransportConfig) -> Self {
        Self {
            config,
            sessions: BTreeMap::new(),
        }
    }
    pub fn open_session(&mut self, connection: ConnectionHandle) -> TransportSession {
        let session = TransportSession {
            connection,
            session_epoch: self.sessions.len() as u64 + 1,
        };
        self.sessions.insert(connection, session.clone());
        session
    }
    pub fn send(
        &self,
        runtime: &mut RuntimeKernel,
        envelope: NetPacketEnvelope,
    ) -> EngineCoreResult<NetTransportMetrics> {
        if envelope.payload.len() > self.config.max_packet_size_bytes {
            return Err(EngineCoreError::InvalidDescriptor(
                "packet exceeds transport packet ceiling",
            ));
        }
        runtime.enqueue_connection_publication(
            ConnectionKey(envelope.connection.0),
            envelope.payload.clone(),
        )?;
        Ok(NetTransportMetrics {
            queued_packets: 1,
            queued_bytes: envelope.payload.len(),
        })
    }
}
