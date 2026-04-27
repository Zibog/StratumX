//! Runtime handle types
//!
//! Cadence: created per runtime instance, owned by a session.
//! Delivery guarantee: ordered reliable — runtime must be established before use.

use crate::session_handles::SessionHandle;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::hash::{Hash, Hasher};

fn redacted_tag_runtime(raw: u64) -> String {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    raw.hash(&mut hasher);
    format!("{:016x}", hasher.finish())
}

/// Opaque runtime handle. Debug output is redacted.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct RuntimeHandle(u64);

impl RuntimeHandle {
    pub fn new(raw: u64) -> Self {
        Self(raw)
    }
    pub fn opaque_tag(&self) -> String {
        redacted_tag_runtime(self.0)
    }
    pub fn raw(&self) -> u64 {
        self.0
    }
}

/// A full runtime handle record with ownership.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeHandleRecord {
    pub runtime_handle: RuntimeHandle,
    pub owner_session_handle: SessionHandle,
    pub issued_at_tick: u64,
}

impl RuntimeHandleRecord {
    pub fn new(
        runtime_handle: RuntimeHandle,
        owner_session_handle: SessionHandle,
        issued_at_tick: u64,
    ) -> Self {
        Self {
            runtime_handle,
            owner_session_handle,
            issued_at_tick,
        }
    }
}

impl fmt::Debug for RuntimeHandle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("RuntimeHandle")
            .field(&format_args!("<opaque:{}>", self.opaque_tag()))
            .finish()
    }
}
