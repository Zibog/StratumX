use bitflags::bitflags;
use engine_core::ComponentTypeId;
use engine_storage_access::ScratchClass;
use smallvec::SmallVec;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QueryLocality {
    Cache,
    Spatial,
    Partition,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Partitionability {
    None,
    Region,
    Chunk,
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct QueryAccessMode: u8 {
        const READ = 0b0001;
        const WRITE = 0b0010;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AccessDescriptor {
    pub mode: QueryAccessMode,
    pub publication_rights: bool,
    pub scratch: Option<ScratchClass>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FilterOp {
    With,
    Without,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FilterConstraint {
    pub op: FilterOp,
    pub component: ComponentTypeId,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JoinRule {
    pub required: SmallVec<[ComponentTypeId; 4]>,
}
