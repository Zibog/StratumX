use crate::journal_digest::compute_journal_digest;
use crate::MutationError;
use crate::{
    ApplyFlags, ApplyPayload, DeferredWrite, IdempotenceClass, MutationApplyMode,
    MutationConflictPolicy,
};
use engine_core::{ComponentTypeId, StableDigest64};
use smallvec::SmallVec;
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MutationApplyPlan {
    pub batch_order: u64,
    pub operation_count: usize,
    pub all_or_nothing: bool,
    pub(crate) structural: SmallVec<[ComponentTypeId; 8]>,
    pub(crate) writes: SmallVec<[DeferredWrite; 16]>,
    pub(crate) contains_non_idempotent_write: bool,
    journal_digest: StableDigest64,
}

impl MutationApplyPlan {
    pub fn from_payload(payload: &ApplyPayload) -> Result<Self, MutationError> {
        Self::from_payload_with_policy(
            payload,
            MutationConflictPolicy::Reject,
            MutationApplyMode::Normal,
        )
    }

    pub fn from_payload_with_policy(
        payload: &ApplyPayload,
        conflict_policy: MutationConflictPolicy,
        mode: MutationApplyMode,
    ) -> Result<Self, MutationError> {
        validate_payload_shape(payload)?;

        let structural = normalize_structural(payload, conflict_policy)?;
        let writes = normalize_writes(payload, conflict_policy, &structural)?;
        let contains_non_idempotent_write = writes
            .iter()
            .any(|write| write.idempotence == IdempotenceClass::NonIdempotent);

        if replay_guard_missing(payload, mode, contains_non_idempotent_write) {
            return Err(MutationError::ReplayGuardRequired);
        }

        let journal_digest = compute_journal_digest(
            payload.batch_order,
            payload.family_tag.0,
            payload.region_tag.0,
            &structural,
            &writes,
        );

        Ok(Self {
            batch_order: payload.batch_order,
            operation_count: structural.len() + writes.len(),
            all_or_nothing: true,
            structural,
            writes,
            contains_non_idempotent_write,
            journal_digest,
        })
    }

    pub fn journal_digest(&self) -> StableDigest64 {
        self.journal_digest
    }

    pub fn structural_components(&self) -> &[ComponentTypeId] {
        &self.structural
    }

    pub fn deferred_writes(&self) -> &[DeferredWrite] {
        &self.writes
    }
}

fn validate_payload_shape(payload: &ApplyPayload) -> Result<(), MutationError> {
    if payload.batch_order == 0 {
        return Err(MutationError::InvalidBatchOrder);
    }

    if payload.change_set.structural.is_empty() && payload.change_set.writes.is_empty() {
        return Err(MutationError::EmptyChangeset);
    }

    Ok(())
}

fn normalize_structural(
    payload: &ApplyPayload,
    conflict_policy: MutationConflictPolicy,
) -> Result<SmallVec<[ComponentTypeId; 8]>, MutationError> {
    let mut structural = SmallVec::new();
    let mut seen = BTreeSet::new();
    for component in &payload.change_set.structural {
        if seen.insert(*component) {
            structural.push(*component);
        } else if conflict_policy == MutationConflictPolicy::Reject {
            return Err(MutationError::DuplicateOperation);
        }
    }
    Ok(structural)
}

fn normalize_writes(
    payload: &ApplyPayload,
    conflict_policy: MutationConflictPolicy,
    structural: &[ComponentTypeId],
) -> Result<SmallVec<[DeferredWrite; 16]>, MutationError> {
    let structural_set: BTreeSet<_> = structural.iter().copied().collect();
    let mut writes = SmallVec::new();
    let mut write_indices = BTreeMap::new();

    for write in &payload.change_set.writes {
        if structural_set.contains(&write.component) {
            return Err(MutationError::ConflictingOperation);
        }
        normalize_write(write, conflict_policy, &mut writes, &mut write_indices)?;
    }

    Ok(writes)
}

fn normalize_write(
    write: &DeferredWrite,
    conflict_policy: MutationConflictPolicy,
    writes: &mut SmallVec<[DeferredWrite; 16]>,
    write_indices: &mut BTreeMap<ComponentTypeId, usize>,
) -> Result<(), MutationError> {
    if let Some(existing_index) = write_indices.get(&write.component).copied() {
        return resolve_duplicate_write(write, conflict_policy, writes, existing_index);
    }

    write_indices.insert(write.component, writes.len());
    writes.push(write.clone());
    Ok(())
}

fn resolve_duplicate_write(
    write: &DeferredWrite,
    conflict_policy: MutationConflictPolicy,
    writes: &mut SmallVec<[DeferredWrite; 16]>,
    existing_index: usize,
) -> Result<(), MutationError> {
    match conflict_policy {
        MutationConflictPolicy::Reject if write.idempotence == IdempotenceClass::Idempotent => {
            Err(MutationError::DuplicateOperation)
        }
        MutationConflictPolicy::Reject => Err(MutationError::ConflictingOperation),
        MutationConflictPolicy::LastWriteWins => {
            replace_idempotent_write(write, writes, existing_index)
        }
    }
}

fn replace_idempotent_write(
    write: &DeferredWrite,
    writes: &mut SmallVec<[DeferredWrite; 16]>,
    existing_index: usize,
) -> Result<(), MutationError> {
    let existing = &writes[existing_index];
    if existing.idempotence == IdempotenceClass::Idempotent
        && write.idempotence == IdempotenceClass::Idempotent
    {
        writes[existing_index] = write.clone();
        Ok(())
    } else {
        Err(MutationError::ConflictingOperation)
    }
}

fn replay_guard_missing(
    payload: &ApplyPayload,
    mode: MutationApplyMode,
    contains_non_idempotent_write: bool,
) -> bool {
    mode == MutationApplyMode::Replay
        && contains_non_idempotent_write
        && !payload.flags.contains(ApplyFlags::REPLAY_GUARDED)
}
