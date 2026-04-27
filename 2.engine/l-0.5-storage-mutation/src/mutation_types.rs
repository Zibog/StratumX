use bitflags::bitflags;
use engine_core::ComponentTypeId;
use smallvec::SmallVec;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IdempotenceClass {
    Idempotent,
    NonIdempotent,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FamilyTag(pub u16);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RegionTag(pub u32);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeferredWrite {
    pub component: ComponentTypeId,
    pub bytes: SmallVec<[u8; 32]>,
    pub idempotence: IdempotenceClass,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MutationBuffer {
    pub(crate) writes: SmallVec<[DeferredWrite; 16]>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChangeSet {
    pub structural: SmallVec<[ComponentTypeId; 8]>,
    pub writes: SmallVec<[DeferredWrite; 16]>,
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct ApplyFlags: u8 {
        const SEGMENTED = 0b0001;
        const ALLOW_TOMBSTONE_COMPACTION = 0b0010;
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApplyPayload {
    pub family_tag: FamilyTag,
    pub region_tag: RegionTag,
    pub batch_order: u64,
    pub flags: ApplyFlags,
    pub change_set: ChangeSet,
}
