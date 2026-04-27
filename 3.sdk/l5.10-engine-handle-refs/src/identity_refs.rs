//! Identity reference types
//!
//! Cadence: created per identity, carries visibility and status metadata.
//! Delivery guarantee: ordered reliable — identity must be established before use.

use crate::session_handles::SessionHandle;
use serde::{Deserialize, Serialize};

/// Class of identity being referenced.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum IdentityClass {
    Session,
    Object,
    Runtime,
}

/// Visibility scope for an identity reference.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum IdentityVisibilityScope {
    SessionLocal,
    RuntimeLocal,
    Public,
}

/// Status of an identity reference.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum IdentityRefStatus {
    Active,
    Stale,
    Revoked,
}

/// An identity reference with metadata.
///
/// Cadence: carried on object and state refs.
/// Delivery guarantee: ordered reliable.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IdentityRef {
    pub identity_class: IdentityClass,
    pub tag: String,
    pub external_name: Option<String>,
    pub visibility_scope: IdentityVisibilityScope,
    pub status: IdentityRefStatus,
    pub source_session_handle: Option<SessionHandle>,
}

impl IdentityRef {
    pub fn new(
        identity_class: IdentityClass,
        tag: String,
        visibility_scope: IdentityVisibilityScope,
        status: IdentityRefStatus,
    ) -> Self {
        Self {
            identity_class,
            tag,
            external_name: None,
            visibility_scope,
            status,
            source_session_handle: None,
        }
    }

    pub fn with_external_name(mut self, name: String) -> Self {
        self.external_name = Some(name);
        self
    }

    pub fn with_source_session(mut self, session: SessionHandle) -> Self {
        self.source_session_handle = Some(session);
        self
    }
}
