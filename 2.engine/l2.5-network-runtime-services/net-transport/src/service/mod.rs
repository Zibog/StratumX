mod ack;
mod process;
mod send;

use crate::{
    ConnectionHandle, NetTransportConfig, NetworkFailureReason, NetworkSessionId, SessionState,
    TransportSession,
};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug)]
pub(crate) struct SessionRecord {
    pub(crate) session: TransportSession,
    pub(crate) next_sequence: u64,
    pub(crate) last_processed_sequence: u64,
    pub(crate) last_acked_sequence: u64,
    pub(crate) in_flight_sequences: BTreeSet<u64>,
    pub(crate) processed_sequences: BTreeSet<u64>,
}

#[derive(Debug)]
pub struct NetTransportService {
    config: NetTransportConfig,
    sessions_by_id: BTreeMap<NetworkSessionId, SessionRecord>,
    session_by_connection: BTreeMap<ConnectionHandle, NetworkSessionId>,
    next_session_id: u64,
    next_session_epoch: u64,
    next_packet_id: u64,
}

impl NetTransportService {
    pub fn new(config: NetTransportConfig) -> Self {
        Self {
            config,
            sessions_by_id: BTreeMap::new(),
            session_by_connection: BTreeMap::new(),
            next_session_id: 1,
            next_session_epoch: 1,
            next_packet_id: 1,
        }
    }

    pub fn open_session(&mut self, connection: ConnectionHandle) -> TransportSession {
        if let Some(existing_session_id) = self.session_by_connection.remove(&connection) {
            self.sessions_by_id.remove(&existing_session_id);
        }

        let session = TransportSession {
            connection,
            session_id: NetworkSessionId(self.next_session_id),
            session_epoch: self.next_session_epoch,
            state: SessionState::Active,
        };
        self.next_session_id = self.next_session_id.saturating_add(1);
        self.next_session_epoch = self.next_session_epoch.saturating_add(1);

        self.session_by_connection
            .insert(connection, session.session_id);
        self.sessions_by_id.insert(
            session.session_id,
            SessionRecord {
                session: session.clone(),
                next_sequence: 1,
                last_processed_sequence: 0,
                last_acked_sequence: 0,
                in_flight_sequences: BTreeSet::new(),
                processed_sequences: BTreeSet::new(),
            },
        );
        session
    }

    pub fn validate_active_session(
        &self,
        session: &TransportSession,
    ) -> Result<(), NetworkFailureReason> {
        let active_session_id = self
            .session_by_connection
            .get(&session.connection)
            .ok_or(NetworkFailureReason::InvalidSession)?;
        if *active_session_id != session.session_id {
            return Err(NetworkFailureReason::InvalidSession);
        }

        let record = self
            .sessions_by_id
            .get(&session.session_id)
            .ok_or(NetworkFailureReason::InvalidSession)?;
        if record.session.session_epoch != session.session_epoch {
            return Err(NetworkFailureReason::StaleSessionEpoch);
        }
        if record.session.state != SessionState::Active {
            return Err(NetworkFailureReason::InvalidSession);
        }
        Ok(())
    }

    /// Close session.
    pub fn close_session(&mut self, connection: ConnectionHandle) -> bool {
        let Some(session_id) = self.session_by_connection.remove(&connection) else {
            return false;
        };
        self.sessions_by_id.remove(&session_id).is_some()
    }

    /// Check if session is active.
    pub fn is_session_active(&self, connection: ConnectionHandle) -> bool {
        let Some(session_id) = self.session_by_connection.get(&connection) else {
            return false;
        };
        self.sessions_by_id
            .get(session_id)
            .map(|record| record.session.state == SessionState::Active)
            .unwrap_or(false)
    }
}
