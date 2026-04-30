#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MutationBatchId(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ApplyTransactionId(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FamilyTag(pub u16);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RegionTag(pub u32);
