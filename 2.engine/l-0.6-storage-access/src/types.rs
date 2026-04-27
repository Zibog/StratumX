use bitflags::bitflags;
use engine_storage_layout::LocalityClass;
use serde::{Deserialize, Serialize};

/// Identifier for a traversal plan.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TraversalPlanId(pub u64);

/// Scratch memory classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ScratchClass {
    /// Borrowed temporary scratch space.
    Borrowed,
    /// Owned persistent scratch space.
    Owned,
}

// Access mode flags for read/write operations.
bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub struct AccessMode: u8 {
        /// Read-only access.
        const READ = 0b0001;
        /// Direct write access.
        const WRITE = 0b0010;
        /// Staged mutation handoff.
        const STAGED = 0b0100;
        /// Mixed read+staged mode.
        const MIXED = Self::READ.bits() | Self::STAGED.bits();
    }
}

/// Descriptor for storage access operations.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AccessDescriptor {
    /// Access mode flags.
    pub mode: AccessMode,
    /// Associated traversal plan.
    pub plan_id: TraversalPlanId,
    /// Memory locality class.
    pub locality: LocalityClass,
    /// Scratch memory classification.
    pub scratch: ScratchClass,
    /// Whether staged mutation handoff is enabled.
    pub staged_mutation_handoff: bool,
}
