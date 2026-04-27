//! State reference types
//!
//! Cadence: created per state snapshot or selection.
//! Delivery guarantee: ordered reliable for snapshot, best-effort for ephemeral.

use crate::identity_refs::IdentityRef;
use crate::runtime_handles::RuntimeHandle;
use serde::{Deserialize, Serialize};

/// Class of state being referenced.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum StateClass {
    Snapshot,
    Selection,
    Layout,
    Diagnostics,
    Build,
}

/// Retention class for state references.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StateRetentionClass {
    Ephemeral,
    CursorScoped,
    SnapshotScoped,
}

/// A state reference with ownership and retention metadata.
///
/// Cadence: carried on artifact refs and state observations.
/// Delivery guarantee: depends on retention class.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StateRef {
    pub state_class: StateClass,
    pub owner: IdentityRef,
    pub runtime_handle: RuntimeHandle,
    pub snapshot_epoch: u64,
    pub fact_class_set: Vec<StateClass>,
    pub retention_class: StateRetentionClass,
}

impl StateRef {
    pub fn new(
        state_class: StateClass,
        owner: IdentityRef,
        runtime_handle: RuntimeHandle,
        snapshot_epoch: u64,
        retention_class: StateRetentionClass,
    ) -> Self {
        Self {
            state_class,
            owner,
            runtime_handle,
            snapshot_epoch,
            fact_class_set: Vec::new(),
            retention_class,
        }
    }

    pub fn with_fact_classes(mut self, facts: Vec<StateClass>) -> Self {
        self.fact_class_set = facts;
        self
    }
}
