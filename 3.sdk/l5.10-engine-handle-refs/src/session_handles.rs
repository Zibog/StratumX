//! Session handle types
//!
//! Cadence: created once per session, carried on all packets.
//! Delivery guarantee: ordered reliable — session must be established before any other traffic.

use serde::{Deserialize, Serialize};
use std::fmt;
use std::hash::{Hash, Hasher};

fn redacted_tag(raw: u64) -> String {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    raw.hash(&mut hasher);
    format!("{:016x}", hasher.finish())
}

/// Opaque session handle. Debug output is redacted.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct SessionHandle(u64);

impl SessionHandle {
    pub fn new(raw: u64) -> Self {
        Self(raw)
    }
    pub fn opaque_tag(&self) -> String {
        redacted_tag(self.0)
    }
    pub fn raw(&self) -> u64 {
        self.0
    }
}

/// Origin class for a session.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SessionOriginClass {
    Transport,
    LocalAttach,
}

/// Status of a session handle record.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SessionHandleStatus {
    Open,
    Draining,
    Closed,
}

/// A full session handle record with metadata.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SessionHandleRecord {
    pub session_handle: SessionHandle,
    pub session_scope_id: u64,
    pub origin_class: SessionOriginClass,
    pub status: SessionHandleStatus,
    pub issued_at_tick: u64,
}

impl SessionHandleRecord {
    pub fn new(
        session_handle: SessionHandle,
        session_scope_id: u64,
        origin_class: SessionOriginClass,
        issued_at_tick: u64,
    ) -> Self {
        Self {
            session_handle,
            session_scope_id,
            origin_class,
            status: SessionHandleStatus::Open,
            issued_at_tick,
        }
    }

    pub fn with_status(mut self, status: SessionHandleStatus) -> Self {
        self.status = status;
        self
    }
}

impl fmt::Debug for SessionHandle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("SessionHandle")
            .field(&format_args!("<opaque:{}>", self.opaque_tag()))
            .finish()
    }
}
