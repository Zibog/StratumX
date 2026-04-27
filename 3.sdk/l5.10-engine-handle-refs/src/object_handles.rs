//! Object handle types
//!
//! Cadence: created per engine object, owned by a session.
//! Delivery guarantee: ordered reliable — object must be created before use.

use crate::identity_refs::IdentityRef;
use crate::session_handles::SessionHandle;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::hash::{Hash, Hasher};

fn redacted_tag_object(raw: u64) -> String {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    raw.hash(&mut hasher);
    format!("{:016x}", hasher.finish())
}

/// Opaque object handle. Debug output is redacted.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct ObjectHandle(u64);

impl ObjectHandle {
    pub fn new(raw: u64) -> Self {
        Self(raw)
    }
    pub fn opaque_tag(&self) -> String {
        redacted_tag_object(self.0)
    }
    pub fn raw(&self) -> u64 {
        self.0
    }
}

/// A full object handle record with ownership and identity.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ObjectHandleRecord {
    pub object_handle: ObjectHandle,
    pub owner_session_handle: SessionHandle,
    pub identity_ref: IdentityRef,
}

impl ObjectHandleRecord {
    pub fn new(
        object_handle: ObjectHandle,
        owner_session_handle: SessionHandle,
        identity_ref: IdentityRef,
    ) -> Self {
        Self {
            object_handle,
            owner_session_handle,
            identity_ref,
        }
    }
}

impl fmt::Debug for ObjectHandle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("ObjectHandle")
            .field(&format_args!("<opaque:{}>", self.opaque_tag()))
            .finish()
    }
}
