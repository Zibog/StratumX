use crate::{ApplyFlags, ApplyPayload, ChangeSet, FamilyTag, MutationBuffer, RegionTag};
use engine_core::{EngineCoreError, EngineCoreResult};
use engine_storage_access::WriteWindow;
use smallvec::SmallVec;

pub fn queue_deferred_writes(
    window: &WriteWindow,
    buffer: MutationBuffer,
) -> EngineCoreResult<ChangeSet> {
    if !window.descriptor.staged_mutation_handoff {
        return Err(EngineCoreError::InvalidDescriptor(
            "deferred writes require staged handoff window",
        ));
    }
    Ok(buffer.into_change_set(SmallVec::new()))
}

pub fn make_apply_payload(
    family_tag: FamilyTag,
    region_tag: RegionTag,
    batch_order: u64,
    change_set: ChangeSet,
) -> EngineCoreResult<ApplyPayload> {
    if batch_order == 0 {
        return Err(EngineCoreError::InvalidDescriptor(
            "batch order must be non-zero",
        ));
    }
    Ok(ApplyPayload {
        family_tag,
        region_tag,
        batch_order,
        flags: ApplyFlags::SEGMENTED,
        change_set,
    })
}
