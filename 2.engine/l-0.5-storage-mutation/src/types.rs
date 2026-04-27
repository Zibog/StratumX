use bitflags::bitflags;
use engine_core::ComponentTypeId;
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;

/// Idempotence classification for deferred writes.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize,
)]
pub enum IdempotenceClass {
    /// Operation is idempotent, can be safely reapplied.
    Idempotent,
    /// Operation is not idempotent, must be applied exactly once.
    NonIdempotent,
}

/// Family-level marking for mutation cohesion.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct FamilyTag(pub u16);

/// Region-level marking for spatial scope.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RegionTag(pub u32);

/// A deferred write operation on a component.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeferredWrite {
    /// Which component is being written.
    pub component: ComponentTypeId,
    /// Serialized bytes for the write.
    pub bytes: SmallVec<[u8; 32]>,
    /// Whether this write is idempotent.
    pub idempotence: IdempotenceClass,
}

/// Mutation buffer holding staged deferred writes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MutationBuffer {
    pub(crate) writes: SmallVec<[DeferredWrite; 16]>,
}

/// Changeset combining structural and write changes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChangeSet {
    /// Structural changes (component additions/removals).
    pub structural: SmallVec<[ComponentTypeId; 8]>,
    /// Deferred writes to apply.
    pub writes: SmallVec<[DeferredWrite; 16]>,
}

/// Flags controlling apply behavior.
bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub struct ApplyFlags: u8 {
        /// Split application across multiple segments.
        const SEGMENTED = 0b0001;
        /// Allow tombstone compaction during apply.
        const ALLOW_TOMBSTONE_COMPACTION = 0b0010;
    }
}

/// Complete mutation payload ready for application.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ApplyPayload {
    /// Family-level cohesion tag.
    pub family_tag: FamilyTag,
    /// Region scope tag.
    pub region_tag: RegionTag,
    /// Ordering within batch.
    pub batch_order: u64,
    /// Application flags.
    pub flags: ApplyFlags,
    /// The changeset to apply.
    pub change_set: ChangeSet,
}
