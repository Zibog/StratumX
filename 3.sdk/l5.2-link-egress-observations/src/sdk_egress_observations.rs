#![deny(unused_imports)]
#![deny(unused_variables)]
#![deny(dead_code)]

pub use editor_authoring_observations::*;
pub use vertical_slice_observations::*;

mod editor_authoring_observations;
mod vertical_slice_observations;

use engine_handle_refs::{RuntimeHandle, StateClass, StateRef};
use sdk_compat::CompatibilityProfile;

pub const MAX_BATCH_RECORDS: usize = 256;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ObservationBatchId(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObservationKind {
    PacketAccepted,
    ControlApplied,
    SnapshotPublished,
    ObjectRetired,
    ObjectRestored,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObservationRecord {
    pub cursor: u64,
    pub kind: ObservationKind,
    pub summary: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObservationBatch {
    pub observation_batch_id: ObservationBatchId,
    pub source_runtime_handle: RuntimeHandle,
    pub source_state_ref: StateRef,
    pub fact_class_set: Vec<StateClass>,
    pub publication_cursor: u64,
    pub emitted_at_tick: u64,
    pub profile: CompatibilityProfile,
    pub records: Vec<ObservationRecord>,
}

pub fn batch_after(batch: &ObservationBatch, cursor: u64, max_records: usize) -> ObservationBatch {
    let max_records = max_records.clamp(1, MAX_BATCH_RECORDS);
    let records = batch
        .records
        .iter()
        .filter(|record| record.cursor > cursor)
        .take(max_records)
        .cloned()
        .collect::<Vec<_>>();
    let publication_cursor = records.last().map(|record| record.cursor).unwrap_or(cursor);
    ObservationBatch {
        publication_cursor,
        records,
        ..batch.clone()
    }
}
