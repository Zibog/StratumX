//! Artifact reference types
//!
//! Cadence: created per artifact (preview, validation, build, release, evidence).
//! Delivery guarantee: artifact refs are ordered reliable, content is best-effort.

use crate::runtime_handles::RuntimeHandle;
use crate::state_refs::StateRef;
use serde::{Deserialize, Serialize};

/// Kind of artifact being referenced.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum ArtifactKind {
    Preview,
    Validation,
    Build,
    Release,
    Evidence,
}

/// Retention policy for an artifact.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ArtifactRetentionPolicy {
    Ephemeral,
    SnapshotScoped,
    Persistent,
}

/// An artifact reference with source and retention metadata.
///
/// Cadence: produced on artifact observations.
/// Delivery guarantee: ordered reliable for the ref, best-effort for content.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArtifactRef {
    pub artifact_kind: ArtifactKind,
    pub source_runtime_handle: RuntimeHandle,
    pub source_state_ref: Option<StateRef>,
    pub content_digest: String,
    pub retention_policy: ArtifactRetentionPolicy,
}

impl ArtifactRef {
    pub fn new(
        artifact_kind: ArtifactKind,
        source_runtime_handle: RuntimeHandle,
        content_digest: String,
        retention_policy: ArtifactRetentionPolicy,
    ) -> Self {
        Self {
            artifact_kind,
            source_runtime_handle,
            source_state_ref: None,
            content_digest,
            retention_policy,
        }
    }

    pub fn with_state_ref(mut self, state: StateRef) -> Self {
        self.source_state_ref = Some(state);
        self
    }
}
