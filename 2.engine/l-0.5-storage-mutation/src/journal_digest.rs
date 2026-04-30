use engine_core::{ComponentTypeId, StableDigest64, StableDigestBuilder};

use crate::{ApplyPayload, DeferredWrite, IdempotenceClass};

pub(crate) fn raw_payload_journal_digest(payload: &ApplyPayload) -> StableDigest64 {
    let mut structural = payload.change_set.structural.to_vec();
    structural.sort_unstable_by_key(|component| component.0);

    let mut writes = payload.change_set.writes.to_vec();
    writes.sort_unstable_by(|left, right| {
        left.component
            .0
            .cmp(&right.component.0)
            .then_with(|| idempotence_code(left).cmp(&idempotence_code(right)))
            .then_with(|| left.bytes.as_slice().cmp(right.bytes.as_slice()))
    });

    compute_journal_digest(
        payload.batch_order,
        payload.family_tag.0,
        payload.region_tag.0,
        &structural,
        &writes,
    )
}

pub(crate) fn compute_journal_digest(
    batch_order: u64,
    family_tag: u16,
    region_tag: u32,
    structural: &[ComponentTypeId],
    writes: &[DeferredWrite],
) -> StableDigest64 {
    let mut builder = StableDigestBuilder::new();
    builder.write_bytes(b"engine_storage_mutation.journal.v1");
    builder.write_u64(batch_order);
    builder.write_u16(family_tag);
    builder.write_u32(region_tag);
    builder.write_u64(structural.len() as u64);
    for component in structural {
        builder.write_u64(component.0);
    }
    builder.write_u64(writes.len() as u64);
    for write in writes {
        builder.write_u64(write.component.0);
        builder.write_u8(idempotence_code(write));
        builder.write_bytes(&write.bytes);
    }
    builder.finish()
}

fn idempotence_code(write: &DeferredWrite) -> u8 {
    match write.idempotence {
        IdempotenceClass::Idempotent => 1,
        IdempotenceClass::NonIdempotent => 2,
    }
}
